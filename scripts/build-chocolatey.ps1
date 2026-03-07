#!/usr/bin/env pwsh
# Chocolatey パッケージ生成スクリプト

param(
    [string]$Version = ""
)

$ErrorActionPreference = "Stop"

# ルートディレクトリを取得
$rootDir = Split-Path -Parent $PSScriptRoot

# .envファイルを読み込む
$envFile = Join-Path $rootDir ".env"
if (Test-Path $envFile) {
    Write-Host "📄 .envファイルを読み込んでいます..." -ForegroundColor Cyan
    Get-Content $envFile | ForEach-Object {
        if ($_ -match '^\s*([^#][^=]+)\s*=\s*(.+)\s*$') {
            $key = $matches[1].Trim()
            $value = $matches[2].Trim()
            Set-Variable -Name $key -Value $value -Scope Script
        }
    }
}

# コマンドライン引数でバージョンを上書き可能
if ([string]::IsNullOrEmpty($Version)) {
    if (Get-Variable -Name "VERSION" -ErrorAction SilentlyContinue) {
        $Version = $script:VERSION
    } else {
        $Version = "1.0.0"
    }
}

# APP_NAME_KEBABが設定されていない場合はデフォルト値を使用
if (-not (Get-Variable -Name "APP_NAME_KEBAB" -ErrorAction SilentlyContinue)) {
    $APP_NAME_KEBAB = "tauri-vue3-app"
}

Write-Host "=== Chocolatey Package Generation ===" -ForegroundColor Cyan
Write-Host "App Name: $APP_NAME_KEBAB" -ForegroundColor Green
Write-Host "Version: $Version" -ForegroundColor Green
$chocoDir = Join-Path $rootDir ".choco"
$bundleDir = Join-Path $rootDir "backend\target\release\bundle\msi"

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

# Chocolatey toolsスクリプトをテンプレートから生成
Write-Host "`n📝 Generating Chocolatey tools scripts..." -ForegroundColor Cyan

# chocolateyinstall.ps1を生成
$installTemplate = Join-Path $chocoDir "tools\chocolateyinstall.ps1.template"
$installScript = Join-Path $chocoDir "tools\chocolateyinstall.ps1"

if (Test-Path $installTemplate) {
    $installContent = Get-Content $installTemplate -Raw
    $installContent = $installContent -replace '{{VERSION}}', $Version
    $installContent = $installContent -replace '{{APP_NAME}}', $script:APP_NAME
    $installContent = $installContent -replace '{{APP_NAME_KEBAB}}', $APP_NAME_KEBAB
    $installContent = $installContent -replace '{{PROJECT_URL}}', $script:PROJECT_URL
    $installContent = $installContent -replace '{{CHECKSUM64}}', $checksum
    Set-Content -Path $installScript -Value $installContent -NoNewline
    Write-Host "Generated: chocolateyinstall.ps1" -ForegroundColor Green
}

# chocolateyuninstall.ps1を生成
$uninstallTemplate = Join-Path $chocoDir "tools\chocolateyuninstall.ps1.template"
$uninstallScript = Join-Path $chocoDir "tools\chocolateyuninstall.ps1"

if (Test-Path $uninstallTemplate) {
    $uninstallContent = Get-Content $uninstallTemplate -Raw
    $uninstallContent = $uninstallContent -replace '{{APP_NAME}}', $script:APP_NAME
    $uninstallContent = $uninstallContent -replace '{{APP_NAME_KEBAB}}', $APP_NAME_KEBAB
    Set-Content -Path $uninstallScript -Value $uninstallContent -NoNewline
    Write-Host "Generated: chocolateyuninstall.ps1" -ForegroundColor Green
}

# nuspecファイルを更新（テンプレートから生成）
$nuspecTemplate = Join-Path $chocoDir "app.nuspec.template"
$nuspecFile = Join-Path $chocoDir "$APP_NAME_KEBAB.nuspec"

if (-not (Test-Path $nuspecTemplate)) {
    Write-Error "Template file not found: $nuspecTemplate"
    exit 1
}

# テンプレートからコピーして全ての変数を置換
$nuspecContent = Get-Content $nuspecTemplate -Raw
$nuspecContent = $nuspecContent -replace '{{VERSION}}', $Version
$nuspecContent = $nuspecContent -replace '{{APP_NAME}}', $script:APP_NAME
$nuspecContent = $nuspecContent -replace '{{APP_NAME_KEBAB}}', $APP_NAME_KEBAB
$nuspecContent = $nuspecContent -replace '{{APP_DESCRIPTION}}', $script:APP_DESCRIPTION
$nuspecContent = $nuspecContent -replace '{{APP_SUMMARY}}', $script:APP_SUMMARY
$nuspecContent = $nuspecContent -replace '{{AUTHOR_NAME}}', $script:AUTHOR_NAME
$nuspecContent = $nuspecContent -replace '{{GITHUB_USER}}', $script:GITHUB_USER
$nuspecContent = $nuspecContent -replace '{{PROJECT_URL}}', $script:PROJECT_URL
$nuspecContent = $nuspecContent -replace '{{DOCS_URL}}', $script:DOCS_URL
$nuspecContent = $nuspecContent -replace '{{CHOCOLATEY_TAGS}}', $script:CHOCOLATEY_TAGS
Set-Content -Path $nuspecFile -Value $nuspecContent -NoNewline

Write-Host "Updated version to $Version" -ForegroundColor Green

# Chocolateyパッケージをビルド
Write-Host "`nBuilding Chocolatey package..." -ForegroundColor Cyan
Push-Location $chocoDir
try {
    choco pack
    Write-Host "`nChocolatey package created successfully!" -ForegroundColor Green
    Write-Host "Package location: $chocoDir\$APP_NAME_KEBAB.$Version.nupkg" -ForegroundColor Yellow
} finally {
    Pop-Location
}

Write-Host "`n=== Next Steps ===" -ForegroundColor Cyan
Write-Host "1. Test the package locally (requires admin):" -ForegroundColor White
Write-Host "   choco install $APP_NAME_KEBAB -source $chocoDir -y" -ForegroundColor Gray
Write-Host "2. Push to Chocolatey Community Repository:" -ForegroundColor White
Write-Host "   choco push $chocoDir\$APP_NAME_KEBAB.$Version.nupkg --source https://push.chocolatey.org/" -ForegroundColor Gray
