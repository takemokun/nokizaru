use crate::domain::{SlackEvent, SlackCommand, SlackEventService, SlackCommandService, SlackError};
use std::sync::Arc;

/// イベント処理ユースケース
pub struct ProcessEventUsecase {
    event_service: Arc<SlackEventService>,
}

impl ProcessEventUsecase {
    pub fn new(event_service: Arc<SlackEventService>) -> Self {
        Self { event_service }
    }

    pub async fn execute(&self, event: SlackEvent) -> Result<(), SlackError> {
        tracing::debug!("Executing process event usecase");
        self.event_service.process_event(event).await
    }
}

/// コマンド実行ユースケース
pub struct ExecuteCommandUsecase {
    command_service: Arc<SlackCommandService>,
}

impl ExecuteCommandUsecase {
    pub fn new(command_service: Arc<SlackCommandService>) -> Self {
        Self { command_service }
    }

    pub async fn execute(&self, command: SlackCommand) -> Result<String, SlackError> {
        tracing::debug!("Executing command usecase: {}", command.command);
        self.command_service.execute_command(command).await
    }
}
