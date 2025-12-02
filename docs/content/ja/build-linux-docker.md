# Linux向けビルド（Docker使用）

macOSからLinux向けのビルドをDocker経由で実行する方法

## 📋 前提条件

- Docker Desktop for Mac がインストールされていること
- 十分なディスクスペース（初回は約5GB必要）

## 🚀 使用方法

### x86_64 (AMD64) 向けビルド

```bash
# プロジェクトルートで実行
./scripts/build-linux-docker.sh x64

# または app ディレクトリから
pnpm run build:tauri:linux-docker-x64
```

### ARM64 (AArch64) 向けビルド

```bash
# プロジェクトルートで実行
./scripts/build-linux-docker.sh arm64

# または app ディレクトリから
pnpm run build:tauri:linux-docker-arm64
```

## 📦 生成される成果物

ビルド成果物は以下のディレクトリに生成されます：

```text
app/src-tauri/target/
  ├── x86_64-unknown-linux-gnu/release/bundle/
  │   ├── deb/           # Debian/Ubuntuパッケージ
  │   ├── rpm/           # Red Hat/Fedoraパッケージ
  │   └── appimage/      # AppImage（配布推奨）
  │
  └── aarch64-unknown-linux-gnu/release/bundle/
      ├── deb/
      ├── rpm/
      └── appimage/
```

## ⚙️ 内部動作

1. `Dockerfile.linux-build` から Docker イメージをビルド
   - Rust 1.83 + Debian Bookworm ベース
   - Tauri の依存関係（WebKit2GTK、GTK3等）をインストール
   - Node.js 22.x と pnpm をインストール

2. Docker コンテナ内で Tauri ビルドを実行
   - プロジェクトディレクトリをマウント
   - ターゲットアーキテクチャを指定してビルド

3. 成果物を macOS 側のディレクトリに出力

## 🔧 トラブルシューティング

### Docker イメージのリビルド

```bash
docker build -f Dockerfile.linux-build -t dropwebp-linux-builder --no-cache .
```

### Docker イメージの削除

```bash
docker rmi dropwebp-linux-builder
```

### ビルドキャッシュのクリア

```bash
rm -rf app/src-tauri/target/x86_64-unknown-linux-gnu
rm -rf app/src-tauri/target/aarch64-unknown-linux-gnu
```

## 📝 注意事項

- 初回ビルドは Docker イメージのビルドとダウンロードで時間がかかります（20-30分程度）
- 2回目以降は Docker イメージが再利用されるため高速です（10-15分程度）
- ARM64 向けビルドは x86_64 向けよりも時間がかかる場合があります

## 🎯 推奨配布形式

- **AppImage**: 配布推奨（すべてのLinuxディストリビューションで動作）
- **.deb**: Debian/Ubuntu系ユーザー向け
- **.rpm**: Red Hat/Fedora系ユーザー向け
