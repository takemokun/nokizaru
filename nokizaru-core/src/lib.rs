pub mod container;

// モジュール層を名前空間として再エクスポート
pub mod module {
    pub use agent;
    pub use slack;
}

// 共有層を名前空間として再エクスポート
pub mod shared {
    pub use contract;
    pub use shared_domain as domain;
    pub use shared_infrastructure as infrastructure;
}

// DIコンテナの公開
pub use container::AppContainer;

// 外部クレート再エクスポート（テスト・統合用）
pub use shared_infrastructure::{init_logging, AppConfig, create_pool, run_migrations};
