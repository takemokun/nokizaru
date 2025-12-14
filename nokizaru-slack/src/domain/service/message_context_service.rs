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
        let mut seen_messages = std::collections::HashSet::new();

        for (idx, context) in contexts.iter().enumerate() {
            if idx > 0 {
                output.push_str("\n---\n");
            }

            // Get channel info from target message
            let channel_name = context
                .target_message
                .get("channel")
                .and_then(|c| c.get("name"))
                .and_then(|n| n.as_str())
                .unwrap_or("unknown");

            output.push_str(&format!("#{}:\n", channel_name));

            // Collect all messages in chronological order
            let mut all_messages = Vec::new();

            // Before messages
            for msg in &context.before_messages {
                if let Some(ts) = msg.get("ts").and_then(|t| t.as_str()) {
                    if seen_messages.insert(ts.to_string()) {
                        all_messages.push((ts, msg, false));
                    }
                }
            }

            // Target message
            if let Some(ts) = context.target_message.get("ts").and_then(|t| t.as_str()) {
                seen_messages.insert(ts.to_string());
                all_messages.push((ts, &context.target_message, true));
            }

            // After messages
            for msg in &context.after_messages {
                if let Some(ts) = msg.get("ts").and_then(|t| t.as_str()) {
                    if seen_messages.insert(ts.to_string()) {
                        all_messages.push((ts, msg, false));
                    }
                }
            }

            // Sort by timestamp
            all_messages.sort_by(|a, b| a.0.cmp(b.0));

            // Format messages
            for (_, msg, is_target) in all_messages {
                let formatted = Self::format_message(msg);
                if is_target {
                    output.push_str(&format!(">>> {}\n", formatted));
                } else {
                    output.push_str(&format!("{}\n", formatted));
                }
            }

            // Thread replies (if any)
            if !context.threads.is_empty() {
                output.push_str("\nThreads:\n");
                for thread in &context.threads {
                    for reply in &thread.replies {
                        if let Some(ts) = reply.get("ts").and_then(|t| t.as_str()) {
                            if seen_messages.insert(ts.to_string()) {
                                output.push_str(&format!("  {}\n", Self::format_message(reply)));
                            }
                        }
                    }
                }
            }
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
