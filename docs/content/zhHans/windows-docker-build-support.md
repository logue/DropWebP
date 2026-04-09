# Windows Docker Linux 构建支持

## 概要

项目已支持在 Windows 上通过 Docker 构建 Linux 包（.deb / .rpm）。

## 使用方法

```powershell
pnpm run build:tauri:linux-x64
pnpm run build:tauri:linux-arm64
pwsh .\scripts\build-linux-docker.ps1 -Target x64
```

## 说明

- 需要先启动 Docker Desktop
- 可在 `.env` 调整 CPU/内存
- 产物路径：`app/src-tauri/target/<target>/release/bundle/`
