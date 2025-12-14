use std::sync::Arc;

use crate::{AgentService, MessageContextService, SlackApiClient, SlackError, SlackEvent, SlackMessage, SlackMessageRepository};

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
            } => {
                self.handle_app_mention(channel, user, text, ts).await
            }
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
        
        

        // AgentService.test を実行
        tracing::info!("Starting agent test for input: {}", text);
        // match self.agent_service.test(&text).await {
        //     Ok(answer) => {
        //         // AIの回答をSlackに送信
        //         tracing::info!("Agent test completed successfully, sending answer to Slack");
        //         let message = SlackMessage {
        //             channel_id: channel,
        //             user_id: user_id,
        //             text: answer,
        //             timestamp: String::new(),
        //             thread_ts: None,
        //         };
        //         self.slack_api
        //             .send_message(&message)
        //             .await
        //             .map_err(|e| {
        //                 tracing::error!("Failed to send message to Slack: {}", e);
        //                 e
        //             })?;
        //         
        //         tracing::info!("Answer sent to Slack successfully");
        //         Ok(())
        //     }
        //     Err(e) => {
        //         // エラーメッセージをSlackに送信
        //         tracing::error!("Agent test failed with error: {}", e);
        //         let error_text = format!("❌ Agent processing failed: {}", e);
        //         let error_message = SlackMessage {
        //             channel_id: channel,
        //             user_id: user_id,
        //             text: error_text,
        //             timestamp: String::new(),
        //             thread_ts: None,
        //         };
        //         
        //         // エラーメッセージの送信を試みる
        //         if let Err(send_err) = self.slack_api.send_message(&error_message).await {
        //             tracing::error!("Failed to send error message to Slack: {}", send_err);
        //         }
        //         
        //         Err(SlackError::ApiError(format!("Agent processing failed: {}", e)))
        //     }
        // }
        //
        //
        let result = self.agent_service.test(&text).await;
        match result {
            Ok(answer) => {
                self.slack_api.send_message(&SlackMessage {
                    channel_id: channel,
                    user_id: user_id,
                    text: answer,
                    timestamp: String::new(),
                    thread_ts: None,
                }).await?;
            },
            Err(e) => {
                let error_text = format!("❌ Agent processing failed: {}", e);
                tracing::error!("{}", error_text);
            }
        };

        //
        // self.slack_api.send_message(&SlackMessage {
        //     channel_id: channel,
        //     user_id: user_id,
        //     text: result,
        //     timestamp: String::new(),
        //     thread_ts: None,
        // }).await?;
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

        // llm に text を渡して、routing を決定するロジック
        //
        let tmp_query = "課題";

        let context = self.context_service.execute(&tmp_query).await?;
        // slack の メッセージコンテキストを、ai 用のcontext に変換するロジック
        // context を一緒に ai にぶん投げる
        Ok(())
    }
}
