use crate::error::AppError;
use crate::options::HighBitDepthImage;
use mozjpeg::{ColorSpace, Compress, ScanMode};
use serde::{Deserialize, Serialize};

/// デフォルトの画質
const DEFAULT_QUALITY: u8 = 95;

/// JPEG (MozJPEG) エンコードオプション
///
/// MozJPEGは、Mozillaが開発した高品質なJPEGエンコーダーです。
/// 標準のJPEGエンコーダーよりも優れた圧縮率と画質を提供します。
/// Guetzliほど時間はかかりませんが、標準JPEGよりも高品質です。
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JpegOptions {
    /// 画質 (1-100の範囲、デフォルト: 95)
    /// 85以上を推奨（高品質）
    pub quality: u8,

    /// プログレッシブJPEG（推奨）
    /// Webでの読み込みパフォーマンスが向上
    #[serde(default = "default_progressive")]
    pub progressive: bool,

    /// 最適化（推奨）
    /// ファイルサイズをさらに削減
    #[serde(default = "default_optimize")]
    pub optimize: bool,
}

fn default_progressive() -> bool {
    true
}

fn default_optimize() -> bool {
    true
}

impl Default for JpegOptions {
    fn default() -> Self {
        Self {
            quality: DEFAULT_QUALITY,
            progressive: true,
            optimize: true,
        }
    }
}

/// 画像をJPEG (MozJPEG) 形式でエンコードします
///
/// # 引数
/// - `img`: 変換対象の高ビット深度画像
/// - `_icc_profile`: ICCプロファイル（現在未使用）
/// - `options`: JPEGエンコードオプション
///
/// # 戻り値
/// - 成功した場合はエンコードされたJPEGデータを `Vec<u8>` として返します
/// - 失敗した場合は `AppError` を返します
///
/// # 注意
/// - MozJPEGは標準JPEGより高品質ですが、Guetzliよりは速い
/// - 透明度はサポートされません（RGBAはRGBに変換されます）
/// - 推奨画質範囲は85-100です
pub fn encode(
    img: &HighBitDepthImage,
    _icc_profile: Option<Vec<u8>>,
    options: &JpegOptions,
) -> Result<Vec<u8>, AppError> {
    println!("Starting MozJPEG encoding...");
    println!(
        "Quality: {}, Progressive: {}, Optimize: {}",
        options.quality, options.progressive, options.optimize
    );

    // 画質の検証
    if options.quality < 85 {
        println!(
            "Warning: Quality {} is below recommended minimum of 85 for high-quality JPEG",
            options.quality
        );
    }

    // 画像の次元とRGBデータを取得
    let (width, height, rgb_data) = match img {
        HighBitDepthImage::Rgb(buf) => {
            let (w, h) = buf.dimensions();
            // f32からu8への変換
            let data: Vec<u8> = buf
                .as_raw()
                .iter()
                .map(|&x| (x.clamp(0.0, 1.0) * 255.0) as u8)
                .collect();
            (w, h, data)
        }
        HighBitDepthImage::Rgba(buf) | HighBitDepthImage::Argb(buf) => {
            let (w, h) = buf.dimensions();
            // RGBAをRGBに変換（透明度を破棄）
            println!("Warning: Converting RGBA to RGB (alpha channel will be discarded)");
            let mut rgb_data = Vec::with_capacity((w * h * 3) as usize);

            for pixel in buf.pixels() {
                let r = (pixel[0].clamp(0.0, 1.0) * 255.0) as u8;
                let g = (pixel[1].clamp(0.0, 1.0) * 255.0) as u8;
                let b = (pixel[2].clamp(0.0, 1.0) * 255.0) as u8;
                rgb_data.push(r);
                rgb_data.push(g);
                rgb_data.push(b);
            }

            (w, h, rgb_data)
        }
    };

    println!(
        "Image dimensions: {}x{}, data size: {} bytes",
        width,
        height,
        rgb_data.len()
    );

    // MozJPEGコンプレッサーの設定
    let mut comp = Compress::new(ColorSpace::JCS_RGB);

    comp.set_size(width as usize, height as usize);
    comp.set_quality(options.quality as f32);

    if options.optimize {
        comp.set_optimize_coding(true);
    }

    if options.progressive {
        comp.set_scan_optimization_mode(ScanMode::AllComponentsTogether);
        comp.set_progressive_mode();
    }

    // 圧縮データを格納するVec
    let mut jpeg_data = Vec::new();

    // 圧縮を開始（メモリライターを渡す）
    let mut comp = comp
        .start_compress(std::io::Cursor::new(&mut jpeg_data))
        .map_err(|e| AppError::Encode(format!("Failed to start JPEG compression: {}", e)))?;

    // 画像データを書き込み
    println!("Writing image data...");
    comp.write_scanlines(&rgb_data[..])
        .map_err(|e| AppError::Encode(format!("Failed to write scanlines: {}", e)))?;

    // 圧縮を完了
    comp.finish()
        .map_err(|e| AppError::Encode(format!("Failed to finish JPEG compression: {}", e)))?;

    println!("MozJPEG encoding completed: {} bytes", jpeg_data.len());

    Ok(jpeg_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_options() {
        let options = JpegOptions::default();
        assert_eq!(options.quality, DEFAULT_QUALITY);
        assert!(options.progressive);
        assert!(options.optimize);
    }
}
