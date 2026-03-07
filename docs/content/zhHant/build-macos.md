# 為 macOS 建構 Tauri Vue3 App

本指南將引導您在 macOS 系統上設置開發環境並建構 Tauri Vue3 App。

## 先決條件

開始之前，請確保您有：

- macOS 10.15 (Catalina) 或更高版本
- 安裝軟體的管理員權限
- 對終端命令的基本了解

## 步驟 1：安裝 Xcode Command Line Tools

首先，安裝 Xcode Command Line Tools，它提供包括 `clang` 和 `make` 在內的基本開發工具：

```bash
xcode-select --install
```

這將開啟一個對話框，詢問您是否要安裝命令列開發工具。點擊 **安裝** 並等待安裝完成。

### 驗證安裝

檢查工具是否正確安裝：

```bash
clang --version
```

您應該看到類似的輸出：

```text
Apple clang version 15.0.0 (clang-1500.0.40.1)
Target: arm64-apple-darwin23.0.0
Thread model: posix
```

## 步驟 2：安裝 Homebrew

Homebrew 是 macOS 的套件管理器，使開發工具和程式庫的安裝變得容易。

### 安裝 Homebrew

開啟終端並執行：

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### 將 Homebrew 新增到 PATH

對於 Apple Silicon Mac (M1/M2/M3)，將 Homebrew 新增到您的 PATH：

```bash
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

對於 Intel Mac，Homebrew 安裝在 `/usr/local` 並且應該已經在您的 PATH 中。

### 驗證 Homebrew 安裝

```bash
brew --version
```

## 步驟 3：安裝 Rust

Tauri Vue3 App 使用 Rust 建構，因此您需要安裝 Rust 工具鏈。

### 透過 rustup 安裝 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

當提示時，選擇選項 1（預設安裝）。

### 設定您的 Shell

```bash
source ~/.cargo/env
```

### 驗證 Rust 安裝

```bash
rustc --version
cargo --version
```

您應該看到 `rustc` 和 `cargo` 的版本資訊。

## 步驟 4：安裝 Node.js

Tauri Vue3 App 的前端使用 Vue.js 建構，需要 Node.js。

### 透過 Homebrew 安裝 Node.js

```bash
brew install node
```

### 驗證 Node.js 安裝

```bash
node --version
npm --version
```

## 步驟 5：安裝 pnpm

Tauri Vue3 App 使用 pnpm 作為套件管理器，以獲得更好的效能和磁碟效率。

### 安裝 pnpm

```bash
brew install pnpm
```

### 驗證 pnpm 安裝

```bash
pnpm --version
```

## 步驟 6：設置 vcpkg 並安裝相依性

此項目使用 vcpkg 進行 C/C++ 程式庫的靜態連結。請編輯 `backend/setup-vcpkg.sh` 以定義你需要的任意程式庫。

### 安裝 vcpkg

```bash
# 複製 vcpkg
git clone https://github.com/Microsoft/vcpkg.git ~/Developer/vcpkg

# 引導 vcpkg
cd ~/Developer/vcpkg
./bootstrap-vcpkg.sh

# 設置環境變數（新增到 ~/.zshrc）
echo 'export VCPKG_ROOT="$HOME/Developer/vcpkg"' >> ~/.zshrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

### 安裝相依性

使用自動安裝腳本（推薦）：

```bash
cd ~/path/to/tauri-vuetify-starter/backend
./setup-vcpkg.sh
```

或手動安裝：

```bash
cd ~/Developer/vcpkg

# Apple Silicon (M1/M2/M3) 範例
./vcpkg install <package>:arm64-osx

# Intel Mac 範例
./vcpkg install <package>:x64-osx
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

## 步驟 8：平台特定注意事項

### Apple Silicon (M1/M2/M3) Mac

如果您使用 Apple Silicon Mac，某些相依性可能需要專門為 `arm64` 架構編譯。大多數現代套件都會自動處理這個問題，但如果遇到問題：

```bash
# 檢查您的架構
uname -m
# 應該輸出：arm64

# 如果需要，您可以強制 Rust 為正確的目標建構
rustup target add aarch64-apple-darwin
```

### Intel Mac

對於 Intel Mac，預設的 `x86_64` 目標應該可以正常工作：

```bash
# 檢查您的架構
uname -m
# 應該輸出：x86_64

# 確保安裝了正確的 Rust 目標
rustup target add x86_64-apple-darwin
```

### 程式碼簽署（可選）

如果您想分發建構的應用程式，您需要使用 Apple Developer 證書進行簽署：

```bash
# 檢查可用的簽署身分
security find-identity -v -p codesigning

# 如果您有開發者證書，Tauri 可以自動簽署
# 將此新增到您的 tauri.conf.json：
{
  "bundle": {
    "macOS": {
      "signing": {
        "identity": "Developer ID Application: Your Name (TEAM_ID)"
      }
    }
  }
}
```

## 故障排除

### 常見問題

1. **權限被拒絕錯誤**

   ```bash
   # 修復 Homebrew 權限
   sudo chown -R $(whoami) /opt/homebrew
   ```

2. **安裝後找不到命令**

   ```bash
   # 重新載入您的 shell 設定檔
   source ~/.zshrc
   # 或重新啟動您的終端
   ```

3. **原生相依性建構失敗**

   ```bash
   # 清理建構快取
   cargo clean
   pnpm clean

   # 重新建構所有內容
   pnpm install
   pnpm tauri build
   ```

4. **Rust 目標問題**

   ```bash
   # 列出已安裝的目標
   rustup target list --installed

   # 為您的系統新增正確的目標
   rustup target add aarch64-apple-darwin  # Apple Silicon
   rustup target add x86_64-apple-darwin   # Intel
   ```

### 為 Intel Mac 建構

您可以在 Apple Silicon Mac 上為 Intel Mac 建構二進位檔案，或直接在 Intel Mac 上建構。

#### 方法 1：Universal Binary（建議）

最簡單的方法是建構同時包含 ARM64 和 x86_64 二進位檔案的 Universal Binary：

```bash
pnpm run build:tauri:mac-universal
```

此方法不需要安裝額外的程式庫，並生成可在所有 Mac 上執行的單一二進位檔案。

#### 方法 2：僅 Intel 建構

如果您只需要 Intel 專用二進位檔案：

**在 Apple Silicon Mac 上交叉編譯：**

1. 安裝 x86_64 Homebrew 和所需程式庫：

   ```bash
   # 如果尚未安裝 x86_64 Homebrew
   arch -x86_64 /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

   # 安裝 x86_64 程式庫
   arch -x86_64 /usr/local/bin/brew install libavif jpeg-xl
   ```

   或使用提供的指令碼：

   ```bash
   bash scripts/setup-x86-libs.sh
   ```

2. 為 x86_64 目標建構：

   ```bash
   pnpm run build:tauri:mac-x64
   ```

**在 Intel Mac 上建構：**

在 Intel Mac 上可以直接建構：

```bash
pnpm run build:tauri:mac-x64
```

#### 建構目標概覽

| 命令                        | 架構                       | 平台          |
| --------------------------- | -------------------------- | ------------- |
| `build:tauri:mac-arm64`     | ARM64                      | Apple Silicon |
| `build:tauri:mac-x64`       | x86_64                     | Intel Mac     |
| `build:tauri:mac-universal` | Universal (ARM64 + x86_64) | 所有 Mac      |

#### 建構產物位置

建構產物根據目標生成在以下位置：

```
backend/target/
├── aarch64-apple-darwin/release/   # ARM64 建構
│   └── bundle/
├── x86_64-apple-darwin/release/    # Intel 建構
│   └── bundle/
└── universal-apple-darwin/release/ # Universal 建構
    └── bundle/
```

### 獲取協助

如果您遇到此處未涵蓋的問題：

1. 檢查 [Tauri Vue3 App 存儲庫](https://github.com/logue/tauri-vuetify-starter) 的已知問題
2. 查看 [Tauri v2 文件](https://v2.tauri.app/start/prerequisites/) 以獲取 macOS 特定指導
3. 搜尋現有的 GitHub 問題或建立新問題

## 下一步

成功建構 Tauri Vue3 App 後：

1. **執行測試**：執行 `pnpm test` 確保一切正常工作
2. **開發**：使用 `pnpm tauri dev` 進行熱重載開發
3. **自訂**：探索程式碼基礎並進行修改
4. **分發**：使用 `pnpm tauri build` 建立可分發的套件

您現在已準備好在 macOS 上開發和建構 Tauri Vue3 App！
