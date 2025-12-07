use crate::domain::service::AgentService;
use anyhow::Result;

pub struct AgentUsecase {
    service: AgentService,
}

impl AgentUsecase {
    pub fn new() -> Result<Self> {
        Ok(Self {
            service: AgentService::new()?,
        })
    }

    pub async fn execute_prompt(&self, text: &str) -> Result<String> {
        self.service.process_text(text).await
    }
}
