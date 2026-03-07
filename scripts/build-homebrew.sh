#!/bin/bash
# Homebrew Formula 生成スクリプト

set -e

# プロジェクトルートディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# .envファイルを読み込む
if [ -f "$ROOT_DIR/.env" ]; then
    echo "📄 .envファイルを読み込んでいます..."
    set -a
    source "$ROOT_DIR/.env"
    set +a
fi

# コマンドライン引数でバージョンを上書き可能
VERSION="${1:-${VERSION:-1.0.0}}"
APP_NAME_KEBAB="${APP_NAME_KEBAB:-tauri-vue3-app}"

# ケバブケースからパスカルケースに変換（Rubyクラス名用）
# 例: tauri-vue3-app → TauriVue3App
CLASS_NAME=$(echo "$APP_NAME_KEBAB" | sed -r 's/(^|-)([a-z0-9])/\U\2/g')

echo "=== Homebrew Formula Generation ==="
echo "App Name: $APP_NAME_KEBAB"
echo "Class Name: $CLASS_NAME"
echo "Version: $VERSION"
HOMEBREW_DIR="$ROOT_DIR/.homebrew"
BUNDLE_DIR="$ROOT_DIR/backend/target/release/bundle"

# DMGファイルを探す（Universal版のみ）
DMG_UNIVERSAL=$(find "$BUNDLE_DIR/dmg" -name "*universal*.dmg" | head -n 1)

if [ -z "$DMG_UNIVERSAL" ]; then
    echo "Error: Universal DMG file not found in $BUNDLE_DIR/dmg"
    exit 1
fi

echo "Found DMG file:"
echo "  Universal: $(basename "$DMG_UNIVERSAL")"

# チェックサムを計算
SHA256_UNIVERSAL=$(shasum -a 256 "$DMG_UNIVERSAL" | cut -d' ' -f1)

echo ""
echo "SHA256 Checksum:"
echo "  Universal: $SHA256_UNIVERSAL"

# Formulaファイルを更新（テンプレートから生成）
TEMPLATE_FILE="$HOMEBREW_DIR/app.rb.template"
FORMULA_FILE="$HOMEBREW_DIR/${APP_NAME_KEBAB}.rb"

if [ ! -f "$TEMPLATE_FILE" ]; then
    echo "Error: Template file not found: $TEMPLATE_FILE"
    exit 1
fi

# テンプレートから全ての変数を置換
sed -e "s/{{VERSION}}/$VERSION/g" \
    -e "s/{{CLASS_NAME}}/$CLASS_NAME/g" \
    -e "s|{{APP_NAME}}|$APP_NAME|g" \
    -e "s/{{APP_NAME_KEBAB}}/$APP_NAME_KEBAB/g" \
    -e "s|{{HOMEBREW_DESC}}|$HOMEBREW_DESC|g" \
    -e "s|{{HOMEPAGE_URL}}|$HOMEPAGE_URL|g" \
    -e "s|{{PROJECT_URL}}|$PROJECT_URL|g" \
    -e "s/{{SHA256_UNIVERSAL}}/$SHA256_UNIVERSAL/g" \
    "$TEMPLATE_FILE" > "$FORMULA_FILE"

echo ""
echo "Formula updated successfully!"
echo "Formula location: $FORMULA_FILE"

echo ""
echo "=== Next Steps ==="
echo "1. Test the formula locally:"
echo "   brew install --formula $FORMULA_FILE"
echo "2. Create a tap repository and push the formula"
echo "3. Users can install with:"
echo "   brew tap ${GITHUB_USER}/tap"
echo "   brew install ${APP_NAME_KEBAB}"
