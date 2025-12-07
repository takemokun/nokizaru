use thiserror::Error;

/// 共通エラー型
#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Entity not found")]
    NotFound,

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
