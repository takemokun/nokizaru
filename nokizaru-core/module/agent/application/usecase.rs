use crate::domain::service::AgentService;
use crate::domain::memory::MemoryMessage;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use contract::{SlackMessageContract, TextProcessorContract};

pub struct AgentUsecase {
    service: Arc<RwLock<AgentService>>,
    slack_repository: Option<Arc<dyn SlackMessageContract>>,
}

impl AgentUsecase {
    pub fn new() -> Result<Self> {
        Ok(Self {
            service: Arc::new(RwLock::new(AgentService::new()?)),
            slack_repository: None,
        })
    }

    /// Slackリポジトリを設定
    pub fn with_slack_repository(
        mut self,
        repository: Arc<dyn SlackMessageContract>,
    ) -> Self {
        self.slack_repository = Some(repository);
        self
    }

    /// チャンネル履歴をメモリに読み込む
    pub async fn load_channel_context(
        &self,
        channel_id: &str,
        limit: Option<i32>,
    ) -> Result<()> {
        if let Some(ref repo) = self.slack_repository {
            let history = repo.fetch_channel_history(channel_id, limit).await?;

            // Slack形式 → Memory形式に変換
            let memory_messages: Vec<MemoryMessage> = history
                .into_iter()
                .map(|msg| MemoryMessage {
                    user_id: msg.user.or(msg.bot_id),
                    text: msg.text,
                    timestamp: msg.ts,
                })
                .collect();

            let mut service = self.service.write().await;
            service.memory_mut().add_channel_messages(memory_messages);
        }
        Ok(())
    }

    /// プロンプト実行（チャンネルコンテキスト付き）
    pub async fn execute_prompt_with_channel(
        &self,
        channel_id: &str,
        text: &str,
    ) -> Result<String> {
        // チャンネル履歴を読み込む（最新20件）
        self.load_channel_context(channel_id, Some(20)).await?;

        let service = self.service.read().await;
        service.process_text_with_context(text).await
    }
}

/// TextProcessorContractトレイトの実装（Slackモジュールとの連携用）
#[async_trait]
impl TextProcessorContract for AgentUsecase {
    async fn process_with_channel(&self, channel_id: &str, text: &str) -> Result<String> {
        self.execute_prompt_with_channel(channel_id, text).await
    }
}
