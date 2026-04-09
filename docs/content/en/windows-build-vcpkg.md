# Windows Build with vcpkg

This guide explains how to build on Windows with statically linked dependencies from vcpkg.

## Important Triplet

Use `x64-windows-static-release` for release builds.

## Prerequisites

- Visual Studio 2019/2022 (MSVC)
- Rust toolchain
- Git
- PowerShell
- LLVM/Clang (for bindgen in `jxl-sys`)

## Install LLVM

```powershell
winget install LLVM.LLVM
clang --version
```

## Install vcpkg

```powershell
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat
$env:VCPKG_ROOT = "C:\vcpkg"
```

Create custom triplet:

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

Install dependencies:

```powershell
.\vcpkg install aom:x64-windows-static-release
.\vcpkg install libavif[aom]:x64-windows-static-release
.\vcpkg install libjxl:x64-windows-static-release
.\vcpkg install libwebp:x64-windows-static-release
.\vcpkg install openjpeg:x64-windows-static-release
.\vcpkg install libjpeg-turbo:x64-windows-static-release
.\vcpkg install lcms:x64-windows-static-release
```

## Build

```powershell
$env:VCPKG_ROOT = "C:\vcpkg"
$env:VCPKGRS_TRIPLET = "x64-windows-static-release"
cd app\src-tauri
cargo build --release
```
