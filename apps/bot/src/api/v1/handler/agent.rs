use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{api::v1::dto::{AgentRequest, AgentResponse}, AppContainer};

pub async fn handle_agent_request(
    State(container): State<Arc<AppContainer>>,
    Json(payload): Json<AgentRequest>,
) -> Response {
    match container.agent_usecase.execute_prompt(&payload.text).await {
        Ok(result) => {
            let response = AgentResponse { result };
            Json(response).into_response()
        }
        Err(e) => {
            tracing::error!("Agent processing failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Agent processing failed: {}", e),
            )
                .into_response()
        }
    }
}
