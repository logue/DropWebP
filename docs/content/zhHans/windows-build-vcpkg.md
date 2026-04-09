# Windows 使用 vcpkg 构建

本文介绍在 Windows 上使用 vcpkg 静态链接 C/C++ 依赖库。

## 重要 Triplet

发布构建建议使用 `x64-windows-static-release`。

## 前置条件

- Visual Studio 2019/2022 (MSVC)
- Rust toolchain
- Git
- PowerShell
- LLVM/Clang（jxl-sys bindgen 需要）

## 安装 LLVM

```powershell
winget install LLVM.LLVM
clang --version
```

## 安装 vcpkg

```powershell
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat
$env:VCPKG_ROOT = "C:\vcpkg"
```

## 构建

```powershell
$env:VCPKG_ROOT = "C:\vcpkg"
$env:VCPKGRS_TRIPLET = "x64-windows-static-release"
cd app\src-tauri
cargo build --release
```
