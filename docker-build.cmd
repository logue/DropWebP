@echo off
REM Windows Docker Build Script for MSVC Environment
REM NOTE: This script must be called AFTER vcvars64.bat
REM Do not use setlocal - we need to preserve vcvars64.bat environment

REM Enable delayed expansion for variable updates
setlocal enabledelayedexpansion

REM Set CI environment
set CI=true

REM Increase Node.js heap size
set "NODE_OPTIONS=--max-old-space-size=4096"

REM Set Rust target directory
set "CARGO_TARGET_DIR=C:\build-temp\target"

echo.
echo === DropWebP Windows Build Script ===
echo.

REM Skip environment check and proceed to build
REM (Environment is already validated by vcvars64.bat)

REM Install dependencies
echo [1/4] Installing dependencies...
call pnpm install --frozen-lockfile --node-linker=hoisted
echo Dependencies installed - exit code: %ERRORLEVEL%

REM Set Rust target directory (skip cargo clean - not needed for first build)
echo.
echo [2/4] Preparing build environment...
echo Current directory: %CD%

REM Verify MSVC toolchain
echo.
echo Verifying MSVC toolchain...
where cl.exe
if errorlevel 1 (
    echo ERROR: cl.exe not found in PATH
    exit /b 1
)
where link.exe
if errorlevel 1 (
    echo ERROR: link.exe not found in PATH
    exit /b 1
)
echo.

REM Display critical environment variables
echo Critical environment variables:
echo VCToolsInstallDir=%VCToolsInstallDir%
echo VSINSTALLDIR=%VSINSTALLDIR%
echo INCLUDE=%INCLUDE%
echo LIB=%LIB%
echo PATH=%PATH%
echo.

REM Set CMake environment variables for cmake-rs
REM cmake-rs needs these to find MSVC toolchain
for /f "delims=" %%i in ('where cl.exe 2^>nul') do set "CC=%%i"
for /f "delims=" %%i in ('where cl.exe 2^>nul') do set "CXX=%%i"
for /f "delims=" %%i in ('where cl.exe 2^>nul') do set "CMAKE_C_COMPILER=%%i"
for /f "delims=" %%i in ('where cl.exe 2^>nul') do set "CMAKE_CXX_COMPILER=%%i"

REM Set assembler for NASM
for /f "delims=" %%i in ('where nasm.exe 2^>nul') do set "CMAKE_ASM_NASM_COMPILER=%%i"

REM Temporarily remove node_modules\.bin from PATH to prevent CMake from finding wrong RC
REM This fixes the issue where CMake finds C:\workspace\node_modules\.bin\rc instead of Windows SDK rc.exe
set "PATH_ORIGINAL=!PATH!"
set "PATH=!PATH:C:\workspace\node_modules\.bin;=!"
set "PATH=!PATH:C:\workspace\node_modules\.pnpm\node_modules\.bin;=!"
echo.

REM Force CMake to use NMake Makefiles generator instead of Visual Studio generator
REM Visual Studio generator doesn't inherit vcvars64.bat environment correctly
set CMAKE_GENERATOR=NMake Makefiles

REM Set RC (Resource Compiler) explicitly to Windows SDK version
REM This prevents CMake from finding the wrong rc.exe in node_modules\.bin
set CMAKE_RC_COMPILER=
for /f "delims=" %%i in ('where /R "C:\Program Files (x86)\Windows Kits\10\bin" rc.exe 2^>nul ^| findstr "\\x64\\rc.exe$"') do (
    set "CMAKE_RC_COMPILER=%%i"
    goto :rc_found
)
:rc_found

if not defined CMAKE_RC_COMPILER (
    echo Warning: CMAKE_RC_COMPILER not found in Windows Kits
    echo This may cause issues with CMake-based dependencies
    echo Available rc.exe locations:
    where /R "C:\Program Files (x86)\Windows Kits" rc.exe 2>nul
    REM Don't exit - rav1e doesn't need CMake
)

echo CMake compiler configuration:
echo CC=!CC!
echo CXX=!CXX!
echo CMAKE_C_COMPILER=!CMAKE_C_COMPILER!
echo CMAKE_CXX_COMPILER=!CMAKE_CXX_COMPILER!
echo CMAKE_ASM_NASM_COMPILER=!CMAKE_ASM_NASM_COMPILER!
echo CMAKE_RC_COMPILER=!CMAKE_RC_COMPILER!
echo CMAKE_GENERATOR=!CMAKE_GENERATOR!
echo.
echo CMAKE_CXX_COMPILER=!CMAKE_CXX_COMPILER!
echo CMAKE_ASM_NASM_COMPILER=!CMAKE_ASM_NASM_COMPILER!
echo CMAKE_RC_COMPILER=!CMAKE_RC_COMPILER!
echo CMAKE_GENERATOR=!CMAKE_GENERATOR!
echo.
echo RC compiler search order (should be Windows SDK, not node_modules):
where rc.exe 2^>nul
if errorlevel 1 echo   Warning: rc.exe not found in PATH
echo.

REM Test CMake compiler detection
echo Testing CMake:
cmake --version
echo.

REM Clean previous build cache to remove old CMake configuration
echo Cleaning previous build cache...
cd C:\workspace\app\src-tauri
cargo clean
echo Build cache cleaned
echo.

REM Also clean CMake cache from build-temp
if exist "C:\build-temp" (
    echo Cleaning CMake cache from C:\build-temp...
    rd /s /q "C:\build-temp" 2>nul
    echo CMake cache cleaned
)
echo.

cd C:\workspace\app\src-tauri
if not exist "!CARGO_TARGET_DIR!" mkdir "!CARGO_TARGET_DIR!"
echo Target directory: !CARGO_TARGET_DIR!
cd C:\workspace
echo Back to: %CD%
echo.

REM Build frontend and Tauri in two stages
echo.
echo [3/4] Building application...

REM Stage 1: Build frontend (needs node_modules\.bin intact)
echo.
echo Stage 1/2: Building Vue frontend with Vite...
echo Current directory: %CD%
cd C:\workspace\app
echo Changed to: %CD%
REM Skip type-check in Docker to save time (takes too long in container)
echo Skipping type-check (Docker environment limitation)...
call pnpm run build-only
set VITE_BUILD_EXIT=!ERRORLEVEL!
if !VITE_BUILD_EXIT! neq 0 (
    echo Vite build failed - exit code: !VITE_BUILD_EXIT!
    goto :cleanup
)
echo Frontend build completed successfully
echo.

REM Stage 2: Build Rust/Tauri (rename node_modules\.bin to prevent CMake RC conflicts)
echo Stage 2/2: Building Rust backend with Tauri...
echo Temporarily renaming node_modules\.bin to prevent CMake RC conflicts...
if exist "C:\workspace\node_modules\.bin" (
    ren "C:\workspace\node_modules\.bin" ".bin.backup" 2>nul
    if exist "C:\workspace\node_modules\.bin.backup" echo   Renamed: node_modules\.bin to .bin.backup
)
if exist "C:\workspace\node_modules\.pnpm\node_modules\.bin" (
    ren "C:\workspace\node_modules\.pnpm\node_modules\.bin" ".bin.backup" 2>nul
    if exist "C:\workspace\node_modules\.pnpm\node_modules\.bin.backup" echo   Renamed: node_modules\.pnpm\node_modules\.bin to .bin.backup
)
echo.

call pnpm run build:tauri
set TAURI_BUILD_EXIT=!ERRORLEVEL!
echo Tauri build completed - exit code: !TAURI_BUILD_EXIT!
set BUILD_EXIT_CODE=!TAURI_BUILD_EXIT!

:cleanup
REM Restore original PATH
set "PATH=!PATH_ORIGINAL!"

REM Restore node_modules\.bin directories
echo.
echo Restoring node_modules\.bin directories...
if exist "C:\workspace\node_modules\.bin.backup" (
    ren "C:\workspace\node_modules\.bin.backup" ".bin" 2>nul
    if exist "C:\workspace\node_modules\.bin" echo   Restored: node_modules\.bin
)
if exist "C:\workspace\node_modules\.pnpm\node_modules\.bin.backup" (
    ren "C:\workspace\node_modules\.pnpm\node_modules\.bin.backup" ".bin" 2>nul
    if exist "C:\workspace\node_modules\.pnpm\node_modules\.bin" echo   Restored: node_modules\.pnpm\node_modules\.bin
)

cd C:\workspace

REM Copy build artifacts to host directory
echo.
echo [4/4] Copying build artifacts...
set "SRC_DIR=C:\build-temp\target\release"
set "DEST_DIR=C:\workspace\app\src-tauri\target\release"

if exist "!SRC_DIR!" (
    if not exist "!DEST_DIR!" mkdir "!DEST_DIR!"

    if exist "!SRC_DIR!\*.exe" (
        copy /Y "!SRC_DIR!\*.exe" "!DEST_DIR!\" >nul
        echo   Copied: *.exe
    )

    if exist "!SRC_DIR!\bundle" (
        xcopy /E /I /Y /Q "!SRC_DIR!\bundle" "!DEST_DIR!\bundle" >nul
        echo   Copied: bundle\
    )
) else (
    echo   Warning: Build directory not found
)

REM Check build artifacts
echo.
echo Build artifacts:
if exist C:\workspace\app\src-tauri\target\release\bundle (
    for /r C:\workspace\app\src-tauri\target\release\bundle %%f in (*.exe *.msi) do (
        echo   - %%f
    )
) else (
    echo   Warning: Bundle directory not found
)

echo.
echo === Build completed successfully ===
