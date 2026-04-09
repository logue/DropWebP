# Windows Docker Linux 建置支援

## 摘要

專案已支援在 Windows 上透過 Docker 建置 Linux 套件（.deb / .rpm）。

## 使用方式

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
pwsh .\scripts\build-linux-docker.ps1 -Target x64
```

## 說明

- 需要先啟動 Docker Desktop
- 可在 `.env` 調整 CPU/記憶體
- 產物路徑：`app/src-tauri/target/<target>/release/bundle/`
