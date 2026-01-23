# Windows Docker環境でのビルドスクリプト
# MSVC環境でTauriアプリケーションをビルド

$ErrorActionPreference = "Stop"

# vcvars64.batで既に環境変数が設定されているはず
# 追加でCI環境として設定
$env:CI = "true"

# Node.jsのヒープサイズを増やす（メモリ不足エラーを防ぐ）
$env:NODE_OPTIONS = "--max-old-space-size=4096"

Write-Host "`n=== DropWebP Windows Build Script ===" -ForegroundColor Cyan

# 環境確認
Write-Host "`n[1/5] Checking environment..." -ForegroundColor Yellow
Write-Host "Node.js: " -NoNewline; node --version
Write-Host "pnpm: " -NoNewline; pnpm --version
Write-Host "Cargo: " -NoNewline; cargo --version
Write-Host "Rustc: " -NoNewline; rustc --version
Write-Host "NASM: " -NoNewline; nasm -version | Select-Object -First 1
Write-Host "CMake: " -NoNewline; cmake --version | Select-Object -First 1

# 依存関係のインストール
Write-Host "`n[2/5] Installing dependencies..." -ForegroundColor Yellow
pnpm install --frozen-lockfile --node-linker=hoisted

# Rustのキャッシュをクリア（初回ビルドの場合）
Write-Host "`n[3/5] Cleaning Rust cache..." -ForegroundColor Yellow
Set-Location app\src-tauri

# ホストのtargetディレクトリとの競合を避けるため、コンテナ内の別の場所を使用
$env:CARGO_TARGET_DIR = "C:\build-temp\target"
New-Item -ItemType Directory -Force -Path $env:CARGO_TARGET_DIR | Out-Null

cargo clean
Set-Location ..\..

# フロントエンド＋Tauriビルド（一度だけ実行）
Write-Host "`n[4/5] Building application..." -ForegroundColor Yellow
Set-Location app
# pnpm buildは内部でtype-check、build-only、build:tauriを並行実行する
pnpm build
Set-Location ..

# ビルド成果物をホストディレクトリにコピー
Write-Host "`n[5/5] Copying build artifacts..." -ForegroundColor Yellow
$targetDir = "C:\build-temp\target\release"
$destDir = "C:\workspace\app\src-tauri\target\release"

if (Test-Path $targetDir) {
    # ディレクトリを作成
    New-Item -ItemType Directory -Force -Path $destDir | Out-Null

    # EXEファイルをコピー
    if (Test-Path "$targetDir\*.exe") {
        Copy-Item "$targetDir\*.exe" -Destination $destDir -Force
        Write-Host "  Copied: *.exe" -ForegroundColor Green
    }

    # bundleディレクトリをコピー
    if (Test-Path "$targetDir\bundle") {
        Copy-Item "$targetDir\bundle" -Destination $destDir -Recurse -Force
        Write-Host "  Copied: bundle\" -ForegroundColor Green
    }
} else {
    Write-Host "  Warning: Build directory not found" -ForegroundColor Yellow
}

# ビルド成果物の確認
Write-Host "`n[6/6] Build artifacts:" -ForegroundColor Yellow
$bundlePath = "C:\workspace\app\src-tauri\target\release\bundle"
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
