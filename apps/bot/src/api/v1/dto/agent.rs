use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Agent request payload
#[derive(Debug, Deserialize, ToSchema)]
pub struct AgentRequest {
    /// The Slack channel ID to load context from
    #[schema(example = "C01234ABC56")]
    pub channel_id: String,

    /// The prompt text to process
    #[schema(example = "What is the weather today?")]
    pub text: String,
}

/// Agent response payload
#[derive(Debug, Serialize, ToSchema)]
pub struct AgentResponse {
    /// The processed result from the agent
    #[schema(example = "I don't have access to real-time weather data...")]
    pub result: String,
}
