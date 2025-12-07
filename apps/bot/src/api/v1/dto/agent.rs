use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AgentRequest {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct AgentResponse {
    pub result: String,
}
