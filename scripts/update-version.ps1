#!/usr/bin/env pwsh
# Update version across package.json, tauri.conf.json, and Cargo.toml from .env

param(
    [string]$EnvPath = "../.env"
)

$ErrorActionPreference = "Stop"

Write-Host "Reading version from .env..." -ForegroundColor Cyan

# Read version from .env
if (-not (Test-Path $EnvPath)) {
    Write-Error "Error: .env file not found at $EnvPath"
    exit 1
}

$version = ""
$envContent = Get-Content $EnvPath
foreach ($line in $envContent) {
    if ($line -match "^VERSION=(.+)$") {
        $version = $Matches[1].Trim()
        break
    }
}

if ([string]::IsNullOrEmpty($version)) {
    Write-Error "Error: VERSION not found in .env"
    exit 1
}

Write-Host "Found version: $version" -ForegroundColor Green

# Update frontend/package.json
Write-Host "Updating frontend/package.json..." -ForegroundColor Cyan
$packageJsonPath = "../frontend/package.json"
if (Test-Path $packageJsonPath) {
    $packageJson = Get-Content $packageJsonPath -Raw | ConvertFrom-Json
    $packageJson.version = $version
    $packageJson | ConvertTo-Json -Depth 100 | Set-Content $packageJsonPath -Encoding UTF8
    Write-Host "✓ Updated frontend/package.json to version $version" -ForegroundColor Green
} else {
    Write-Warning "Warning: $packageJsonPath not found"
}

# Update backend/tauri.conf.json
Write-Host "Updating backend/tauri.conf.json..." -ForegroundColor Cyan
$tauriConfPath = "../backend/tauri.conf.json"
if (Test-Path $tauriConfPath) {
    $tauriConf = Get-Content $tauriConfPath -Raw | ConvertFrom-Json
    $tauriConf.version = $version
    $tauriConf | ConvertTo-Json -Depth 100 | Set-Content $tauriConfPath -Encoding UTF8
    Write-Host "✓ Updated backend/tauri.conf.json to version $version" -ForegroundColor Green
} else {
    Write-Warning "Warning: $tauriConfPath not found"
}

# Update backend/Cargo.toml
Write-Host "Updating backend/Cargo.toml..." -ForegroundColor Cyan
$cargoTomlPath = "../backend/Cargo.toml"
if (Test-Path $cargoTomlPath) {
    $cargoContent = Get-Content $cargoTomlPath -Raw
    $cargoContent = $cargoContent -replace 'version = "[^"]+"', "version = `"$version`""
    Set-Content $cargoTomlPath -Value $cargoContent -Encoding UTF8 -NoNewline
    Write-Host "✓ Updated backend/Cargo.toml to version $version" -ForegroundColor Green
} else {
    Write-Warning "Warning: $cargoTomlPath not found"
}

Write-Host "`nVersion update complete! All files now use version $version" -ForegroundColor Green
