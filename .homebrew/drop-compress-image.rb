class DropCompressImage < Formula
  desc "Desktop application that converts images to WebP/Avif/JPEG XL format"
  homepage "https://github.com/logue/DropWebP"
  version "3.0.1"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/logue/DropWebP/releases/download/v#{version}/Drop.Compress.Image_#{version}_aarch64.dmg"
      sha256 "" # Will be filled after build
    else
      url "https://github.com/logue/DropWebP/releases/download/v#{version}/Drop.Compress.Image_#{version}_x64.dmg"
      sha256 "" # Will be filled after build
    end
  end

  def install
    prefix.install "drop-compress-image.app"
  end

  def caveats
    <<~EOS
      Drop Compress Image has been installed to:
        #{prefix}

      To use it, you can:
        1. Open it from Applications folder
        2. Or run: open "#{prefix}/drop-compress-image.app"
    EOS
  end

  test do
    assert_predicate prefix/"drop-compress-image.app", :exist?
  end
end
