#!/bin/bash
# Intel Mac向けビルドのためにx86_64版ライブラリをインストールするスクリプト

set -e

# プロジェクトルートディレクトリを取得
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# .envファイルを読み込む
if [ -f "$ROOT_DIR/.env" ]; then
    set -a
    source "$ROOT_DIR/.env"
    set +a
fi

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}🔧 Intel Mac向けビルド環境をセットアップ${NC}"
echo ""

# x86_64版Homebrewがインストールされているか確認
if [ ! -f /usr/local/bin/brew ]; then
    echo -e "${YELLOW}x86_64版Homebrewがインストールされていません${NC}"
    echo ""
    echo "以下のコマンドでインストールできます："
    echo ""
    echo "  arch -x86_64 /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
    echo ""
    echo -e "${YELLOW}インストール後、このスクリプトを再実行してください${NC}"
    exit 1
fi

echo -e "${GREEN}✓ x86_64版Homebrewが見つかりました${NC}"
echo ""

# 必要なライブラリをインストール
echo -e "${BLUE}📦 x86_64版ライブラリをインストール中...${NC}"

LIBS=("libavif" "jpeg-xl")

for lib in "${LIBS[@]}"; do
    echo -e "${BLUE}Installing $lib...${NC}"
    if arch -x86_64 /usr/local/bin/brew list "$lib" &>/dev/null; then
        echo -e "${GREEN}✓ $lib はすでにインストールされています${NC}"
    else
        arch -x86_64 /usr/local/bin/brew install "$lib"
        echo -e "${GREEN}✓ $lib をインストールしました${NC}"
    fi
done

echo ""
echo -e "${GREEN}✅ セットアップ完了！${NC}"
echo ""
echo "次のコマンドでIntel Mac向けビルドを実行できます："
echo "  cd app && pnpm run build:tauri:mac-x64"
