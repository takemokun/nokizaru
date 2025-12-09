use async_trait::async_trait;

/// テキスト処理契約インターフェース
/// Agentモジュールが提供するテキスト処理機能にアクセスするための共有契約
#[async_trait]
pub trait TextProcessorContract: Send + Sync {
    /// チャンネルコンテキスト付きでテキストを処理
    async fn process_with_channel(
        &self,
        channel_id: &str,
        text: &str,
    ) -> anyhow::Result<String>;
}
