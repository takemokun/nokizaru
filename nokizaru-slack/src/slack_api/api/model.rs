use serde::{Deserialize, Serialize};
use super::conversations::{ThreadInfo, SlackHistoryMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessage {
    /// メッセージタイプ
    #[serde(rename = "type")]
    pub msg_type: String,
    /// ユーザーID（オプション）
    #[serde(default)]
    pub user: Option<String>,
    /// Bot ID（オプション）
    #[serde(default)]
    pub bot_id: Option<String>,
    /// メッセージテキスト
    pub text: String,
    /// タイムスタンプ
    pub ts: String,
    /// チャンネル情報（検索結果用）
    #[serde(default)]
    pub channel: Option<ChannelInfo>,
    /// ユーザー名（検索結果用）
    #[serde(default)]
    pub username: Option<String>,
}

/// チャンネル情報（検索結果内）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelInfo {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

/// メッセージコンテキスト（検索結果と周辺情報）
#[derive(Debug, Clone)]
pub struct MessageContext {
    pub target_message: SlackMessage,
    pub before_messages: Vec<SlackHistoryMessage>,
    pub after_messages: Vec<SlackHistoryMessage>,
    pub threads: Vec<ThreadInfo>,
}
