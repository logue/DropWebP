class DropCompressImage < Formula
  desc "Desktop application that converts images to WebP/Avif/JPEG XL format"
  homepage "https://github.com/logue/DropWebP"
  version "{{VERSION}}"

  on_macos do
    url "https://github.com/logue/DropWebP/releases/download/v#{version}/Drop.Compress.Image_#{version}_universal.dmg"
    sha256 "{{SHA256_UNIVERSAL}}"
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
