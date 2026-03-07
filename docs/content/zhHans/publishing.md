# 发布包

本指南说明如何将 Tauri Vue3 App 发布到 Chocolatey 和 Homebrew。

## 前提条件

### Chocolatey (Windows)

- 安装 Chocolatey: <https://chocolatey.org/install>
- 在 <https://community.chocolatey.org/> 创建账户
- 从账户页面获取 API 密钥

### Homebrew (macOS)

- 为 Homebrew tap 创建 GitHub 仓库（例如，`homebrew-tap`）
- 生成具有 `repo` 范围的 GitHub Personal Access Token

## 构建包

### 1. 构建 Tauri 应用程序

```bash
pnpm build:tauri
```

这将在以下位置创建特定平台的安装程序：

- Windows: `backend/target/release/bundle/msi/`
- macOS: `backend/target/release/bundle/dmg/`
- Linux: `backend/target/release/bundle/deb/` 或 `appimage/`

### 2. 生成 Chocolatey 包 (Windows)

```powershell
pnpm package:chocolatey
```

或手动执行：

```powershell
.\scripts\build-chocolatey.ps1 -Version {VERSION}
```

这将执行以下操作：

- 计算 MSI 文件的 SHA256 校验和
- 使用正确的校验和更新 `chocolateyinstall.ps1`
- 创建 `.choco/{APP_NAME_KEBAB}.{VERSION}.nupkg`

### 3. 生成 Homebrew Formula (macOS)

```bash
pnpm package:homebrew
```

或手动执行：

```bash
./scripts/build-homebrew.sh {VERSION}
```

这将执行以下操作：

- 计算 ARM64 和 x64 DMG 文件的 SHA256 校验和
- 使用校验和更新 `.homebrew/{APP_NAME_KEBAB}.rb`

## 发布

### Chocolatey

1. **首先在本地测试：**

```powershell
choco install {APP_NAME_KEBAB} -source .choco
```

2. **推送到 Chocolatey Community Repository：**

```powershell
choco apikey --key YOUR-API-KEY --source https://push.chocolatey.org/
choco push .choco/{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

3. **监控审核队列：**

- 访问 <https://community.chocolatey.org/packages/{APP_NAME_KEBAB}>
- 包将由审核员审查（通常在 48 小时内）

### Homebrew

1. **创建 tap 仓库**（仅首次）：

```bash
# 在 GitHub 上创建新仓库：homebrew-tap
git clone https://github.com/YOUR-USERNAME/homebrew-tap.git
cd homebrew-tap
```

2. **复制 formula：**

```bash
cp .homebrew/{APP_NAME_KEBAB}.rb Formula/{APP_NAME_KEBAB}.rb
git add Formula/{APP_NAME_KEBAB}.rb
git commit -m "Add {APP_NAME_KEBAB} {VERSION}"
git push
```

3. **用户现在可以安装：**

```bash
brew tap YOUR-USERNAME/tap
brew install {APP_NAME_KEBAB}
```

### 通过 GitHub Actions 自动发布

仓库包含一个 GitHub Actions 工作流（`.github/workflows/release.yml`），可自动化整个过程。

#### 设置 Secrets

将这些 secrets 添加到 GitHub 仓库设置中：

1. **CHOCOLATEY_API_KEY**: 您的 Chocolatey API 密钥
2. **HOMEBREW_TAP_TOKEN**: tap 仓库的 GitHub Personal Access Token

#### 触发发布

1. **更新 `tauri.conf.json` 和 `package.json` 中的版本**

2. **创建并推送 git 标签：**

```bash
git tag v{VERSION}
git push origin v{VERSION}
```

3. **工作流将自动执行以下操作：**
   - 为 Windows、macOS（ARM64 和 x64）和 Linux 构建
   - 创建 GitHub Release
   - 生成 Chocolatey 包并推送到社区仓库
   - 生成 Homebrew formula 并创建 PR 到 tap 仓库

## 版本管理

始终保持以下版本同步：

- `package.json` → `version`
- `backend/tauri.conf.json` → `version`
- `backend/Cargo.toml` → `version`

## 故障排除

### Chocolatey

**问题**：包审核失败

- 在 <https://community.chocolatey.org/> 查看审核评论
- 常见问题：校验和不正确、缺少依赖项、安装问题
- 修复后增加版本号重新提交

**问题**：校验和不匹配

- 确保自校验和计算后 MSI 文件未更改
- 使用 `pnpm package:chocolatey` 重新构建包

### Homebrew

**问题**：安装时 SHA256 不匹配

- 验证 DMG 文件已上传到 GitHub releases
- 确保 formula 中的 URL 与实际发布资产匹配
- 重新计算校验和：`shasum -a 256 *.dmg`

**问题**：应用在 macOS 上无法打开

- 确保应用已正确签名和公证
- 检查 Gatekeeper 是否阻止：`xattr -d com.apple.quarantine /Applications/Tauri\ Vue3\ App.app`

## 资源

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
