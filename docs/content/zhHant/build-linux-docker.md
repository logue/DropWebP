# Linux建置（使用Docker）

如何在Windows、macOS或Linux上使用Docker建置Linux二進位檔案

## 📋 前置要求

### 所有平台通用

- Docker Desktop或Docker Engine
- pnpm (v10.2.0或更高版本)
- 8GB以上記憶體（16GB推薦）
- 20GB以上空閒磁碟空間

### 平台特定要求

#### Windows

- Windows 10/11 (64位元)
- WSL 2（推薦）
- PowerShell 5.1或更高版本

#### macOS

- macOS 10.15或更高版本
- Bash
- Docker Desktop for Mac

#### Linux

- 64位元Linux發行版
- Docker Engine 20.10或更高版本
- Bash

## 🚀 使用方法

### 在Windows上建置

```powershell
# 從專案根目錄執行
pnpm run build:tauri:linux-x64    # x86_64 Linux
pnpm run build:tauri:linux-arm64  # ARM64 Linux

# 或直接執行腳本
pwsh .\scripts\build-linux-docker.ps1 -Target x64
pwsh .\scripts\build-linux-docker.ps1 -Target arm64
```

### 在macOS / Linux上建置

```bash
# 從專案根目錄執行
bash scripts/build-linux-docker.sh x64    # x86_64 Linux
bash scripts/build-linux-docker.sh arm64  # ARM64 Linux

# 或從app目錄執行
pnpm run build:tauri:linux-docker-x64
pnpm run build:tauri:linux-docker-arm64
```

## 📦 建置產物

建置產物將生成在以下目錄中：

```text
app/src-tauri/target/
  ├── x86_64-unknown-linux-gnu/release/bundle/
  │   ├── deb/           # Debian/Ubuntu軟體包
  │   ├── rpm/           # Red Hat/Fedora軟體包
  │   └── appimage/      # AppImage（推薦用於分發）
  │
  └── aarch64-unknown-linux-gnu/release/bundle/
      ├── deb/
      ├── rpm/
      └── appimage/
```

## ⚙️ 運作原理

1. 從`Dockerfile.linux-build`建置Docker映像
   - 基於Rust 1.83 + Debian Bookworm
   - 安裝Tauri相依套件（WebKit2GTK、GTK3等）
   - 安裝Node.js 22.x和pnpm

2. 在Docker容器內執行Tauri建置
   - 掛載專案目錄
   - 使用指定的目標架構進行建置

3. 將產物輸出到macOS目錄

## 🔧 疑難排解

### 重新建置Docker映像

```bash
docker build -f Dockerfile.linux-build -t dropwebp-linux-builder --no-cache .
```

### 刪除Docker映像

```bash
docker rmi dropwebp-linux-builder
```

### 清除建置快取

```bash
rm -rf app/src-tauri/target/x86_64-unknown-linux-gnu
rm -rf app/src-tauri/target/aarch64-unknown-linux-gnu
```

## 📝 注意事項

- 初次建置由於Docker映像建置和下載需要較長時間（20-30分鐘）
- 後續建置會重用Docker映像，速度較快（10-15分鐘）
- ARM64建置可能比x86_64建置需要更長時間

## 🎯 推薦的分發格式

- **AppImage**：推薦用於分發（適用於所有Linux發行版）
- **.deb**：適用於Debian/Ubuntu使用者
- **.rpm**：適用於Red Hat/Fedora使用者
