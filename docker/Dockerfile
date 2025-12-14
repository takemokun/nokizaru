# ==========================================
# Builder Stage - Rustビルド環境
# ==========================================
FROM rust:1.91-slim-bookworm AS builder

# 作業ディレクトリ設定
WORKDIR /app

# システム依存パッケージのインストール
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# ソースコードをコピー
COPY Cargo.toml Cargo.lock ./
COPY nokizaru-core nokizaru-core
COPY nokizaru-api nokizaru-api

# リリースビルド
RUN cargo build --release --bin nokizaru-api

# ==========================================
# Runtime Stage - 最小限のランタイム環境
# ==========================================
FROM debian:bookworm-slim

# 必要なランタイムライブラリのインストール
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# アプリケーション用ユーザー作成（セキュリティ向上）
RUN useradd -m -u 1000 appuser

# 作業ディレクトリ設定
WORKDIR /app

# ビルドステージからバイナリをコピー
COPY --from=builder /app/target/release/nokizaru-api /app/nokizaru-api

# マイグレーションファイルをコピー（オプション）
COPY nokizaru-core/migrations /app/migrations

# 所有権変更
RUN chown -R appuser:appuser /app

# 非rootユーザーで実行
USER appuser

# ヘルスチェック設定
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${SERVER_PORT:-3000}/health || exit 1

# アプリケーション起動
CMD ["/app/nokizaru-api"]
