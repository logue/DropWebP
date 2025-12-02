#!/bin/bash
# macOSからLinux向けビルドを実行するスクリプト

set -e

# プロジェクトルートディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# 色付き出力
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🐳 Docker経由でLinux向けビルドを実行${NC}"
echo ""

# ターゲットアーキテクチャを引数から取得（デフォルト: x86_64）
TARGET="${1:-x86_64-unknown-linux-gnu}"

case "$TARGET" in
    x64|x86_64|amd64)
        TARGET="x86_64-unknown-linux-gnu"
        ARCH_NAME="x86_64 (AMD64)"
        ;;
    arm64|aarch64)
        TARGET="aarch64-unknown-linux-gnu"
        ARCH_NAME="ARM64 (AArch64)"
        ;;
    *)
        echo -e "${YELLOW}⚠️  不明なターゲット: $TARGET${NC}"
        echo "使用方法: $0 [x64|arm64]"
        exit 1
        ;;
esac

echo -e "${GREEN}ターゲット:${NC} $ARCH_NAME ($TARGET)"
echo ""

# Dockerイメージをビルド
echo -e "${BLUE}📦 Dockerイメージをビルド中...${NC}"
cd "$PROJECT_ROOT"
docker build -f Dockerfile.linux-build -t dropwebp-linux-builder .

echo ""
echo -e "${BLUE}🔨 Linux向けアプリケーションをビルド中...${NC}"

# Dockerコンテナ内でビルドを実行
docker run --rm \
    -v "$PROJECT_ROOT:/workspace" \
    -e BUILD_TARGET="$TARGET" \
    dropwebp-linux-builder

echo ""
echo -e "${GREEN}✅ ビルド完了！${NC}"
echo ""
echo -e "${GREEN}📦 成果物の場所:${NC}"
echo "   $PROJECT_ROOT/app/src-tauri/target/$TARGET/release/bundle/"
echo ""

# 成果物のサイズを表示
if [ -d "$PROJECT_ROOT/app/src-tauri/target/$TARGET/release/bundle/deb" ]; then
    echo -e "${GREEN}📊 .deb パッケージ:${NC}"
    du -h "$PROJECT_ROOT/app/src-tauri/target/$TARGET/release/bundle/deb/"*.deb 2>/dev/null || true
fi

if [ -d "$PROJECT_ROOT/app/src-tauri/target/$TARGET/release/bundle/appimage" ]; then
    echo -e "${GREEN}📊 AppImage:${NC}"
    du -h "$PROJECT_ROOT/app/src-tauri/target/$TARGET/release/bundle/appimage/"*.AppImage 2>/dev/null || true
fi

echo ""
echo -e "${YELLOW}💡 ヒント:${NC}"
echo "   - ARM64用にビルド: $0 arm64"
echo "   - x64用にビルド:   $0 x64"
