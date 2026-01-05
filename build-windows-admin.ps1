# Windows用ビルドヘルパースクリプト
# NASMの一時的なアンインストールとビルド

Write-Host "=== Windows Build Helper ===" -ForegroundColor Cyan
Write-Host ""

# 管理者権限チェック
$isAdmin = ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "このスクリプトは管理者権限が必要です。" -ForegroundColor Red
    Write-Host "PowerShellを右クリックして「管理者として実行」で起動してください。" -ForegroundColor Yellow
    exit 1
}

Write-Host "管理者権限を確認しました。" -ForegroundColor Green
Write-Host ""

# NASMがインストールされているか確認
$nasmInstalled = $false
try {
    $null = Get-Command nasm -ErrorAction Stop
    $nasmInstalled = $true
    Write-Host "NASMが検出されました。" -ForegroundColor Yellow
} catch {
    Write-Host "NASMは検出されませんでした。" -ForegroundColor Green
}

if ($nasmInstalled) {
    Write-Host ""
    Write-Host "[ステップ 1/4] NASMを一時的にアンインストールします..." -ForegroundColor Yellow
    choco uninstall nasm -y

    if ($LASTEXITCODE -ne 0) {
        Write-Host "警告: NASMのアンインストールに失敗しました。手動でアンインストールしてください。" -ForegroundColor Red
    } else {
        Write-Host "NASMをアンインストールしました。" -ForegroundColor Green
    }
}

Write-Host ""
Write-Host "[ステップ 2/4] ビルドキャッシュをクリアします..." -ForegroundColor Yellow
Set-Location "$PSScriptRoot\app\src-tauri"
cargo clean
Write-Host "キャッシュをクリアしました。" -ForegroundColor Green

Write-Host ""
Write-Host "[ステップ 3/4] アプリケーションをビルドします..." -ForegroundColor Yellow
Set-Location "$PSScriptRoot\app"

# build-msvc.ps1を使用してビルド
Set-Location src-tauri
.\build-msvc.ps1

$buildSuccess = ($LASTEXITCODE -eq 0)

if ($nasmInstalled) {
    Write-Host ""
    Write-Host "[ステップ 4/4] NASMを再インストールします..." -ForegroundColor Yellow
    choco install nasm -y

    if ($LASTEXITCODE -eq 0) {
        Write-Host "NASMを再インストールしました。" -ForegroundColor Green
    } else {
        Write-Host "警告: NASMの再インストールに失敗しました。" -ForegroundColor Yellow
        Write-Host "手動で再インストールしてください: choco install nasm -y" -ForegroundColor Yellow
    }
}

Write-Host ""
if ($buildSuccess) {
    Write-Host "=== ビルド成功！ ===" -ForegroundColor Green
} else {
    Write-Host "=== ビルド失敗 ===" -ForegroundColor Red
    Write-Host "詳細はWINDOWS_BUILD_FIX.mdを参照してください。" -ForegroundColor Yellow
}

Set-Location $PSScriptRoot
