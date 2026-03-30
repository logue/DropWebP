# Supported Format Guide

## Supported Input Formats

Drop Compress Image supports the following input image formats:

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

## Supported Output Formats

This section explains the supported output formats for this program.

### PNG (Oxipng)

PNG (Portable Network Graphics) is a format that allows you to save images without any loss of image quality.
This program uses Zopfli, a special compression technology developed by Google that makes PNGs "smaller," though it takes longer to process.

#### PNG Features

- **Non-destructive (Lossless) Compression**: No loss of original image quality, no matter how many times you save.
- **Transparency (Alpha)**: Allows for a transparent background.

#### PNG Usage Scenarios

- **Logos, Icons, and Charts**: Images with clear borders and few colors, or images where you want a transparent background.
- **Website Materials**: Used when you want to avoid image quality degradation.
- **Preserve Original Data**: Images that may be repeatedly edited.

### JPEG (Jpegli Compression)

JPEG (Joint Photographic Experts Group) is a format primarily used for photographs, emphasizing "small size, even at the expense of some image quality."
Jpegli, used in this program, is a technology developed by Google to further reduce JPEG size while minimizing visual degradation.

#### Characteristics of JPEG

- **Lossy Compression**: To reduce file size, information that is difficult for the human eye to see is removed, resulting in a slight degradation in image quality (the higher the compression ratio, the more noticeable the degradation).
- **High Compression Ratio**: Complex images with many colors, such as photographs, can be made extremely small.
- **jpegli**: Compared to traditional JPEG, it can achieve higher image quality for the same file size, and even smaller file sizes for the same image quality. It uses the same technology as JPEG XL, enabling high-quality compression.

#### JPEG Usage Scenarios

- **General Photos**: Images with many colors and gradients, such as landscapes and portraits.
- **Large Photos on Websites**: When file size is the top priority for faster loading.

### WebP

WebP is a new image format developed by Google that combines the best features of JPEG and PNG.

#### WebP Features

- **High Compression and High Image Quality**: Maintains higher image quality than PNG while maintaining the same compression rate as JPEG.
- **Lossy/Lossless Support**: Supports both lossy compression (for photos) and lossless compression (for illustrations).
- **Transparency (Alpha) and Animation**: Supports background transparency like PNG, and animations like GIF (higher image quality and smaller files than GIF).

#### WebP Usage Scenarios

- **General Website Images**: Replace most images, including photos, icons, and animations, with this format to improve site speed.
- **Universal Image Format**: Useful when you don't want to worry about choosing a file format.

### AVIF

AVIF (AV1 Image Format) is a newer, "next-generation" image format than WebP. It was created using high-quality video technology (known as AV1).

#### AVIF Features

- **Currently Top-Class Compression Rate**: Achieves smaller file sizes with the same image quality as JPEG or WebP.
- **High-Quality Color Representation**: Supports HDR (High Dynamic Range) and a wide color gamut, allowing for richer expression of contrast and vivid colors.
- **Transparency (Alpha) and Animation**: Supports transparency and animation.

#### AVIF Usage Scenarios

- **Web Content Requiring High Image Quality**: Ideal for achieving both high image quality and fast loading times, such as when used for thumbnail images by Netflix, etc.
- **Further Website Speed**: Considered when aiming for even faster speeds than WebP.

### JPEG XL

JPEG XL is a new format developed to overcome the shortcomings of the previous JPEG standard and become the ultimate image format.

#### Features of JPEG XL

- **High Compatibility**: Convert existing JPEG images to smaller sizes with little or no loss in image quality.
- **All-Around Support**: Achieve high compression and image quality for any type of image, including photos, illustrations, and charts.
- **Multi-Features**: Supports lossless and lossy compression, transparency (alpha), animation, and high-quality color (wide color gamut and HDR).
- **Progressive Display**: Images gradually become clearer, reducing the perceived wait time, especially for large images.

#### Use Cases for JPEG XL

- **Image Format Unification**: It is expected that this single format will eventually replace many formats, including JPEG, PNG, and GIF.
- **Image Archiving**: Ideal for saving JPEG images in a smaller size without degrading the original.

## Summary of Output Formats

| Format        | Main Benefits                                                                         | Suitable Uses                                                                      | Lossy | Lossless | Alpha | HDR | Compression Ratio | Compression Load |
| ------------- | ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ----- | -------- | ----- | --- | ----------------- | ---------------- |
| PNG (Oxipng)  | No image degradation, background transparency.                                        | Logos, icons, diagrams.                                                            | ❌️    | ✅️       | ✅️    | ❌️  | Low               | High             |
| JPEG (jpegli) | Photos can be made very small with high-quality compression.                          | Commonly used photos on the web.                                                   | ✅️    | ❌️       | ❌️    | ❌️  | High              | Low              |
| WebP          | High compression, high quality, supports transparency and animation.                  | General-purpose images for websites.                                               | ✅️    | ✅️       | ✅️    | ❌️  | Medium            | Medium           |
| AVIF          | **Highest compression ratio and high image quality**, wide color gamut (HDR) support. | For websites that require both high image quality and fast display.                | ✅️    | ❌️[^1]   | ✅️    | ✅️  | High              | High             |
| JPEG XL       | **The ultimate format** that supports everything and is compatible with JPEG.         | For reducing the size of existing JPEGs and unifying image formats for the future. | ✅️    | ✅️       | ✅️    | ✅️  | High              | Low              |

[^1]:
    AVIF is lossless by definition, but this program does not implement it due to the high load and large file size.
    For lossless use, we recommend using JPEG XL.
    Reference: <https://github.com/AOMediaCodec/av1-avif/issues/111>
