.PHONY: help build run test clean docker-build docker-up docker-down docker-logs setup-env

# デフォルトターゲット
help:
	@echo "Nokizaru Bot - Available Commands"
	@echo "=================================="
	@echo "Development:"
	@echo "  make setup-env    - Create .env file from example"
	@echo "  make build        - Build the project"
	@echo "  make run          - Run the bot locally"
	@echo "  make test         - Run tests"
	@echo "  make clean        - Clean build artifacts"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-up    - Start services with Docker Compose"
	@echo "  make docker-down  - Stop Docker services"
	@echo "  make docker-logs  - View Docker logs"
	@echo ""
	@echo "Database (Supabase):"
	@echo "  make db-setup     - Setup Supabase database (first time)"
	@echo "  make db-migrate   - Run database migrations"
	@echo "  make db-status    - Check migration status"
	@echo "  make db-reset     - Reset database (WARNING: destructive)"

# 環境設定
setup-env:
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo "✅ .env file created. Please edit it with your credentials."; \
	else \
		echo "⚠️  .env file already exists."; \
	fi

# ビルド
build:
	cargo build --release

# 実行
run:
	cargo run --bin nokizaru-bot

# テスト
test:
	cargo test

# クリーンアップ
clean:
	cargo clean
	rm -rf target/

# Docker操作
docker-build:
	docker build -t nokizaru-bot:latest .

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs -f bot

docker-restart:
	docker-compose restart bot

# データベース操作（Supabase対応）
db-migrate:
	@echo "🔄 Running Diesel migrations on Supabase..."
	cd apps/bot && diesel migration run
	@echo "✅ Migrations completed"

db-reset:
	@echo "⚠️  WARNING: This will reset your Supabase database!"
	@read -p "Are you sure? (y/N): " confirm && [ "$$confirm" = "y" ] || exit 1
	cd apps/bot && diesel database reset

db-status:
	@echo "📊 Migration status:"
	cd apps/bot && diesel migration list

db-setup:
	@echo "🚀 Setting up Supabase database..."
	@echo "1. Ensure DATABASE_URL is set in .env"
	@echo "2. Running migrations..."
	$(MAKE) db-migrate
	@echo "✅ Database setup complete!"

# 開発用（全ビルド）
dev-setup: setup-env
	@echo "🚀 Setting up development environment..."
	cargo build
	@echo "✅ Development setup complete!"

# 本番ビルド
production-build: clean build
	@echo "✅ Production build complete!"
