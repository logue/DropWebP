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
VERSION="${1:-${VERSION:-3.0.2}}"

echo "=== Homebrew Formula Generation ==="
echo "Version: $VERSION"
HOMEBREW_DIR="$ROOT_DIR/.homebrew"
BUNDLE_DIR="$ROOT_DIR/app/src-tauri/target/release/bundle"

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

# Formulaファイルを更新
FORMULA_FILE="$HOMEBREW_DIR/drop-compress-image.rb"

# テンプレートからプレースホルダーを置換
sed -e "s/{{VERSION}}/$VERSION/g" \
    -e "s/{{SHA256_UNIVERSAL}}/$SHA256_UNIVERSAL/g" \
    "$FORMULA_FILE" > "$FORMULA_FILE.tmp" && mv "$FORMULA_FILE.tmp" "$FORMULA_FILE"

echo ""
echo "Formula updated successfully!"
echo "Formula location: $FORMULA_FILE"

echo ""
echo "=== Next Steps ==="
echo "1. Test the formula locally:"
echo "   brew install --formula $FORMULA_FILE"
echo "2. Create a tap repository and push the formula"
echo "3. Users can install with:"
echo "   brew tap logue/tap"
echo "   brew install drop-compress-image"
