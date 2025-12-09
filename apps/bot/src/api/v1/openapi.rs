use axum::{response::IntoResponse, Json};
use utoipa::OpenApi;

use super::dto::{
    AgentRequest, AgentResponse, ErrorResponse, SlackCommandDto, SlackCommandResponseDto,
    SlackEventPayloadDto,
};

/// API Documentation structure
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Nokizaru Bot API",
        version = "1.0.0",
        description = "API for Nokizaru Slack Bot with AI Agent capabilities",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
        )
    ),
    paths(
        crate::api::v1::handler::slack::handle_health_check,
        crate::api::v1::handler::agent::handle_agent_request,
        crate::api::v1::handler::slack::handle_slack_events,
        crate::api::v1::handler::slack::handle_slack_commands,
    ),
    components(
        schemas(
            AgentRequest,
            AgentResponse,
            SlackEventPayloadDto,
            SlackCommandDto,
            SlackCommandResponseDto,
            ErrorResponse,
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Agent", description = "AI Agent processing endpoints"),
        (name = "Slack", description = "Slack integration endpoints"),
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server"),
    )
)]
pub struct ApiDoc;

/// OpenAPI JSON endpoint handler
pub async fn openapi_json() -> impl IntoResponse {
    Json(ApiDoc::openapi())
}
