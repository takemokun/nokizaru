use serde::{Deserialize, Serialize};
use crate::slack_api::{SlackApi, client::ClientResult};

/// reactions.add リクエスト
#[derive(Debug, Clone, Serialize)]
pub struct AddReactionRequest {
    pub channel: String,
    pub timestamp: String,
    pub name: String,
}

/// reactions.add レスポンス
#[derive(Debug, Clone, Deserialize)]
pub struct AddReactionResponse {
    pub ok: bool,
}

impl SlackApi {
    /// リアクション追加
    pub async fn add_reaction(
        &self,
        channel: &str,
        ts: &str,
        emoji: &str,
    ) -> ClientResult<AddReactionResponse> {
        let request = AddReactionRequest {
            channel: channel.to_string(),
            timestamp: ts.to_string(),
            name: emoji.to_string(),
        };

        self.client.http_post("reactions.add", &request).await
    }
}
