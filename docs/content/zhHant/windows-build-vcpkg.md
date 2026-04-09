# Windows 使用 vcpkg 建置

本文介紹如何在 Windows 上使用 vcpkg 靜態連結 C/C++ 相依庫。

## 重要 Triplet

發佈建置建議使用 `x64-windows-static-release`。

## 前置需求

- Visual Studio 2019/2022 (MSVC)
- Rust toolchain
- Git
- PowerShell
- LLVM/Clang（jxl-sys bindgen 需要）

## 安裝 LLVM

```powershell
winget install LLVM.LLVM
clang --version
```

## 安裝 vcpkg

```powershell
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat
$env:VCPKG_ROOT = "C:\vcpkg"
```

## 建置

```powershell
$env:VCPKG_ROOT = "C:\vcpkg"
$env:VCPKGRS_TRIPLET = "x64-windows-static-release"
cd app\src-tauri
cargo build --release
```
