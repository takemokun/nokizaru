use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use utoipa;
use nokizaru_core::AppContainer;

use crate::api::v1::dto::{AgentRequest, AgentResponse, ErrorResponse};

const AGENT_TAG: &str = "Agent";

/// Process agent request
///
/// Sends a prompt to the agent and returns the processed result.
#[utoipa::path(
    post,
    path = "/api/v1/agent",
    request_body = AgentRequest,
    responses(
        (status = 200, description = "Agent processed successfully", body = AgentResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse),
    ),
    tag = AGENT_TAG,
)]
pub async fn handle_agent_request(
    State(container): State<Arc<AppContainer>>,
    Json(payload): Json<AgentRequest>,
) -> Response {
    match container.agent_usecase.execute_prompt_with_channel(&payload.channel_id, &payload.text).await {
        Ok(result) => {
            let response = AgentResponse { result };
            Json(response).into_response()
        }
        Err(e) => {
            tracing::error!("Agent processing failed: {}", e);
            let error_response = ErrorResponse::new(format!("Agent processing failed: {}", e));
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}
