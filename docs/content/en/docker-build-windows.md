# Linux Build with Docker on Windows

This guide explains how to build Linux packages from Windows using Docker.

## Prerequisites

- Windows 10/11 (64-bit)
- Docker Desktop (WSL 2 backend recommended)
- PowerShell 5.1+
- At least 8 GB RAM (16 GB recommended)
- At least 20 GB free disk space

## Build Commands

From the project root:

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

Or run the script directly:

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

Build with AppImage:

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage
```

## Build Settings (.env)

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## Output Artifacts

```text
app/src-tauri/target/<target>/release/bundle/
```

- `.deb`
- `.rpm`
- `.AppImage` (optional)

## Troubleshooting

### Docker Desktop is not running

Start Docker Desktop and try again.

### Out of memory

Increase Docker memory or `BUILD_MEMORY` in `.env`.

### Slow builds

Increase CPU/memory, and keep Docker cache volumes.

## References

- [docker-build.md](docker-build.md)
- [Tauri Documentation](https://tauri.app/v1/guides/building/)
- [Docker Desktop for Windows](https://docs.docker.com/desktop/windows/)
