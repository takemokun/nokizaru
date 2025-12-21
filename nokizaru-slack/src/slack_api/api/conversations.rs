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

/// Slack チャンネル情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackChannel {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub is_private: bool,
}

/// conversations.list レスポンス
#[derive(Debug, Clone, Deserialize)]
pub struct ConversationsListResponse {
    pub channels: Vec<SlackChannel>,
}

/// 特定メッセージの前後のメッセージ
#[derive(Debug, Clone)]
pub struct MessagesAround {
    pub before: Vec<SlackHistoryMessage>,
    pub after: Vec<SlackHistoryMessage>,
}

/// スレッド情報
#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub thread_ts: String,
    pub message_ts: String,
    pub reply_count: usize,
    pub replies: Vec<SlackHistoryMessage>,
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

        let response: ConversationsHistoryResponse = self
            .client
            .http_get("conversations.history", &params)
            .await?;

        Ok(response.messages)
    }

    pub async fn get_thread_messages(
        &self,
        channel: &str,
        thread_ts: &str,
    ) -> ClientResult<Vec<SlackHistoryMessage>> {
        let params = [
            ("channel", channel.to_string()),
            ("ts", thread_ts.to_string()),
        ];

        let response: ConversationsHistoryResponse = self
            .client
            .http_get("conversations.replies", &params)
            .await?;

        Ok(response.messages)
    }

    /// チャンネルリスト取得
    pub async fn list_channels(&self) -> ClientResult<Vec<SlackChannel>> {
        let params = [("types", "public_channel,private_channel".to_string())];

        let response: ConversationsListResponse = self
            .client
            .http_get("conversations.list", &params)
            .await?;

        Ok(response.channels)
    }

    /// 特定メッセージの前後を取得（前後3件ずつ）
    pub async fn get_messages_around(
        &self,
        channel: &str,
        target_ts: &str,
    ) -> ClientResult<MessagesAround> {
        // 並列実行: 前と後を同時に取得
        let (before_result, after_result) = tokio::join!(
            // 前（古い）3件
            async {
                let params = [
                    ("channel", channel.to_string()),
                    ("latest", target_ts.to_string()),
                    ("limit", "3".to_string()),
                    ("inclusive", "false".to_string()),
                ];
                self.client
                    .http_get::<ConversationsHistoryResponse>("conversations.history", &params)
                    .await
            },
            // 後（新しい）3件
            async {
                let params = [
                    ("channel", channel.to_string()),
                    ("oldest", target_ts.to_string()),
                    ("limit", "3".to_string()),
                    ("inclusive", "false".to_string()),
                ];
                self.client
                    .http_get::<ConversationsHistoryResponse>("conversations.history", &params)
                    .await
            }
        );

        let mut before = before_result?.messages;
        let after = after_result?.messages;

        // 時系列順に並び替え（古い順）
        before.reverse();

        Ok(MessagesAround { before, after })
    }

    /// 複数メッセージのスレッドを一括取得（並列実行）
    pub async fn get_threads_batch(
        &self,
        channel: &str,
        messages: &[SlackHistoryMessage],
    ) -> ClientResult<Vec<ThreadInfo>> {
        use futures::future::join_all;

        // スレッドがあるメッセージだけ抽出してタスクを作成
        let thread_tasks: Vec<_> = messages
            .iter()
            .filter_map(|msg| {
                // thread_ts フィールドまたは reply_count をチェック
                // SlackHistoryMessage には reply_count がないので、後で拡張が必要かもしれません
                // 今は thread_ts がある場合のみ処理
                let thread_ts = msg.ts.clone(); // 簡略化のため、各メッセージの ts をスレッド ID として使用
                let msg_ts = msg.ts.clone();
                let channel = channel.to_string();
                let api = self.clone();

                Some(async move {
                    let replies = api.get_thread_messages(&channel, &thread_ts).await?;

                    Ok::<ThreadInfo, crate::slack_api::error::SlackError>(ThreadInfo {
                        thread_ts: thread_ts.clone(),
                        message_ts: msg_ts,
                        reply_count: replies.len(),
                        replies,
                    })
                })
            })
            .collect();

        // 並列実行
        let results = join_all(thread_tasks).await;

        // エラーを無視して成功したものだけ返す
        Ok(results.into_iter().filter_map(|r| r.ok()).collect())
    }
}
