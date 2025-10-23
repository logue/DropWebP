# macOS용 DropWebP 빌드

이 가이드는 macOS 시스템에서 개발 환경을 설정하고 DropWebP를 빌드하는 과정을 안내합니다.

## 사전 요구사항

시작하기 전에 다음이 필요합니다:

- macOS 10.15 (Catalina) 이상
- 소프트웨어 설치를 위한 관리자 권한
- 터미널 명령어에 대한 기본적인 지식

## 단계 1: Xcode Command Line Tools 설치

먼저 `clang`과 `make`를 포함한 필수 개발 도구를 제공하는 Xcode Command Line Tools를 설치합니다:

```bash
xcode-select --install
```

이렇게 하면 명령줄 개발자 도구를 설치할지 묻는 대화 상자가 열립니다. **설치**를 클릭하고 설치가 완료될 때까지 기다리세요.

### 설치 확인

도구가 올바르게 설치되었는지 확인하세요:

```bash
clang --version
```

다음과 같은 출력이 표시되어야 합니다:

```text
Apple clang version 15.0.0 (clang-1500.0.40.1)
Target: arm64-apple-darwin23.0.0
Thread model: posix
```

## 단계 2: Homebrew 설치

Homebrew는 개발 도구와 라이브러리 설치를 쉽게 해주는 macOS용 패키지 관리자입니다.

### Homebrew 설치

터미널을 열고 실행하세요:

```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

### PATH에 Homebrew 추가

Apple Silicon Mac (M1/M2/M3)의 경우, PATH에 Homebrew를 추가하세요:

```bash
echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zshrc
source ~/.zshrc
```

Intel Mac의 경우, Homebrew는 `/usr/local`에 설치되어 이미 PATH에 포함되어 있어야 합니다.

### Homebrew 설치 확인

```bash
brew --version
```

## 단계 3: Rust 설치

DropWebP는 Rust로 빌드되므로 Rust 툴체인을 설치해야 합니다.

### rustup을 통한 Rust 설치

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

프롬프트가 표시되면 옵션 1 (기본 설치)을 선택하세요.

### 셸 구성

```bash
source ~/.cargo/env
```

### Rust 설치 확인

```bash
rustc --version
cargo --version
```

`rustc`와 `cargo` 모두에 대한 버전 정보가 표시되어야 합니다.

## 단계 4: Node.js 설치

DropWebP의 프론트엔드는 Vue.js로 빌드되어 Node.js가 필요합니다.

### Homebrew를 통한 Node.js 설치

```bash
brew install node
```

### Node.js 설치 확인

```bash
node --version
npm --version
```

## 단계 5: pnpm 설치

DropWebP는 성능과 디스크 효율성을 위해 pnpm을 패키지 관리자로 사용합니다.

### pnpm 설치

```bash
npm install -g pnpm
```

### pnpm 설치 확인

```bash
pnpm --version
```

## 단계 6: 추가 종속성 설치

빌드에 필요한 추가 도구를 설치합니다:

```bash
# CMake 설치 (일부 네이티브 종속성에 필요)
brew install cmake

# pkg-config 설치 (라이브러리 링킹에 필요)
brew install pkg-config
```

## 단계 7: DropWebP 복제 및 빌드

이제 DropWebP를 복제하고 빌드할 준비가 되었습니다.

### 리포지토리 복제

```bash
git clone https://github.com/logue/DropWebP.git
cd DropWebP
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

## 단계 8: 플랫폼별 고려사항

### Apple Silicon (M1/M2/M3) Mac

Apple Silicon Mac을 사용하는 경우, 일부 종속성은 `arm64` 아키텍처용으로 특별히 컴파일되어야 할 수 있습니다. 최신 패키지들은 이를 자동으로 처리하지만, 문제가 발생하면:

```bash
# 아키텍처 확인
uname -m
# 출력: arm64

# 필요한 경우, Rust가 올바른 타겟용으로 빌드하도록 강제할 수 있습니다
rustup target add aarch64-apple-darwin
```

### Intel Mac

Intel Mac의 경우, 기본 `x86_64` 타겟이 문제없이 작동해야 합니다:

```bash
# 아키텍처 확인
uname -m
# 출력: x86_64

# 올바른 Rust 타겟이 설치되었는지 확인
rustup target add x86_64-apple-darwin
```

### 코드 서명 (선택사항)

빌드된 애플리케이션을 배포하려면 Apple Developer 인증서로 서명해야 합니다:

```bash
# 사용 가능한 서명 신원 확인
security find-identity -v -p codesigning

# 개발자 인증서가 있다면, Tauri가 자동으로 서명할 수 있습니다
# tauri.conf.json에 다음을 추가하세요:
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

## 문제 해결

### 일반적인 문제

1. **권한 거부 오류**

   ```bash
   # Homebrew 권한 수정
   sudo chown -R $(whoami) /opt/homebrew
   ```

2. **설치 후 명령어를 찾을 수 없음**

   ```bash
   # 셸 프로파일 다시 로드
   source ~/.zshrc
   # 또는 터미널 재시작
   ```

3. **네이티브 종속성으로 인한 빌드 실패**

   ```bash
   # 빌드 캐시 정리
   cargo clean
   pnpm clean

   # 모든 것을 다시 빌드
   pnpm install
   pnpm tauri build
   ```

4. **Rust 타겟 문제**

   ```bash
   # 설치된 타겟 나열
   rustup target list --installed

   # 시스템에 맞는 올바른 타겟 추가
   rustup target add aarch64-apple-darwin  # Apple Silicon
   rustup target add x86_64-apple-darwin   # Intel
   ```

### 도움 받기

여기서 다루지 않은 문제가 발생하면:

1. 알려진 문제에 대해 [DropWebP 리포지토리](https://github.com/logue/DropWebP) 확인
2. macOS 관련 가이드는 [Tauri v2 문서](https://v2.tauri.app/start/prerequisites/) 참고
3. 기존 GitHub 이슈 검색하거나 새 이슈 생성

## 다음 단계

DropWebP가 성공적으로 빌드되면:

1. **테스트 실행**: `pnpm test`를 실행하여 모든 것이 올바르게 작동하는지 확인
2. **개발**: 핫 리로딩이 포함된 개발에는 `pnpm tauri dev` 사용
3. **사용자 정의**: 코드베이스를 탐색하고 수정 사항 적용
4. **배포**: 배포 가능한 패키지를 만들려면 `pnpm tauri build` 사용

이제 macOS에서 DropWebP를 개발하고 빌드할 준비가 되었습니다!
