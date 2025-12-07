use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found: {0}")]
    NotFound(Uuid),

    #[error("User not found by Slack ID: {0}")]
    NotFoundBySlackId(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("User already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid user data: {0}")]
    InvalidData(String),
}
