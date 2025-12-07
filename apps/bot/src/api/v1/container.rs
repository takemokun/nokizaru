use std::sync::Arc;

use crate::module::agent::AgentUsecase;
use crate::module::slack::{
    ExecuteCommandUsecase, ProcessEventUsecase, SlackApiClient, SlackCommandService,
    SlackEventService,
};
use crate::module::user::{DieselUserRepository, UserService};
use crate::shared::infrastructure::{AppConfig, DbPool};

/// DIコンテナ - アプリケーション全体の依存関係を管理
#[derive(Clone)]
pub struct AppContainer {
    // Slack Usecases
    pub process_event_usecase: Arc<ProcessEventUsecase>,
    pub execute_command_usecase: Arc<ExecuteCommandUsecase>,

    // User Usecases (将来の拡張用)
    pub user_service: Arc<UserService>,

    pub agent_usecase: Arc<AgentUsecase>,

    // Configuration
    pub config: Arc<AppConfig>,
}

impl AppContainer {
    /// DIコンテナを構築
    pub fn new(config: AppConfig, db_pool: DbPool) -> Self {
        // Infrastructure層
        let slack_client = Arc::new(SlackApiClient::new(config.slack.bot_token.clone()));
        let user_repository = Arc::new(DieselUserRepository::new(db_pool));

        // Domain Services
        let slack_event_service = Arc::new(SlackEventService::new(slack_client.clone()));
        let slack_command_service = Arc::new(SlackCommandService::new(slack_client.clone()));
        let user_service = Arc::new(UserService::new(user_repository));

        // Application Usecases
        let process_event_usecase = Arc::new(ProcessEventUsecase::new(slack_event_service));
        let execute_command_usecase = Arc::new(ExecuteCommandUsecase::new(slack_command_service));
        let agent_usecase = Arc::new(
            AgentUsecase::new()
                .expect("Failed to initialize AgentUsecase - check OPENAI_API_KEY environment variable")
        );

        Self {
            process_event_usecase,
            execute_command_usecase,
            user_service,
            agent_usecase,
            config: Arc::new(config),
        }
    }

    /// Signing Secret取得（ミドルウェア用）
    pub fn signing_secret(&self) -> &str {
        &self.config.slack.signing_secret
    }
}
