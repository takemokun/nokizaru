use serde::{Deserialize, Serialize};

use crate::slack_api::{SlackApi, client::ClientResult};

/// chat.postMessage リクエスト
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostMessageRequest {
    pub channel_id: String,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ts: Option<String>,
}

/// chat.update リクエスト
#[derive(Debug, Clone, Serialize)]
pub struct UpdateMessageRequest {
    pub channel: String,
    pub text: String,
}

/// chat.postMessage レスポンス
#[derive(Debug, Clone, Deserialize)]
pub struct PostMessageResponse {
    pub ts: String,
    pub channel: String,
    #[serde(default)]
    pub message: Option<serde_json::Value>,
}

/// chat.update レスポンス
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateMessageResponse {
    pub channel: String,
    pub ts: String,
    pub text: String,
}

/// chat.delete リクエスト
#[derive(Debug, Clone, Serialize)]
pub struct DeleteMessageRequest {
    pub channel: String,
    pub ts: String,
}

/// chat.delete レスポンス
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteMessageResponse {
    pub ok: bool,
    pub channel: String,
    pub ts: String,
}

impl SlackApi {
    /// メッセージ送信
    pub async fn post_message(
        &self,
        request: &PostMessageRequest,
    ) -> ClientResult<PostMessageResponse> {
        // 型指定により RS = PostMessageResponse と推論される
        self.client.http_post("chat.postMessage", request).await
    }

    /// メッセージを更新
    pub async fn update_message(
        &self,
        channel: &str,
        text: &str,
    ) -> ClientResult<UpdateMessageResponse> {
        let request = UpdateMessageRequest {
            channel: channel.to_string(),
            text: text.to_string(),
        };

        self.client.http_post("chat.update", &request).await
    }

    /// メッセージを削除
    pub async fn delete_message(
        &self,
        channel: &str,
        ts: &str,
    ) -> ClientResult<DeleteMessageResponse> {
        let request = DeleteMessageRequest {
            channel: channel.to_string(),
            ts: ts.to_string(),
        };

        self.client.http_post("chat.delete", &request).await
    }
}
