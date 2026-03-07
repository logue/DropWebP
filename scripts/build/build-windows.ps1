# Windows向けビルドスクリプト
# Tauri v2アプリケーションをMSVCでビルド
# ClangCLが不要なバージョン

$ErrorActionPreference = "Stop"

Write-Host "=== Tauri Vue3 App - Windows Build ===" -ForegroundColor Cyan
Write-Host ""

# ビルド環境の確認
Write-Host "[1/6] Checking build environment..." -ForegroundColor Yellow
try {
    $nodeVersion = node --version
    $pnpmVersion = pnpm --version
    $cargoVersion = cargo --version
    $rustcVersion = rustc --version
    $cmakeVersion = cmake --version | Select-Object -First 1

    Write-Host "  Node.js: $nodeVersion" -ForegroundColor Green
    Write-Host "  pnpm: $pnpmVersion" -ForegroundColor Green
    Write-Host "  Cargo: $cargoVersion" -ForegroundColor Green
    Write-Host "  Rustc: $rustcVersion" -ForegroundColor Green
    Write-Host "  CMake: $cmakeVersion" -ForegroundColor Green
} catch {
    Write-Host "Error: Required tools not found. Please install Node.js, Rust, and CMake." -ForegroundColor Red
    exit 1
}

# 依存関係のインストール
Write-Host ""
Write-Host "[2/6] Installing dependencies..." -ForegroundColor Yellow
pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Failed to install dependencies" -ForegroundColor Red
    exit 1
}

# クリーンビルド（オプション）
if ($args -contains "--clean") {
    Write-Host ""
    Write-Host "[3/6] Cleaning previous build..." -ForegroundColor Yellow
    Remove-Item -Path "backend\target" -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item -Path "frontend\dist" -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "  Clean completed" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "[3/6] Skipping clean (use --clean to force clean build)" -ForegroundColor Gray
}

# フロントエンドビルド
Write-Host ""
Write-Host "[4/6] Building frontend..." -ForegroundColor Yellow
Set-Location frontend
pnpm run build-only
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Frontend build failed" -ForegroundColor Red
    Set-Location ..
    exit 1
}
Set-Location ..

# Rustバックエンドビルド（MSVC専用スクリプト使用）
Write-Host ""
Write-Host "[5/6] Building Rust backend with MSVC..." -ForegroundColor Yellow
Set-Location backend

# jpegxl-srcのパッチ適用とビルド
.\build-msvc.ps1
if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: Rust backend build failed" -ForegroundColor Red
    Set-Location ..\..
    exit 1
}

Set-Location ..\..

# Tauriバンドルの作成
Write-Host ""
Write-Host "[6/6] Creating Tauri bundles..." -ForegroundColor Yellow
Set-Location frontend
pnpm tauri build
if ($LASTEXITCODE -ne 0) {
    Write-Host "Warning: Bundle creation encountered issues" -ForegroundColor Yellow
} else {
    Write-Host "  Bundle creation completed" -ForegroundColor Green
}
Set-Location ..

# ビルド成果物の表示
Write-Host ""
Write-Host "=== Build Artifacts ===" -ForegroundColor Cyan
$bundlePath = "backend\target\release\bundle"
if (Test-Path $bundlePath) {
    $installers = Get-ChildItem -Path $bundlePath -Recurse -File | `
        Where-Object { $_.Extension -in @('.exe', '.msi', '.nsis') }

    if ($installers.Count -gt 0) {
        foreach ($file in $installers) {
            $size = [math]::Round($file.Length / 1MB, 2)
            Write-Host "  $($file.FullName) ($size MB)" -ForegroundColor Green
        }
    } else {
        Write-Host "  No installers found" -ForegroundColor Yellow
    }
} else {
    Write-Host "  Bundle directory not found" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "=== Build completed successfully! ===" -ForegroundColor Green
Write-Host ""
Write-Host "To run the application:" -ForegroundColor Cyan
Write-Host "  .\backend\target\release\tauri-vue3-app.exe" -ForegroundColor White
