# Linux向けビルド（Docker使用）

Windows、macOS、LinuxからDocker経由でLinux向けのビルドを実行する方法

## 📋 前提条件

### すべてのプラットフォーム共通

- Docker Desktop または Docker Engine
- pnpm (v10.2.0以上)
- 8GB以上のRAM（16GB推奨）
- 20GB以上のディスク空き容量

### プラットフォーム別

#### Windows

- Windows 10/11 (64-bit)
- WSL 2（推奨）
- PowerShell 5.1以上

#### macOS

- macOS 10.15以上
- Bash
- Docker Desktop for Mac

#### Linux

- 64-bit Linux distribution
- Docker Engine 20.10以上
- Bash

## 🚀 使用方法

### Windows環境でのビルド

```powershell
# プロジェクトルートで実行
pnpm run build:tauri:linux-x64    # x86_64 Linux
pnpm run build:tauri:linux-arm64  # ARM64 Linux

# または直接スクリプトを実行
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

### macOS / Linux環境でのビルド

```bash
# プロジェクトルートで実行
bash scripts/build-linux-docker.sh x64    # x86_64 Linux
bash scripts/build-linux-docker.sh arm64  # ARM64 Linux

# または frontend ディレクトリから
pnpm run build:tauri:linux-docker-x64
pnpm run build:tauri:linux-docker-arm64
```

## 📦 生成される成果物

### デフォルト（AppImage無効）

ビルド成果物は以下のディレクトリに生成されます：

```text
backend/target/
  ├── x86_64-unknown-linux-gnu/release/bundle/
  │   ├── deb/           # Debian/Ubuntuパッケージ
  │   └── rpm/           # Red Hat/Fedoraパッケージ
  │
  └── aarch64-unknown-linux-gnu/release/bundle/
      ├── deb/
      └── rpm/
```

### AppImage有効時

```powershell
# Windows
pwsh .\scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage

# macOS/Linux
INCLUDE_APPIMAGE=true bash scripts/build-linux-docker.sh x64
```

上記に加えて `backend/target/.../bundle/appimage/` にAppImageが生成されます。

> **注意**: AppImageのビルドにはFUSEが必要で、Docker環境では制限があります。

## ⚙️ ビルド設定のカスタマイズ

`.env` ファイルでビルド設定をカスタマイズできます：

```bash
# Docker Build Settings
BUILD_CPUS=4              # 使用するCPUコア数
BUILD_MEMORY=8g           # メモリ制限
CARGO_BUILD_JOBS=4        # Cargoの並列ジョブ数
MAKEFLAGS=-j4             # Makeの並列度
INCLUDE_APPIMAGE=false    # AppImageを含めるか
```

### パフォーマンス最適化

#### 高性能マシン向け設定

```bash
BUILD_CPUS=12
BUILD_MEMORY=16g
CARGO_BUILD_JOBS=12
MAKEFLAGS=-j12
```

#### 一般的なマシン向け設定

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
```

## ⚙️ 内部動作

1. Dockerイメージの構築
   - Rust + Debian Bookworm ベース
   - Tauri の依存関係（WebKit2GTK、GTK3等）をインストール
   - Node.js と pnpm をインストール

2. Docker コンテナ内で Tauri ビルドを実行
   - プロジェクトディレクトリをマウント
   - ターゲットアーキテクチャを指定してビルド
   - Dockerボリュームを使用したキャッシュ管理

3. 成果物をホスト側のディレクトリに出力

## 🔧 トラブルシューティング

### Dockerが見つからない（Windows）

```
❌ エラー: Docker Desktop が起動していません。
```

**解決方法**: Docker Desktopを起動してから、再度実行してください。

### メモリ不足

**症状**: ビルド中にメモリエラー

**解決方法**:

1. `.env`でメモリ制限を増やす
2. Docker Desktopのリソース設定でメモリを増やす（Settings → Resources → Memory）
3. 並列ビルド数を減らす（`BUILD_CPUS`を減らす）

### ビルドが遅い

**解決方法**:

- `.env`で並列度を増やす
- Docker Desktopのリソース（CPU、メモリ）を増やす
- SSDを使用する
- キャッシュボリュームを活用する

### ビルドキャッシュのクリア

#### Windows

```powershell
# x86_64キャッシュをクリア
docker volume rm tauri-vue3-cargo-cache-linux-amd64
docker volume rm tauri-vue3-pnpm-cache-linux-amd64
docker volume rm tauri-vue3-target-cache-linux-amd64

# ARM64キャッシュをクリア
docker volume rm tauri-vue3-cargo-cache-linux-arm64
docker volume rm tauri-vue3-pnpm-cache-linux-arm64
docker volume rm tauri-vue3-target-cache-linux-arm64
```

#### macOS / Linux

```bash
# x86_64キャッシュをクリア
docker volume rm tauri-vue3-cargo-cache-linux-amd64
docker volume rm tauri-vue3-pnpm-cache-linux-amd64
docker volume rm tauri-vue3-target-cache-linux-amd64

# ARM64キャッシュをクリア
docker volume rm tauri-vue3-cargo-cache-linux-arm64
docker volume rm tauri-vue3-pnpm-cache-linux-arm64
docker volume rm tauri-vue3-target-cache-linux-arm64
```

### Docker イメージのリビルド

```bash
# x86_64用
docker build -f Dockerfile.linux-x64 -t tauri-vue3-linux-x64-builder --no-cache .

# ARM64用
docker build -f Dockerfile.linux-arm64 -t tauri-vue3-linux-arm64-builder --no-cache .
```

## 📝 注意事項

- 初回ビルドは Docker イメージのビルドとダウンロードで時間がかかります（20-40分程度）
- 2回目以降はキャッシュが利用されるため高速です（5-15分程度）
- ARM64 向けビルドは x86_64 向けよりも時間がかかる場合があります
- Windows環境ではWSL 2を使用することを推奨します

## 🎯 推奨配布形式

- **.deb**: Debian/Ubuntu系ユーザー向け（デフォルトで生成）
- **.rpm**: Red Hat/Fedora系ユーザー向け（デフォルトで生成）
- **AppImage**: 配布推奨（すべてのLinuxディストリビューションで動作）※オプション

## 📚 関連ドキュメント

- [ルートディレクトリのDOCKER_BUILD.md](../../../DOCKER_BUILD.md) - 全プラットフォーム対応の詳細ガイド
- [ルートディレクトリのDOCKER_BUILD_WINDOWS.md](../../../DOCKER_BUILD_WINDOWS.md) - Windows固有の詳細手順
