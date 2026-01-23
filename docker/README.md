# Docker Configuration

このディレクトリには、プラットフォーム横断ビルド用のDockerファイルが含まれています。

## Dockerファイル

### Linux ビルド用

- `Dockerfile.linux-x64` - x86_64 (AMD64) Linux用ビルド環境
- `Dockerfile.linux-arm64` - ARM64 (AArch64) Linux用ビルド環境

### Windows ビルド用（実験的）

- `Dockerfile.windows-x64` - x86_64 Windows用ビルド環境
- `Dockerfile.windows-x86_64-msvc` - MSVC ツールチェーン用

## 使用方法

### Linux ビルド

プロジェクトルートから：

```bash
# x64ビルド
./scripts/docker/docker-build.sh x64

# ARM64ビルド
./scripts/docker/docker-build.sh arm64
```

Windowsから（PowerShell）：

```powershell
# x64ビルド
pnpm run build:tauri:linux-docker-x64

# ARM64ビルド
pnpm run build:tauri:linux-docker-arm64
```

## 詳細情報

- [Docker Build Guide](../docs/content/ja/docker-build.md) - Linux/macOSでの使用方法
- [Docker Build on Windows](../docs/content/ja/docker-build-windows.md) - Windowsでの使用方法
- [Build Scripts](../scripts/docker/) - Dockerビルドスクリプト
