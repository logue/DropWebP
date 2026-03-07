# 패키지 배포

이 가이드는 Tauri Vue3 App을 Chocolatey와 Homebrew에 배포하는 방법을 설명합니다.

## 사전 요구사항

### Chocolatey (Windows)

- Chocolatey 설치: <https://chocolatey.org/install>
- <https://community.chocolatey.org/>에서 계정 생성
- 계정 페이지에서 API 키 가져오기

### Homebrew (macOS)

- Homebrew tap용 GitHub 저장소 생성 (예: `homebrew-tap`)
- `repo` 범위를 가진 GitHub Personal Access Token 생성

## 패키지 빌드

### 1. Tauri 애플리케이션 빌드

```bash
pnpm build:tauri
```

다음 위치에 플랫폼별 설치 프로그램이 생성됩니다:

- Windows: `backend/target/release/bundle/msi/`
- macOS: `backend/target/release/bundle/dmg/`
- Linux: `backend/target/release/bundle/deb/` 또는 `appimage/`

### 2. Chocolatey 패키지 생성 (Windows)

```powershell
pnpm package:chocolatey
```

또는 수동으로:

```powershell
.\scripts\build-chocolatey.ps1 -Version {VERSION}
```

다음 작업이 수행됩니다:

- MSI 파일의 SHA256 체크섬 계산
- 올바른 체크섬으로 `chocolateyinstall.ps1` 업데이트
- `.choco/{APP_NAME_KEBAB}.{VERSION}.nupkg` 생성

### 3. Homebrew Formula 생성 (macOS)

```bash
pnpm package:homebrew
```

또는 수동으로:

```bash
./scripts/build-homebrew.sh {VERSION}
```

다음 작업이 수행됩니다:

- ARM64 및 x64 DMG 파일의 SHA256 체크섬 계산
- 체크섬으로 `.homebrew/{APP_NAME_KEBAB}.rb` 업데이트

## 배포

### Chocolatey

1. **먼저 로컬에서 테스트:**

```powershell
choco install {APP_NAME_KEBAB} -source .choco
```

2. **Chocolatey Community Repository에 푸시:**

```powershell
choco apikey --key YOUR-API-KEY --source https://push.chocolatey.org/
choco push .choco/{APP_NAME_KEBAB}.{VERSION}.nupkg --source https://push.chocolatey.org/
```

3. **승인 대기열 모니터링:**

- <https://community.chocolatey.org/packages/{APP_NAME_KEBAB}>로 이동
- 패키지는 중재자에 의해 검토됩니다 (보통 48시간 이내)

### Homebrew

1. **tap 저장소 생성** (최초 1회):

```bash
# GitHub에 새 저장소 생성: homebrew-tap
git clone https://github.com/YOUR-USERNAME/homebrew-tap.git
cd homebrew-tap
```

2. **formula 복사:**

```bash
cp .homebrew/{APP_NAME_KEBAB}.rb Formula/{APP_NAME_KEBAB}.rb
git add Formula/{APP_NAME_KEBAB}.rb
git commit -m "Add {APP_NAME_KEBAB} {VERSION}"
git push
```

3. **사용자는 다음과 같이 설치 가능:**

```bash
brew tap YOUR-USERNAME/tap
brew install {APP_NAME_KEBAB}
```

### GitHub Actions를 통한 자동 배포

저장소에는 전체 프로세스를 자동화하는 GitHub Actions 워크플로우(`.github/workflows/release.yml`)가 포함되어 있습니다.

#### Secret 설정

GitHub 저장소 설정에 다음 secret을 추가합니다:

1. **CHOCOLATEY_API_KEY**: Chocolatey API 키
2. **HOMEBREW_TAP_TOKEN**: tap 저장소용 GitHub Personal Access Token

#### 릴리스 트리거

1. **`tauri.conf.json`과 `package.json`의 버전 업데이트**

2. **git 태그 생성 및 푸시:**

```bash
git tag v{VERSION}
git push origin v{VERSION}
```

3. **워크플로우가 자동으로 다음을 수행:**
   - Windows, macOS (ARM64 & x64), Linux용 빌드
   - GitHub Release 생성
   - Chocolatey 패키지 생성 및 커뮤니티 저장소에 푸시
   - Homebrew formula 생성 및 tap 저장소에 PR 생성

## 버전 관리

항상 다음의 버전을 동기화하세요:

- `package.json` → `version`
- `backend/tauri.conf.json` → `version`
- `backend/Cargo.toml` → `version`

## 문제 해결

### Chocolatey

**문제**: 패키지 승인 실패

- <https://community.chocolatey.org/>에서 중재 댓글 확인
- 일반적인 문제: 잘못된 체크섬, 누락된 종속성, 설치 문제
- 수정 후 버전을 증가시켜 재제출

**문제**: 체크섬 불일치

- 체크섬 계산 이후 MSI 파일이 변경되지 않았는지 확인
- `pnpm package:chocolatey`로 패키지 재빌드

### Homebrew

**문제**: 설치 중 SHA256 불일치

- DMG 파일이 GitHub releases에 업로드되었는지 확인
- formula의 URL이 실제 릴리스 자산과 일치하는지 확인
- 체크섬 재계산: `shasum -a 256 *.dmg`

**문제**: macOS에서 앱이 열리지 않음

- 앱이 올바르게 서명되고 공증되었는지 확인
- Gatekeeper가 차단하는지 확인: `xattr -d com.apple.quarantine /Applications/Tauri\ Vue3\ App.app`

## 리소스

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
