use std::{env, sync::Arc};

use nokizaru_slack::{
    EventService, ExecuteCommandUsecase, MessageContextService, ProcessEventUsecase,
    SlackApiClient, SlackCommandService,
};

use nokizaru_core::AgentService;
use serde::Deserialize;

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
    pub fn new(config: AppConfig) -> Self {
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

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub slack: SlackConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SlackConfig {
    pub bot_token: String,
    pub signing_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .map_err(|_| ConfigError::InvalidPort)?,
            },
            slack: SlackConfig {
                bot_token: env::var("SLACK_BOT_TOKEN")
                    .map_err(|_| ConfigError::MissingEnvVar("SLACK_BOT_TOKEN".to_string()))?,
                signing_secret: env::var("SLACK_SIGNING_SECRET")
                    .map_err(|_| ConfigError::MissingEnvVar("SLACK_SIGNING_SECRET".to_string()))?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .map_err(|_| ConfigError::MissingEnvVar("DATABASE_URL".to_string()))?,
            },
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid port number")]
    InvalidPort,
}
