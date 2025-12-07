use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::domain::{SlackMessage, SlackMessageRepository, SlackError};

/// Slack API クライアント実装
pub struct SlackApiClient {
    http_client: Client,
    bot_token: String,
}

impl SlackApiClient {
    pub fn new(bot_token: String) -> Self {
        Self {
            http_client: Client::new(),
            bot_token,
        }
    }
}

#[async_trait]
impl SlackMessageRepository for SlackApiClient {
    async fn send_message(&self, message: &SlackMessage) -> Result<(), SlackError> {
        let url = "https://slack.com/api/chat.postMessage";

        let payload = json!({
            "channel": message.channel_id,
            "text": message.text,
            "thread_ts": message.thread_ts,
        });

        let response = self.http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| SlackError::MessageSendFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SlackError::MessageSendFailed(
                format!("HTTP {}", response.status())
            ));
        }

        let result: serde_json::Value = response.json().await
            .map_err(|e| SlackError::MessageSendFailed(e.to_string()))?;

        if !result.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            let error = result.get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");
            return Err(SlackError::MessageSendFailed(error.to_string()));
        }

        Ok(())
    }

    async fn send_reply(
        &self,
        message: &SlackMessage,
        thread_ts: &str,
    ) -> Result<(), SlackError> {
        let mut reply = message.clone();
        reply.thread_ts = Some(thread_ts.to_string());
        self.send_message(&reply).await
    }

    async fn update_message(
        &self,
        channel_id: &str,
        timestamp: &str,
        new_text: &str,
    ) -> Result<(), SlackError> {
        let url = "https://slack.com/api/chat.update";

        let payload = json!({
            "channel": channel_id,
            "ts": timestamp,
            "text": new_text,
        });

        let response = self.http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| SlackError::MessageSendFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SlackError::MessageSendFailed(
                format!("HTTP {}", response.status())
            ));
        }

        Ok(())
    }
}
