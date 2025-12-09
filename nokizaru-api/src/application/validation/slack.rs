use thiserror::Error;

#[derive(Error, Debug)]
pub enum SlackValidationError {
    #[error("Invalid Slack user ID format")]
    InvalidUserId,

    #[error("Invalid Slack channel ID format")]
    InvalidChannelId,

    #[error("Command text is too long (max: {max}, got: {actual})")]
    CommandTextTooLong { max: usize, actual: usize },

    #[error("Empty command text")]
    EmptyCommandText,
}

/// Slack User ID のバリデーション
pub fn validate_slack_user_id(user_id: &str) -> Result<(), SlackValidationError> {
    if user_id.is_empty() || !user_id.starts_with('U') {
        return Err(SlackValidationError::InvalidUserId);
    }
    Ok(())
}

/// Slack Channel ID のバリデーション
pub fn validate_slack_channel_id(channel_id: &str) -> Result<(), SlackValidationError> {
    if channel_id.is_empty() || !channel_id.starts_with('C') {
        return Err(SlackValidationError::InvalidChannelId);
    }
    Ok(())
}

/// コマンドテキストのバリデーション
pub fn validate_command_text(text: &str, max_length: usize) -> Result<(), SlackValidationError> {
    if text.is_empty() {
        return Err(SlackValidationError::EmptyCommandText);
    }

    if text.len() > max_length {
        return Err(SlackValidationError::CommandTextTooLong {
            max: max_length,
            actual: text.len(),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_user_id() {
        assert!(validate_slack_user_id("U1234567890").is_ok());
    }

    #[test]
    fn test_invalid_user_id() {
        assert!(validate_slack_user_id("").is_err());
        assert!(validate_slack_user_id("C1234567890").is_err());
    }

    #[test]
    fn test_valid_channel_id() {
        assert!(validate_slack_channel_id("C1234567890").is_ok());
    }

    #[test]
    fn test_invalid_channel_id() {
        assert!(validate_slack_channel_id("").is_err());
        assert!(validate_slack_channel_id("U1234567890").is_err());
    }

    #[test]
    fn test_command_text_validation() {
        assert!(validate_command_text("test", 100).is_ok());
        assert!(validate_command_text("", 100).is_err());
        assert!(validate_command_text("a".repeat(101).as_str(), 100).is_err());
    }
}
