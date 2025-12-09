#!/bin/bash
# Build script for macOS with maximum compatibility across Apple Silicon chips
# This script ensures the binary works on M1, M2, M3, and future M-series chips

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

echo "🔨 Building DropWebP for macOS with universal compatibility..."
echo "Version: ${VERSION:-unknown}"

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cd app/src-tauri
cargo clean

# Build for Apple Silicon (ARM64) with generic optimizations
echo "🍎 Building for Apple Silicon (aarch64-apple-darwin)..."
export MACOSX_DEPLOYMENT_TARGET=11.0  # Minimum macOS version for Apple Silicon
cargo build --release --target aarch64-apple-darwin

# Optional: Build universal binary (ARM64 + x86_64)
# Uncomment if you want to support Intel Macs as well
# echo "🔧 Building for Intel (x86_64-apple-darwin)..."
# cargo build --release --target x86_64-apple-darwin

# echo "📦 Creating universal binary..."
# lipo -create \
#   target/aarch64-apple-darwin/release/drop-compress-image \
#   target/x86_64-apple-darwin/release/drop-compress-image \
#   -output target/release/drop-compress-image-universal

echo "✅ Build complete!"
echo "📍 Binary location: app/src-tauri/target/aarch64-apple-darwin/release/"

# Display binary info
echo ""
echo "🔍 Binary information:"
file target/aarch64-apple-darwin/release/drop-compress-image
ls -lh target/aarch64-apple-darwin/release/drop-compress-image

echo ""
echo "💡 To test on different Macs:"
echo "   1. Copy the binary to the target Mac"
echo "   2. Run: chmod +x drop-compress-image"
echo "   3. Run: ./drop-compress-image"
