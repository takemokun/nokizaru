use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
struct ConversationsHistoryResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    messages: Option<Vec<Message>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    #[serde(rename = "type")]
    msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    ts: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_count: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 環境変数の読み込み
    dotenvy::dotenv().ok();

    // コマンドライン引数の取得
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let channel = &args[1];
    let limit: i32 = args
        .get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    // SLACK_BOT_TOKEN の取得
    let bot_token = env::var("SLACK_BOT_TOKEN")
        .context("SLACK_BOT_TOKEN environment variable not set")?;

    println!("📥 Fetching messages from Slack...");
    println!("   Channel: {}", channel);
    println!("   Limit: {}", limit);
    println!();

    // メッセージ取得
    let messages = fetch_messages(&bot_token, channel, limit).await?;

    // メッセージ表示
    if messages.is_empty() {
        println!("📭 No messages found");
    } else {
        println!("📬 Found {} messages:", messages.len());
        println!("{}", "=".repeat(80));

        for (i, msg) in messages.iter().enumerate() {
            print_message(i + 1, msg);
            if i < messages.len() - 1 {
                println!("{}", "-".repeat(80));
            }
        }

        println!("{}", "=".repeat(80));
    }

    Ok(())
}

async fn fetch_messages(
    bot_token: &str,
    channel: &str,
    limit: i32,
) -> Result<Vec<Message>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://slack.com/api/conversations.history")
        .header("Authorization", format!("Bearer {}", bot_token))
        .query(&[("channel", channel), ("limit", &limit.to_string())])
        .send()
        .await
        .context("Failed to send request to Slack API")?;

    let status = response.status();
    let body = response.text().await.context("Failed to read response body")?;

    if !status.is_success() {
        anyhow::bail!("Slack API returned error status: {} - {}", status, body);
    }

    let history_response: ConversationsHistoryResponse = serde_json::from_str(&body)
        .context(format!("Failed to parse response: {}", body))?;

    if !history_response.ok {
        let error = history_response.error.unwrap_or_else(|| "unknown".to_string());
        anyhow::bail!("Slack API error: {}", error);
    }

    Ok(history_response.messages.unwrap_or_default())
}

fn print_message(index: usize, msg: &Message) {
    // タイムスタンプをパース
    let ts_parts: Vec<&str> = msg.ts.split('.').collect();
    let timestamp = if let Some(ts) = ts_parts.first() {
        if let Ok(secs) = ts.parse::<i64>() {
            use chrono::{DateTime, Utc};
            let dt = DateTime::<Utc>::from_timestamp(secs, 0)
                .unwrap_or_else(|| Utc::now());
            dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
        } else {
            msg.ts.clone()
        }
    } else {
        msg.ts.clone()
    };

    // メッセージヘッダー
    println!("#{} [{}]", index, timestamp);

    // ユーザー情報
    if let Some(ref user) = msg.user {
        println!("👤 User: {}", user);
    } else if let Some(ref bot_id) = msg.bot_id {
        println!("🤖 Bot: {}", bot_id);
    }

    // メッセージ本文
    if let Some(ref text) = msg.text {
        println!("💬 {}", text);
    }

    // スレッド情報
    if msg.thread_ts.is_some() {
        if let Some(count) = msg.reply_count {
            println!("🧵 Thread: {} replies", count);
        } else {
            println!("🧵 In thread");
        }
    }

    // タイムスタンプ（生データ）
    println!("⏱  TS: {}", msg.ts);
}

fn print_usage(program: &str) {
    println!("📥 Slack Message Fetcher");
    println!();
    println!("Usage:");
    println!("  {} <channel_id> [limit]", program);
    println!();
    println!("Arguments:");
    println!("  <channel_id>  Slack channel ID (e.g., C01234ABC56)");
    println!("  [limit]       Number of messages to fetch (default: 10, max: 100)");
    println!();
    println!("Environment:");
    println!("  SLACK_BOT_TOKEN  Required: Bot User OAuth Token (xoxb-...)");
    println!();
    println!("Required Scopes:");
    println!("  channels:history  - Public channel history");
    println!("  groups:history    - Private channel history");
    println!("  im:history        - Direct message history");
    println!("  mpim:history      - Group DM history");
    println!();
    println!("Examples:");
    println!("  # Fetch 10 messages (default)");
    println!("  {} C01234ABC56", program);
    println!();
    println!("  # Fetch 20 messages");
    println!("  {} C01234ABC56 20", program);
    println!();
    println!("  # Fetch 100 messages");
    println!("  {} C01234ABC56 100", program);
    println!();
    println!("  # With environment variable");
    println!("  SLACK_BOT_TOKEN=xoxb-... {} C01234ABC56", program);
}
