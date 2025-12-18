use crate::slack_api::client::SlackHttpClient;

pub struct SlackApi {
    pub client: SlackHttpClient,
}

/// Slack API
///
/// bot token または user token を用いて初期化します。
/// bot token では search API など一部のAPIが利用できないため、用途に応じて使い分けてください。
///
/// ```
///  // Bot token or User token
///  let token = "xoxb-xxxxxxxxxx-xxxxxxxxxxxx-xxxxxxxxxxxxxx".to_string();
///  let client = SlackApi::new(token);
/// ```
impl SlackApi {
    /// 新しいSlack APIクライアントを作成
    pub fn new(token: String) -> Self {
        Self {
            client: SlackHttpClient::new(token),
        }
    }
}
