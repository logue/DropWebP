# jpegxl-srcのビルドスクリプトをオーバーライドしてMSVCを使用
# ClangCLの代わりにMSVCツールセットを使用
$env:CMAKE_GENERATOR = "Visual Studio 17 2022"
$env:CMAKE_GENERATOR_TOOLSET = $null  # ClangCL指定を削除
$env:CMAKE_GENERATOR_PLATFORM = "x64"

# libaom-sysでアセンブラなしでビルド（NASMなしでもビルド可能）
# この環境変数がlibaom-sysのbuild.rsで読み取られてCMakeに渡される
$env:AOM_TARGET_CPU = "generic"

Write-Host "Configuring build environment..." -ForegroundColor Cyan
Write-Host "  CMake Generator: Visual Studio 17 2022" -ForegroundColor Gray
Write-Host "  libaom: Building without assembler (generic CPU target)" -ForegroundColor Gray
Write-Host "" -ForegroundColor Gray

# NASMを一時的にPATHから削除（libaom-sysのmultipass最適化エラーを回避）
# libaom-sysはNASMが見つからない場合、C/C++実装にフォールバックする
Write-Host "Temporarily removing NASM from PATH..." -ForegroundColor Yellow
$originalPath = $env:PATH
$env:PATH = ($env:PATH -split ';' | Where-Object { $_ -notlike '*NASM*' }) -join ';'
Write-Host "NASM removed from PATH" -ForegroundColor Green

# jpegxl-srcのCargoキャッシュディレクトリを検索
$jpegxlSrcDir = Get-ChildItem "$env:USERPROFILE\.cargo\registry\src\index.crates.io-*\jpegxl-src-*" -ErrorAction SilentlyContinue | Select-Object -First 1

if ($jpegxlSrcDir) {
    Write-Host "`nFound jpegxl-src at: $jpegxlSrcDir" -ForegroundColor Cyan
    $buildRsPath = Join-Path $jpegxlSrcDir "build.rs"

    if (Test-Path $buildRsPath) {
        Write-Host "Patching jpegxl-src build.rs to remove ClangCL requirement..." -ForegroundColor Yellow

        $content = Get-Content $buildRsPath -Raw

        # ClangCL指定を削除するパッチ
        # 元: .define("T", "ClangCL")
        # 変更後: 削除（デフォルトのMSVCツールセットを使用）
        $content = $content -replace '\.define\("T",\s*"ClangCL"\)', ''

        Set-Content $buildRsPath $content -NoNewline
        Write-Host "Patched successfully" -ForegroundColor Green
    } else {
        Write-Host "Warning: build.rs not found at $buildRsPath" -ForegroundColor Yellow
    }
} else {
    Write-Host "Info: jpegxl-src not found in cargo cache yet. It will be downloaded during first build." -ForegroundColor Cyan
}

Write-Host "`nBuilding with MSVC toolset (NASM disabled)..." -ForegroundColor Cyan
cargo build --release --verbose

# PATH環境変数を復元
$env:PATH = $originalPath

if ($LASTEXITCODE -eq 0) {
    Write-Host "`nBuild completed successfully!" -ForegroundColor Green
} else {
    Write-Host "`nBuild failed with exit code $LASTEXITCODE" -ForegroundColor Red
    exit $LASTEXITCODE
}
