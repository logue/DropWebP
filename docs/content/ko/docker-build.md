# Docker를 이용한 Linux 빌드

이 문서는 Docker 기반 Linux 패키징 절차를 설명합니다.

## 지원 호스트

- Windows
- macOS (Intel / Apple Silicon)
- Linux

## 빠른 시작

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

## 요구 사항

- Docker Desktop 또는 Docker Engine
- pnpm 10.2.0+
- RAM 8GB 이상
- 여유 디스크 20GB 이상

## .env 옵션

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## 캐시 볼륨

- `dropwebp-cargo-cache-linux-amd64`
- `dropwebp-pnpm-cache-linux-amd64`
- `dropwebp-target-cache-linux-amd64`
- `dropwebp-cargo-cache-linux-arm64`
- `dropwebp-pnpm-cache-linux-arm64`
- `dropwebp-target-cache-linux-arm64`
