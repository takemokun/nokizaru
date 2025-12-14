use crate::domain::{SlackCommand, SlackError, SlackMessageRepository};
use std::sync::Arc;

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
