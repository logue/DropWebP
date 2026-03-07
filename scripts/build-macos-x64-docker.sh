#!/bin/bash
# Build x86_64 macOS binary using Docker
# This script helps avoid libaom cross-compilation issues when building from Apple Silicon

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "🐳 Building x86_64 macOS binary in Docker..."
echo "Project root: $PROJECT_ROOT"

# Create Dockerfile for macOS x86_64 build
cat > "$PROJECT_ROOT/Dockerfile.macos-x64" << 'EOF'
FROM --platform=linux/amd64 rust:1.92-slim

# Install dependencies for building macOS binaries
RUN apt-get update && apt-get install -y \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    pkg-config \
    yasm \
    nasm \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install macOS cross-compilation tools
RUN rustup target add x86_64-apple-darwin

# Install osxcross for macOS SDK
WORKDIR /opt
RUN git clone https://github.com/tpoechtrager/osxcross.git && \
    cd osxcross && \
    wget -nc https://github.com/joseluisq/macosx-sdks/releases/download/11.3/MacOSX11.3.sdk.tar.xz && \
    mv MacOSX11.3.sdk.tar.xz tarballs/ && \
    UNATTENDED=yes OSX_VERSION_MIN=11.0 ./build.sh

# Set up environment for cross-compilation
ENV PATH="/opt/osxcross/target/bin:$PATH"
ENV CC=o64-clang
ENV CXX=o64-clang++

WORKDIR /workspace
EOF

# Build Docker image
echo "📦 Building Docker image..."
docker build -t tauri-vue3-macos-x64-builder -f "$PROJECT_ROOT/Dockerfile.macos-x64" "$PROJECT_ROOT"

# Build the project
echo "🔨 Building x86_64 macOS binary..."
docker run --rm \
    -v "$PROJECT_ROOT:/workspace" \
    -w /workspace/backend \
    tauri-vue3-macos-x64-builder \
    cargo build --release --target x86_64-apple-darwin

echo "✅ Build complete!"
echo "Binary location: $PROJECT_ROOT/backend/target/x86_64-apple-darwin/release/tauri-vue3-app"
