# Linux용 빌드 (Docker 사용)

macOS에서 Docker를 사용하여 Linux 바이너리를 빌드하는 방법

## 📋 전제 조건

- Docker Desktop for Mac이 설치되어 있어야 합니다
- 충분한 디스크 공간 (초기 빌드에 약 5GB 필요)

## 🚀 사용 방법

### x86_64 (AMD64)용 빌드

```bash
# 프로젝트 루트에서 실행
./scripts/build-linux-docker.sh x64

# 또는 app 디렉토리에서
pnpm run build:tauri:linux-docker-x64
```

### ARM64 (AArch64)용 빌드

```bash
# 프로젝트 루트에서 실행
./scripts/build-linux-docker.sh arm64

# 또는 app 디렉토리에서
pnpm run build:tauri:linux-docker-arm64
```

## 📦 빌드 결과물

빌드 결과물은 다음 디렉토리에 생성됩니다:

```text
app/src-tauri/target/
  ├── x86_64-unknown-linux-gnu/release/bundle/
  │   ├── deb/           # Debian/Ubuntu 패키지
  │   ├── rpm/           # Red Hat/Fedora 패키지
  │   └── appimage/      # AppImage (배포 권장)
  │
  └── aarch64-unknown-linux-gnu/release/bundle/
      ├── deb/
      ├── rpm/
      └── appimage/
```

## ⚙️ 작동 방식

1. `Dockerfile.linux-build`에서 Docker 이미지 빌드
   - Rust 1.83 + Debian Bookworm 기반
   - Tauri 종속성 설치 (WebKit2GTK, GTK3 등)
   - Node.js 22.x 및 pnpm 설치

2. Docker 컨테이너 내에서 Tauri 빌드 실행
   - 프로젝트 디렉토리 마운트
   - 지정된 대상 아키텍처로 빌드

3. macOS 디렉토리에 결과물 출력

## 🔧 문제 해결

### Docker 이미지 재빌드

```bash
docker build -f Dockerfile.linux-build -t dropwebp-linux-builder --no-cache .
```

### Docker 이미지 제거

```bash
docker rmi dropwebp-linux-builder
```

### 빌드 캐시 삭제

```bash
rm -rf app/src-tauri/target/x86_64-unknown-linux-gnu
rm -rf app/src-tauri/target/aarch64-unknown-linux-gnu
```

## 📝 참고 사항

- 초기 빌드는 Docker 이미지 빌드 및 다운로드로 인해 시간이 오래 걸립니다 (20-30분)
- 이후 빌드는 Docker 이미지가 재사용되므로 빠릅니다 (10-15분)
- ARM64 빌드는 x86_64 빌드보다 시간이 더 걸릴 수 있습니다

## 🎯 권장 배포 형식

- **AppImage**: 배포 권장 (모든 Linux 배포판에서 작동)
- **.deb**: Debian/Ubuntu 사용자용
- **.rpm**: Red Hat/Fedora 사용자용
