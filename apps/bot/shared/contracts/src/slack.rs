use async_trait::async_trait;

/// Slackメッセージの情報
#[derive(Debug, Clone)]
pub struct SlackMessageInfo {
    pub channel_id: String,
    pub user_id: String,
    pub text: String,
    pub timestamp: String,
    pub thread_ts: Option<String>,
}

/// Slackチャンネル履歴メッセージ
#[derive(Debug, Clone)]
pub struct SlackHistoryMessageInfo {
    /// ユーザーID（オプション）
    pub user: Option<String>,
    /// Bot ID（オプション）
    pub bot_id: Option<String>,
    /// メッセージテキスト
    pub text: String,
    /// タイムスタンプ
    pub ts: String,
}

/// Slackメッセージ操作のための契約インターフェース
/// 他のモジュールがSlack機能にアクセスするための共有契約
#[async_trait]
pub trait SlackMessageContract: Send + Sync {
    /// メッセージを送信
    async fn send_message(&self, message: &SlackMessageInfo) -> anyhow::Result<()>;

    /// スレッド返信を送信
    async fn send_reply(
        &self,
        message: &SlackMessageInfo,
        thread_ts: &str,
    ) -> anyhow::Result<()>;

    /// メッセージを更新
    async fn update_message(
        &self,
        channel_id: &str,
        timestamp: &str,
        new_text: &str,
    ) -> anyhow::Result<()>;

    /// チャンネル履歴を取得
    async fn fetch_channel_history(
        &self,
        channel_id: &str,
        limit: Option<i32>,
    ) -> anyhow::Result<Vec<SlackHistoryMessageInfo>>;
}
