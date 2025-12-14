use std::sync::Arc;

use crate::module::slack::{
    ExecuteCommandUsecase, ProcessEventUsecase, SlackApiClient, SlackCommandService,
     MessageContextService, EventService, AgentService,
};
use crate::shared::infrastructure::{AppConfig, DbPool};

/// DIコンテナ - アプリケーション全体の依存関係を管理
#[derive(Clone)]
pub struct AppContainer {
    // Slack Usecases
    pub process_event_usecase: Arc<ProcessEventUsecase>,
    pub execute_command_usecase: Arc<ExecuteCommandUsecase>,

    // Configuration
    pub config: Arc<AppConfig>,
}

impl AppContainer {
    pub fn new(config: AppConfig, _db_pool: DbPool) -> Self {
        // Infrastructure層
        let slack_client = Arc::new(SlackApiClient::new(config.slack.bot_token.clone()));

        // Domain Services
        let slack_command_service = Arc::new(SlackCommandService::new(slack_client.clone()));
        let slack_context_service = Arc::new(MessageContextService::new());
        let agent_service = Arc::new(AgentService);
        let slack_event_service = Arc::new(EventService::new(
            slack_context_service,
            agent_service,
            slack_client.clone(),
        ));

        // Application Usecases
        let process_event_usecase = Arc::new(ProcessEventUsecase::new(slack_event_service));
        let execute_command_usecase = Arc::new(ExecuteCommandUsecase::new(slack_command_service));

        Self {
            process_event_usecase,
            execute_command_usecase,
            config: Arc::new(config),
        }
    }

    /// Signing Secret取得（ミドルウェア用）
    pub fn signing_secret(&self) -> &str {
        &self.config.slack.signing_secret
    }
}
