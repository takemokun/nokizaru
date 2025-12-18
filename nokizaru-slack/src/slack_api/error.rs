use thiserror::Error;

/// Slack API レイヤーのエラー型
///
/// このエラー型はslack_api配下でのみ使用され、
/// infrastructure層で domain::SlackError に変換されます。
#[derive(Error, Debug)]
pub enum SlackError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Slack API error: {0}")]
    ApiError(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] serde_json::Error),
}
