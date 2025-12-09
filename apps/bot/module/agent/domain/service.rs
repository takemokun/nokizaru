use anyhow::{Context, Result};
use rig::completion::Prompt;
use rig::providers::openai;
use super::memory::AgentMemory;

pub struct AgentService {
    client: openai::Client,
    model: String,
    memory: AgentMemory,
}

impl AgentService {
    pub fn new() -> Result<Self> {
        let client = openai::Client::from_env();
        Ok(Self {
            client,
            model: "gpt-4-turbo-preview".to_string(),
            memory: AgentMemory::new(50), // 最大50メッセージ保持
        })
    }

    /// メモリを取得（可変参照）
    pub fn memory_mut(&mut self) -> &mut AgentMemory {
        &mut self.memory
    }

    /// コンテキストを考慮してプロンプト実行
    pub async fn process_text_with_context(&self, input: &str) -> Result<String> {
        let agent = self.client.agent(&self.model).build();

        // メモリからコンテキストを構築
        let context = self.memory.build_context_string();
        let enriched_prompt = if context.is_empty() {
            input.to_string()
        } else {
            format!(
                "{}\n=== User Question ===\n{}",
                context,
                input
            )
        };

        let response = agent
            .prompt(&enriched_prompt)
            .await
            .context("Failed to execute AI prompt")?;

        Ok(response)
    }
}
