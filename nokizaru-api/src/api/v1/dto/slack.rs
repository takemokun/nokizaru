use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Slackイベントペイロード（API DTO）
#[derive(Debug, Deserialize, ToSchema)]
pub struct SlackEventPayloadDto {
    /// Event type (e.g., "url_verification", "event_callback")
    #[serde(rename = "type")]
    #[schema(example = "event_callback")]
    pub payload_type: String,

    /// Challenge string for URL verification
    #[schema(example = "3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P")]
    pub challenge: Option<String>,

    /// The actual event data
    pub event: Option<serde_json::Value>,
}

/// Slackコマンドリクエスト（API DTO）
#[derive(Debug, Deserialize, ToSchema)]
pub struct SlackCommandDto {
    /// The command name (e.g., "/ask")
    #[schema(example = "/ask")]
    pub command: String,

    /// The text following the command
    #[schema(example = "What is the status of project X?")]
    pub text: String,

    /// The Slack user ID who invoked the command
    #[schema(example = "U01234ABC56")]
    pub user_id: String,

    /// The channel ID where the command was invoked
    #[schema(example = "C01234ABC56")]
    pub channel_id: String,

    /// URL to send delayed responses to
    #[schema(example = "https://hooks.slack.com/commands/1234/5678")]
    pub response_url: String,

    /// Trigger ID for opening modals
    #[schema(example = "13345224609.738474920.8088930838d88f008e0")]
    pub trigger_id: String,
}

/// Slackコマンドレスポンス
#[derive(Debug, Serialize, ToSchema)]
pub struct SlackCommandResponseDto {
    /// Response type: "in_channel" or "ephemeral"
    #[schema(example = "in_channel")]
    pub response_type: String,

    /// The response text to display
    #[schema(example = "Command executed successfully")]
    pub text: String,
}

impl SlackCommandResponseDto {
    pub fn in_channel(text: String) -> Self {
        Self {
            response_type: "in_channel".to_string(),
            text,
        }
    }

    pub fn ephemeral(text: String) -> Self {
        Self {
            response_type: "ephemeral".to_string(),
            text,
        }
    }
}
