use super::{SlackError, SlackHistoryMessage, SlackMessage};
use async_trait::async_trait;

/// Slackメッセージ送信のためのリポジトリインターフェース
#[async_trait]
pub trait SlackMessageRepository: Send + Sync {
    async fn send_message(&self, message: &SlackMessage) -> Result<(), SlackError>;

    async fn send_reply(&self, message: &SlackMessage, thread_ts: &str) -> Result<(), SlackError>;

    async fn fetch_channel_history(
        &self,
        channel_id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<SlackHistoryMessage>, SlackError>;
}
