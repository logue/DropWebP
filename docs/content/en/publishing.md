# Publishing Packages

This guide explains how to publish Tauri Vue3 App to Chocolatey and Homebrew.

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
