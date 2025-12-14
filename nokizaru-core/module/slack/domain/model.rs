use serde::{Deserialize, Serialize};

/// Slackメッセージのドメインモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackMessage {
    pub channel_id: String,
    pub user_id: String,
    pub text: String,
    pub timestamp: String,
    pub thread_ts: Option<String>,
}

/// Slackイベントのドメインモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SlackEvent {
    #[serde(rename = "message")]
    Message {
        channel: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bot_id: Option<String>,
        text: String,
        ts: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        thread_ts: Option<String>,
    },
    #[serde(rename = "app_mention")]
    AppMention {
        channel: String,
        user: String,
        text: String,
        ts: String,
    },
}

/// Slackコマンドのドメインモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackCommand {
    pub command: String,
    pub text: String,
    pub user_id: String,
    pub channel_id: String,
    pub response_url: String,
    pub trigger_id: String,
}

/// Slackインタラクションのドメインモデル
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackInteraction {
    pub interaction_type: String,
    pub user_id: String,
    pub channel_id: String,
    pub action_id: String,
    pub value: Option<String>,
}

/// Slackチャンネル履歴メッセージ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlackHistoryMessage {
    /// メッセージタイプ
    #[serde(rename = "type")]
    pub msg_type: String,
    /// ユーザーID（オプション）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Bot ID（オプション）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    /// メッセージテキスト
    pub text: String,
    /// タイムスタンプ
    pub ts: String,
}
