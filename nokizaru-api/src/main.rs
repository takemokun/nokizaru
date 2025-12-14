use nokizaru_api::api::v1::{create_router, AppConfig, AppContainer};
use nokizaru_core::{create_pool, run_migrations};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ロギング初期化
    init_logging();
    tracing::info!("🚀 Nokizaru Bot starting...");

    // 設定読み込み
    let config = AppConfig::from_env()?;
    tracing::info!("✅ Configuration loaded");

    // データベース接続プール作成
    let db_pool = create_pool().await;
    tracing::info!("✅ Database connection pool created");

    // マイグレーション実行
    run_migrations(&db_pool).await?;
    tracing::info!("✅ Database migrations completed");

    // DIコンテナ構築
    let container = std::sync::Arc::new(AppContainer::new(config.clone()));
    tracing::info!("✅ DI container initialized");

    // ルーター構築
    let app = create_router(container);
    tracing::info!("✅ Router configured");

    // サーバー起動
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("🎯 Server listening on {}", addr);
    tracing::info!("📝 Health check: http://{}/health", addr);
    tracing::info!("📨 Slack events: http://{}/slack/events", addr);
    tracing::info!("⚡ Slack commands: http://{}/slack/commands", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

fn init_logging() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,nokizaru=debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
