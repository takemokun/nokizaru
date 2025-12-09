use serde::{Deserialize, Serialize};

/// AIエージェントのメモリ（シンプル版：チャンネル履歴のみ）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemory {
    /// チャンネルメッセージ履歴
    pub channel_history: Vec<MemoryMessage>,
    /// 最大メッセージ保持数
    pub max_messages: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMessage {
    /// ユーザーID
    pub user_id: Option<String>,
    /// メッセージ内容
    pub text: String,
    /// タイムスタンプ
    pub timestamp: String,
}

impl AgentMemory {
    pub fn new(max_messages: usize) -> Self {
        Self {
            channel_history: Vec::new(),
            max_messages,
        }
    }

    /// チャンネル履歴を追加
    pub fn add_channel_messages(&mut self, messages: Vec<MemoryMessage>) {
        self.channel_history.extend(messages);

        // 古いメッセージを削除（上限維持）
        if self.channel_history.len() > self.max_messages {
            let drain_count = self.channel_history.len() - self.max_messages;
            self.channel_history.drain(0..drain_count);
        }
    }

    /// メモリをクリア
    pub fn clear(&mut self) {
        self.channel_history.clear();
    }

    /// コンテキスト文字列を生成（AI用）
    pub fn build_context_string(&self) -> String {
        if self.channel_history.is_empty() {
            return String::new();
        }

        let mut context = String::new();
        context.push_str("=== Recent Channel Messages ===\n");

        // 最新20件を取得（古い順）
        for msg in self.channel_history.iter().rev().take(20).collect::<Vec<_>>().iter().rev() {
            let user = msg.user_id.as_deref().unwrap_or("Unknown");
            context.push_str(&format!("[{}] {}: {}\n", msg.timestamp, user, msg.text));
        }
        context.push_str("\n");

        context
    }
}
