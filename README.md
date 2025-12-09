# Nokizaru - Slack Bot Application

Rustで構築されたモダンなSlackボットアプリケーション。Modular Monolithアーキテクチャとレイヤードアーキテクチャを採用し、Supabaseをデータベースとして使用。

## 🏗️ アーキテクチャ

### Modular Monolith + Clean Architecture

```
nokizaru/apps/bot/
├── src/
│   ├── api/           # 🆕 API層（プレゼンテーション）
│   │   └── v1/       # APIバージョン1
│   │       ├── handler/   # リクエストハンドラー
│   │       ├── dto/       # Data Transfer Objects
│   │       ├── middleware/# ミドルウェア
│   │       ├── routes.rs  # ルート定義
│   │       └── container.rs # DIコンテナ
│   ├── application/  # 🆕 共通Application層
│   │   └── validation/
│   ├── lib.rs        # 🆕 ライブラリエントリーポイント
│   └── main.rs       # アプリケーションエントリーポイント
├── module/           # ビジネスモジュール
│   ├── slack/       # Slack統合
│   │   ├── domain/
│   │   ├── application/
│   │   └── infrastructure/
│   └── user/        # ユーザー管理
│       ├── domain/
│       ├── application/
│       └── infrastructure/
└── shared/          # 共有コンポーネント
    ├── domain/
    └── infrastructure/
```

詳細は [`ARCHITECTURE.md`](./ARCHITECTURE.md) を参照。

### レイヤー構成

- **Domain層**: ビジネスロジック、エンティティ、リポジトリインターフェース
- **Application層**: ユースケース、ハンドラー
- **Infrastructure層**: 外部サービス連携（Slack API、Supabase）

## 🚀 セットアップ

### 前提条件

- Rust 1.75以降
- Slackワークスペース
- **Supabaseプロジェクト**（データベース）
  - Supabase セットアップ手順: [`docs/SUPABASE_SETUP.md`](./docs/SUPABASE_SETUP.md)

### 1. 環境変数の設定

```bash
make setup-env
# または
cp .env.example .env
```

`.env`を編集して以下を設定:
- `SLACK_BOT_TOKEN`: Slack Bot User OAuth Token
- `SLACK_SIGNING_SECRET`: Slack Signing Secret
- `DATABASE_URL`: PostgreSQL接続文字列（Supabase or ローカル）

### 2. データベースセットアップ

#### オプション A: Docker Compose（推奨）

```bash
# PostgreSQL + Bot を一括起動
docker-compose up -d

# マイグレーション自動実行
```

#### オプション B: Supabase（推奨）

```bash
# 1. Supabase接続情報を .env に設定
# 詳細手順: docs/SUPABASE_SETUP.md
DATABASE_URL=postgresql://postgres.[project-ref]:[password]@aws-0-[region].pooler.supabase.com:5432/postgres

# 2. Dieselマイグレーション実行
cd apps/bot
diesel migration run

# または Supabase SQL Editorで手動実行
# migrations/2024-01-01-000001_create_users/up.sql の内容を実行
```

### 3. 依存関係のインストール

```bash
cargo build
```

### 4. Slackアプリの設定

1. [Slack API](https://api.slack.com/apps)で新しいアプリを作成
2. **OAuth & Permissions**:
   - Bot Token Scopes: `chat:write`, `app_mentions:read`, `channels:history`
3. **Event Subscriptions**:
   - Request URL: `https://your-domain.com/slack/events`
   - Subscribe to bot events: `app_mention`, `message.channels`
4. **Slash Commands**:
   - `/hello`: Request URL `https://your-domain.com/slack/commands`
   - `/help`: Request URL `https://your-domain.com/slack/commands`

## 🏃 実行

### Docker Compose（推奨）

```bash
# バックグラウンドで起動
make docker-up
# または
docker-compose up -d

# ログ確認
make docker-logs

# 停止
make docker-down
```

### ローカル開発

```bash
# 開発モード
make run
# または
cargo run --bin nokizaru-bot

# リリースビルド
make build
./target/release/nokizaru-bot
```

## 📝 利用可能なエンドポイント

- `GET /health` - ヘルスチェック
- `POST /slack/events` - Slackイベント受信
- `POST /slack/commands` - Slackコマンド受信

## 🧪 テスト

```bash
cargo test
```

## 📚 技術スタック

- **フレームワーク**: Axum 0.7
- **非同期ランタイム**: Tokio 1.40
- **Slack SDK**: slack-morphism 2.6
- **ORM**: Diesel 2.2 + diesel-async 0.5
- **データベース**: Supabase PostgreSQL / ローカルPostgreSQL
- **エラーハンドリング**: thiserror, anyhow
- **ロギング**: tracing, tracing-subscriber
- **コンテナ**: Docker + Docker Compose

## 🔐 セキュリティ

- Slack署名検証による不正リクエストの防止
- HMAC-SHA256による署名検証
- タイムスタンプチェックによるリプレイ攻撃対策

## 📦 デプロイ

### Docker デプロイ

```bash
# Dockerイメージビルド
make docker-build

# イメージの実行
docker run -d \
  --name nokizaru-bot \
  -p 3000:3000 \
  --env-file .env \
  nokizaru-bot:latest
```

### 推奨デプロイ先
- **Fly.io**: `fly deploy` でDockerイメージをデプロイ
- **Railway**: GitHub連携で自動デプロイ
- **AWS ECS/Fargate**: コンテナオーケストレーション
- **Google Cloud Run**: サーバーレスコンテナ

## 📄 ライセンス

MIT