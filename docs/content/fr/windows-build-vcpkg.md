# Build Windows avec vcpkg

Ce document explique comment utiliser vcpkg sur Windows pour lier statiquement les dependances C/C++ (libaom, libavif, libjxl, libwebp, etc.).

## Important: utiliser un triplet release

Le triplet par defaut `x64-windows-static` peut causer des erreurs de linkage en build Rust release.

Triplet recommande:

- `x64-windows-static-release`

## Prerequis

- Visual Studio 2019/2022 (MSVC)
- Rust (rustup recommande)
- Git
- PowerShell
- LLVM/Clang (necessaire pour bindgen de jxl-sys)

## Installer LLVM/Clang

### Methode recommandee

```powershell
winget install LLVM.LLVM
```

Verification:

```powershell
clang --version
```

Si necessaire:

```powershell
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"
```

## Installer vcpkg

```powershell
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat

$env:VCPKG_ROOT = "C:\vcpkg"
[System.Environment]::SetEnvironmentVariable('VCPKG_ROOT', 'C:\vcpkg', 'User')
```

## Creer le triplet release

```powershell
@"
set(VCPKG_TARGET_ARCHITECTURE x64)
set(VCPKG_CRT_LINKAGE static)
set(VCPKG_LIBRARY_LINKAGE static)
set(VCPKG_BUILD_TYPE release)
"@ | Out-File -Encoding utf8 C:\vcpkg\triplets\x64-windows-static-release.cmake
```

## Installer les dependances

### Methode automatique

```powershell
cd path\to\DropWebP\app\src-tauri
.\setup-vcpkg.ps1
```

### Methode manuelle

```powershell
cd C:\vcpkg
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

## Depannage

### `libaom-sys` / `libavif-sys` non detectes

```powershell
cd app\src-tauri
cargo clean -p libaom-sys -p libavif-sys
vcpkg list | findstr aom
vcpkg list | findstr avif
cargo clean
cargo build --release
```

### Erreur `__imp__CrtDbgReport`

Cause frequente: triplet `x64-windows-static` utilise au lieu de `x64-windows-static-release`.

### Erreur `cannot open input file 'aom.lib'`

```powershell
echo $env:VCPKG_ROOT
echo $env:VCPKGRS_TRIPLET
dir "$env:VCPKG_ROOT\installed\x64-windows-static-release\lib"
```

## Liens utiles

- [vcpkg](https://github.com/Microsoft/vcpkg)
- [vcpkg-rs](https://github.com/mcgoo/vcpkg-rs)
- [libavif](https://github.com/AOMediaCodec/libavif)
- [libjxl](https://github.com/libjxl/libjxl)
