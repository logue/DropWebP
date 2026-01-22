# Windows環境からDockerを使ってLinux向けビルドを実行するスクリプト

param(
    [string]$Target = "x64",
    [switch]$IncludeAppImage = $false
)

$ErrorActionPreference = "Stop"

# Dockerが動作しているか確認
try {
    $dockerVersion = docker version --format '{{.Server.Version}}' 2>&1
    if ($LASTEXITCODE -ne 0) {
        throw "Docker is not running"
    }
} catch {
    Write-Host "`n❌ エラー: Docker Desktop が起動していません。" -ForegroundColor Red
    Write-Host "Docker Desktop を起動してから、再度実行してください。`n" -ForegroundColor Yellow
    exit 1
}

# プロジェクトルートディレクトリを取得
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptDir

# .envファイルを読み込む
$EnvFile = Join-Path $ProjectRoot ".env"
if (Test-Path $EnvFile) {
    Write-Host "📄 .envファイルを読み込んでいます..." -ForegroundColor Blue
    Get-Content $EnvFile | ForEach-Object {
        if ($_ -match '^\s*([^#][^=]+)\s*=\s*(.+)\s*$') {
            $name = $matches[1].Trim()
            $value = $matches[2].Trim()
            [Environment]::SetEnvironmentVariable($name, $value, "Process")
        }
    }
}

Write-Host "`n🐳 Docker経由でLinux向けビルドを実行`n" -ForegroundColor Blue

# ターゲットアーキテクチャを設定
switch ($Target.ToLower()) {
    { $_ -in "x64", "x86_64", "amd64" } {
        $BuildTarget = "x86_64-unknown-linux-gnu"
        $ArchName = "x86_64 (AMD64)"
        $Dockerfile = "Dockerfile.linux-x64"
        $ImageName = "dropwebp-linux-x64-builder"
        $Platform = "linux/amd64"
    }
    { $_ -in "arm64", "aarch64" } {
        $BuildTarget = "aarch64-unknown-linux-gnu"
        $ArchName = "ARM64 (AArch64)"
        $Dockerfile = "Dockerfile.linux-arm64"
        $ImageName = "dropwebp-linux-arm64-builder"
        $Platform = "linux/arm64"
    }
    default {
        Write-Host "⚠️  不明なターゲット: $Target" -ForegroundColor Yellow
        Write-Host "使用方法: .\build-linux-docker.ps1 [-Target x64|arm64] [-IncludeAppImage]"
        exit 1
    }
}

Write-Host "ターゲット: $ArchName ($BuildTarget)" -ForegroundColor Green
Write-Host "Dockerfile: $Dockerfile" -ForegroundColor Green
Write-Host "プラットフォーム: $Platform" -ForegroundColor Green

# AppImageを含めるかどうか
if ($IncludeAppImage) {
    Write-Host "AppImage: 有効（FUSEが必要）" -ForegroundColor Green
    $BundleTargets = ""
} else {
    Write-Host "AppImage: 無効（Docker環境では.deb, .rpmのみ）" -ForegroundColor Yellow
    $BundleTargets = "deb,rpm"
}

Write-Host ""

# CPUコア数とメモリの設定
$DockerBuildArgs = @()
$DockerRunArgs = @()

$BuildCpus = $env:BUILD_CPUS
$BuildMemory = $env:BUILD_MEMORY
$CargoBuildJobs = $env:CARGO_BUILD_JOBS

if ($BuildCpus) {
    Write-Host "CPUコア数: $BuildCpus" -ForegroundColor Green
    $DockerBuildArgs += "--cpus=$BuildCpus"
    $DockerRunArgs += "--cpus=$BuildCpus"

    if (-not $CargoBuildJobs) {
        $CargoBuildJobs = $BuildCpus
    }
}

if ($BuildMemory) {
    Write-Host "メモリ制限: $BuildMemory" -ForegroundColor Green
    $DockerBuildArgs += "--memory=$BuildMemory"
    $DockerRunArgs += "--memory=$BuildMemory"
}

if ($CargoBuildJobs) {
    Write-Host "Cargo並列度: $CargoBuildJobs" -ForegroundColor Green
    $DockerRunArgs += "-e", "CARGO_BUILD_JOBS=$CargoBuildJobs"
}

if ($BuildCpus) {
    $MakeFlags = $env:MAKEFLAGS
    if (-not $MakeFlags) {
        $MakeFlags = "-j$BuildCpus"
    }
    Write-Host "Make並列度: $MakeFlags" -ForegroundColor Green
    $DockerRunArgs += "-e", "MAKEFLAGS=$MakeFlags"
}

Write-Host ""

# Dockerイメージをビルド
Write-Host "📦 Dockerイメージをビルド中..." -ForegroundColor Blue
Set-Location $ProjectRoot
$buildCmd = "docker build --platform `"$Platform`" $($DockerBuildArgs -join ' ') -f `"$Dockerfile`" -t `"$ImageName`" ."
Write-Host "実行コマンド: $buildCmd" -ForegroundColor DarkGray
Invoke-Expression $buildCmd

Write-Host "`n🔨 Linux向けアプリケーションをビルド中...`n" -ForegroundColor Blue

# キャッシュ用のDockerボリューム名
$PlatformSafe = $Platform -replace '/', '-'
$CargoVolume = "dropwebp-cargo-cache-$PlatformSafe"
$PnpmVolume = "dropwebp-pnpm-cache-$PlatformSafe"
$TargetVolume = "dropwebp-target-cache-$PlatformSafe"

# ボリュームが存在しない場合は作成
docker volume create $CargoVolume 2>&1 | Out-Null
docker volume create $PnpmVolume 2>&1 | Out-Null
docker volume create $TargetVolume 2>&1 | Out-Null

Write-Host "キャッシュボリューム:" -ForegroundColor Green
Write-Host "  - Cargo: $CargoVolume"
Write-Host "  - pnpm: $PnpmVolume"
Write-Host "  - Target: $TargetVolume"
Write-Host ""

# Dockerコンテナ内でビルドを実行
# Windowsパスをlinuxパスに変換（WSLスタイル）
$ProjectRootLinux = $ProjectRoot -replace '\\', '/' -replace '^([A-Z]):', { "/mnt/$($_.Groups[1].Value.ToLower())" }

Write-Host "ビルドを実行中..." -ForegroundColor Blue
$runArgs = @(
    "run", "--rm",
    "--platform", $Platform,
    "--privileged",
    "--security-opt", "apparmor=unconfined",
    "--security-opt", "seccomp=unconfined",
    "-v", "${ProjectRoot}:/workspace",
    "-v", "${CargoVolume}:/root/.cargo/registry",
    "-v", "${PnpmVolume}:/pnpm/store",
    "-v", "${TargetVolume}:/workspace/app/src-tauri/target",
    "-e", "BUILD_TARGET=$BuildTarget",
    "-e", "APPIMAGE_EXTRACT_AND_RUN=1",
    "-e", "VERBOSE=1"
)

if ($BundleTargets) {
    $runArgs += "-e", "TAURI_BUNDLER_TARGETS=$BundleTargets"
}

$runArgs += $DockerRunArgs
$runArgs += $ImageName

docker @runArgs

Write-Host "`n✅ ビルド完了！`n" -ForegroundColor Green

Write-Host "📋 成果物をホストにコピー中..." -ForegroundColor Blue

# ホスト側のディレクトリを作成
$TargetDir = Join-Path $ProjectRoot "app\src-tauri\target\$BuildTarget\release"
New-Item -ItemType Directory -Force -Path $TargetDir | Out-Null

# Dockerボリュームからホストに成果物をコピー
docker run --rm `
    -v "${TargetVolume}:/data" `
    -v "${ProjectRoot}\app\src-tauri\target:/output" `
    alpine sh -c "cp -r /data/$BuildTarget/release/bundle /output/$BuildTarget/release/ 2>/dev/null || echo 'Bundle not found in volume'"

Write-Host ""
Write-Host "📦 成果物の場所:" -ForegroundColor Green
Write-Host "   $TargetDir\bundle\"
Write-Host ""

# 成果物のサイズを表示
$BundleDir = Join-Path $TargetDir "bundle"

if (Test-Path (Join-Path $BundleDir "deb")) {
    Write-Host "📊 .deb パッケージ:" -ForegroundColor Green
    Get-ChildItem -Path (Join-Path $BundleDir "deb") -Filter "*.deb" -ErrorAction SilentlyContinue | ForEach-Object {
        $sizeMB = [math]::Round($_.Length / 1MB, 2)
        Write-Host "   - $($_.Name) ($sizeMB MB)"
    }
}

if (Test-Path (Join-Path $BundleDir "rpm")) {
    Write-Host "📊 .rpm パッケージ:" -ForegroundColor Green
    Get-ChildItem -Path (Join-Path $BundleDir "rpm") -Filter "*.rpm" -ErrorAction SilentlyContinue | ForEach-Object {
        $sizeMB = [math]::Round($_.Length / 1MB, 2)
        Write-Host "   - $($_.Name) ($sizeMB MB)"
    }
}

if (Test-Path (Join-Path $BundleDir "appimage")) {
    Write-Host "📊 AppImage:" -ForegroundColor Green
    Get-ChildItem -Path (Join-Path $BundleDir "appimage") -Filter "*.AppImage" -ErrorAction SilentlyContinue | ForEach-Object {
        $sizeMB = [math]::Round($_.Length / 1MB, 2)
        Write-Host "   - $($_.Name) ($sizeMB MB)"
    }
}

Write-Host ""
Write-Host "💡 ヒント:" -ForegroundColor Yellow
Write-Host "   - ARM64用にビルド: .\build-linux-docker.ps1 -Target arm64"
Write-Host "   - x64用にビルド:   .\build-linux-docker.ps1 -Target x64"
Write-Host "   - AppImageを含める: .\build-linux-docker.ps1 -Target x64 -IncludeAppImage"
