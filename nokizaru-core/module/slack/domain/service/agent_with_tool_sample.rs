use std::{error::Error as StdError, fmt};

use anyhow::Result;
use rig::{
    client::CompletionClient,
    completion::{Prompt, ToolDefinition},
    providers::openai,
    tool::Tool,
};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct TestResponse {
    #[schemars(description = "Input message category. only 'Question' or 'NonQuestion'")]
    pub category: String,
    #[schemars(description = "Is the input message a question?")]
    pub is_question: bool,
}

pub struct SubmitAnswerTool<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> SubmitAnswerTool<T> {
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct SubmitToolError(String);

impl fmt::Display for SubmitToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SubmitToolError: {}", self.0)
    }
}

impl StdError for SubmitToolError {}

impl<T> Tool for SubmitAnswerTool<T>
where
    T: JsonSchema + for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static,
{
    const NAME: &'static str = "submit_answer";
    type Error = SubmitToolError;
    type Args = T;
    type Output = T;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description:
                "Submit your final structured annswer after gathering all necessary information."
                    .to_string(),
            parameters: json!(schema_for!(T)),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(args)
    }
}

pub struct AgentService;

impl AgentService {
    pub async fn test_sturcutured_output_by_tool(&self, input: &str) -> Result<()> {
        println!("Input: {}", input);
        let openai = openai::Client::from_env();
        let agent = openai
            .agent(openai::GPT_4_1_MINI)
            .preamble(
                "You are judge message is question or not. \n\
                ALWAYS submit your final answer using submit_answer tool. ",
            )
            .tool(SubmitAnswerTool::<TestResponse>::new())
            .build();

        let response = agent.prompt(input).await?;
        println!("Agent response: {:?}", response);
        Ok(())
    }
}
