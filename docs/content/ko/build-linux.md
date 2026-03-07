# Linux용 Tauri Vue3 App 빌드

이 가이드는 Ubuntu 24.04 LTS(및 유사한 Debian 기반 배포판)에서 개발 환경을 설정하고 Tauri Vue3 App를 빌드하는 과정을 안내합니다。

## 사전 요구사항

시작하기 전에 다음이 필요합니다:

- Ubuntu 24.04 LTS 또는 유사한 Debian 기반 배포판
- 소프트웨어 설치를 위한 sudo 권한
- 터미널 명령어에 대한 기본적인 지식

## 단계 1: 시스템 패키지 업데이트

먼저 최신 버전이 있는지 확인하기 위해 시스템 패키지를 업데이트합니다:

```bash
sudo apt update
sudo apt upgrade -y
```

## 단계 2: 빌드 종속성 설치

Tauri 개발에 필요한 필수 빌드 도구 및 라이브러리를 설치합니다:

```bash
# 빌드 필수 요소 및 개발 라이브러리 설치
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

### 패키지 설명

- **build-essential**: GCC, G++, make 제공
- **libssl-dev**: OpenSSL 개발 라이브러리
- **libgtk-3-dev**: UI용 GTK3 개발 라이브러리
- **libayatana-appindicator3-dev**: 시스템 트레이 지원
- **librsvg2-dev**: SVG 렌더링 지원
- **libwebkit2gtk-4.1-dev**: Tauri의 webview용 WebKit
- **patchelf**: AppImage용 ELF 바이너리 패처

### 설치 확인

```bash
gcc --version
```

GCC 버전 13.x 이상이 표시되어야 합니다.

## 단계 3: Rust 설치

Tauri Vue3 App는 Rust로 빌드되므로 Rust 툴체인을 설치해야 합니다.

### rustup을 통한 Rust 설치

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

프롬프트가 표시되면 옵션 1(기본 설치)을 선택하세요.

### 셸 구성

```bash
source $HOME/.cargo/env
```

영구적으로 만들려면 셸 프로파일에 추가하세요:

```bash
echo 'source $HOME/.cargo/env' >> ~/.bashrc
source ~/.bashrc
```

### Rust 설치 확인

```bash
rustc --version
cargo --version
```

`rustc`와 `cargo` 모두에 대한 버전 정보가 표시되어야 합니다.

## 단계 4: Node.js 설치

Tauri Vue3 App의 프론트엔드는 Vue.js로 빌드되어 Node.js가 필요합니다.

### NodeSource 저장소를 통한 Node.js 설치

```bash
# Node.js 22.x (LTS) 설치
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs
```

### Node.js 설치 확인

```bash
node --version
npm --version
```

Node.js 버전 22.x 이상이 표시되어야 합니다.

## 단계 5: pnpm 설치

Tauri Vue3 App는 성능과 디스크 효율성을 위해 pnpm을 패키지 관리자로 사용합니다.

### pnpm 설치

```bash
npm install -g pnpm
```

### pnpm 설치 확인

```bash
pnpm --version
```

## 단계 6: vcpkg 설정 및 종속성 설치

이 프로젝트는 vcpkg를 사용해 C/C++ 라이브러리를 정적 링크합니다. 필요한 라이브러리는 `backend/setup-vcpkg.sh`를 수정해 정의하세요.

### vcpkg 선행 조건 설치

```bash
# vcpkg에 필요한 도구 설치
sudo apt install -y curl zip unzip tar cmake pkg-config
```

### vcpkg 설치

```bash
# vcpkg 복제
git clone https://github.com/Microsoft/vcpkg.git ~/vcpkg

# vcpkg 부트스트랩
cd ~/vcpkg
./bootstrap-vcpkg.sh

# 환경 변수 설정(~/.bashrc에 추가)
echo 'export VCPKG_ROOT="$HOME/vcpkg"' >> ~/.bashrc
echo 'export PATH="$VCPKG_ROOT:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### 종속성 설치

자동 설치 스크립트 사용(권장):

```bash
cd ~/path/to/tauri-vuetify-starter/backend
./setup-vcpkg.sh
```

또는 수동으로 설치:

```bash
cd ~/vcpkg

# x64 Linux 예시
./vcpkg install <package>:x64-linux

# ARM64 Linux 예시
./vcpkg install <package>:arm64-linux
```

설치되는 라이브러리는 `backend/setup-vcpkg.sh`의 설정에 따라 달라집니다.

### 설치 확인

```bash
./vcpkg list
```

## 단계 7: Tauri Vue3 App 복제 및 빌드

이제 Tauri Vue3 App를 복제하고 빌드할 준비가 되었습니다.

### 리포지토리 복제

```bash
git clone https://github.com/logue/tauri-vuetify-starter.git
cd tauri-vuetify-starter
```

### 프론트엔드 종속성 설치

```bash
# 모든 워크스페이스 종속성 설치
pnpm install
```

### Tauri CLI v2 설치

```bash
# Tauri CLI v2를 전역으로 설치
pnpm add -g @tauri-apps/cli@next
```

### 애플리케이션 빌드

개발용:

```bash
# 개발 모드로 실행
pnpm dev:tauri
```

프로덕션용:

```bash
# 프로덕션용 빌드
pnpm build:tauri
```

빌드된 애플리케이션은 `backend/target/release/`에 있습니다.

## 단계 8: 배포 형식

Linux의 Tauri는 여러 배포 형식을 생성할 수 있습니다:

### AppImage (권장)

AppImage는 대부분의 Linux 배포판에서 작동하는 범용 패키지 형식입니다:

```bash
pnpm build:tauri
```

AppImage는 `backend/target/release/bundle/appimage/`에 있습니다.

### Debian 패키지 (.deb)

Debian/Ubuntu 기반 배포판용:

```bash
pnpm build:tauri
```

.deb 패키지는 `backend/target/release/bundle/deb/`에 있습니다.

설치:

```bash
sudo dpkg -i backend/target/release/bundle/deb/*.deb
```

### RPM 패키지 (.rpm)

Red Hat/Fedora 기반 배포판의 경우 추가 도구를 설치해야 합니다:

```bash
sudo apt install -y rpm
pnpm build:tauri
```

.rpm 패키지는 `backend/target/release/bundle/rpm/`에 있습니다.

## 문제 해결

### 일반적인 문제

1. **libwebkit2gtk-4.1 누락**

   webkit 라이브러리 누락에 대한 오류가 발생하면:

   ```bash
   # 이전 webkit 버전 시도
   sudo apt install -y libwebkit2gtk-4.0-dev
   ```

2. **npm/pnpm 권한 거부**

   ```bash
   # npm 전역 디렉토리 권한 수정
   mkdir -p ~/.npm-global
   npm config set prefix '~/.npm-global'
   echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.bashrc
   source ~/.bashrc
   ```

3. **네이티브 종속성으로 인한 빌드 실패**

   ```bash
   # 빌드 캐시 정리
   cargo clean
   pnpm clean

   # 모든 것을 다시 빌드
   pnpm install
   pnpm build:tauri
   ```

4. **AppImage 실행 불가**

   ```bash
   # AppImage를 실행 가능하게 만들기
   chmod +x backend/target/release/bundle/appimage/*.AppImage
   ```

5. **GLIBC 버전 누락**

   GLIBC 버전에 대한 오류가 표시되면 Ubuntu 24.04 LTS 이상인지 확인하세요:

   ```bash
   ldd --version
   ```

### 그래픽 드라이버 문제

최적의 성능을 위해 적절한 그래픽 드라이버가 설치되어 있는지 확인하세요:

```bash
# NVIDIA용
sudo ubuntu-drivers autoinstall

# AMD용
sudo apt install -y mesa-vulkan-drivers

# Intel용
sudo apt install -y intel-media-va-driver
```

### 도움 받기

여기서 다루지 않은 문제가 발생하면:

1. 알려진 문제에 대해 [Tauri Vue3 App 리포지토리](https://github.com/logue/tauri-vuetify-starter) 확인
2. Linux 관련 가이드는 [Tauri v2 문서](https://v2.tauri.app/start/prerequisites/) 참고
3. 기존 GitHub 이슈 검색하거나 새 이슈 생성

## 다음 단계

Tauri Vue3 App가 성공적으로 빌드되면:

1. **테스트 실행**: `pnpm test`를 실행하여 모든 것이 올바르게 작동하는지 확인
2. **개발**: 핫 리로딩이 포함된 개발에는 `pnpm dev:tauri` 사용
3. **사용자 정의**: 코드베이스를 탐색하고 수정 사항 적용
4. **배포**: 배포 가능한 패키지를 만들려면 `pnpm build:tauri` 사용

이제 Linux에서 Tauri Vue3 App를 개발하고 빌드할 준비가 되었습니다!
