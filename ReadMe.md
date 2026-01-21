<div align="center">

![logo](./app/src-tauri/icons/Square284x284Logo.png)

</div>

# 🚀 Drop Compress Image: The Modern Image Converter

Drop Compress Image is a powerful, fast, and versatile GUI tool for converting your images into next-generation formats. Built with performance in mind, it leverages modern codecs to bring you the best in speed, quality, and file size. Say goodbye to outdated formats and hello to the future of web images!

This project was created to provide a comprehensive conversion solution, supporting a wide range of input formats and exporting to highly efficient formats like **AVIF**, **JPEG XL**, and **WebP**.

## ✨ Features

**Wide Format Support**: Convert from dozens of classic and modern image formats.

**Next-Gen Output**: Export images to AVIF, JPEG XL (also JPEG transcode), and WebP (both lossy and lossless) for superior compression and quality.

**High Performance**: Built with Rust for speed and reliability you can count on.

**International Ready**: Fully localized interface available in multiple languages.

## 🔀 Supported Formats

ImageForge handles a vast array of file types for both input and output.

### Input Formats

We support a huge range of input formats, including:

- AV1 Image Format (`*.avif`)
- Microsoft Windows Bitmap Image (`*.bmp`)
- Direct Draw Surface (`*.dds`)
- Farbfeld (`*.ff`)
- Graphics Interchange Format (`*.gif`)
- Radiance High Dynamic Range image file (`*.hdr`)
- Computer icon encoded in ICO file format (`*.ico`)
- Joint Photographic Experts Group (`*.jpg`, `*.jpeg`)
- OpenEXR image (`*.exr`)
- Portable Network Graphic (`*.png`)
- Portable Any Map (`*.pnm`)
- Quite OK Image Format (`*.qoi`)
- Truevision Graphics Adapter (`*.tga`)
- Tagged Image File Format (`*.tif`, `*.tiff`)
- WebP (`*.webp`)
- JPEG 2000 (`*.jp2`, `*.j2c`, `*.j2k`, `*.jpf`, `*.jpx`, `*.jpm`, `*.mj2`, `*.jph`)
- JPEG XL (`*.jxl`)

> **Note about HEIC/HEIF files**: HEIC/HEIF format is not supported due to licensing constraints (LGPL-3.0).
> If you need to convert HEIC files, please use macOS Preview app (File → Export → JPEG) or other tools first.
> Modern iPhones now capture photos in JPEG XL format, which is fully supported.

...and all other formats supported by the excellent [Rust image crate](https://docs.rs/image/latest/image/codecs/index.html).

### Output Formats

Optimize your images by converting them to these modern, highly efficient formats:

- **AVIF** (`.avif`)
- **JPEG XL** (`.jxl`) - **_JPEG transcode_** supported.
- **WebP** (`.webp`) - **_Lossy & Lossless_** supported (libwebp v1.6.0)
- **PNG** (`.png`) - Implemented with Oxipng. Zopfli compression.
- **JPEG** (`.jpg`) - Implemented with jpegli.

## 📦 Installation

### Package Managers

#### Windows (Chocolatey)

```powershell
choco install drop-compress-image
```

#### macOS (Homebrew)

```bash
brew tap logue/tap
brew install drop-compress-image
```

### Manual Installation

Download the latest release from the [Releases page](https://github.com/logue/DropWebP/releases):

- **Windows**: `.msi` installer
- **macOS**: `.dmg` disk image (Apple Silicon)
- **Linux**: `.deb` or `.AppImage`

## 🛠️ Building from Source

### Quick Start

#### Windows

```powershell
# Install vcpkg and dependencies first
git clone https://github.com/Microsoft/vcpkg.git C:\vcpkg
cd C:\vcpkg
.\bootstrap-vcpkg.bat

# Set environment variable
$env:VCPKG_ROOT = "C:\vcpkg"

# Install dependencies (see docs/content/en/build-windows.md for details)
cd path\to\DropWebP\app\src-tauri
.\setup-vcpkg.ps1

# Build the application
cd ..
pnpm install
pnpm run build:tauri
```

For detailed Windows build instructions including vcpkg setup, see [Windows Build Guide](./docs/content/en/build-windows.md).

**Build Issues?** See [WINDOWS_BUILD_FIX.md](WINDOWS_BUILD_FIX.md) for troubleshooting.

#### macOS / Linux

```bash
cd app
pnpm install
pnpm run build:tauri
```

For detailed build instructions, see the documentation in `docs/content/{lang}/build-*.md`.

## 🌐 Localization

The user interface is available in the following languages:

- 🇬🇧 **English**
- 🇫🇷 **French**
- 🇯🇵 **Japanese** (日本語)
- 🇰🇷 **Korean** (한국어)
- 🇨🇳 **Chinese** (中文)

## License

©2023, 2025-2026 by Logue. Licensed under the [MIT License](LICENSE).
