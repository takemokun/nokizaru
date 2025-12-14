use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Form, Json,
};
use std::sync::Arc;
use utoipa;
use nokizaru_core::AppContainer;
use nokizaru_core::module::slack::{SlackCommand, SlackEvent};

use crate::api::v1::dto::{ErrorResponse, SlackCommandDto, SlackCommandResponseDto, SlackEventPayloadDto};

const SLACK_TAG: &str = "Slack";

/// Handle Slack events
///
/// Processes incoming Slack events including URL verification challenges
/// and various event types.
#[utoipa::path(
    post,
    path = "/api/v1/slack/events",
    request_body = SlackEventPayloadDto,
    responses(
        (status = 200, description = "Event processed successfully", body = String,
         example = json!({"challenge": "3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P"})),
        (status = 400, description = "Invalid event payload", body = ErrorResponse),
        (status = 500, description = "Event processing failed", body = ErrorResponse),
    ),
    tag = SLACK_TAG,
)]
pub async fn handle_slack_events(
    State(container): State<Arc<AppContainer>>,
    Json(payload): Json<SlackEventPayloadDto>,
) -> Response {
    // URL検証チャレンジへの応答
    if payload.payload_type == "url_verification" {
        if let Some(challenge) = payload.challenge {
            return Json(serde_json::json!({ "challenge": challenge })).into_response();
        }
    }

    // イベント処理
    if let Some(event_value) = payload.event {
        match serde_json::from_value::<SlackEvent>(event_value) {
            Ok(event) => {
                // バックグラウンドで処理を実行（Slackに即座にレスポンスを返すため）
                let container_clone = Arc::clone(&container);
                tokio::spawn(async move {
                    match container_clone.process_event_usecase.execute(event).await {
                        Ok(_) => {
                            tracing::info!("✅ Event processed successfully in background");
                        }
                        Err(e) => {
                            tracing::error!("❌ Background event processing failed: {}", e);
                        }
                    }
                });

                // Slackに即座に200 OKを返す（3秒タイムアウトを防ぐ）
                (StatusCode::OK, "Event accepted").into_response()
            }
            Err(e) => {
                tracing::error!("Failed to parse event: {}", e);
                let error_response = ErrorResponse::new("Invalid event payload");
                (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
            }
        }
    } else {
        let error_response = ErrorResponse::new("Invalid event payload");
        (StatusCode::BAD_REQUEST, Json(error_response)).into_response()
    }
}

/// Handle Slack slash commands
///
/// Processes slash commands sent from Slack workspace.
#[utoipa::path(
    post,
    path = "/api/v1/slack/commands",
    request_body(content = SlackCommandDto, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200, description = "Command executed successfully", body = SlackCommandResponseDto),
        (status = 500, description = "Command execution failed", body = ErrorResponse),
    ),
    tag = SLACK_TAG,
)]
pub async fn handle_slack_commands(
    State(container): State<Arc<AppContainer>>,
    Form(dto): Form<SlackCommandDto>,
) -> Response {
    // DTOをドメインモデルに変換
    let command = SlackCommand {
        command: dto.command,
        text: dto.text,
        user_id: dto.user_id,
        channel_id: dto.channel_id,
        response_url: dto.response_url,
        trigger_id: dto.trigger_id,
    };

    match container.execute_command_usecase.execute(command).await {
        Ok(response_text) => {
            Json(SlackCommandResponseDto::in_channel(response_text)).into_response()
        }
        Err(e) => {
            tracing::error!("Command execution failed: {}", e);
            let error_response = ErrorResponse::new("Command execution failed");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

/// Health check endpoint
///
/// Returns server health status.
#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Server is healthy", body = String, example = json!("OK")),
    ),
    tag = "Health",
)]
pub async fn handle_health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
