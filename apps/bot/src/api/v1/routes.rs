use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use super::{
    container::AppContainer,
    handler::{handle_health_check, handle_slack_events, handle_slack_commands, handle_agent_request},
};

/// API v1 ルーター構築
pub fn create_router(container: Arc<AppContainer>) -> Router {
    Router::new()
        .route("/health", get(handle_health_check))
        .route("/slack/events", post(handle_slack_events))
        .route("/slack/commands", post(handle_slack_commands))
        .route("/agent", post(handle_agent_request))
        .layer(TraceLayer::new_for_http())
        .with_state(container)
}
