# vcpkg setup script for Windows
# This script installs required C/C++ libraries via vcpkg for static linking
# Note: AVIF encoding uses rav1e (Rust), so libaom is not required

param(
    [string]$VcpkgRoot = $env:VCPKG_ROOT,
    [string]$Triplet = "x64-windows-static-release"
)

Write-Host "=== vcpkg Setup for Drop Compress Image ===" -ForegroundColor Cyan

# Check if vcpkg is installed
if (-not $VcpkgRoot) {
    Write-Host "ERROR: VCPKG_ROOT environment variable is not set" -ForegroundColor Red
    Write-Host "Please install vcpkg and set VCPKG_ROOT:" -ForegroundColor Yellow
    Write-Host "  1. git clone https://github.com/Microsoft/vcpkg.git" -ForegroundColor Yellow
    Write-Host "  2. cd vcpkg && .\bootstrap-vcpkg.bat" -ForegroundColor Yellow
    Write-Host "  3. Set VCPKG_ROOT environment variable to vcpkg directory" -ForegroundColor Yellow
    exit 1
}

$vcpkgExe = Join-Path $VcpkgRoot "vcpkg.exe"
if (-not (Test-Path $vcpkgExe)) {
    Write-Host "ERROR: vcpkg.exe not found at $vcpkgExe" -ForegroundColor Red
    Write-Host "Please run bootstrap-vcpkg.bat first" -ForegroundColor Yellow
    exit 1
}

Write-Host "Found vcpkg at: $vcpkgExe" -ForegroundColor Green
Write-Host "Using triplet: $Triplet" -ForegroundColor Green
Write-Host ""

# Install all C/C++ dependencies
# Note: aom and libavif removed - using rav1e (Rust) for AVIF encoding
$packages = @(
    "libjxl:$Triplet",             # libjxl (JPEG XL)
    "libwebp:$Triplet",            # libwebp (WebP codec)
    "openjpeg:$Triplet",           # OpenJPEG (JPEG 2000)
    "libjpeg-turbo:$Triplet",      # libjpeg-turbo (for jpegli)
    "lcms:$Triplet"                # Little CMS (color management)
)

foreach ($package in $packages) {
    Write-Host "Installing $package..." -ForegroundColor Cyan
    & $vcpkgExe install $package

    if ($LASTEXITCODE -ne 0) {
        Write-Host "WARNING: Failed to install $package" -ForegroundColor Yellow
    } else {
        Write-Host "Successfully installed $package" -ForegroundColor Green
    }
    Write-Host ""
}

Write-Host "=== Setup Complete ===" -ForegroundColor Cyan
Write-Host "You can now build the project with:" -ForegroundColor Green
Write-Host "  cargo build --release" -ForegroundColor Green
Write-Host ""
Write-Host "Note: Make sure VCPKG_ROOT environment variable is set in your shell" -ForegroundColor Yellow
Write-Host "AVIF encoding uses rav1e (Rust-based), so libaom is not required" -ForegroundColor Cyan
