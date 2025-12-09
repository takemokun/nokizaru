// API層
pub mod api;

// 共通Application層
pub mod application;

// モジュール層を名前空間として再エクスポート
pub mod module {
    pub use agent;
    pub use slack;
}

// 共有層を名前空間として再エクスポート
pub mod shared {
    pub use contracts;
    pub use shared_domain as domain;
    pub use shared_infrastructure as infrastructure;
}

// モジュール公開
pub use api::{AppContainer, create_router};
pub use application::validation;

// 外部クレート再エクスポート（テスト・統合用）
pub use shared_infrastructure::{init_logging, AppConfig, create_pool, run_migrations};
