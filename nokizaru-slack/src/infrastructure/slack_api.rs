use anyhow::Result;
use futures::future::join_all;
use reqwest::Client;
use serde_json::{json, Value};

#[derive(Debug, Clone)]
pub struct MessagesAround {
    before: Vec<Value>,
    after: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub thread_ts: String,
    pub message_ts: String,
    pub reply_count: usize,
    pub replies: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct MessageContext {
    pub target_message: Value,
    pub before_messages: Vec<Value>,
    pub after_messages: Vec<Value>,
    pub threads: Vec<ThreadInfo>,
}

pub struct SlackApi {
    client: Client,
    token: String,
}

impl SlackApi {
    pub fn new(token: String) -> Self {
        // タイムアウトを設定したクライアントを作成
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))  // 30秒タイムアウト
            .connect_timeout(std::time::Duration::from_secs(10))  // 接続タイムアウト10秒
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            token,
        }
    }

    /// メッセージ送信
    pub async fn post_message(&self, channel: &str, text: &str) -> Result<Value> {
        let res: Value = self
            .client
            .post("https://slack.com/api/chat.postMessage")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&json!({
                "channel": channel,
                "text": text
            }))
            .send()
            .await?
            .json()
            .await?;

        if res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!("✅ Message posted successfully");
        } else {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
        }

        Ok(res)
    }

    /// チャンネル履歴取得
    pub async fn fetch_history(&self, channel: &str, limit: u32) -> Result<Vec<Value>> {
        let res: Value = self
            .client
            .get("https://slack.com/api/conversations.history")
            .header("Authorization", format!("Bearer {}", self.token))
            .query(&[("channel", channel), ("limit", &limit.to_string())])
            .send()
            .await?
            .json()
            .await?;

        if !res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
            return Ok(vec![]);
        }

        Ok(res["messages"].as_array().unwrap_or(&vec![]).clone())
    }

    /// スレッド返信取得
    pub async fn fetch_thread_replies(&self, channel: &str, thread_ts: &str) -> Result<Vec<Value>> {
        let res: Value = self
            .client
            .get("https://slack.com/api/conversations.replies")
            .header("Authorization", format!("Bearer {}", self.token))
            .query(&[("channel", channel), ("ts", thread_ts)])
            .send()
            .await?
            .json()
            .await?;

        if !res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
            return Ok(vec![]);
        }

        Ok(res["messages"].as_array().unwrap_or(&vec![]).clone())
    }

    /// リアクション追加
    pub async fn add_reaction(&self, channel: &str, ts: &str, emoji: &str) -> Result<Value> {
        let res: Value = self
            .client
            .post("https://slack.com/api/reactions.add")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&json!({
                "channel": channel,
                "timestamp": ts,
                "name": emoji
            }))
            .send()
            .await?
            .json()
            .await?;

        if res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!("👍 Reaction '{}' added", emoji);
        } else {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
        }

        Ok(res)
    }

    /// ユーザーリスト取得
    pub async fn list_users(&self) -> Result<Vec<Value>> {
        let res: Value = self
            .client
            .get("https://slack.com/api/users.list")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?
            .json()
            .await?;

        if !res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
            return Ok(vec![]);
        }

        Ok(res["members"].as_array().unwrap_or(&vec![]).clone())
    }

    /// チャンネルリスト取得
    pub async fn list_channels(&self) -> Result<Vec<Value>> {
        let res: Value = self
            .client
            .get("https://slack.com/api/conversations.list")
            .header("Authorization", format!("Bearer {}", self.token))
            .query(&[("types", "public_channel,private_channel")])
            .send()
            .await?
            .json()
            .await?;

        if !res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
            return Ok(vec![]);
        }

        Ok(res["channels"].as_array().unwrap_or(&vec![]).clone())
    }

    /// メッセージ更新
    pub async fn update_message(&self, channel: &str, ts: &str, new_text: &str) -> Result<Value> {
        let res: Value = self
            .client
            .post("https://slack.com/api/chat.update")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&json!({
                "channel": channel,
                "ts": ts,
                "text": new_text
            }))
            .send()
            .await?
            .json()
            .await?;

        if res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!("✏️  Message updated");
        } else {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
        }

        Ok(res)
    }

    /// メッセージ削除
    pub async fn delete_message(&self, channel: &str, ts: &str) -> Result<Value> {
        let res: Value = self
            .client
            .post("https://slack.com/api/chat.delete")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&json!({
                "channel": channel,
                "ts": ts
            }))
            .send()
            .await?
            .json()
            .await?;

        if res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!("🗑️  Message deleted");
        } else {
            println!(
                "❌ Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
        }

        Ok(res)
    }

    /// メッセージ検索（関連度順 or 新しい順）
    pub async fn search_messages(&self, query: &str, sort: &str, count: u32) -> Result<Vec<Value>> {
        let res: Value = self
            .client
            .get("https://slack.com/api/search.messages")
            .header("Authorization", format!("Bearer {}", self.token))
            .query(&[
                ("query", query),
                ("count", &count.to_string()),
                ("sort", sort), // "score" or "timestamp"
            ])
            .send()
            .await?
            .json()
            .await?;

        if !res.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
            println!(
                "❌ Search Error: {}",
                res.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("unknown")
            );
            return Ok(vec![]);
        }

        Ok(res["messages"]["matches"]
            .as_array()
            .unwrap_or(&vec![])
            .clone())
    }

    /// 特定メッセージの前後を取得（前後3件ずつ）
    pub async fn get_messages_around(
        &self,
        channel: &str,
        target_ts: &str,
    ) -> Result<MessagesAround> {
        // 並列実行: 前と後を同時に取得
        let (before_result, after_result) = tokio::join!(
            // 前（古い）3件
            self.client
                .get("https://slack.com/api/conversations.history")
                .header("Authorization", format!("Bearer {}", &self.token))
                .query(&[
                    ("channel", channel),
                    ("latest", target_ts),
                    ("limit", "3"),
                    ("inclusive", "false"),
                ])
                .send(),
            // 後（新しい）3件
            self.client
                .get("https://slack.com/api/conversations.history")
                .header("Authorization", format!("Bearer {}", &self.token))
                .query(&[
                    ("channel", channel),
                    ("oldest", target_ts),
                    ("limit", "3"),
                    ("inclusive", "false"),
                ])
                .send()
        );

        let before_msgs: Value = before_result?.json().await?;
        let after_msgs: Value = after_result?.json().await?;

        let mut before = before_msgs["messages"]
            .as_array()
            .unwrap_or(&vec![])
            .clone();
        let after = after_msgs["messages"].as_array().unwrap_or(&vec![]).clone();

        // 時系列順に並び替え（古い順）
        before.reverse();

        Ok(MessagesAround { before, after })
    }

    /// 複数メッセージのスレッドを一括取得（並列実行）
    pub async fn get_threads_batch(
        &self,
        channel: &str,
        messages: &[Value],
    ) -> Result<Vec<ThreadInfo>> {
        // スレッドがあるメッセージだけ抽出
        let thread_tasks: Vec<_> = messages
            .iter()
            .filter_map(|msg| {
                // スレッドの一部 or スレッドの親
                let thread_ts = msg.get("thread_ts").and_then(|v| v.as_str()).or_else(|| {
                    // スレッドの親の場合
                    if msg.get("reply_count").and_then(|v| v.as_u64()).unwrap_or(0) > 0 {
                        msg.get("ts").and_then(|v| v.as_str())
                    } else {
                        None
                    }
                });

                thread_ts.map(|ts| {
                    let channel = channel.to_string();
                    let ts = ts.to_string();
                    let msg_ts = msg
                        .get("ts")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let client = self.client.clone();
                    let token = self.token.clone();

                    async move {
                        let replies_result: Value = client
                            .get("https://slack.com/api/conversations.replies")
                            .header("Authorization", format!("Bearer {}", token))
                            .query(&[("channel", &channel), ("ts", &ts)])
                            .send()
                            .await?
                            .json()
                            .await?;

                        let replies = replies_result["messages"]
                            .as_array()
                            .unwrap_or(&vec![])
                            .clone();

                        Ok::<ThreadInfo, anyhow::Error>(ThreadInfo {
                            thread_ts: ts,
                            message_ts: msg_ts,
                            reply_count: replies.len(),
                            replies,
                        })
                    }
                })
            })
            .collect();

        // 並列実行
        let results = join_all(thread_tasks).await;

        // エラーを無視して成功したものだけ返す
        Ok(results.into_iter().filter_map(|r| r.ok()).collect())
    }

    /// 統合検索: 関連度5件 + 新しい順5件 + 前後3件 + スレッド
    pub async fn search_with_full_context(&self, query: &str) -> Result<Vec<MessageContext>> {
        println!("🔍 Step 1: Searching messages...");

        // 並列実行: 関連度順と新しい順を同時に検索
        let (relevance_results, recency_results) = tokio::join!(
            self.search_messages(query, "score", 5),     // 関連度順
            self.search_messages(query, "timestamp", 5), // 新しい順
        );

        let mut all_messages = relevance_results?;
        all_messages.extend(recency_results?);

        // 重複除去（ts でユニーク化）
        let mut seen = std::collections::HashSet::new();
        all_messages.retain(|msg| {
            let ts = msg.get("ts").and_then(|v| v.as_str()).unwrap_or("");
            seen.insert(ts.to_string())
        });

        println!("   ✓ Found {} unique messages", all_messages.len());

        if all_messages.is_empty() {
            return Ok(vec![]);
        }

        // 検索結果のメッセージ一覧を表示
        println!("\n   📋 Search Results:");
        for (i, msg) in all_messages.iter().enumerate() {
            let user = msg
                .get("username")
                .and_then(|u| u.as_str())
                .or_else(|| msg.get("user").and_then(|u| u.as_str()))
                .unwrap_or("unknown");
            let text = msg.get("text").and_then(|t| t.as_str()).unwrap_or("");
            let ts = msg.get("ts").and_then(|t| t.as_str()).unwrap_or("");
            let channel = msg
                .get("channel")
                .and_then(|c| c.get("name"))
                .and_then(|n| n.as_str())
                .or_else(|| {
                    msg.get("channel")
                        .and_then(|c| c.get("id"))
                        .and_then(|i| i.as_str())
                })
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
            let channel_id = msg["channel"]["id"].as_str().unwrap_or("");
            let message_ts = msg["ts"].as_str().unwrap_or("");
            
            println!("   [{}/{}] Processing message...", idx, total_messages);
            
            // タイムアウト付きで前後のメッセージを取得
            let around_result = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                self.get_messages_around(channel_id, message_ts)
            ).await;
            
            let around = match around_result {
                Ok(Ok(data)) => data,
                Ok(Err(e)) => {
                    eprintln!("   [{}/{}] Error getting messages around: {}", idx, total_messages, e);
                    continue;  // エラーの場合はスキップ
                }
                Err(_) => {
                    eprintln!("   [{}/{}] Timeout getting messages around", idx, total_messages);
                    continue;  // タイムアウトの場合はスキップ
                }
            };
            
            // 前後のメッセージ全てのスレッドを取得
            let mut all_msgs = around.before.clone();
            all_msgs.push(msg.clone());
            all_msgs.extend(around.after.clone());
            
            let threads_result = tokio::time::timeout(
                std::time::Duration::from_secs(10),
                self.get_threads_batch(channel_id, &all_msgs)
            ).await;
            
            let threads = match threads_result {
                Ok(Ok(data)) => data,
                Ok(Err(e)) => {
                    eprintln!("   [{}/{}] Error getting threads: {}", idx, total_messages, e);
                    Vec::new()  // エラーの場合は空のベクター
                }
                Err(_) => {
                    eprintln!("   [{}/{}] Timeout getting threads", idx, total_messages);
                    Vec::new()  // タイムアウトの場合は空のベクター
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
}
