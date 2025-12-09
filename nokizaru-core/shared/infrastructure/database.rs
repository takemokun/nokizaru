use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use std::env;

pub type DbPool = Pool<AsyncPgConnection>;

pub async fn create_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .build(config)
        .await
        .expect("Failed to create pool")
}

pub async fn run_migrations(pool: &DbPool) -> anyhow::Result<()> {
    let _conn = pool.get().await
        .map_err(|e| anyhow::anyhow!("Failed to get connection from pool: {}", e))?;

    // マイグレーション実行ロジック
    // 本番環境では diesel_migrations クレートを使用することを推奨
    tracing::info!("Database connection established");

    Ok(())
}
