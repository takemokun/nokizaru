use crate::api::v1::container::AppContainer;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use super::{
    handler::{docs_html, handle_health_check, handle_slack_commands, handle_slack_events},
    openapi::openapi_json,
};

/// API v1 router with OpenAPI documentation
pub fn create_router(container: Arc<AppContainer>) -> Router {
    let api_routes = Router::new()
        .route("/health", get(handle_health_check))
        .route("/slack/events", post(handle_slack_events))
        .route("/slack/commands", post(handle_slack_commands))
        .layer(TraceLayer::new_for_http())
        .with_state(container);

    // Create router with API documentation
    Router::new()
        .route("/docs", get(docs_html))
        .route("/api-docs/openapi.json", get(openapi_json))
        .nest("/api/v1", api_routes)
}
