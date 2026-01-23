# Build Scripts

このディレクトリには、プロジェクトのビルドとデプロイに使用するスクリプトが含まれています。

## バージョン管理

**すべてのビルドスクリプトは、ルートディレクトリの`.env`ファイルからバージョン情報を自動的に読み取ります。**

```dotenv
# .env
VERSION=3.2.1
PROJECT_NAME=drop-compress-image
GITHUB_USER=logue
GITHUB_REPO=DropWebP
```

バージョンを変更する場合は、`.env`ファイルの`VERSION`のみを更新してください。各パッケージマネージャーのテンプレートファイル（`.choco/drop-compress-image.nuspec`, `.homebrew/drop-compress-image.rb`）は、プレースホルダー `{{VERSION}}` を使用しており、ビルド時に自動的に置換されます。

## ディレクトリ構造

```
scripts/
├── build/              # ネイティブビルドスクリプト
│   ├── build-windows-admin.ps1
│   └── build-windows.ps1
├── docker/             # Dockerビルドスクリプト
│   ├── docker-build.cmd
│   ├── docker-build.ps1
│   └── docker-build.sh
├── build-chocolatey.ps1    # Chocolateyパッケージ作成
├── build-homebrew.sh       # Homebrewフォーミュラ作成
├── build-linux-docker.ps1  # Linuxクロスビルド (PowerShell)
├── build-linux-docker.sh   # Linuxクロスビルド (Bash)
├── build-macos-compatible.sh
├── build-macos-x64-docker.sh
└── setup-x86-libs.sh       # x86ライブラリセットアップ
```

## スクリプト一覧

### ネイティブビルド

#### `build/build-windows.ps1`

Windows用のネイティブビルドスクリプト（vcpkg使用）

```powershell
.\scripts\build\build-windows.ps1
```

#### `build/build-windows-admin.ps1`

管理者権限が必要なWindows向けビルド

### Dockerビルド

#### `docker/docker-build.sh`

Linux向けクロスプラットフォームビルド（Bash）

```bash
./scripts/docker/docker-build.sh x64    # x86_64
./scripts/docker/docker-build.sh arm64  # ARM64
```

#### `docker/docker-build.ps1`

Linux向けクロスプラットフォームビルド（PowerShell）

```powershell
.\scripts\docker\docker-build.ps1 -Target x64
.\scripts\docker\docker-build.ps1 -Target arm64
```

#### `build-linux-docker.sh` / `build-linux-docker.ps1`

プロジェクトルートから実行するLinuxビルドラッパー

```bash
# Bashから
./scripts/build-linux-docker.sh x64

# PowerShellから
.\scripts\build-linux-docker.ps1 -Target x64
```

### パッケージング

#### `build-chocolatey.ps1`

Windows用Chocolateyパッケージを作成

`.env`からバージョンを読み取り、`.choco/drop-compress-image.nuspec`の`{{VERSION}}`プレースホルダーを置換します。

```powershell
# .envから自動読み取り
pnpm run package:chocolatey

# バージョンを明示的に指定
.\scripts\build-chocolatey.ps1 -Version 3.2.1
```

**プロセス**:

1. `.env`からバージョンを読み取り
2. MSIファイルのSHA256チェックサムを計算
3. `{{VERSION}}`を実際のバージョンに置換
4. `.nupkg`パッケージを生成

詳細: [.choco/README.md](../.choco/README.md)

#### `build-homebrew.sh`

macOS用Homebrewフォーミュラを生成

`.env`からバージョンを読み取り、`.homebrew/drop-compress-image.rb`の`{{VERSION}}`と`{{SHA256_*}}`プレースホルダーを置換します。

```bash
# .envから自動読み取り
pnpm run package:homebrew

# バージョンを明示的に指定
bash scripts/build-homebrew.sh 3.2.1
```

**プロセス**:

1. `.env`からバージョンを読み取り
2. ARM64とx64版DMGファイルのSHA256を計算
3. `{{VERSION}}`、`{{SHA256_AARCH64}}`、`{{SHA256_X64}}`を置換
4. Formulaファイルを更新

詳細: [.homebrew/README.md](../.homebrew/README.md)

### macOSビルド

#### `build-macos-compatible.sh`

Apple Silicon互換ビルド（M1/M2/M3）

#### `build-macos-x64-docker.sh`

Intel Mac用クロスビルド

#### `setup-x86-libs.sh`

x86_64ライブラリのセットアップ

## 詳細情報

- [Development Documentation](../docs/content/ja/) - ビルド手順の詳細
- [Docker Configuration](../docker/) - Dockerファイルの説明
- [Main README](../ReadMe.md) - プロジェクト概要
