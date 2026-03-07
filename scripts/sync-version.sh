#!/bin/bash
set -e

# バージョン同期スクリプト
# .envファイルからVERSIONを読み込み、各設定ファイルに同期します

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "🔄 プロジェクトバージョンを同期中..."

# .envファイルが存在するかチェック
ENV_FILE="$PROJECT_ROOT/.env"
if [ ! -f "$ENV_FILE" ]; then
    echo "❌ .envファイルが見つかりません: $ENV_FILE"
    exit 1
fi

# .envファイルを読み込み
echo "📄 .envファイルを読み込み中..."
set -a
source "$ENV_FILE"
set +a

if [ -z "$VERSION" ]; then
    echo "❌ VERSIONが.envファイルで定義されていません"
    exit 1
fi

echo "📋 バージョン: $VERSION"

# Cargo.tomlのバージョンを更新
CARGO_TOML="$PROJECT_ROOT/backend/Cargo.toml"
if [ -f "$CARGO_TOML" ]; then
    echo "🔧 Cargo.tomlを更新中..."
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "$CARGO_TOML"
    echo "  ✅ $CARGO_TOML: version = $VERSION"
else
    echo "⚠️  Cargo.tomlが見つかりません: $CARGO_TOML"
fi

# tauri.conf.jsonのバージョンを更新
TAURI_CONF="$PROJECT_ROOT/backend/tauri.conf.json"
if [ -f "$TAURI_CONF" ]; then
    echo "🔧 tauri.conf.jsonを更新中..."
    sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$TAURI_CONF"
    echo "  ✅ $TAURI_CONF: \"version\": \"$VERSION\""
else
    echo "⚠️  tauri.conf.jsonが見つかりません: $TAURI_CONF"
fi

# frontend/package.jsonのバージョンを更新または追加
PACKAGE_JSON="$PROJECT_ROOT/frontend/package.json"
if [ -f "$PACKAGE_JSON" ]; then
    echo "🔧 frontend/package.jsonを更新中..."
    if grep -q '"version"' "$PACKAGE_JSON"; then
        # versionフィールドが存在する場合、更新
        sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" "$PACKAGE_JSON"
    else
        # versionフィールドが存在しない場合、nameの後に追加
        sed -i.bak "/\"name\": \".*\",/a\\
  \"version\": \"$VERSION\"," "$PACKAGE_JSON"
    fi
    echo "  ✅ $PACKAGE_JSON: \"version\": \"$VERSION\""
else
    echo "⚠️  frontend/package.jsonが見つかりません: $PACKAGE_JSON"
fi

# バックアップファイルを削除
find "$PROJECT_ROOT" -name "*.bak" -type f -delete 2>/dev/null || true

echo ""
echo "✅ バージョン同期完了！"
echo "📋 同期されたバージョン: $VERSION"
