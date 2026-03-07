# 為 Linux 建構 Tauri Vue3 App

本指南將引導您在 Ubuntu 24.04 LTS（和類似的基於 Debian 的發行版）上設置開發環境並建構 Tauri Vue3 App。

## 先決條件

開始之前，請確保您有：

- Ubuntu 24.04 LTS 或類似的基於 Debian 的發行版
- 安裝軟體的 sudo 權限
- 對終端命令的基本了解

## 步驟 1：更新系統套件

首先，更新系統套件以確保您有最新版本：

```bash
sudo apt update
sudo apt upgrade -y
```

## 步驟 2：安裝建構相依性

安裝 Tauri 開發所需的基本建構工具和程式庫：

```bash
# 安裝建構必需品和開發程式庫
sudo apt install -y \
  build-essential \
  curl \
  wget \
  file \
  libssl-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libwebkit2gtk-4.1-dev \
  patchelf
```

### 這些套件的作用

- **build-essential**: 提供 GCC、G++ 和 make
- **libssl-dev**: OpenSSL 開發程式庫
- **libgtk-3-dev**: UI 的 GTK3 開發程式庫
- **libayatana-appindicator3-dev**: 系統匣支援
- **librsvg2-dev**: SVG 渲染支援
- **libwebkit2gtk-4.1-dev**: Tauri 的 webview 用 WebKit
- **patchelf**: AppImage 的 ELF 二進位修補程式

### 驗證安裝

```bash
gcc --version
```

您應該看到顯示 GCC 版本 13.x 或更高版本的輸出。

## 步驟 3：安裝 Rust

Tauri Vue3 App 使用 Rust 建構，因此您需要安裝 Rust 工具鏈。

### 透過 rustup 安裝 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

當提示時，選擇選項 1（預設安裝）。

### 設定您的 Shell

```bash
source $HOME/.cargo/env
```

要使其永久化，請將其新增到您的 shell 設定檔：

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

### 驗證 Rust 安裝

```bash
rustc --version
cargo --version
```

您應該看到 `rustc` 和 `cargo` 的版本資訊。

## 步驟 4：安裝 Node.js

Tauri Vue3 App 的前端使用 Vue.js 建構，需要 Node.js。

### 透過 NodeSource 存儲庫安裝 Node.js

```bash
# 安裝 Node.js 22.x (LTS)
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
```

### 驗證 Node.js 安裝

```bash
node --version
npm --version
```

您應該看到 Node.js 版本 22.x 或更高版本。

## 步驟 5：安裝 pnpm

Tauri Vue3 App 使用 pnpm 作為套件管理器，以獲得更好的效能和磁碟效率。

### 安裝 pnpm

```bash
npm install -g pnpm
```

### 驗證 pnpm 安裝

```bash
pnpm --version
```

## 步驟 6：設置 vcpkg 並安裝相依性

此項目使用 vcpkg 進行 C/C++ 程式庫的靜態連結。請編輯 `backend/setup-vcpkg.sh` 以定義你需要的任意程式庫。

### 安裝 vcpkg 先決條件

```bash
# 安裝 vcpkg 所需的工具
sudo apt install -y curl zip unzip tar cmake pkg-config
```

### 安裝 vcpkg

```bash
# 複製 vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg

# 引導 vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh

# 設置環境變數（新增到 ~/.bashrc）
echo 'export VCPKG_ROOT="$HOME/vcpkg"' >> ~/.bashrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 安裝相依性

使用自動安裝腳本（推薦）：

```bash
cd ~/path/to/tauri-vuetify-starter/backend
./setup-vcpkg.sh
```

或手動安裝：

```bash
cd ~/vcpkg

# x64 Linux 範例
./vcpkg install <package>:x64-linux

# ARM64 Linux 範例
./vcpkg install <package>:arm64-linux
```

安裝哪些程式庫取決於你在 `backend/setup-vcpkg.sh` 中的定義。

### 驗證安裝

```bash
./vcpkg list
```

## 步驟 7：複製和建構 Tauri Vue3 App

現在您已準備好複製和建構 Tauri Vue3 App。

### 複製存儲庫

```bash
git clone https://github.com/logue/tauri-vuetify-starter.git
cd tauri-vuetify-starter
```

### 安裝前端相依性

```bash
# 安裝所有工作區相依性
pnpm install
```

### 安裝 Tauri CLI v2

```bash
# 全域安裝 Tauri CLI v2
pnpm add -g @tauri-apps/cli@next
```

### 建構應用程式

開發模式：

```bash
# 在開發模式下執行
pnpm dev:tauri
```

生產模式：

```bash
# 為生產建構
pnpm build:tauri
```

建構的應用程式將在 `backend/target/release/` 中。

## 步驟 8：分發格式

Linux 上的 Tauri 可以生成多種分發格式：

### AppImage（推薦）

AppImage 是一種通用套件格式，可在大多數 Linux 發行版上執行：

```bash
pnpm build:tauri
```

AppImage 將在 `backend/target/release/bundle/appimage/` 中。

### Debian 套件 (.deb)

對於基於 Debian/Ubuntu 的發行版：

```bash
pnpm build:tauri
```

.deb 套件將在 `backend/target/release/bundle/deb/` 中。

安裝它：

```bash
sudo dpkg -i backend/target/release/bundle/deb/*.deb
```

### RPM 套件 (.rpm)

對於基於 Red Hat/Fedora 的發行版，您需要安裝額外的工具：

```bash
sudo apt install -y rpm
pnpm build:tauri
```

.rpm 套件將在 `backend/target/release/bundle/rpm/` 中。

## 故障排除

### 常見問題

1. **缺少 libwebkit2gtk-4.1**

   如果您收到有關缺少 webkit 程式庫的錯誤：

   ```bash
   # 嘗試舊版 webkit
   sudo apt install -y libwebkit2gtk-4.0-dev
   ```

2. **npm/pnpm 權限被拒絕**

   ```bash
   # 修復 npm 全域目錄權限
   mkdir -p ~/.npm-global
   npm config set prefix '~/.npm-global'
   echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **原生相依性建構失敗**

   ```bash
   # 清理建構快取
   cargo clean
   pnpm clean

   # 重新建構所有內容
   pnpm install
   pnpm build:tauri
   ```

4. **AppImage 不可執行**

   ```bash
   # 使 AppImage 可執行
   chmod +x backend/target/release/bundle/appimage/*.AppImage
   ```

5. **缺少 GLIBC 版本**

   如果您看到有關 GLIBC 版本的錯誤，請確保您使用的是 Ubuntu 24.04 LTS 或更新版本：

   ```bash
   ldd --version
   ```

### 圖形驅動程式問題

為了獲得最佳效能，請確保已安裝適當的圖形驅動程式：

```bash
# 對於 NVIDIA
sudo ubuntu-drivers autoinstall

# 對於 AMD
sudo apt install -y mesa-vulkan-drivers

# 對於 Intel
sudo apt install -y intel-media-va-driver
```

### 獲取協助

如果您遇到此處未涵蓋的問題：

1. 檢查 [Tauri Vue3 App 存儲庫](https://github.com/logue/tauri-vuetify-starter) 的已知問題
2. 查看 [Tauri v2 文件](https://v2.tauri.app/start/prerequisites/) 以獲取 Linux 特定指導
3. 搜尋現有的 GitHub 問題或建立新問題

## 下一步

成功建構 Tauri Vue3 App 後：

1. **執行測試**：執行 `pnpm test` 確保一切正常工作
2. **開發**：使用 `pnpm dev:tauri` 進行熱重載開發
3. **自訂**：探索程式碼基礎並進行修改
4. **分發**：使用 `pnpm build:tauri` 建立可分發的套件

您現在已準備好在 Linux 上開發和建構 Tauri Vue3 App！
