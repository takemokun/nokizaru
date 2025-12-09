use async_trait::async_trait;
use super::{SlackMessage, SlackHistoryMessage, SlackError};

/// Slackメッセージ送信のためのリポジトリインターフェース
#[async_trait]
pub trait SlackMessageRepository: Send + Sync {
    async fn send_message(&self, message: &SlackMessage) -> Result<(), SlackError>;

    async fn send_reply(
        &self,
        message: &SlackMessage,
        thread_ts: &str,
    ) -> Result<(), SlackError>;

    async fn update_message(
        &self,
        channel_id: &str,
        timestamp: &str,
        new_text: &str,
    ) -> Result<(), SlackError>;

    async fn fetch_channel_history(
        &self,
        channel_id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<SlackHistoryMessage>, SlackError>;
}
