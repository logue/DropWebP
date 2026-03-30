# Windows環境でのDockerを使用したLinuxビルド

このガイドでは、Windows環境からDockerを使用してLinux向けのビルドを行う方法を説明します。

## 前提条件

### 1. Docker Desktopのインストール

1. [Docker Desktop for Windows](https://www.docker.com/products/docker-desktop/) をダウンロードしてインストール
2. Docker Desktopを起動し、完全に起動するまで待つ
3. 設定で「Use WSL 2 based engine」が有効になっていることを確認（推奨）

### 2. システム要件

- **Windows 10/11** (64-bit)
- **WSL 2** (推奨)
- **メモリ**: 最低 8GB RAM (16GB推奨)
- **ディスク空き容量**: 最低 20GB

### 3. PowerShellの要件

PowerShell 5.1以上が必要です（Windows 10/11には標準搭載）

```powershell
# PowerShellのバージョン確認
$PSVersionTable.PSVersion
```

## ビルド方法

### 基本的なビルド

プロジェクトルートから以下のコマンドを実行：

```powershell
# x86_64 (AMD64) Linux用
pnpm run build:tauri:linux-x64

# ARM64 Linux用
pnpm run build:tauri:linux-arm64
```

または、直接スクリプトを実行：

```powershell
# x86_64ビルド
pwsh .\scripts\build-linux-docker.ps1 -Target x64

# ARM64ビルド
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

### AppImageを含めたビルド

デフォルトでは `.deb` と `.rpm` のみが生成されます。AppImageも生成したい場合：

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage
```

> **注意**: AppImageのビルドにはFUSEが必要で、Docker環境では制限があります。

## ビルド設定のカスタマイズ

`.env` ファイルでビルド設定をカスタマイズできます：

```bash
# Docker Build Settings
BUILD_CPUS=4              # 使用するCPUコア数
BUILD_MEMORY=8g           # メモリ制限
CARGO_BUILD_JOBS=4        # Cargoの並列ジョブ数
MAKEFLAGS=-j4             # Makeの並列度
INCLUDE_APPIMAGE=false    # AppImageを含めるか
```

## ビルド成果物

ビルドが完了すると、以下の場所に成果物が生成されます：

```
app/src-tauri/target/<target>/release/bundle/
├── deb/
│   └── drop-compress-image_<version>_<arch>.deb
└── rpm/
    └── drop-compress-image-<version>-1.<arch>.rpm
```

例：

- `app/src-tauri/target/x86_64-unknown-linux-gnu/release/bundle/deb/`
- `app/src-tauri/target/aarch64-unknown-linux-gnu/release/bundle/deb/`

## トラブルシューティング

### Docker Desktopが起動していない

```
❌ エラー: Docker Desktop が起動していません。
```

**解決方法**: Docker Desktopを起動してから、再度実行してください。

### メモリ不足エラー

```
error: linking with `cc` failed: exit status: 1
```

**解決方法**: `.env`ファイルでメモリ制限を増やす、またはDocker Desktopの設定でメモリを増やします：

1. Docker Desktop → Settings → Resources → Memory
2. メモリスライダーを増やす（推奨: 8GB以上）
3. Apply & Restartをクリック

### ビルドが遅い

**解決方法**: CPUコア数とメモリを増やす

```bash
# .envファイルに追加
BUILD_CPUS=8
BUILD_MEMORY=16g
CARGO_BUILD_JOBS=8
```

### WSL 2バックエンドが無効

**解決方法**:

1. Docker Desktop → Settings → General
2. "Use the WSL 2 based engine"にチェック
3. Apply & Restart

### ビルドキャッシュをクリアしたい

Dockerボリュームを削除：

```powershell
docker volume rm dropwebp-cargo-cache-linux-amd64
docker volume rm dropwebp-pnpm-cache-linux-amd64
docker volume rm dropwebp-target-cache-linux-amd64
```

ARM64の場合：

```powershell
docker volume rm dropwebp-cargo-cache-linux-arm64
docker volume rm dropwebp-pnpm-cache-linux-arm64
docker volume rm dropwebp-target-cache-linux-arm64
```

## 高度な使用方法

### クロスプラットフォームビルド

Windows環境から両方のアーキテクチャをビルド：

```powershell
# x86_64とARM64の両方をビルド
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

### カスタムDockerfile

独自のDockerfileを使用する場合：

```powershell
docker build -f YourDockerfile.linux-x64 -t your-builder .
```

### デバッグモード

詳細なログ出力を有効にする：

```powershell
$env:VERBOSE = "1"
pwsh .\scripts\build-linux-docker.ps1 -Target x64
```

## パフォーマンス最適化

### 推奨設定（高性能PC）

```bash
# .envファイル
BUILD_CPUS=12
BUILD_MEMORY=16g
CARGO_BUILD_JOBS=12
MAKEFLAGS=-j12
```

### 推奨設定（一般的なPC）

```bash
# .envファイル
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
```

## 参考資料

- [Docker Desktop for Windows](https://docs.docker.com/desktop/windows/)
- [WSL 2](https://docs.microsoft.com/en-us/windows/wsl/install)
- [Tauri Documentation](https://tauri.app/v1/guides/building/)

## 関連ドキュメント

- [DOCKER_BUILD.md](docker-build.md) - macOS/Linuxでのビルド手順
- [WINDOWS_BUILD_VCPKG.md](./WINDOWS_BUILD_VCPKG.md) - Windowsネイティブビルド
- [MACOS_COMPATIBILITY.md](./MACOS_COMPATIBILITY.md) - macOS互換性情報
