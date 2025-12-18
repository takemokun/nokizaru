use serde::{Deserialize, Serialize};

use crate::slack_api::{SlackApi, client::{ClientResult}};

/// conversations.history レスポンス
#[derive(Debug, Clone, Deserialize)]
pub struct ConversationsHistoryResponse {
    pub messages: Vec<SlackHistoryMessage>,
    #[serde(default)]
    pub has_more: bool,
    #[serde(default)]
    pub response_metadata: Option<serde_json::Value>,
}

/// Slack チャンネル履歴メッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackHistoryMessage {
    /// メッセージタイプ
    #[serde(rename = "type")]
    pub msg_type: String,
    /// ユーザーID（オプション）
    #[serde(default)]
    pub user: Option<String>,
    /// Bot ID（オプション）
    #[serde(default)]
    pub bot_id: Option<String>,
    /// メッセージテキスト
    pub text: String,
    /// タイムスタンプ
    pub ts: String,
}

impl SlackApi {
    /// チャンネル履歴取得
    pub async fn get_channel_history(
        &self,
        channel: &str,
        limit: Option<u32>,
    ) -> ClientResult<Vec<SlackHistoryMessage>> {
        let params = [
            ("channel", channel.to_string()),
            ("limit", limit.unwrap_or(100).to_string()),
        ];

        // 型注釈により RS = ConversationsHistoryResponse と推論
        let response: ConversationsHistoryResponse = self
            .client
            .http_get("conversations.history", &params)
            .await?;

        // レスポンスからメッセージ配列を抽出して返す
        Ok(response.messages)
    }
}
