use std::sync::Arc;

use crate::{
    MessageContextService, SlackApiClient, SlackError, SlackEvent, SlackMessage,
    SlackMessageRepository,
};
use nokizaru_core::{AgentService, MessageCategory};

pub struct EventService {
    context_service: Arc<MessageContextService>,
    agent_service: Arc<AgentService>,
    slack_api: Arc<crate::SlackApiClient>,
}

impl EventService {
    pub fn new(
        context_service: Arc<MessageContextService>,
        agent_service: Arc<AgentService>,
        slack_api: Arc<SlackApiClient>,
    ) -> Self {
        Self {
            context_service,
            agent_service,
            slack_api,
        }
    }

    pub async fn execute(&self, event: SlackEvent) -> Result<(), SlackError> {
        match event {
            SlackEvent::Message {
                channel,
                user,
                bot_id,
                text,
                ts,
                thread_ts,
            } => {
                self.handle_message(channel, user, bot_id, text, ts, thread_ts)
                    .await
            }
            SlackEvent::AppMention {
                channel,
                user,
                text,
                ts,
            } => self.handle_app_mention(channel, user, text, ts).await,
        }
    }

    async fn handle_message(
        &self,
        channel: String,
        user: Option<String>,
        bot_id: Option<String>,
        text: String,
        _ts: String,
        _thread_ts: Option<String>,
    ) -> Result<(), SlackError> {
        // ボット自身のメッセージは無視（無限ループ防止）
        if bot_id.is_some() {
            tracing::debug!("Ignoring bot message from bot_id: {:?}", bot_id);
            return Ok(());
        }

        // user が None の場合もスキップ
        let user_id = match user {
            Some(id) => id,
            None => {
                tracing::warn!("Message has no user field, skipping");
                return Ok(());
            }
        };

        tracing::info!(
            "Processing message from user {} in channel {}",
            user_id,
            channel
        );

        tracing::info!("Starting agent test for input: {}", text);

        let reflection_result =
            self.agent_service.reflection(&text).await.map_err(|e| {
                SlackError::ApiError(format!("Reflection failed: {}", e.to_string()))
            })?;

        if !matches!(reflection_result.category, MessageCategory::Question) {
            println!(
                "No further action for category: {:?}",
                reflection_result.category
            );
            return Ok(());
        }

        let search_query = self
            .agent_service
            .query_rewriting(&text)
            .await
            .map_err(|e| {
                SlackError::ApiError(format!("Query rewriting failed: {}", e.to_string()))
            })?;

        println!("Final rewritten queries: {:?}", search_query.queries);

        let contexts = self.context_service
            .execute(search_query.queries[0].as_str())
            .await?;

        println!("Retrieved contexts: {:?}", contexts.len());

        let result = self.agent_service.answer(&text, &contexts).await;
        match result {
            Ok(answer) => {
                self.slack_api
                    .send_message(&SlackMessage {
                        channel_id: channel,
                        user_id: user_id,
                        text: answer,
                        timestamp: String::new(),
                        thread_ts: None,
                    })
                    .await?;
            }
            Err(e) => {
                let error_text = format!("❌ Agent processing failed: {}", e);
                tracing::error!("{}", error_text);
            }
        };

        Ok(())
    }

    async fn handle_app_mention(
        &self,
        channel: String,
        user: String,
        text: String,
        _ts: String,
    ) -> Result<(), SlackError> {
        tracing::info!(
            "Processing app mention from user {} in channel {}. text: {}",
            user,
            channel,
            text
        );

        // なんか

        Ok(())
    }
}
