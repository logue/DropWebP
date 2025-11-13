#!/usr/bin/env pwsh
# Chocolatey パッケージ生成スクリプト

param(
    [string]$Version = "3.0.2"
)

$ErrorActionPreference = "Stop"

Write-Host "=== Chocolatey Package Generation ===" -ForegroundColor Cyan
Write-Host "Version: $Version" -ForegroundColor Green

$rootDir = Split-Path -Parent $PSScriptRoot
$chocoDir = Join-Path $rootDir ".choco"
$bundleDir = Join-Path $rootDir "app\src-tauri\target\release\bundle\msi"

# MSIファイルを探す
$msiFile = Get-ChildItem -Path $bundleDir -Filter "*.msi" | Select-Object -First 1

if (-not $msiFile) {
    Write-Error "MSI file not found in $bundleDir"
    exit 1
}

Write-Host "Found MSI: $($msiFile.Name)" -ForegroundColor Green

# チェックサムを計算
$checksum = (Get-FileHash -Path $msiFile.FullName -Algorithm SHA256).Hash
Write-Host "SHA256: $checksum" -ForegroundColor Yellow

# chocolateyinstall.ps1を更新
$installScript = Join-Path $chocoDir "tools\chocolateyinstall.ps1"
$content = Get-Content $installScript -Raw
$content = $content -replace "checksum64\s*=\s*'[^']*'", "checksum64     = '$checksum'"
$content = $content -replace "\`$version\s*=\s*'[^']*'", "`$version = '$Version'"
Set-Content -Path $installScript -Value $content -NoNewline

# nuspecファイルを更新
$nuspecFile = Join-Path $chocoDir "drop-compress-image.nuspec"
$nuspec = [xml](Get-Content $nuspecFile)
$nuspec.package.metadata.version = $Version
$nuspec.Save($nuspecFile)

Write-Host "Updated version to $Version" -ForegroundColor Green

# Chocolateyパッケージをビルド
Write-Host "`nBuilding Chocolatey package..." -ForegroundColor Cyan
Push-Location $chocoDir
try {
    choco pack
    Write-Host "`nChocolatey package created successfully!" -ForegroundColor Green
    Write-Host "Package location: $chocoDir\drop-compress-image.$Version.nupkg" -ForegroundColor Yellow
} finally {
    Pop-Location
}

Write-Host "`n=== Next Steps ===" -ForegroundColor Cyan
Write-Host "1. Test the package locally:" -ForegroundColor White
Write-Host "   choco install drop-compress-image -source $chocoDir" -ForegroundColor Gray
Write-Host "2. Push to Chocolatey Community Repository:" -ForegroundColor White
Write-Host "   choco push $chocoDir\drop-compress-image.$Version.nupkg --source https://push.chocolatey.org/" -ForegroundColor Gray
