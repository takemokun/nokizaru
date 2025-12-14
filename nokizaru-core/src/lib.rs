// 共有層を名前空間として再エクスポート
pub mod shared {
    pub use shared_infrastructure as infrastructure;
}

pub mod agent_service;

// 外部クレート再エクスポート（テスト・統合用）
pub use shared_infrastructure::{create_pool, run_migrations};
pub use agent_service::*;
