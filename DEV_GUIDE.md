# 開発ガイド

## 🔄 自動リロード開発環境

### 開発モードでの起動

```bash
# 開発モードで起動（cargo-watch による自動リロード）
docker-compose -f docker-compose.dev.yml up

# バックグラウンドで起動
docker-compose -f docker-compose.dev.yml up -d

# ログを確認
docker-compose -f docker-compose.dev.yml logs -f bot
```

### 仕組み

- **ソースコードマウント**: ローカルの `apps/bot/` ディレクトリがコンテナにマウントされる
- **cargo-watch**: ファイル変更を検知して自動的に再ビルド・再起動
- **キャッシュ**: `cargo-cache` と `target-cache` で依存関係のビルド時間を短縮

### コードを変更したら

1. ファイルを保存
2. cargo-watch が自動的に検知してリビルド
3. 数秒後にアプリケーションが再起動
4. すぐに変更が反映される

**出力例**:
```
bot  | [Running 'cargo run --bin nokizaru-bot']
bot  | 🚀 Nokizaru Bot starting...
bot  | ✅ Configuration loaded
bot  | ...
```

### 開発環境の停止

```bash
# 停止
docker-compose -f docker-compose.dev.yml down

# データベースも含めて完全削除
docker-compose -f docker-compose.dev.yml down -v
```

## 🚀 本番モードでの起動

```bash
# 本番モード（最適化ビルド、自動リロードなし）
docker-compose up --build -d

# または
docker-compose -f docker-compose.yml up --build -d
```

## 📝 使い分け

| 用途 | コマンド | 特徴 |
|------|---------|------|
| **開発** | `docker-compose -f docker-compose.dev.yml up` | ✅ 自動リロード<br>✅ 高速な変更反映<br>❌ ビルド最適化なし |
| **本番** | `docker-compose up --build -d` | ✅ 最適化ビルド<br>✅ 小さいイメージサイズ<br>❌ 変更ごとに手動リビルド |

## 🛠️ トラブルシューティング

### cargo-watch が動かない場合

```bash
# コンテナに入って手動確認
docker-compose -f docker-compose.dev.yml exec bot bash

# cargo-watch のインストール確認
cargo watch --version

# 手動で実行
cargo watch -x 'run --bin nokizaru-bot'
```

### ビルドが遅い場合

キャッシュをクリアして再ビルド:
```bash
docker-compose -f docker-compose.dev.yml down -v
docker volume rm nokizaru_cargo-cache nokizaru_target-cache
docker-compose -f docker-compose.dev.yml up --build
```

### データベース接続エラー

```bash
# データベースの状態確認
docker-compose -f docker-compose.dev.yml ps

# データベースログ確認
docker-compose -f docker-compose.dev.yml logs db

# データベース再起動
docker-compose -f docker-compose.dev.yml restart db
```

## 💡 Tips

### 特定のファイルのみ監視

`Dockerfile.dev` の CMD を変更:
```dockerfile
CMD ["cargo", "watch", "-w", "apps/bot/src", "-x", "run --bin nokizaru-bot"]
```

### リビルド時にクリア

```dockerfile
CMD ["cargo", "watch", "-c", "-x", "run --bin nokizaru-bot"]
```

### より詳細なログ

```bash
RUST_LOG=debug docker-compose -f docker-compose.dev.yml up
```
