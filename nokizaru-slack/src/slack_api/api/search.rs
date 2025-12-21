use crate::slack_api::{client::ClientResult, SlackApi, SlackMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchMessagesResponse {
    messages: SearchMessagesMatches,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchMessagesMatches {
    matches: Vec<SlackMessage>,
}

impl SlackApi {
    pub async fn search_messages(
        &self,
        query: &str,
        count: &str,
        sort: &str,
    ) -> ClientResult<Vec<SlackMessage>> {
        let params = [
            ("query", query.to_string()),
            ("count", count.to_string()),
            ("sort", sort.to_string()),
        ];

        let response: SearchMessagesResponse =
            self.client.http_get("search.messages", &params).await?;

        Ok(response.messages.matches)
    }
}
