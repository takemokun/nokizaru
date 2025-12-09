use serde::Deserialize;
use std::env;

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
