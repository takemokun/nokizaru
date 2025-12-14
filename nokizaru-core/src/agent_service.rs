use anyhow::Result;
use rig::{client::CompletionClient, completion::Prompt, providers::openai};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct TestResponse {
    #[schemars(description = "Input message category. only 'Question' or 'NonQuestion'")]
    pub category: String,
    #[schemars(description = "Is the input message a question?")]
    pub is_question: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct SearchQuery {
    #[schemars(description = "List of search queries. maximum 3 queries. minimum 1 query.")]
    pub queries: Vec<String>,
}

#[derive(Debug)]
pub enum MessageCategory {
    Question,
    NonQuestion,
    Unknown,
}

pub struct ReflectionResult {
    pub category: MessageCategory,
}

pub struct AgentService;

impl AgentService {
    pub async fn test(&self, input: &str) -> Result<String> {
        println!("Input: {}", input);

        let reflection_result = self.reflection(input).await?;

        if !matches!(reflection_result.category, MessageCategory::Question) {
            println!(
                "No further action for category: {:?}",
                reflection_result.category
            );
            return Err(anyhow::anyhow!(
                "Input is not a question. Category: {:?}",
                reflection_result.category
            ));
        }

        let rewritten_query = self.query_rewriting(input).await?;
        println!("Final rewritten queries: {:?}", rewritten_query.queries);

        // let contexts = MessageContextService::new()
        //     .execute(rewritten_query.queries[0].as_str())
        //     .await
        //     .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        //
        // println!("Retrieved contexts: {:?}", contexts.len());

        let contexts = "Sample context for testing purposes.".to_string();

        let answer = self.answer(input, &contexts).await?;

        println!("Final answer: {}", answer);
        Ok(answer)
    }

    async fn answer(&self, input: &str, context: &str) -> Result<String> {
        println!("Answering with context length: {}", context.len());
        let openai = openai::Client::from_env();
        let agent = openai
            .agent(openai::GPT_4_1_MINI)
            .preamble(
                "You are a helpful assistant that answers questions based on provided context.",
            )
            .build();

        let prompt = format!("Context:\n{}\n\nQuestion:\n{}", context, input);
        let response = agent.prompt(&prompt).await?;
        println!("Agent response: {}", response);
        Ok(response)
    }

    async fn reflection(&self, input: &str) -> Result<ReflectionResult> {
        let openai = openai::Client::from_env();

        let extractor = openai
            .extractor::<TestResponse>(openai::GPT_4_1_MINI)
            .preamble("You are judge message is question or not.")
            .build();

        let response = extractor.extract(input).await;
        println!("Agent response: {:?}", response);

        let category = match response {
            Ok(res) if res.is_question => Some(MessageCategory::Question),
            Ok(_) => Some(MessageCategory::NonQuestion),
            Err(err) => {
                println!("Failed to extract structured response: {}", err);
                Some(MessageCategory::Unknown)
            }
        };

        Ok(ReflectionResult {
            category: category.unwrap_or(MessageCategory::Unknown),
        })
    }

    async fn query_rewriting(&self, input: &str) -> Result<SearchQuery> {
        let openai = openai::Client::from_env();

        let rewriter = openai
            .extractor::<SearchQuery>(openai::GPT_4_1_MINI)
            .preamble(
                "Extract only search-effective keywords from the user's message. \
                Remove question words (who, what, when, where, why, how, です, ですか), \
                particles (は, が, を, に, で, から, まで, etc.), \
                and sentence-ending expressions. \
                Output only nouns, names, and core search terms. \
                Maximum 3 keywords, minimum 1 keyword.\n\n\
                Examples:\n\
                Input: '課長はだれですか？' → Output: ['課長']\n\
                Input: '明日の会議の場所はどこですか？' → Output: ['明日', '会議', '場所']\n\
                Input: 'プロジェクトの進捗状況を教えて' → Output: ['プロジェクト', '進捗状況']",
            )
            .build();

        let rewritten_query = rewriter.extract(input).await?;
        println!("Rewritten query: {:?}", rewritten_query);
        Ok(rewritten_query)
    }
}
