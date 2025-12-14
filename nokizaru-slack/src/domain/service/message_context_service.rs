use crate::SlackError;
use anyhow::Result;
use std::env;

pub struct MessageContextService {}

impl MessageContextService {
    pub fn new() -> Self {
        Self {}
    }

    /// Format a single message for LLM consumption
    fn format_message(msg: &serde_json::Value) -> String {
        let user = msg
            .get("username")
            .and_then(|u| u.as_str())
            .or_else(|| msg.get("user").and_then(|u| u.as_str()))
            .unwrap_or("unknown");
        
        let text = msg.get("text").and_then(|t| t.as_str()).unwrap_or("");
        
        let ts = msg.get("ts").and_then(|t| t.as_str()).unwrap_or("");
        
        format!("[{}] {}: {}", ts, user, text)
    }

    /// Format contexts into a clear message sequence for LLM input
    fn format_for_llm(contexts: Vec<crate::infrastructure::slack_api::MessageContext>) -> String {
        let mut output = String::new();
        
        for (idx, context) in contexts.iter().enumerate() {
            output.push_str(&format!("\n========== Context {} ==========\n", idx + 1));
            
            // Get channel info from target message
            let channel_name = context.target_message
                .get("channel")
                .and_then(|c| c.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("unknown");
            
            output.push_str(&format!("Channel: #{}\n\n", channel_name));
            
            // Before messages (chronologically ordered)
            if !context.before_messages.is_empty() {
                output.push_str("--- Messages before ---\n");
                for msg in &context.before_messages {
                    output.push_str(&format!("{}\n", Self::format_message(msg)));
                }
                output.push('\n');
            }
            
            // Target message (highlighted)
            output.push_str("--- TARGET MESSAGE ---\n");
            output.push_str(&format!(">>> {}\n\n", Self::format_message(&context.target_message)));
            
            // After messages
            if !context.after_messages.is_empty() {
                output.push_str("--- Messages after ---\n");
                for msg in &context.after_messages {
                    output.push_str(&format!("{}\n", Self::format_message(msg)));
                }
                output.push('\n');
            }
            
            // Thread replies
            if !context.threads.is_empty() {
                output.push_str("--- Related Threads ---\n");
                for thread in &context.threads {
                    output.push_str(&format!("Thread (ts: {}, {} replies):\n", 
                        thread.thread_ts, thread.reply_count));
                    
                    for reply in &thread.replies {
                        output.push_str(&format!("  {}\n", Self::format_message(reply)));
                    }
                    output.push('\n');
                }
            }
            
            output.push_str("========================================\n");
        }
        
        output
    }

    pub async fn execute(&self, query: &str) -> Result<String, SlackError> {
        let user_token = env::var("SLACK_USER_TOKEN").map_err(|e| {
            SlackError::ApiError(format!(
                "SLACK_USER_TOKEN environment variable not set: {}",
                e
            ))
        })?;

        let api = crate::infrastructure::slack_api::SlackApi::new(user_token);

        let contexts = api
            .search_with_full_context(query)
            .await
            .map_err(|e| SlackError::ApiError(format!("Failed to search context: {}", e)))?;

        // Format contexts for LLM input
        let formatted = Self::format_for_llm(contexts);
        
        println!("\n📝 Formatted for LLM:\n{}", formatted);

        Ok(formatted)
    }
}


