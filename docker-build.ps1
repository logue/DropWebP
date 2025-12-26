# Windows Docker環境でのビルドスクリプト
# MSVC環境でTauriアプリケーションをビルド

$ErrorActionPreference = "Stop"

Write-Host "=== DropWebP Windows Build Script ===" -ForegroundColor Cyan

# 環境確認
Write-Host "`n[1/5] Checking environment..." -ForegroundColor Yellow
node --version
pnpm --version
cargo --version
rustc --version

# 依存関係のインストール
Write-Host "`n[2/5] Installing dependencies..." -ForegroundColor Yellow
pnpm install --frozen-lockfile

# フロントエンドビルド
Write-Host "`n[3/5] Building frontend..." -ForegroundColor Yellow
Set-Location app
pnpm build

# Tauriアプリケーションビルド
Write-Host "`n[4/5] Building Tauri application..." -ForegroundColor Yellow
pnpm tauri build

# ビルド成果物の確認
Write-Host "`n[5/5] Build artifacts:" -ForegroundColor Yellow
$bundlePath = "src-tauri\target\release\bundle"
if (Test-Path $bundlePath) {
    Get-ChildItem -Path $bundlePath -Recurse -File | `
        Where-Object { $_.Extension -in @('.exe', '.msi', '.nsis') } | `
        ForEach-Object {
            $size = [math]::Round($_.Length / 1MB, 2)
            Write-Host "  - $($_.FullName) ($size MB)" -ForegroundColor Green
        }
} else {
    Write-Host "  Warning: Bundle directory not found" -ForegroundColor Red
}

Write-Host "`n=== Build completed successfully ===" -ForegroundColor Cyan
