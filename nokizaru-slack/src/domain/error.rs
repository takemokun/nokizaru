use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlackError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Slack API error: {0}")]
    ApiError(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Message send failed: {0}")]
    MessageSendFailed(String),

    #[error("Event processing failed: {0}")]
    EventProcessingFailed(String),

    #[error("Command execution failed: {0}")]
    CommandExecutionFailed(String),

    #[error("Invalid event payload")]
    InvalidEventPayload,
}
