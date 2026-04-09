# Windows에서 vcpkg로 빌드

Windows에서 vcpkg를 사용해 C/C++ 라이브러리를 정적 링크하는 방법입니다.

## 중요 트리플릿

릴리스 빌드는 `x64-windows-static-release` 사용을 권장합니다.

## 준비 사항

- Visual Studio 2019/2022 (MSVC)
- Rust toolchain
- Git
- PowerShell
- LLVM/Clang (jxl-sys bindgen용)

## LLVM 설치

```powershell
winget install LLVM.LLVM
clang --version
```

## vcpkg 설치

```powershell
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat
$env:VCPKG_ROOT = "C:\vcpkg"
```

## 빌드

```powershell
$env:VCPKG_ROOT = "C:\vcpkg"
$env:VCPKGRS_TRIPLET = "x64-windows-static-release"
cd app\src-tauri
cargo build --release
```
