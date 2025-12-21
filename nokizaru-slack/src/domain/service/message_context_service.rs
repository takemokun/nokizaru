use crate::{
    slack_api::{MessageContext, SlackHistoryMessage, SlackMessage},
    SlackError,
};
use anyhow::Result;
use std::env;

pub struct MessageContextService {
    api: crate::slack_api::SlackApi,
}

impl MessageContextService {
    pub fn new() -> Self {
        let user_token = env::var("SLACK_USER_TOKEN").unwrap();
        let api = crate::slack_api::SlackApi::new(user_token);
        Self { api }
    }

    /// Format a single message for LLM consumption
    fn format_message(msg: &SlackMessage) -> String {
        let user = msg
            .username
            .as_deref()
            .or_else(|| msg.user.as_deref())
            .unwrap_or("unknown");

        format!("[{}] {}: {}", msg.ts, user, msg.text)
    }

    /// Format a single history message for LLM consumption
    fn format_history_message(msg: &SlackHistoryMessage) -> String {
        let user = msg.user.as_deref().unwrap_or("unknown");
        format!("[{}] {}: {}", msg.ts, user, msg.text)
    }

    /// Format contexts into a clear message sequence for LLM input
    fn format_for_llm(contexts: Vec<MessageContext>) -> String {
        let mut output = String::new();
        let mut seen_messages = std::collections::HashSet::new();

        for (idx, context) in contexts.iter().enumerate() {
            if idx > 0 {
                output.push_str("\n---\n");
            }

            // Get channel info from target message
            let channel_name = context
                .target_message
                .channel
                .as_ref()
                .and_then(|c| c.name.as_deref())
                .unwrap_or("unknown");

            output.push_str(&format!("#{}:\n", channel_name));

            // Collect all messages in chronological order
            let mut all_messages = Vec::new();

            // Before messages
            for msg in &context.before_messages {
                if seen_messages.insert(msg.ts.clone()) {
                    all_messages.push((msg.ts.as_str(), msg, false));
                }
            }

            // Target message - use a different formatting approach
            let target_ts = context.target_message.ts.as_str();
            seen_messages.insert(target_ts.to_string());
            let target_formatted = Self::format_message(&context.target_message);

            // After messages
            for msg in &context.after_messages {
                if seen_messages.insert(msg.ts.clone()) {
                    all_messages.push((msg.ts.as_str(), msg, false));
                }
            }

            // Sort by timestamp
            all_messages.sort_by(|a, b| a.0.cmp(b.0));

            // Format messages with target highlighted
            for (ts, msg, _) in all_messages {
                if ts == target_ts {
                    output.push_str(&format!(">>> {}\n", target_formatted));
                } else {
                    output.push_str(&format!("{}\n", Self::format_history_message(msg)));
                }
            }

            // Thread replies (if any)
            if !context.threads.is_empty() {
                output.push_str("\nThreads:\n");
                for thread in &context.threads {
                    for reply in &thread.replies {
                        if seen_messages.insert(reply.ts.clone()) {
                            output
                                .push_str(&format!("  {}\n", Self::format_history_message(reply)));
                        }
                    }
                }
            }
        }

        output
    }

    /// 統合検索: 関連度5件 + 新しい順5件 + 前後3件 + スレッド
    pub async fn search_with_full_context(
        &self,
        query: &str,
    ) -> Result<Vec<MessageContext>, SlackError> {
        println!("🔍 Step 1: Searching messages...");

        // 並列実行: 関連度順と新しい順を同時に検索
        let (relevance_results, recency_results) = tokio::join!(
            self.api.search_messages(query, "5", "score"), // 関連度順
            self.api.search_messages(query, "5", "timestamp"), // 新しい順
        );

        let relevance_msgs = relevance_results
            .map_err(|e| SlackError::ApiError(format!("Failed to search by relevance: {}", e)))?;
        let recency_msgs = recency_results
            .map_err(|e| SlackError::ApiError(format!("Failed to search by recency: {}", e)))?;

        let mut all_messages = relevance_msgs;
        all_messages.extend(recency_msgs);

        // 重複除去（ts でユニーク化）
        let mut seen = std::collections::HashSet::new();
        all_messages.retain(|msg| seen.insert(msg.ts.clone()));

        println!("   ✓ Found {} unique messages", all_messages.len());

        if all_messages.is_empty() {
            return Ok(vec![]);
        }

        // 検索結果のメッセージ一覧を表示
        println!("\n   📋 Search Results:");
        for (i, msg) in all_messages.iter().enumerate() {
            let user = msg
                .username
                .as_deref()
                .or_else(|| msg.user.as_deref())
                .unwrap_or("unknown");
            let text = &msg.text;
            let ts = &msg.ts;
            let channel = msg
                .channel
                .as_ref()
                .and_then(|c| c.name.as_deref())
                .or_else(|| msg.channel.as_ref().and_then(|c| c.id.as_deref()))
                .unwrap_or("unknown");

            // 文字境界を考慮して安全にテキストを切り詰める
            let truncated_text = if text.chars().count() > 100 {
                let truncated: String = text.chars().take(100).collect();
                format!("{}...", truncated)
            } else {
                text.to_string()
            };

            println!(
                "      [{}/{}] #{} [{}] {}: {}",
                i + 1,
                all_messages.len(),
                channel,
                ts,
                user,
                truncated_text
            );
        }

        println!("\n🔍 Step 2: Fetching context for each message...");

        let total_messages = all_messages.len();

        // タイムアウト付きで各メッセージのコンテキストを取得
        let mut contexts = Vec::new();
        for (i, msg) in all_messages.iter().enumerate() {
            let idx = i + 1;
            let channel_id = msg
                .channel
                .as_ref()
                .and_then(|c| c.id.as_deref())
                .ok_or_else(|| SlackError::ApiError("Missing channel ID".to_string()))?;
            let message_ts = &msg.ts;

            println!("   [{}/{}] Processing message...", idx, total_messages);

            // タイムアウト付きで前後のメッセージを取得
            let around_result = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                self.api.get_messages_around(channel_id, message_ts),
            )
            .await;

            let around = match around_result {
                Ok(Ok(data)) => data,
                Ok(Err(e)) => {
                    eprintln!(
                        "   [{}/{}] Error getting messages around: {}",
                        idx, total_messages, e
                    );
                    continue; // エラーの場合はスキップ
                }
                Err(_) => {
                    eprintln!(
                        "   [{}/{}] Timeout getting messages around",
                        idx, total_messages
                    );
                    continue; // タイムアウトの場合はスキップ
                }
            };

            // 前後のメッセージ全てのスレッドを取得
            let mut all_msgs = around.before.clone();
            all_msgs.extend(around.after.clone());

            let threads_result = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                self.api.get_threads_batch(channel_id, &all_msgs),
            )
            .await;

            let threads = match threads_result {
                Ok(Ok(data)) => data,
                Ok(Err(e)) => {
                    eprintln!(
                        "   [{}/{}] Error getting threads: {}",
                        idx, total_messages, e
                    );
                    Vec::new() // エラーの場合は空のベクター
                }
                Err(_) => {
                    eprintln!("   [{}/{}] Timeout getting threads", idx, total_messages);
                    Vec::new() // タイムアウトの場合は空のベクター
                }
            };

            println!("   [{}/{}] ✓ Completed", idx, total_messages);

            contexts.push(MessageContext {
                target_message: msg.clone(),
                before_messages: around.before,
                after_messages: around.after,
                threads,
            });
        }

        println!(
            "   ✓ Successfully fetched context for {} messages",
            contexts.len()
        );

        Ok(contexts)
    }

    pub async fn execute(&self, query: &str) -> Result<String, SlackError> {
        let contexts = self.search_with_full_context(query).await?;

        // Format contexts for LLM input
        let formatted = Self::format_for_llm(contexts);

        println!("\n📝 Formatted for LLM:\n{}", formatted);

        Ok(formatted)
    }
}
