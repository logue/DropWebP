# Build Scripts

このディレクトリには、プロジェクトのビルドとデプロイに使用するスクリプトが含まれています。

## バージョン管理

**すべてのビルドスクリプトは、ルートディレクトリの`.env`ファイルからバージョン情報とアプリケーション設定を自動的に読み取ります。**

```dotenv
# .env
VERSION=1.0.0
APP_NAME=Your App Name
APP_NAME_KEBAB=your-app-name
APP_DESCRIPTION=Your app description
AUTHOR_NAME=Your Name
GITHUB_USER=username
GITHUB_REPO=repository-name
PROJECT_URL=https://github.com/username/repository
```

バージョンやアプリケーション情報を変更する場合は、`.env`ファイルのみを更新してください。各パッケージマネージャーのテンプレートファイル（`.choco/app.nuspec.template`, `.homebrew/app.rb.template`）は、プレースホルダー（例: `{{VERSION}}`, `{{APP_NAME}}`）を使用しており、ビルド時に自動的に置換されます。

### テンプレートファイル

- **Chocolatey**: `.choco/app.nuspec.template` → `.choco/{APP_NAME_KEBAB}.nuspec`
- **Homebrew**: `.homebrew/app.rb.template` → `.homebrew/{APP_NAME_KEBAB}.rb`

テンプレートファイルの変数プレースホルダー：

- `{{VERSION}}` - アプリケーションバージョン
- `{{APP_NAME}}` - アプリケーション名
- `{{APP_NAME_KEBAB}}` - ケバブケース形式のアプリケーション名
- `{{APP_DESCRIPTION}}` - アプリケーションの説明
- `{{AUTHOR_NAME}}` - 作者名
- `{{GITHUB_USER}}` - GitHubユーザー名
- `{{PROJECT_URL}}` - プロジェクトURL
- その他、.envファイル内の変数

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

`.env`からバージョンとアプリケーション情報を読み取り、`.choco/app.nuspec.template`から`.choco/{APP_NAME_KEBAB}.nuspec`を生成し、全てのプレースホルダーを置換します。

```powershell
# .envから自動読み取り
pnpm run package:chocolatey

# バージョンを明示的に指定
.\scripts\build-chocolatey.ps1 -Version 1.0.0
```

**プロセス**:

1. `.env`からバージョンとアプリケーション情報を読み取り
2. MSIファイルのSHA256チェックサムを計算
3. `.choco/app.nuspec.template`から全ての変数を置換
4. `.nupkg`パッケージを生成

詳細: [.choco/README.md](../.choco/README.md)

#### `build-homebrew.sh`

macOS用Homebrewフォーミュラを生成

`.env`からバージョンとアプリケーション情報を読み取り、`.homebrew/app.rb.template`から`.homebrew/{APP_NAME_KEBAB}.rb`を生成し、全てのプレースホルダーを置換します。

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
