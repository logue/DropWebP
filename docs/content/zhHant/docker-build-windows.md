# 在 Windows 上使用 Docker 建置 Linux 套件

本文說明如何在 Windows 環境中使用 Docker 建置 Linux 安裝套件。

## 前置需求

- Windows 10/11 (64-bit)
- Docker Desktop（建議啟用 WSL 2）
- PowerShell 5.1+
- 記憶體至少 8GB（建議 16GB）
- 可用磁碟空間至少 20GB

## 建置指令

在專案根目錄執行：

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
```

或直接執行腳本：

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

包含 AppImage：

```powershell
pwsh .\scripts\build-linux-docker.ps1 -Target x64 -IncludeAppImage
```

## .env 建置參數

```bash
BUILD_CPUS=4
BUILD_MEMORY=8g
CARGO_BUILD_JOBS=4
MAKEFLAGS=-j4
INCLUDE_APPIMAGE=false
```

## 輸出目錄

```text
app/src-tauri/target/<target>/release/bundle/
```

## 常見問題

- Docker Desktop 未啟動：先啟動後再重試
- 記憶體不足：提高 Docker 記憶體或 `BUILD_MEMORY`
- 建置較慢：提高 CPU/記憶體並保留快取 Volume
