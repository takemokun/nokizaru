use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use crate::domain::{SlackError, SlackHistoryMessage, SlackMessage, SlackMessageRepository};
use contract::{SlackHistoryMessageInfo, SlackMessageContract, SlackMessageInfo};

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

        let response = self
            .http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| SlackError::MessageSendFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SlackError::MessageSendFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| SlackError::MessageSendFailed(e.to_string()))?;

        if !result.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");
            return Err(SlackError::MessageSendFailed(error.to_string()));
        }

        Ok(())
    }

    async fn send_reply(&self, message: &SlackMessage, thread_ts: &str) -> Result<(), SlackError> {
        let mut reply = message.clone();
        reply.thread_ts = Some(thread_ts.to_string());
        <Self as SlackMessageRepository>::send_message(self, &reply).await
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

        let response = self
            .http_client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .json(&payload)
            .send()
            .await
            .map_err(|e| SlackError::MessageSendFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(SlackError::MessageSendFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        Ok(())
    }

    async fn fetch_channel_history(
        &self,
        channel_id: &str,
        limit: Option<i32>,
    ) -> Result<Vec<SlackHistoryMessage>, SlackError> {
        let url = "https://slack.com/api/conversations.history";
        let limit_value = limit.unwrap_or(20).to_string();

        let response = self
            .http_client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.bot_token))
            .query(&[("channel", channel_id), ("limit", &limit_value)])
            .send()
            .await
            .map_err(|e| SlackError::ApiError(format!("Failed to fetch history: {}", e)))?;

        if !response.status().is_success() {
            return Err(SlackError::ApiError(format!(
                "HTTP {} when fetching history",
                response.status()
            )));
        }

        let result: serde_json::Value = response.json().await.map_err(|e| {
            SlackError::ApiError(format!("Failed to parse history response: {}", e))
        })?;

        if !result.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            let error = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error");
            return Err(SlackError::ApiError(format!(
                "Failed to fetch history: {}",
                error
            )));
        }

        let messages: Vec<SlackHistoryMessage> = result
            .get("messages")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
            .unwrap_or_default();

        Ok(messages)
    }
}

// SlackMessageContract の実装
#[async_trait]
impl SlackMessageContract for SlackApiClient {
    async fn send_message(&self, message: &SlackMessageInfo) -> anyhow::Result<()> {
        let domain_message = SlackMessage {
            channel_id: message.channel_id.clone(),
            user_id: message.user_id.clone(),
            text: message.text.clone(),
            timestamp: message.timestamp.clone(),
            thread_ts: message.thread_ts.clone(),
        };
        <Self as SlackMessageRepository>::send_message(self, &domain_message)
            .await
            .map_err(|e| anyhow::anyhow!("Slack send message failed: {}", e))
    }

    async fn send_reply(&self, message: &SlackMessageInfo, thread_ts: &str) -> anyhow::Result<()> {
        let domain_message = SlackMessage {
            channel_id: message.channel_id.clone(),
            user_id: message.user_id.clone(),
            text: message.text.clone(),
            timestamp: message.timestamp.clone(),
            thread_ts: message.thread_ts.clone(),
        };
        <Self as SlackMessageRepository>::send_reply(self, &domain_message, thread_ts)
            .await
            .map_err(|e| anyhow::anyhow!("Slack send reply failed: {}", e))
    }

    async fn update_message(
        &self,
        channel_id: &str,
        timestamp: &str,
        new_text: &str,
    ) -> anyhow::Result<()> {
        <Self as SlackMessageRepository>::update_message(self, channel_id, timestamp, new_text)
            .await
            .map_err(|e| anyhow::anyhow!("Slack update message failed: {}", e))
    }

    async fn fetch_channel_history(
        &self,
        channel_id: &str,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<SlackHistoryMessageInfo>> {
        let messages =
            <Self as SlackMessageRepository>::fetch_channel_history(self, channel_id, limit)
                .await
                .map_err(|e| anyhow::anyhow!("Slack fetch history failed: {}", e))?;

        Ok(messages
            .into_iter()
            .map(|m| SlackHistoryMessageInfo {
                user: m.user,
                bot_id: m.bot_id,
                text: m.text,
                ts: m.ts,
            })
            .collect())
    }
}
