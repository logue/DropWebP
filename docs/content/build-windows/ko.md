# 개발 환경 설정 (Windows)

Windows에서 Drop Compress Image의 개발 환경을 설정하는 가이드입니다.

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

## 8. vcpkg 설정 (공식 지침)

원하는 디렉토리에서 다음 명령어를 실행하여 vcpkg를 클론합니다.

```powershell
git clone https://github.com/microsoft/vcpkg.git
```

vcpkg 디렉토리로 이동하여 설정 명령어를 실행합니다.

```powershell
cd vcpkg
.\bootstrap-vcpkg.bat
```

vcpkg.exe의 경로(예: `C:\path\to\vcpkg`)를 환경 변수 `PATH`에 추가합니다. 방법: 시스템 환경 변수의 "Path"에 vcpkg 디렉토리를 추가하세요.

또한 환경 변수 `VCPKG_DEFAULT_TRIPLET`을 추가하고 값을 `x64-windows-static-md`로 설정합니다. 이렇게 하면 64비트 Windows용 정적 라이브러리가 기본적으로 설치됩니다.

설치 후 아래 명령어로 경로가 설정되었는지와 버전을 확인합니다.

```powershell
vcpkg version
```

> **경고:** vcpkg 경로에 영숫자가 아닌 문자가 포함되어 있으면 제대로 작동하지 않을 수 있습니다. 드라이브 루트와 같은 위치에 클론하는 것이 좋습니다.

## 9. 필요한 라이브러리 설치

다음 명령어를 실행하여 이미지 변환에 필요한 라이브러리를 설치합니다.

```powershell
vcpkg install libavif libheif libjxl libwebp libjpeg-turbo libpng
```

> **참고:** 설치하는 데 시간이 걸릴 수 있으며 일부 라이브러리는 빌드에 실패할 수 있습니다. 이 경우 vcpkg 문서를 참조하세요.

## 10. 종속성 설치 및 빌드

프로젝트 디렉토리에서 종속성을 설치합니다.

```powershell
pnpm install
```

개발 모드에서 애플리케이션을 시작합니다.

```powershell
cd app
pnpm tauri dev
```

> **완료:** 첫 번째 시작 시 Rust 종속성 컴파일에 시간이 걸릴 수 있습니다. 애플리케이션 창이 나타나면 개발 환경 설정이 완료된 것입니다.
