use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlackError {
    #[error("Slack API error: {0}")]
    ApiError(String),

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
