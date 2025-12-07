use async_trait::async_trait;
use super::{SlackMessage, SlackError};

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
}
