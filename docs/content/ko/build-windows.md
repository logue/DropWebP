# 개발 환경 설정 (Windows)

Windows에서 Drop Compress Image의 개발 환경을 설정하는 가이드입니다.

## 빌드 방법 선택

Windows에서 빌드하는 두 가지 방법이 있습니다:

1. **Docker 환경 빌드 (권장)**: 깨끗한 환경으로 의존성 충돌 방지
2. **네이티브 환경 빌드**: 더 빠르지만 설정이 복잡함

---

## 방법 1: Docker 환경 빌드 (권장)

### 사전 요구사항

- Windows 10/11 Pro, Enterprise 또는 Education (Hyper-V 지원 필요)
- Docker Desktop for Windows

### 단계

1. **Docker Desktop 설치**

   [Docker Desktop](https://www.docker.com/products/docker-desktop)을 다운로드하여 설치합니다.

2. **Windows 컨테이너 모드로 전환**

   Docker Desktop 트레이 아이콘을 우클릭하고 "Switch to Windows containers..."를 선택합니다.

3. **프로젝트 클론**

   ```powershell
   git clone https://github.com/logue/DropWebP.git
   cd DropWebP
   ```

4. **Docker 이미지 빌드** (처음에만, 30-60분 소요)

   ```powershell
   docker build -f Dockerfile.windows-x64 -t dropwebp-windows-builder .
   ```

5. **애플리케이션 빌드**

   ```powershell
   docker run --rm -v ${PWD}:C:\workspace dropwebp-windows-builder
   ```

6. **빌드 결과물 확인**

   빌드 성공 시 실행 파일과 설치 프로그램이 `app/src-tauri/target/release/bundle/` 디렉토리에 생성됩니다.

### Docker 환경의 장점

- ✅ 호스트 환경을 깨끗하게 유지
- ✅ 의존성 충돌 방지
- ✅ 재현 가능한 빌드
- ✅ 깨끗한 빌드 환경
- ✅ CI/CD 파이프라인과의 일관성

---

## 방법 2: 네이티브 환경 빌드

## 1. Chocolatey 설치

관리자 권한으로 PowerShell을 열고 다음 명령어를 실행하여 Chocolatey를 설치합니다.

```powershell
Set-ExecutionPolicy Bypass -Scope Process -Force;
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072;
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
```

설치 후 아래 명령어로 버전을 확인할 수 있습니다.

```powershell
choco -v
```

## 2. Git 설치

Chocolatey를 사용하여 Git을 설치합니다.

```powershell
choco install git -y
```

설치 후 버전을 확인합니다.

```powershell
git --version
```

## 3. 프로젝트 클론

GitHub에서 프로젝트를 클론하고 프로젝트 디렉토리로 이동합니다.

```powershell
git clone https://github.com/logue/DropWebP.git
cd DropWebP
```

## 4. Visual Studio Community 2022 설치

Visual Studio Community 2022를 설치합니다.

```powershell
choco install visualstudio2022community -y
```

다음으로 C++ 데스크톱 개발 워크로드를 설치합니다.

```powershell
choco install visualstudio2022-workload-nativedesktop -y
```

Clang/LLVM 빌드 도구를 설치합니다. 이는 일부 이미지 코덱 라이브러리 빌드에 필요합니다.

```powershell
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Component.VC.Llvm.Clang --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset" -y
```

설치가 완료되면 Visual Studio Installer를 사용하여 설치된 구성 요소를 확인할 수 있습니다.

> **참고:** C++ 데스크톱 개발 워크로드에는 MSVC(마이크로소프트 컴파일러), Windows SDK 및 CMake와 같은 Rust 네이티브 확장 빌드에 필요한 도구가 포함되어 있습니다.

## 5. NASM 및 Ninja 설치

이미지 코덱 라이브러리 빌드에 필요한 NASM 및 Ninja를 설치합니다.

```powershell
choco install nasm ninja -y
```

설치 후 버전을 확인합니다.

```powershell
nasm -v
ninja --version
```

Cargo가 빌드 시 NASM을 찾을 수 있도록 시스템 PATH에 NASM을 추가합니다.

```powershell
[System.Environment]::SetEnvironmentVariable('PATH', [System.Environment]::GetEnvironmentVariable('PATH', 'User') + ';C:\Program Files\NASM', 'User')
```

PATH 변경 사항이 적용되도록 터미널 또는 PowerShell 세션을 다시 시작합니다.

> **참고:** NASM(Netwide Assembler)은 libavif와 같은 최적화된 코덱 라이브러리 빌드에 사용되는 어셈블러입니다. Ninja는 CMake와 함께 자주 사용되는 빠른 빌드 시스템입니다.

## 6. Node.js 및 pnpm 설치

Node.js 및 pnpm을 설치합니다.

```powershell
choco install nodejs pnpm -y
```

설치 후 버전을 확인합니다.

```powershell
node -v
pnpm -v
```

## 7. Rust 설치 (공식 방법)

PowerShell 또는 명령 프롬프트에서 다음 명령어를 실행하여 공식 방법으로 Rust를 설치합니다.

```powershell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

설치 후 버전을 확인합니다.

```powershell
rustc --version
```

> **경고:** Chocolatey를 통해 Rust를 설치할 수도 있지만 MinGW 툴체인으로 설치되어 라이브러리와의 호환성 문제가 발생할 수 있습니다.

## 8. vcpkg 설정

1. vcpkg 리포지토리를 클론합니다:

   ```powershell
   git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
   cd C:\vcpkg
   ```

2. 부트스트랩 스크립트를 실행합니다:

   ```powershell
   .\bootstrap-vcpkg.bat
   ```

3. 환경 변수를 설정합니다(시스템 환경 변수에 추가하는 것이 권장됨):

   ```powershell
   $env:VCPKG_ROOT = "C:\vcpkg"
   [System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
   ```

> **중요:** VCPKG_ROOT 환경 변수는 빌드 시스템이 vcpkg 라이브러리를 찾는 데 필요합니다.

## 9. 종속성 설치

### 릴리스 트리플릿 생성

vcpkg의 기본 트리플릿에는 디버그 심볼이 포함되어 있어 Rust 릴리스 빌드에서 링크 오류가 발생합니다. 사용자 지정 트리플릿을 생성합니다:

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

### 종속성 설치

> **참고 (2026년 2월 업데이트)**: 프로젝트는 이제 Windows에서 AVIF 인코딩을 위해 `rav1e`(Rust 기반 AV1 인코더)를 사용합니다. 이로 인해 `libaom` 및 `aom` 패키지가 더 이상 필요하지 않습니다. `rav1e`는 NASM의 멀티패스 최적화 요구 사항을 회피하고 Windows에서 빌드 안정성을 향상시킵니다.

자동 설치 스크립트 사용(권장):

```powershell
cd DropWebP\app\src-tauri
.\setup-vcpkg.ps1
```

또는 수동으로 설치:

```powershell
cd C:\vcpkg

# x64-windows-static-release 트리플릿으로 설치(릴리스 전용)
# 참고: aom 및 libavif[aom]은 더 이상 필요하지 않습니다(rav1e 사용)
.\vcpkg install libjxl:x64-windows-static-release
.\vcpkg install libwebp:x64-windows-static-release
.\vcpkg install openjpeg:x64-windows-static-release
.\vcpkg install libjpeg-turbo:x64-windows-static-release
.\vcpkg install lcms:x64-windows-static-release
```

설치된 라이브러리:

- **rav1e**: AV1 인코더(Rust 기반, AVIF 인코딩용) - Cargo에 의해 자동으로 빌드됨
- **libjxl**: JPEG XL 이미지 형식
- **libwebp**: WebP 이미지 형식
- **openjpeg**: JPEG 2000 이미지 형식
- **libjpeg-turbo**: JPEG 이미지 처리(jpegli용)
- **lcms**: Little CMS 색상 관리

> **macOS/Linux 사용자 참고**: macOS와 Linux는 NASM 및 CMake 구성이 더 안정적이므로 여전히 `libaom`을 사용할 수 있습니다.

설치 확인:

```powershell
.\vcpkg list | Select-String "jxl|webp|openjpeg|jpeg|lcms"
```

## 10. 애플리케이션 빌드

1. app 디렉토리로 이동하고 종속성을 설치합니다:

   ```powershell
   cd app
   pnpm install
   ```

2. 개발 모드로 애플리케이션을 빌드하고 실행합니다:

   ```powershell
   pnpm run dev:tauri
   ```

3. 프로덕션 빌드의 경우:

   ```powershell
   pnpm run build:tauri
   ```

이제 애플리케이션이 Windows에서 성공적으로 빌드되어야 합니다. 문제가 발생하면 모든 종속성이 올바르게 설치되었고 환경 변수가 올바르게 설정되었는지 확인하세요.

---

## Arm64 Windows 크로스 빌드

x64 Windows 머신에서 Arm64 Windows(Windows on ARM)용으로 크로스 빌드할 수 있습니다.

### 사전 요구사항

- 위에서 설명한 대로 설정된 x64 빌드 환경
- Arm64 대상용 vcpkg 종속성

### 1. Rust 툴체인 추가

```powershell
rustup target add aarch64-pc-windows-msvc
```

### 2. Arm64용 vcpkg 종속성 설치

Arm64용 릴리스 트리플릿 생성(아직 완료하지 않은 경우):

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE arm64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\arm64-windows-static-release.cmake
```

종속성 설치:

```powershell
cd C:\vcpkg

# 참고: aom 및 libavif[aom]은 더 이상 필요하지 않습니다(rav1e 사용)
.\vcpkg install libjxl:arm64-windows-static-release
.\vcpkg install libwebp:arm64-windows-static-release
.\vcpkg install openjpeg:arm64-windows-static-release
.\vcpkg install libjpeg-turbo:arm64-windows-static-release
.\vcpkg install lcms:arm64-windows-static-release
```

### 3. Arm64용 빌드

```powershell
cd path\to\DropWebP\app
pnpm run build:tauri:windows-arm64
```

또는 수동으로 빌드:

```powershell
cd app\src-tauri
cargo build --release --target aarch64-pc-windows-msvc
cd ..
pnpm tauri build --target aarch64-pc-windows-msvc
```

### 참고 사항

- Arm64 바이너리는 Arm64 Windows 장치(Surface Pro X 등)에서만 실행됩니다
- 크로스 빌드된 바이너리는 x64 머신에서 실행할 수 없습니다
- 빌드 결과물은 `app/src-tauri/target/aarch64-pc-windows-msvc/release/`에 생성됩니다
