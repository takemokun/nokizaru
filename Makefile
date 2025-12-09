.PHONY: help build run run-local test clean docker-build docker-up docker-down docker-clean docker-logs docker-restart setup-env

# Database URL for local development (when running diesel commands from host)
# For Docker container, use host.docker.internal (defined in .env)
# For host machine, use 127.0.0.1
# Note: Using 'postgres' database directly. For production, use a dedicated database name.
LOCAL_DATABASE_URL = postgresql://postgres:postgres@127.0.0.1:54422/postgres

# Alternative: Use dedicated database for better isolation
# LOCAL_DATABASE_URL = postgresql://postgres:postgres@127.0.0.1:54422/nokizaru_dev

# デフォルトターゲット
help:
	@echo "Nokizaru - Available Commands"
	@echo "=================================="
	@echo "Development:"
	@echo "  make setup-env    - Create .env file from example"
	@echo "  make build        - Build the project (Cargo)"
	@echo "  make run          - Run with Docker Compose (recommended)"
	@echo "  make run-local    - Run locally without Docker"
	@echo "  make test         - Run tests"
	@echo "  make clean        - Clean build artifacts"
	@echo ""
	@echo "Docker:"
	@echo "  make docker-build - Build Docker image"
	@echo "  make docker-up    - Start services with Docker Compose"
	@echo "  make docker-down  - Stop Docker services"
	@echo "  make docker-clean - Clean Docker containers and volumes"
	@echo "  make docker-logs  - View Docker logs"
	@echo "  make docker-restart - Restart Docker services"
	@echo ""
	@echo "Database:"
	@echo "  make db-setup     - Setup database (first time)"
	@echo "  make db-migrate   - Run database migrations"
	@echo "  make db-status    - Check migration status"
	@echo "  make db-reset     - Reset database (revert + re-run migrations)"
	@echo "  make db-create    - Create dedicated database (optional)"
	@echo "  make db-drop      - Drop dedicated database (optional)"

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

# 実行（Docker推奨）
run: docker-down
	@echo "🚀 Starting Nokizaru with Docker Compose..."
	docker compose -f docker/compose.yml --env-file .env up --build
	@echo "✅ Nokizaru is running!"
	@echo "📝 Health check: http://localhost:3000/health"
	@echo "📊 View logs: make docker-logs"

# ローカル実行（Docker不要）
run-local:
	cargo run --bin nokizaru-api

# テスト
test:
	cargo test

# クリーンアップ
clean:
	cargo clean
	rm -rf target/

# Docker操作
docker-build:
	docker build -f docker/Dockerfile -t nokizaru:latest .

docker-up:
	docker compose -f docker/compose.yml --env-file .env up -d

docker-down:
	docker compose -f docker/compose.yml --env-file .env down

docker-clean:
	@echo "🧹 Cleaning up Docker containers and volumes..."
	docker compose -f docker/compose.yml --env-file .env down -v
	docker system prune -f
	@echo "✅ Docker cleanup complete!"

docker-logs:
	docker compose -f docker/compose.yml --env-file .env logs -f bot

docker-restart:
	docker compose -f docker/compose.yml --env-file .env restart bot

# データベース操作
db-migrate:
	@echo "🔄 Running Diesel migrations..."
	cd nokizaru-core && DATABASE_URL=$(LOCAL_DATABASE_URL) diesel migration run
	@echo "✅ Migrations completed"

db-reset:
	@echo "⚠️  WARNING: This will reset your database!"
	@read -p "Are you sure? (y/N): " confirm && [ "$$confirm" = "y" ] || exit 1
	@echo "🔄 Reverting all migrations..."
	cd nokizaru-core && DATABASE_URL=$(LOCAL_DATABASE_URL) diesel migration revert --all || true
	@echo "🔄 Re-running all migrations..."
	cd nokizaru-core && DATABASE_URL=$(LOCAL_DATABASE_URL) diesel migration run
	@echo "✅ Database reset complete!"

db-status:
	@echo "📊 Migration status:"
	cd nokizaru-core && DATABASE_URL=$(LOCAL_DATABASE_URL) diesel migration list

db-setup:
	@echo "🚀 Setting up database..."
	@echo "1. Ensure Supabase is running locally (supabase start)"
	@echo "2. Running migrations..."
	$(MAKE) db-migrate
	@echo "✅ Database setup complete!"

# Optional: Create dedicated database (for better isolation)
db-create:
	@echo "📦 Creating dedicated database 'nokizaru_dev'..."
	@psql postgresql://postgres:postgres@127.0.0.1:54422/postgres -c "CREATE DATABASE nokizaru_dev;" 2>/dev/null || echo "Database may already exist"
	@echo "✅ Database created!"
	@echo "💡 To use it, update LOCAL_DATABASE_URL in Makefile to use 'nokizaru_dev'"

# Optional: Drop dedicated database
db-drop:
	@echo "⚠️  WARNING: This will drop the 'nokizaru_dev' database!"
	@read -p "Are you sure? (y/N): " confirm && [ "$$confirm" = "y" ] || exit 1
	@psql postgresql://postgres:postgres@127.0.0.1:54422/postgres -c "DROP DATABASE IF EXISTS nokizaru_dev;"
	@echo "✅ Database dropped!"

diesel-setup:
	cd nokizaru-core && DATABASE_URL=$(LOCAL_DATABASE_URL) diesel setup

db-add-migration:
	cd nokizaru-core && DATABASE_URL=$(LOCAL_DATABASE_URL) diesel migration generate $(NAME)

# 開発用（全セットアップ）
dev-setup: setup-env
	@echo "🚀 Setting up development environment..."
	cargo build
	@echo "✅ Development setup complete!"
	@echo ""
	@echo "Next steps:"
	@echo "  1. Edit .env with your credentials"
	@echo "  2. Run 'make db-setup' to setup database"
	@echo "  3. Run 'make run' to start the application"

# 本番ビルド
production-build: clean build
	@echo "✅ Production build complete!"
