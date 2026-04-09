# 使用 Docker 建置 Linux 套件

本文說明如何在多平台上透過 Docker 進行 Linux 打包。

## 支援平台

- Windows
- macOS (Intel / Apple Silicon)
- Linux

## 快速開始

### Windows

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

### macOS / Linux

```bash
bash scripts/build-linux-docker.sh x64
bash scripts/build-linux-docker.sh arm64
```

## 環境需求

- Docker Desktop 或 Docker Engine
- pnpm 10.2.0+
- 記憶體至少 8GB
- 磁碟空間至少 20GB

## .env 參數

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## 快取 Volume

- `dropwebp-cargo-cache-linux-amd64`
- `dropwebp-pnpm-cache-linux-amd64`
- `dropwebp-target-cache-linux-amd64`
- `dropwebp-cargo-cache-linux-arm64`
- `dropwebp-pnpm-cache-linux-arm64`
- `dropwebp-target-cache-linux-arm64`
