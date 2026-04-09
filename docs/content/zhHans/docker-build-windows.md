# 在 Windows 上使用 Docker 构建 Linux 包

本文介绍如何在 Windows 环境中使用 Docker 构建 Linux 安装包。

## 前置条件

- Windows 10/11 (64-bit)
- Docker Desktop（推荐启用 WSL 2）
- PowerShell 5.1+
- 至少 8GB 内存（推荐 16GB）
- 至少 20GB 可用磁盘空间

## 构建命令

在项目根目录执行：

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

或直接执行脚本：

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

包含 AppImage：

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage
```

## .env 构建参数

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## 输出目录

```text
app/src-tauri/target/<target>/release/bundle/
```

## 常见问题

- Docker Desktop 未启动：先启动 Docker 再重试
- 内存不足：提高 Docker 内存或 `BUILD_MEMORY`
- 构建慢：提高 CPU/内存并保留缓存卷
