use anyhow::{Context, Result};
use rig::completion::Prompt;
use rig::providers::openai;

pub struct AgentService {
    client: openai::Client,
    model: String,
}

impl AgentService {
    pub fn new() -> Result<Self> {
        let client = openai::Client::from_env();
        Ok(Self {
            client,
            model: "gpt-4.1-mini".to_string(),
        })
    }

    pub async fn process_text(&self, input: &str) -> Result<String> {
        let agent = self.client.agent(&self.model).build();

        let response = agent
            .prompt(input)
            .await
            .context("Failed to execute AI prompt")?;

        Ok(response)
    }
}
