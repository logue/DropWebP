# 使用 Docker 构建 Linux 包

本文说明如何在多平台上通过 Docker 进行 Linux 打包。

## 支持平台

- Windows
- macOS (Intel / Apple Silicon)
- Linux

## 快速开始

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

## 环境要求

- Docker Desktop 或 Docker Engine
- pnpm 10.2.0+
- 内存至少 8GB
- 磁盘空闲至少 20GB

## .env 参数

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## 缓存卷

- `dropwebp-cargo-cache-linux-amd64`
- `dropwebp-pnpm-cache-linux-amd64`
- `dropwebp-target-cache-linux-amd64`
- `dropwebp-cargo-cache-linux-arm64`
- `dropwebp-pnpm-cache-linux-arm64`
- `dropwebp-target-cache-linux-arm64`
