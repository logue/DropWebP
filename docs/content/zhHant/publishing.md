# 發佈套件

本指南說明如何將 Tauri Vue3 App 發佈到 Chocolatey 和 Homebrew。

## 先決條件

### Chocolatey (Windows)

- 安裝 Chocolatey: <https://chocolatey.org/install>
- 在 <https://community.chocolatey.org/> 建立帳戶
- 從帳戶頁面取得 API 金鑰

### Homebrew (macOS)

- 為 Homebrew tap 建立 GitHub 儲存庫（例如，`homebrew-tap`）
- 產生具有 `repo` 範圍的 GitHub Personal Access Token

## 建置套件

### 1. 建置 Tauri 應用程式

```bash
pnpm build:tauri
```

這將在以下位置建立特定平台的安裝程式：

- Windows: `backend/target/release/bundle/msi/`
- macOS: `backend/target/release/bundle/dmg/`
- Linux: `backend/target/release/bundle/deb/` 或 `appimage/`

### 2. 產生 Chocolatey 套件 (Windows)

```powershell
pnpm package:chocolatey
```

或手動執行：

```powershell
.\scripts\build-chocolatey.ps1 -Version {VERSION}
```

這將執行以下操作：

- 計算 MSI 檔案的 SHA256 校驗和
- 使用正確的校驗和更新 `chocolateyinstall.ps1`
- 建立 `.choco/{APP_NAME_KEBAB}.{VERSION}.nupkg`

### 3. 產生 Homebrew Formula (macOS)

```bash
pnpm package:homebrew
```

或手動執行：

```bash
./scripts/build-homebrew.sh {VERSION}
```

這將執行以下操作：

- 計算 ARM64 和 x64 DMG 檔案的 SHA256 校驗和
- 使用校驗和更新 `.homebrew/{APP_NAME_KEBAB}.rb`

## 發佈

### Chocolatey

1. **首先在本機測試：**

```powershell
choco install {APP_NAME_KEBAB} -source .choco
```

2. **推送到 Chocolatey Community Repository：**

```powershell
choco apikey --key YOUR-API-KEY --source https://push.chocolatey.org/
choco push .choco/{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

3. **監控審核佇列：**

- 前往 <https://community.chocolatey.org/packages/{APP_NAME_KEBAB}>
- 套件將由審核員審查（通常在 48 小時內）

### Homebrew

1. **建立 tap 儲存庫**（僅首次）：

```bash
# 在 GitHub 上建立新儲存庫：homebrew-tap
git clone https://github.com/YOUR-USERNAME/homebrew-tap.git
cd homebrew-tap
```

2. **複製 formula：**

```bash
cp .homebrew/{APP_NAME_KEBAB}.rb Formula/{APP_NAME_KEBAB}.rb
git add Formula/{APP_NAME_KEBAB}.rb
git commit -m "Add {APP_NAME_KEBAB} {VERSION}"
git push
```

3. **使用者現在可以安裝：**

```bash
brew tap YOUR-USERNAME/tap
brew install {APP_NAME_KEBAB}
```

### 透過 GitHub Actions 自動發佈

儲存庫包含一個 GitHub Actions 工作流程（`.github/workflows/release.yml`），可自動化整個過程。

#### 設定 Secrets

將這些 secrets 加入到 GitHub 儲存庫設定中：

1. **CHOCOLATEY_API_KEY**: 您的 Chocolatey API 金鑰
2. **HOMEBREW_TAP_TOKEN**: tap 儲存庫的 GitHub Personal Access Token

#### 觸發發佈

1. **更新 `tauri.conf.json` 和 `package.json` 中的版本**

2. **建立並推送 git 標籤：**

```bash
git tag v{VERSION}
git push origin v{VERSION}
```

3. **工作流程將自動執行以下操作：**
   - 為 Windows、macOS（ARM64 和 x64）和 Linux 建置
   - 建立 GitHub Release
   - 產生 Chocolatey 套件並推送到社群儲存庫
   - 產生 Homebrew formula 並建立 PR 到 tap 儲存庫

## 版本管理

始終保持以下版本同步：

- `package.json` → `version`
- `backend/tauri.conf.json` → `version`
- `backend/Cargo.toml` → `version`

## 疑難排解

### Chocolatey

**問題**：套件審核失敗

- 在 <https://community.chocolatey.org/> 檢視審核評論
- 常見問題：校驗和不正確、缺少相依性、安裝問題
- 修復後增加版本號重新提交

**問題**：校驗和不符

- 確保自校驗和計算後 MSI 檔案未變更
- 使用 `pnpm package:chocolatey` 重新建置套件

### Homebrew

**問題**：安裝時 SHA256 不符

- 驗證 DMG 檔案已上傳到 GitHub releases
- 確保 formula 中的 URL 與實際發佈資產相符
- 重新計算校驗和：`shasum -a 256 *.dmg`

**問題**：應用程式在 macOS 上無法開啟

- 確保應用程式已正確簽署和公證
- 檢查 Gatekeeper 是否封鎖：`xattr -d com.apple.quarantine /Applications/Tauri\ Vue3\ App.app`

## 資源

- [Chocolatey Package Creation](https://docs.chocolatey.org/en-us/create/create-packages)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Tauri Bundling](https://tauri.app/v1/guides/building/)

## Prerequisites

### Chocolatey (Windows)

- Install Chocolatey: <https://chocolatey.org/install>
- Create an account at <https://community.chocolatey.org/>
- Get your API key from your account page

### Homebrew (macOS)

- Create a GitHub repository for your Homebrew tap (e.g., `homebrew-tap`)
- Generate a GitHub Personal Access Token with `repo` scope

## Building Packages

### 1. Build the Tauri Application

```bash
pnpm build:tauri
```

This will create platform-specific installers in:

- Windows: `backend/target/release/bundle/msi/`
- macOS: `backend/target/release/bundle/dmg/`
- Linux: `backend/target/release/bundle/deb/` or `appimage/`

### 2. Generate Chocolatey Package (Windows)

```powershell
pnpm package:chocolatey
```

Or manually:

```powershell
.\scripts\build-chocolatey.ps1 -Version {VERSION}
```

This will:

- Calculate SHA256 checksum of the MSI file
- Update `chocolateyinstall.ps1` with the correct checksum
- Create `.choco/{APP_NAME_KEBAB}.{VERSION}.nupkg`

### 3. Generate Homebrew Formula (macOS)

```bash
pnpm package:homebrew
```

Or manually:

```bash
./scripts/build-homebrew.sh {VERSION}
```

This will:

- Calculate SHA256 checksums for both ARM64 and x64 DMG files
- Update `.homebrew/{APP_NAME_KEBAB}.rb` with checksums

## Publishing

### Chocolatey

1. **Test locally first:**

```powershell
choco install {APP_NAME_KEBAB} -source .choco
```

2. **Push to Chocolatey Community Repository:**

```powershell
choco apikey --key YOUR-API-KEY --source https://push.chocolatey.org/
choco push .choco/{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

3. **Monitor the moderation queue:**

- Go to <https://community.chocolatey.org/packages/{APP_NAME_KEBAB}>
- The package will be reviewed by moderators (usually within 48 hours)

### Homebrew

1. **Create a tap repository** (first time only):

```bash
# Create a new repository on GitHub: homebrew-tap
git clone https://github.com/YOUR-USERNAME/homebrew-tap.git
cd homebrew-tap
```

2. **Copy the formula:**

```bash
cp .homebrew/{APP_NAME_KEBAB}.rb Formula/{APP_NAME_KEBAB}.rb
git add Formula/{APP_NAME_KEBAB}.rb
git commit -m "Add {APP_NAME_KEBAB} {VERSION}"
git push
```

3. **Users can now install with:**

```bash
brew tap YOUR-USERNAME/tap
brew install {APP_NAME_KEBAB}
```

### Automated Publishing via GitHub Actions

The repository includes a GitHub Actions workflow (`.github/workflows/release.yml`) that automates the entire process.

#### Setup Secrets

Add these secrets to your GitHub repository settings:

1. **CHOCOLATEY_API_KEY**: Your Chocolatey API key
2. **HOMEBREW_TAP_TOKEN**: GitHub Personal Access Token for your tap repository

#### Trigger a Release

1. **Update version in `tauri.conf.json` and `package.json`**

2. **Create and push a git tag:**

```bash
git tag v{VERSION}
git push origin v{VERSION}
```

3. **The workflow will automatically:**
   - Build for Windows, macOS (ARM64 & x64), and Linux
   - Create a GitHub Release
   - Generate Chocolatey package and push to community repository
   - Generate Homebrew formula and create PR to tap repository

## Version Management

Always keep versions synchronized across:

- `package.json` → `version`
- `backend/tauri.conf.json` → `version`
- `backend/Cargo.toml` → `version`

## Troubleshooting

### Chocolatey

**Issue**: Package fails moderation

- Check the moderation comments at <https://community.chocolatey.org/>
- Common issues: incorrect checksums, missing dependencies, installation issues
- Fix and resubmit with an incremented version

**Issue**: Checksum mismatch

- Ensure the MSI file hasn't changed since checksum calculation
- Rebuild the package with `pnpm package:chocolatey`

### Homebrew

**Issue**: SHA256 mismatch during installation

- Verify the DMG files are uploaded to GitHub releases
- Ensure URLs in the formula match the actual release assets
- Recalculate checksums: `shasum -a 256 Drop.Compress.Image_*.dmg`

**Issue**: App won't open on macOS

- Ensure the app is properly signed and notarized
- Check if Gatekeeper is blocking: `xattr -d com.apple.quarantine /Applications/Drop\ Compress\ Image.app`

## Resources

- [Chocolatey Package Creation](https://docs.chocolatey.org/en-us/create/create-packages)
- [Homebrew Formula Cookbook](https://docs.brew.sh/Formula-Cookbook)
- [Tauri Bundling](https://tauri.app/v1/guides/building/)
