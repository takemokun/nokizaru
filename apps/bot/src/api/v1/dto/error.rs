use serde::Serialize;
use utoipa::ToSchema;

/// Standard API error response
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message
    #[schema(example = "Invalid request payload")]
    pub error: String,

    /// Optional error details
    #[schema(example = "Field 'text' is required")]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: None,
        }
    }

    pub fn with_details(error: impl Into<String>, details: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            details: Some(details.into()),
        }
    }
}
