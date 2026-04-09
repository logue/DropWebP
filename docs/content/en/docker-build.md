# Linux Build with Docker

This document describes cross-platform Linux packaging with Docker.

## Supported Hosts

- Windows
- macOS (Intel / Apple Silicon)
- Linux

## Quick Start

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

## Requirements

- Docker Desktop or Docker Engine
- pnpm 10.2.0+
- 8 GB RAM minimum
- 20 GB free disk

## Build Flow

1. Build Docker image
2. Install dependencies
3. Build Rust code
4. Build frontend
5. Package with Tauri
6. Copy artifacts

## Environment Variables

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## Outputs

```text
app/src-tauri/target/<target>/release/bundle/
```

## Cache Volumes

- `dropwebp-cargo-cache-linux-amd64`
- `dropwebp-pnpm-cache-linux-amd64`
- `dropwebp-target-cache-linux-amd64`
- `dropwebp-cargo-cache-linux-arm64`
- `dropwebp-pnpm-cache-linux-arm64`
- `dropwebp-target-cache-linux-arm64`

## Troubleshooting

- Docker not found: install/start Docker
- Memory issues: increase Docker memory
- AppImage failures: enable AppImage explicitly
- Platform mismatch: check Docker Buildx
