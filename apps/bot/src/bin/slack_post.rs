use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize)]
struct PostMessageRequest {
    channel: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_ts: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PostMessageResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 環境変数の読み込み
    dotenvy::dotenv().ok();

    // コマンドライン引数の取得
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage(&args[0]);
        std::process::exit(1);
    }

    let channel = &args[1];
    let text = &args[2];
    let thread_ts = args.get(3).cloned();

    // SLACK_BOT_TOKEN の取得
    let bot_token = env::var("SLACK_BOT_TOKEN")
        .context("SLACK_BOT_TOKEN environment variable not set")?;

    println!("📨 Posting message to Slack...");
    println!("   Channel: {}", channel);
    println!("   Message: {}", text);
    if let Some(ref ts) = thread_ts {
        println!("   Thread: {}", ts);
    }
    println!();

    // メッセージ投稿
    let response = post_message(&bot_token, channel, text, thread_ts.as_deref()).await?;

    if response.ok {
        println!("✅ Message posted successfully!");
        if let Some(ts) = response.ts {
            println!("   Timestamp: {}", ts);
        }
        if let Some(ch) = response.channel {
            println!("   Channel: {}", ch);
        }
    } else {
        println!("❌ Failed to post message");
        if let Some(error) = response.error {
            println!("   Error: {}", error);
        }
        std::process::exit(1);
    }

    Ok(())
}

async fn post_message(
    bot_token: &str,
    channel: &str,
    text: &str,
    thread_ts: Option<&str>,
) -> Result<PostMessageResponse> {
    let client = reqwest::Client::new();

    let request = PostMessageRequest {
        channel: channel.to_string(),
        text: text.to_string(),
        thread_ts: thread_ts.map(String::from),
    };

    let response = client
        .post("https://slack.com/api/chat.postMessage")
        .header("Authorization", format!("Bearer {}", bot_token))
        .header("Content-Type", "application/json; charset=utf-8")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to Slack API")?;

    let status = response.status();
    let body = response.text().await.context("Failed to read response body")?;

    if !status.is_success() {
        anyhow::bail!("Slack API returned error status: {} - {}", status, body);
    }

    let post_response: PostMessageResponse = serde_json::from_str(&body)
        .context(format!("Failed to parse response: {}", body))?;

    Ok(post_response)
}

fn print_usage(program: &str) {
    println!("🚀 Slack Message Poster");
    println!();
    println!("Usage:");
    println!("  {} <channel_id> <message> [thread_ts]", program);
    println!();
    println!("Arguments:");
    println!("  <channel_id>  Slack channel ID (e.g., C01234ABC56)");
    println!("  <message>     Message text to post");
    println!("  [thread_ts]   Optional: Thread timestamp for reply");
    println!();
    println!("Environment:");
    println!("  SLACK_BOT_TOKEN  Required: Bot User OAuth Token (xoxb-...)");
    println!();
    println!("Examples:");
    println!("  # Simple message");
    println!("  {} C01234ABC56 \"Hello, Slack!\"", program);
    println!();
    println!("  # Reply to thread");
    println!("  {} C01234ABC56 \"Reply!\" 1234567890.123456", program);
    println!();
    println!("  # With environment variable");
    println!("  SLACK_BOT_TOKEN=xoxb-... {} C01234ABC56 \"Test\"", program);
}
