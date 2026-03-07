#!/bin/bash
# macOSからLinux向けビルドを実行するスクリプト

set -e

# プロジェクトルートディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# .envファイルを読み込む
if [ -f "$PROJECT_ROOT/.env" ]; then
    echo "📄 .envファイルを読み込んでいます..."
    set -a
    source "$PROJECT_ROOT/.env"
    set +a
fi

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
        DOCKERFILE="docker/Dockerfile.linux-x64"
        IMAGE_NAME="tauri-vue3-linux-x64-builder"
        PLATFORM="linux/amd64"
        ;;
    arm64|aarch64)
        TARGET="aarch64-unknown-linux-gnu"
        ARCH_NAME="ARM64 (AArch64)"
        DOCKERFILE="docker/Dockerfile.linux-arm64"
        IMAGE_NAME="tauri-vue3-linux-arm64-builder"
        PLATFORM="linux/arm64"
        ;;
    *)
        echo -e "${YELLOW}⚠️  不明なターゲット: $TARGET${NC}"
        echo "使用方法: $0 [x64|arm64]"
        exit 1
        ;;
esac

echo -e "${GREEN}ターゲット:${NC} $ARCH_NAME ($TARGET)"
echo -e "${GREEN}Dockerfile:${NC} $DOCKERFILE"
echo -e "${GREEN}プラットフォーム:${NC} $PLATFORM"

# AppImageを含めるかどうか（デフォルト: 除外）
INCLUDE_APPIMAGE="${INCLUDE_APPIMAGE:-false}"
if [ "$INCLUDE_APPIMAGE" = "true" ]; then
    echo -e "${GREEN}AppImage:${NC} 有効（FUSEが必要）"
    BUNDLE_TARGETS=""
else
    echo -e "${YELLOW}AppImage:${NC} 無効（Docker環境では.deb, .rpmのみ）"
    BUNDLE_TARGETS="deb,rpm"
fi

echo ""

# CPUコア数とメモリの設定
DOCKER_BUILD_ARGS=""
DOCKER_RUN_ARGS=""

if [ -n "$BUILD_CPUS" ]; then
    echo -e "${GREEN}CPUコア数:${NC} $BUILD_CPUS"
    DOCKER_BUILD_ARGS="$DOCKER_BUILD_ARGS --cpus=$BUILD_CPUS"
    DOCKER_RUN_ARGS="$DOCKER_RUN_ARGS --cpus=$BUILD_CPUS"
fi

if [ -n "$BUILD_MEMORY" ]; then
    echo -e "${GREEN}メモリ制限:${NC} $BUILD_MEMORY"
    DOCKER_BUILD_ARGS="$DOCKER_BUILD_ARGS --memory=$BUILD_MEMORY"
    DOCKER_RUN_ARGS="$DOCKER_RUN_ARGS --memory=$BUILD_MEMORY"
fi

# Cargo並列度の設定
CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-${BUILD_CPUS}}"
if [ -n "$CARGO_BUILD_JOBS" ]; then
    echo -e "${GREEN}Cargo並列度:${NC} $CARGO_BUILD_JOBS"
    DOCKER_RUN_ARGS="$DOCKER_RUN_ARGS -e CARGO_BUILD_JOBS=$CARGO_BUILD_JOBS"
fi

# Make並列度の設定
if [ -n "$BUILD_CPUS" ]; then
    MAKEFLAGS="${MAKEFLAGS:--j$BUILD_CPUS}"
    echo -e "${GREEN}Make並列度:${NC} $MAKEFLAGS"
    DOCKER_RUN_ARGS="$DOCKER_RUN_ARGS -e MAKEFLAGS='$MAKEFLAGS'"
fi

echo ""

# Dockerイメージをビルド
echo -e "${BLUE}📦 Dockerイメージをビルド中...${NC}"
cd "$PROJECT_ROOT"
docker build --platform "$PLATFORM" $DOCKER_BUILD_ARGS -f "$DOCKERFILE" -t "$IMAGE_NAME" .

echo ""
echo -e "${BLUE}🔨 Linux向けアプリケーションをビルド中...${NC}"

# キャッシュ用のDockerボリューム名
CARGO_CACHE_VOLUME="tauri-vue3-cargo-cache-${PLATFORM//\//-}"
PNPM_CACHE_VOLUME="tauri-vue3-pnpm-cache-${PLATFORM//\//-}"
TARGET_CACHE_VOLUME="tauri-vue3-target-cache-${PLATFORM//\//-}"
NODE_MODULES_VOLUME="tauri-vue3-node-modules-${PLATFORM//\//-}"

# ボリュームが存在しない場合は作成
docker volume create "$CARGO_CACHE_VOLUME" >/dev/null 2>&1 || true
docker volume create "$PNPM_CACHE_VOLUME" >/dev/null 2>&1 || true
docker volume create "$TARGET_CACHE_VOLUME" >/dev/null 2>&1 || true
docker volume create "$NODE_MODULES_VOLUME" >/dev/null 2>&1 || true

echo -e "${GREEN}キャッシュボリューム:${NC}"
echo "  - Cargo: $CARGO_CACHE_VOLUME"
echo "  - pnpm: $PNPM_CACHE_VOLUME"
echo "  - Target: $TARGET_CACHE_VOLUME"
echo "  - Node modules: $NODE_MODULES_VOLUME (ホスト環境から完全に分離)"
echo ""

# Dockerコンテナ内でビルドを実行
# --privileged: AppImageビルドに必要
# --security-opt apparmor=unconfined: AppArmorを無効化
# --security-opt seccomp=unconfined: seccompを無効化
docker run --rm \
    --platform "$PLATFORM" \
    --privileged \
    --security-opt apparmor=unconfined \
    --security-opt seccomp=unconfined \
    -v "$PROJECT_ROOT:/workspace" \
    -v "$CARGO_CACHE_VOLUME:/root/.cargo/registry" \
    -v "$PNPM_CACHE_VOLUME:/pnpm/store" \
    -v "$TARGET_CACHE_VOLUME:/workspace/backend/target" \
    -v "$NODE_MODULES_VOLUME:/workspace/frontend/node_modules" \
    -e BUILD_TARGET="$TARGET" \
    -e TAURI_BUNDLER_TARGETS="$BUNDLE_TARGETS" \
    -e APPIMAGE_EXTRACT_AND_RUN=1 \
    -e VERBOSE=1 \
    $DOCKER_RUN_ARGS \
    "$IMAGE_NAME"

if [ $? -ne 0 ]; then
    echo ""
    echo -e "${RED}❌ ビルドエラーが発生しました${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}✅ ビルド完了！${NC}"
echo ""
echo -e "${BLUE}📋 成果物をホストにコピー中...${NC}"

# ホスト側のディレクトリを作成
mkdir -p "$PROJECT_ROOT/backend/target/$TARGET/release/bundle"

# Dockerボリュームから成果物（bundleディレクトリのみ）をホストにコピー
docker run --rm \
    --platform "$PLATFORM" \
    -v "$TARGET_CACHE_VOLUME:/data" \
    -v "$PROJECT_ROOT:/output" \
    alpine sh -c "if [ -d '/data/$TARGET/release/bundle' ]; then cp -rv /data/$TARGET/release/bundle/* /output/backend/target/$TARGET/release/bundle/ && echo '✅ コピー完了'; else echo '❌ bundle ディレクトリが見つかりません: /data/$TARGET/release/bundle'; find /data -name '*.deb' -o -name '*.rpm' 2>/dev/null || echo 'パッケージファイルが見つかりません'; exit 1; fi"

if [ $? -ne 0 ]; then
    echo -e "${YELLOW}⚠️  成果物のコピーに失敗しました${NC}"
    echo "Dockerボリュームの内容を確認しています..."
    docker run --rm -v "$TARGET_CACHE_VOLUME:/data" alpine sh -c "echo 'Volume contents:'; ls -la /data/$TARGET/release/ 2>/dev/null || ls -la /data/ 2>/dev/null || echo 'Volume is empty'"
    exit 1
fi

echo ""
echo -e "${GREEN}📦 成果物の場所:${NC}"
echo "   $PROJECT_ROOT/backend/target/$TARGET/release/bundle/"
echo ""

# 成果物のサイズを表示
if [ -d "$PROJECT_ROOT/backend/target/$TARGET/release/bundle/deb" ]; then
    echo -e "${GREEN}📊 .deb パッケージ:${NC}"
    du -h "$PROJECT_ROOT/backend/target/$TARGET/release/bundle/deb/"*.deb 2>/dev/null || true
fi

if [ -d "$PROJECT_ROOT/backend/target/$TARGET/release/bundle/rpm" ]; then
    echo -e "${GREEN}📊 .rpm パッケージ:${NC}"
    du -h "$PROJECT_ROOT/backend/target/$TARGET/release/bundle/rpm/"*.rpm 2>/dev/null || true
fi

if [ -d "$PROJECT_ROOT/backend/target/$TARGET/release/bundle/appimage" ]; then
    echo -e "${GREEN}📊 AppImage:${NC}"
    du -h "$PROJECT_ROOT/backend/target/$TARGET/release/bundle/appimage/"*.AppImage 2>/dev/null || true
fi

echo ""
echo -e "${YELLOW}💡 ヒント:${NC}"
echo "   - ARM64用にビルド: $0 arm64"
echo "   - x64用にビルド:   $0 x64"
