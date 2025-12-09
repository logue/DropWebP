# jpegxl-srcのビルドスクリプトをオーバーライドしてMSVCを使用
$env:CMAKE_GENERATOR_TOOLSET = "host=x64"
$env:CMAKE_GENERATOR_PLATFORM = "x64"

# jpegxl-srcのCargoキャッシュディレクトリ
$jpegxlSrcDir = Get-ChildItem "$env:USERPROFILE\.cargo\registry\src\index.crates.io-*\jpegxl-src-*" | Select-Object -First 1

if ($jpegxlSrcDir) {
    Write-Host "Patching jpegxl-src build.rs..."
    $buildRsPath = Join-Path $jpegxlSrcDir "build.rs"

    if (Test-Path $buildRsPath) {
        $content = Get-Content $buildRsPath -Raw
        $content = $content -replace '\.define\("T", "ClangCL"\)', '.define("T", "host=x64")'
        Set-Content $buildRsPath $content
        Write-Host "Patched successfully"
    }
}

Write-Host "Building with MSVC toolset..."
cargo build --release
