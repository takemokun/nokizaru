use serde::{Deserialize, Serialize};

/// Slackイベントペイロード（API DTO）
#[derive(Debug, Deserialize)]
pub struct SlackEventPayloadDto {
    #[serde(rename = "type")]
    pub payload_type: String,
    pub challenge: Option<String>,
    pub event: Option<serde_json::Value>,
}

/// Slackコマンドリクエスト（API DTO）
#[derive(Debug, Deserialize)]
pub struct SlackCommandDto {
    pub command: String,
    pub text: String,
    pub user_id: String,
    pub channel_id: String,
    pub response_url: String,
    pub trigger_id: String,
}

/// Slackコマンドレスポンス
#[derive(Debug, Serialize)]
pub struct SlackCommandResponseDto {
    pub response_type: String,
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
