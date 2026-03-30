#!/bin/bash
set -e

# CI環境であることを明示（pnpmがTTYなしで動作するため）
export CI=true

# .envファイルから環境変数を安全に読み込み（未クォートのスペースを含む値に対応）
if [ -f "/workspace/.env" ]; then
    echo "📄 .envファイルを読み込み中..."
    while IFS= read -r line || [ -n "$line" ]; do
        case "$line" in
            ''|'#'*)
                continue
                ;;
        esac

        key="${line%%=*}"
        value="${line#*=}"

        # 両端の空白を除去
        key="${key#${key%%[![:space:]]*}}"
        key="${key%${key##*[![:space:]]}}"
        value="${value#${value%%[![:space:]]*}}"
        value="${value%${value##*[![:space:]]}}"

        # 先頭末尾が同じ引用符なら除去
        if [[ "$value" == \"*\" && "$value" == *\" ]]; then
            value="${value:1:${#value}-2}"
        elif [[ "$value" == \'*\' && "$value" == *\' ]]; then
            value="${value:1:${#value}-2}"
        fi

        [ -n "$key" ] && export "$key=$value"
    done < /workspace/.env

    echo "  VERSION=$VERSION"
    echo "  PROJECT_NAME=$PROJECT_NAME"

    # バージョン情報を同期
    echo "🔄 バージョン情報を同期中..."
    if [ -n "$VERSION" ]; then
        # Cargo.toml のバージョンを更新
        sed -i "s/^version = \".*\"/version = \"$VERSION\"/" /workspace/backend/Cargo.toml
        echo "  Cargo.toml: version = $VERSION"

        # tauri.conf.json のバージョンを更新
        sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" /workspace/backend/tauri.conf.json
        echo "  tauri.conf.json: version = $VERSION"
    fi
fi

# pkg-configがクロスコンパイルとして動作することを許可
# Docker内でネイティブビルドしているが、Rustのターゲット指定でクロスコンパイルと誤認されるため
export PKG_CONFIG_ALLOW_CROSS=1

# AppImage作成時にFUSEを使用しない（Docker内での実行のため）
export APPIMAGE_EXTRACT_AND_RUN=1
export NO_STRIP=1
export LINUXDEPLOY_OUTPUT_VERSION=1
export VERBOSE=1

echo "🐧 Linux向けビルドを開始..."
echo "📋 環境変数:"
echo "  APPIMAGE_EXTRACT_AND_RUN=$APPIMAGE_EXTRACT_AND_RUN"
echo "  NO_STRIP=$NO_STRIP"
echo ""

# appディレクトリのみで依存関係をインストール
# (ルートのpostinstallスクリプトがdocsパッケージを要求するため、appのみインストール)
echo "📦 アプリ依存関係をインストール中..."
cd /workspace
# node_modulesはDockerボリュームにマウントされているため、ホスト環境とは完全に分離
# 初回またはpackage.json変更時のみインストールが必要
if [ ! -d "frontend/node_modules/.pnpm" ]; then
    echo "  初回インストール中..."
    pnpm install --filter frontend --frozen-lockfile --ignore-scripts
else
    echo "  既存のnode_modulesを使用（必要に応じて更新）"
    pnpm install --filter frontend --frozen-lockfile --ignore-scripts --offline 2>/dev/null || \
    pnpm install --filter frontend --frozen-lockfile --ignore-scripts
fi

# ビルドターゲットを環境変数から取得（デフォルト: x86_64）
TARGET="${BUILD_TARGET:-x86_64-unknown-linux-gnu}"
echo "🎯 ターゲット: $TARGET"

# ターゲットに応じた環境変数を設定
if [ "$TARGET" = "x86_64-unknown-linux-gnu" ]; then
    # x86_64はネイティブビルド（Docker内でx86_64環境）
    export CC=gcc
    export CXX=g++
    export AR=ar
    export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
    # libavif-sys, libaom-sysビルド用の環境変数
    export CMAKE_TOOLCHAIN_FILE=""
    export CMAKE_PREFIX_PATH="/usr"
    export LD_LIBRARY_PATH="/usr/lib/x86_64-linux-gnu:$LD_LIBRARY_PATH"
    echo "🔧 ネイティブx86_64ビルド"
    echo "🔧 コンパイラ: CC=$CC, CXX=$CXX"
elif [ "$TARGET" = "aarch64-unknown-linux-gnu" ]; then
    # ARM64はネイティブビルド
    export CC=gcc
    export CXX=g++
    export AR=ar
    export PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
    # libavif-sys, libaom-sysビルド用の環境変数
    export CMAKE_TOOLCHAIN_FILE=""
    export CMAKE_PREFIX_PATH="/usr"
    export LD_LIBRARY_PATH="/usr/lib/aarch64-linux-gnu:$LD_LIBRARY_PATH"
    echo "🔧 ネイティブARM64ビルド"
    echo "🔧 コンパイラ: CC=$CC, CXX=$CXX"
fi

# Tauriビルドを実行
echo "🔨 Tauriアプリケーションをビルド中..."

# Docker環境ではAppImageを除外（linuxdeployがFUSEを必要とするため）
if [ -z "$TAURI_BUNDLER_TARGETS" ]; then
    BUNDLE_TARGETS="deb,rpm"
    echo "📦 ビルドターゲット: deb, rpm (AppImageはDocker環境では除外)"
else
    BUNDLE_TARGETS="$TAURI_BUNDLER_TARGETS"
    echo "📦 ビルドターゲット: $TAURI_BUNDLER_TARGETS"
fi

node /workspace/scripts/run-tauri-build.mjs --target "$TARGET" --bundles "$BUNDLE_TARGETS"

echo "✅ ビルド完了！"

# 成果物の場所を表示
BUNDLE_PATH="/workspace/backend/target/$TARGET/release/bundle/"
echo "📦 成果物の場所: $BUNDLE_PATH"

# 成果物をリスト表示
if [ -d "$BUNDLE_PATH" ]; then
    ls -lh "$BUNDLE_PATH"
    find "$BUNDLE_PATH" -type f \( -name "*.deb" -o -name "*.rpm" -o -name "*.AppImage" \) -exec ls -lh {} \;
else
    echo "⚠️  bundle ディレクトリが見つかりません"
    echo "target/ の内容:"
    find "/workspace/backend/target" -name "*.deb" -o -name "*.rpm" 2>/dev/null || echo "パッケージファイルが見つかりません"
fi
