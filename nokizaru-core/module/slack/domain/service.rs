use super::{SlackCommand, SlackError, SlackEvent, SlackMessage, SlackMessageRepository};
use contract::TextProcessorContract;
use std::sync::Arc;

/// Slackイベント処理のドメインサービス
pub struct SlackEventService {
    message_repository: Arc<dyn SlackMessageRepository>,
}

impl SlackEventService {
    pub fn new(message_repository: Arc<dyn SlackMessageRepository>) -> Self {
        Self { message_repository }
    }

    pub async fn process_event(
        &self,
        event: SlackEvent,
        text_processor: Arc<dyn TextProcessorContract>,
    ) -> Result<(), SlackError> {
        match event {
            SlackEvent::Message {
                channel,
                user,
                text,
                ts,
                thread_ts,
            } => {
                self.handle_message(channel, user, text, ts, thread_ts)
                    .await
            }
            SlackEvent::AppMention {
                channel,
                user,
                text,
                ts,
            } => {
                self.handle_app_mention(channel, user, text, ts, text_processor)
                    .await
            }
        }
    }

    async fn handle_message(
        &self,
        channel: String,
        user: String,
        _text: String,
        _ts: String,
        _thread_ts: Option<String>,
    ) -> Result<(), SlackError> {
        tracing::info!(
            "Processing message from user {} in channel {}",
            user,
            channel
        );
        // メッセージ処理ロジック
        Ok(())
    }

    async fn handle_app_mention(
        &self,
        channel: String,
        user: String,
        text: String,
        ts: String,
        text_processor: Arc<dyn TextProcessorContract>,
    ) -> Result<(), SlackError> {
        tracing::info!(
            "Processing app mention from user {} in channel {}",
            user,
            channel
        );

        // TextProcessorを使ってチャンネルコンテキスト付きで応答を生成
        let ai_response = text_processor
            .process_with_channel(&channel, &text)
            .await
            .map_err(|e| SlackError::MessageSendFailed(format!("Text processing failed: {}", e)))?;

        // メンションへの返信
        let reply = SlackMessage {
            channel_id: channel,
            user_id: user,
            text: ai_response,
            timestamp: ts.clone(),
            thread_ts: Some(ts),
        };

        self.message_repository.send_message(&reply).await?;

        Ok(())
    }
}

/// Slackコマンド処理のドメインサービス
pub struct SlackCommandService {
    _message_repository: Arc<dyn SlackMessageRepository>,
}

impl SlackCommandService {
    pub fn new(message_repository: Arc<dyn SlackMessageRepository>) -> Self {
        Self {
            _message_repository: message_repository,
        }
    }

    pub async fn execute_command(&self, command: SlackCommand) -> Result<String, SlackError> {
        match command.command.as_str() {
            "/hello" => Ok(format!("こんにちは、<@{}>さん！", command.user_id)),
            "/help" => Ok(self.get_help_text()),
            _ => Err(SlackError::CommandExecutionFailed(format!(
                "Unknown command: {}",
                command.command
            ))),
        }
    }

    fn get_help_text(&self) -> String {
        r#"
利用可能なコマンド:
• /hello - 挨拶を返します
• /help - このヘルプメッセージを表示します
        "#
        .to_string()
    }
}
