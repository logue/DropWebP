# Linux用Drop Compress Imageのビルド

このガイドでは、Ubuntu 24.04 LTS（および類似のDebianベースディストリビューション）での開発環境のセットアップとDrop Compress Imageのビルド手順を説明します。

## 前提条件

開始する前に、以下が必要です：

- Ubuntu 24.04 LTSまたは類似のDebianベースディストリビューション
- ソフトウェアインストール用のsudo権限
- ターミナルコマンドの基本的な知識

## ステップ 1: システムパッケージの更新

まず、システムパッケージを更新して最新バージョンであることを確認します：

```bash
sudo apt update
sudo apt upgrade -y
```

## ステップ 2: ビルド依存関係のインストール

Tauri開発に必要な基本的なビルドツールとライブラリをインストールします：

```bash
# ビルド必須パッケージと開発ライブラリをインストール
sudo apt install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libwebkit2gtk-4.1-dev \
  patchelf
```

### パッケージの説明

- **build-essential**: GCC、G++、makeを提供
- **libssl-dev**: OpenSSL開発ライブラリ
- **libgtk-3-dev**: UI用GTK3開発ライブラリ
- **libayatana-appindicator3-dev**: システムトレイサポート
- **librsvg2-dev**: SVGレンダリングサポート
- **libwebkit2gtk-4.1-dev**: TauriのWebView用WebKit
- **patchelf**: AppImage用ELFバイナリパッチャー

### インストールの確認

```bash
gcc --version
```

GCCバージョン13.x以降が表示されます。

## ステップ 3: Rustのインストール

Drop Compress ImageはRustで構築されているため、Rustツールチェインをインストールする必要があります。

### rustup経由でRustをインストール

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

プロンプトが表示されたら、オプション1（デフォルトインストール）を選択します。

### シェルの設定

```bash
source $HOME/.cargo/env
```

永続化するには、シェルプロファイルに追加します：

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

### Rustインストールの確認

```bash
rustc --version
cargo --version
```

`rustc`と`cargo`の両方でバージョン情報が表示されます。

## ステップ 4: Node.jsのインストール

Drop Compress ImageのフロントエンドはVue.jsで構築されており、Node.jsが必要です。

### NodeSourceリポジトリ経由でNode.jsをインストール

```bash
# Node.js 22.x (LTS) をインストール
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
```

### Node.jsインストールの確認

```bash
node --version
npm --version
```

Node.jsバージョン22.x以降が表示されます。

## ステップ 5: pnpmのインストール

Drop Compress Imageは、パフォーマンスとディスク効率を向上させるためにpnpmをパッケージマネージャーとして使用します。

### pnpmのインストール

```bash
npm install -g pnpm
```

### pnpmインストールの確認

```bash
pnpm --version
```

## ステップ 6: vcpkgのセットアップと依存関係のインストール

このプロジェクトではvcpkgを使用してC/C++画像処理ライブラリ（libaom、libavif、libjxl等）を管理します。

### vcpkg前提パッケージのインストール

```bash
# vcpkgに必要なツールをインストール
sudo apt install -y curl zip unzip tar cmake pkg-config
```

### vcpkgのインストール

```bash
# vcpkgをクローン
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg

# vcpkgをブートストラップ
cd ~/vcpkg
./bootstrap-vcpkg.sh

# 環境変数を設定（~/.bashrc に追加）
echo 'export VCPKG_ROOT="$HOME/vcpkg"' >> ~/.bashrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 依存関係のインストール

自動インストールスクリプトを使用（推奨）:

```bash
cd ~/path/to/DropWebP/app/src-tauri
./setup-vcpkg.sh
```

または手動でインストール：

```bash
cd ~/vcpkg

# x64 Linux の場合
./vcpkg install aom:x64-linux
./vcpkg install libavif[aom]:x64-linux
./vcpkg install libjxl:x64-linux
./vcpkg install libwebp:x64-linux
./vcpkg install openjpeg:x64-linux
./vcpkg install libjpeg-turbo:x64-linux
./vcpkg install lcms:x64-linux

# ARM64 Linux の場合
./vcpkg install aom:arm64-linux
./vcpkg install libavif[aom]:arm64-linux
./vcpkg install libjxl:arm64-linux
./vcpkg install libwebp:arm64-linux
./vcpkg install openjpeg:arm64-linux
./vcpkg install libjpeg-turbo:arm64-linux
./vcpkg install lcms:arm64-linux
```

インストールされるライブラリ：

- **libaom**: AV1エンコーダー（AVIF形式用、**必須**）
- **libavif**: AVIF画像フォーマット
- **libjxl**: JPEG XL画像フォーマット
- **libwebp**: WebP画像フォーマット
- **openjpeg**: JPEG 2000画像フォーマット
- **libjpeg-turbo**: JPEG画像処理（jpegli用）
- **lcms**: Little CMS カラーマネジメント

### インストール確認

```bash
./vcpkg list | grep -E "aom|avif|jxl|webp|openjpeg|jpeg|lcms"
```

## ステップ 7: Drop Compress Imageのクローンとビルド

これでDrop Compress Imageをクローンしてビルドする準備が整いました。

### リポジトリのクローン

```bash
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

### フロントエンド依存関係のインストール

```bash
# すべてのワークスペース依存関係をインストール
pnpm install
```

### Tauri CLI v2のインストール

```bash
# Tauri CLI v2をグローバルにインストール
pnpm add -g @tauri-apps/cli@next
```

### アプリケーションのビルド

開発用：

```bash
# 開発モードで実行
pnpm dev:tauri
```

本番用：

```bash
# 本番用にビルド
pnpm build:tauri
```

ビルドされたアプリケーションは`app/src-tauri/target/release/`にあります。

## ステップ 8: 配布形式

LinuxのTauriは複数の配布形式を生成できます：

### AppImage（推奨）

AppImageは、ほとんどのLinuxディストリビューションで動作するユニバーサルパッケージ形式です：

```bash
pnpm build:tauri
```

AppImageは`app/src-tauri/target/release/bundle/appimage/`にあります。

### Debianパッケージ (.deb)

Debian/Ubuntuベースのディストリビューション用：

```bash
pnpm build:tauri
```

.debパッケージは`app/src-tauri/target/release/bundle/deb/`にあります。

インストール：

```bash
sudo dpkg -i app/src-tauri/target/release/bundle/deb/*.deb
```

### RPMパッケージ (.rpm)

Red Hat/Fedoraベースのディストリビューション用には、追加ツールのインストールが必要です：

```bash
sudo apt install -y rpm
pnpm build:tauri
```

.rpmパッケージは`app/src-tauri/target/release/bundle/rpm/`にあります。

## トラブルシューティング

### よくある問題

1. **libwebkit2gtk-4.1がない**

   webkitライブラリが見つからないエラーが出る場合：

   ```bash
   # 古いwebkitバージョンを試す
   sudo apt install -y libwebkit2gtk-4.0-dev
   ```

2. **npm/pnpmの権限拒否**

   ```bash
   # npmグローバルディレクトリの権限を修正
   mkdir -p ~/.npm-global
   npm config set prefix '~/.npm-global'
   echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **ネイティブ依存関係でのビルド失敗**

   ```bash
   # ビルドキャッシュをクリア
   cargo clean
   pnpm clean

   # すべてを再ビルド
   pnpm install
   pnpm build:tauri
   ```

4. **AppImageが実行可能でない**

   ```bash
   # AppImageを実行可能にする
   chmod +x app/src-tauri/target/release/bundle/appimage/*.AppImage
   ```

5. **GLIBCバージョンが見つからない**

   GLIBCバージョンに関するエラーが表示される場合は、Ubuntu 24.04 LTS以降であることを確認してください：

   ```bash
   ldd --version
   ```

### グラフィックスドライバの問題

最適なパフォーマンスを得るには、適切なグラフィックスドライバがインストールされていることを確認してください：

```bash
# NVIDIA用
sudo ubuntu-drivers autoinstall

# AMD用
sudo apt install -y mesa-vulkan-drivers

# Intel用
sudo apt install -y intel-media-va-driver
```

### ヘルプを得る

ここでカバーされていない問題が発生した場合：

1. [Drop Compress Imageリポジトリ](https://github.com/logue/DropWebP)で既知の問題を確認
2. Linux固有のガイダンスについて[Tauri v2ドキュメント](https://v2.tauri.app/start/prerequisites/)を確認
3. 既存のGitHub Issueを検索するか、新しいIssueを作成

## 次のステップ

Drop Compress Imageのビルドが成功したら：

1. **テストの実行**: `pnpm test`を実行してすべてが正しく動作することを確認
2. **開発**: ホットリロードでの開発には`pnpm dev:tauri`を使用
3. **カスタマイズ**: コードベースを探索して変更を加える
4. **配布**: 配布可能なパッケージを作成するには`pnpm build:tauri`を使用

これでLinuxでDrop Compress Imageを開発およびビルドする準備が整いました！
