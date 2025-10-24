#!/bin/bash
# Homebrew Formula 生成スクリプト

set -e

VERSION="${1:-2.3.0}"

echo "=== Homebrew Formula Generation ==="
echo "Version: $VERSION"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HOMEBREW_DIR="$ROOT_DIR/.homebrew"
BUNDLE_DIR="$ROOT_DIR/app/src-tauri/target/release/bundle"

# DMGファイルを探す
DMG_AARCH64=$(find "$BUNDLE_DIR/dmg" -name "*aarch64*.dmg" | head -n 1)
DMG_X64=$(find "$BUNDLE_DIR/dmg" -name "*x64*.dmg" | head -n 1)

if [ -z "$DMG_AARCH64" ] || [ -z "$DMG_X64" ]; then
    echo "Error: DMG files not found in $BUNDLE_DIR/dmg"
    exit 1
fi

echo "Found DMG files:"
echo "  ARM64: $(basename "$DMG_AARCH64")"
echo "  x64:   $(basename "$DMG_X64")"

# チェックサムを計算
SHA256_AARCH64=$(shasum -a 256 "$DMG_AARCH64" | cut -d' ' -f1)
SHA256_X64=$(shasum -a 256 "$DMG_X64" | cut -d' ' -f1)

echo ""
echo "SHA256 Checksums:"
echo "  ARM64: $SHA256_AARCH64"
echo "  x64:   $SHA256_X64"

# Formulaファイルを更新
FORMULA_FILE="$HOMEBREW_DIR/drop-compress-image.rb"

cat > "$FORMULA_FILE" << EOF
class DropCompressImage < Formula
  desc "Desktop application that converts images to WebP/Avif/JPEG XL format"
  homepage "https://github.com/logue/DropWebP"
  version "$VERSION"
  
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/logue/DropWebP/releases/download/v#{version}/Drop.Compress.Image_#{version}_aarch64.dmg"
      sha256 "$SHA256_AARCH64"
    else
      url "https://github.com/logue/DropWebP/releases/download/v#{version}/Drop.Compress.Image_#{version}_x64.dmg"
      sha256 "$SHA256_X64"
    end
  end

  def install
    prefix.install "Drop Compress Image.app"
  end

  def caveats
    <<~EOS
      Drop Compress Image has been installed to:
        #{prefix}

      To use it, you can:
        1. Open it from Applications folder
        2. Or run: open "#{prefix}/Drop Compress Image.app"
    EOS
  end

  test do
    assert_predicate prefix/"Drop Compress Image.app", :exist?
  end
end
EOF

echo ""
echo "Formula updated successfully!"
echo "Formula location: $FORMULA_FILE"

echo ""
echo "=== Next Steps ==="
echo "1. Test the formula locally:"
echo "   brew install --formula $FORMULA_FILE"
echo "2. Create a tap repository and push the formula"
echo "3. Users can install with:"
echo "   brew tap logue/tap"
echo "   brew install drop-compress-image"
