#!/bin/bash
set -e

# CI環境であることを明示（pnpmがTTYなしで動作するため）
export CI=true

# pkg-configがクロスコンパイルとして動作することを許可
# Docker内でネイティブビルドしているが、Rustのターゲット指定でクロスコンパイルと誤認されるため
export PKG_CONFIG_ALLOW_CROSS=1

echo "🐧 Linux向けビルドを開始..."

# プロジェクトルートで依存関係をインストール
echo "📦 ルート依存関係をインストール中..."
pnpm install

# appディレクトリで依存関係をインストール
echo "📦 アプリ依存関係をインストール中..."
cd app
pnpm install

# ビルドターゲットを環境変数から取得（デフォルト: x86_64）
TARGET="${BUILD_TARGET:-x86_64-unknown-linux-gnu}"
echo "🎯 ターゲット: $TARGET"

# Tauriビルドを実行
echo "🔨 Tauriアプリケーションをビルド中..."
pnpm tauri build --target "$TARGET"

echo "✅ ビルド完了！"
echo "📦 成果物の場所: /workspace/app/src-tauri/target/$TARGET/release/bundle/"

# 成果物をリスト表示
ls -lh "/workspace/app/src-tauri/target/$TARGET/release/bundle/" 2>/dev/null || true
