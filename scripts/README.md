# Build Scripts

このディレクトリには、プロジェクトのビルドとデプロイに使用するスクリプトが含まれています。

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

```powershell
pnpm run package:chocolatey
```

#### `build-homebrew.sh`

macOS用Homebrewフォーミュラを生成

```powershell
pnpm run package:homebrew
```

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
