use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;

use crate::api::v1::{
    container::AppContainer,
    dto::{SlackCommandDto, SlackCommandResponseDto, SlackEventPayloadDto},
};
use crate::module::slack::{SlackCommand, SlackEvent};

/// Slackイベントハンドラー
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
            Ok(event) => match container.process_event_usecase.execute(event).await {
                Ok(_) => (StatusCode::OK, "Event processed").into_response(),
                Err(e) => {
                    tracing::error!("Event processing failed: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, "Event processing failed").into_response()
                }
            },
            Err(e) => {
                tracing::error!("Failed to parse event: {}", e);
                (StatusCode::BAD_REQUEST, "Invalid event payload").into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, "Invalid event payload").into_response()
    }
}

/// Slackコマンドハンドラー
pub async fn handle_slack_commands(
    State(container): State<Arc<AppContainer>>,
    axum::Form(dto): axum::Form<SlackCommandDto>,
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
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Command execution failed",
            )
                .into_response()
        }
    }
}

/// ヘルスチェックハンドラー
pub async fn handle_health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
