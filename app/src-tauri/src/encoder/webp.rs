use super::common::{
    apply_tone_mapping, convert_f32_to_u8, get_encoding_recommendations,
    handle_icc_profile_embedding, log_encoding_analysis, provide_icc_recommendations,
    EncodingAnalysis, ToneMappingType,
};
use crate::{encoder::extract_pixel_data, error::AppError, options::HighBitDepthImage};
use serde::{Deserialize, Serialize};
// 必要なものを use
use libwebp_sys::{
    WebPConfig, WebPConfigInit, WebPEncode, WebPFree, WebPMemoryWrite, WebPMemoryWriter,
    WebPMemoryWriterInit, WebPPicture, WebPPictureFree, WebPPictureImportRGB,
    WebPPictureImportRGBA, WebPPictureInit, WebPValidateConfig,
};
use std::ffi::{c_int, c_void};

/// WebP format encoding options
/// quality: 0-100 (0 is lowest quality, 100 is highest quality)
/// lossless: true/false (whether to use lossless compression)
/// method: 0-6 (0 is fast, 6 is high quality)
/// autofilter: true/false (whether to use automatic filtering)
/// hint: Image hint (WebPImageHint enumeration)
/// Note: When lossless is true, quality is ignored)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebpOptions {
    pub quality: f32,
    pub lossless: bool,
    pub method: u8,
    pub autofilter: bool,
    pub hint: WebPImageHint,
    // pub preset: WebPPreset,
}

/// WebPの画像ヒント
/// - Default: 標準的な用途
/// - Picture: 写真やリアルな画像向け
/// - Photo: 写真向け
/// - Graph: 図やイラスト向け
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum WebPImageHint {
    Default = libwebp_sys::WEBP_HINT_DEFAULT as isize,
    Picture = libwebp_sys::WEBP_HINT_PICTURE as isize,
    Photo = libwebp_sys::WEBP_HINT_PHOTO as isize,
    Graph = libwebp_sys::WEBP_HINT_GRAPH as isize,
    Last = libwebp_sys::WEBP_HINT_LAST as isize,
}

/// Encode image to WebP format with advanced content analysis
/// # Arguments
/// - `pixel_data`: Source image to encode (HighBitDepthImage)
/// - `icc_profile`: ICC profile for color management
/// - `options`: WebP encoding options (WebpOptions)
/// # Returns
/// - Success: WebP format byte data as Vec<u8>
/// - Failure: AppError
/// # Notes
/// - Uses `libwebp-sys` crate for WebP encoding. Build requires `libwebp` library installed on system
/// - Performs content analysis for optimal encoding settings
pub fn encode(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &WebpOptions,
) -> Result<Vec<u8>, AppError> {
    println!("WebP: Starting WebP encoding process...");

    // Perform content analysis for optimal encoding
    let analysis = EncodingAnalysis::analyze(pixel_data, icc_profile.as_deref());
    log_encoding_analysis(&analysis, "WebP");
    get_encoding_recommendations(&analysis, "WebP");

    // Get image dimensions and pixel data
    let (width, height) = match pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };
    let (pixels_f32, is_rgba) = extract_pixel_data(pixel_data);

    println!(
        "WebP: Image properties - {}x{}, {} channels",
        width,
        height,
        if is_rgba { 4 } else { 3 }
    );
    println!(
        "WebP: Encoding settings - Quality: {}, Lossless: {}",
        options.quality, options.lossless
    );

    // Apply tone mapping if HDR content is detected
    let processed_pixels = if analysis.tone_mapping_required {
        println!(
            "WebP: Applying tone mapping for HDR content (max luminance: {:.3})",
            analysis.max_luminance
        );
        apply_tone_mapping(&pixels_f32, is_rgba, ToneMappingType::Reinhard, 1.0)
    } else if analysis.has_wide_gamut {
        println!("WebP: Processing wide gamut content");
        pixels_f32.to_vec()
    } else {
        pixels_f32.to_vec()
    };

    // Convert f32 pixels to u8 (WebP encoders primarily work with 8-bit input)
    let pixels_u8 = convert_f32_to_u8(&processed_pixels);
    println!(
        "WebP: Encoding settings - Quality: {}, Lossless: {}, Method: {}, Hint: {:?}",
        options.quality, options.lossless, options.method, options.hint
    );

    // ★ 4. & 5. を libwebp-sys2 (Advanced API) を使って置き換え
    let webp_data = unsafe {
        // 1. Config (エンコード設定) の初期化と設定
        let mut config: WebPConfig = std::mem::zeroed();
        if WebPConfigInit(&mut config) == 0 {
            return Err(AppError::Encode("WebPConfigInit failed".into()));
        }

        // Options から Config に値を設定
        config.quality = options.quality;
        config.lossless = if options.lossless { 1 } else { 0 };
        config.method = options.method as c_int;
        config.image_hint = options.hint as u32; // enum の値を u32 として渡す
        config.autofilter = if options.autofilter { 1 } else { 0 };
        // config.sns_strength = ...; // 他にも多くの設定が可能

        // 設定が妥当か検証
        if WebPValidateConfig(&config) == 0 {
            return Err(AppError::Encode("Invalid WebPConfig".into()));
        }

        // 2. Picture (画像データ) の初期化
        let mut picture: WebPPicture = std::mem::zeroed();
        if WebPPictureInit(&mut picture) == 0 {
            return Err(AppError::Encode("WebPPictureInit failed".into()));
        }
        picture.width = width as c_int;
        picture.height = height as c_int;

        // 3. Picture へピクセルデータをインポート
        let import_result = if is_rgba {
            // RGBA
            let stride = width as i32 * 4;
            WebPPictureImportRGBA(&mut picture, pixels_u8.as_ptr(), stride)
        } else {
            // RGB
            let stride = width as i32 * 3;
            WebPPictureImportRGB(&mut picture, pixels_u8.as_ptr(), stride)
        };

        if import_result == 0 {
            WebPPictureFree(&mut picture); // 失敗したら Picture を解放
            return Err(AppError::Encode("WebPPictureImport failed".into()));
        }

        // 4. メモリ書き込み用の Writer を準備
        let mut writer: WebPMemoryWriter = std::mem::zeroed();
        WebPMemoryWriterInit(&mut writer);
        picture.writer = std::mem::transmute(WebPMemoryWrite as *const ()); // 書き込み関数へのポインタ
        picture.custom_ptr = &mut writer as *mut _ as *mut c_void; // 書き込み先

        // 5. エンコード実行
        let encode_result = WebPEncode(&config, &mut picture);

        // 6. Picture リソースを解放 (必須)
        // ピクセルデータはインポート時にコピーされているため、エンコード後すぐに解放してOK
        WebPPictureFree(&mut picture);

        if encode_result == 0 {
            // エラー時も Writer のメモリを解放
            WebPFree(writer.mem as *mut c_void);
            return Err(AppError::Encode(format!(
                "WebPEncode failed (error code: {})",
                picture.error_code
            )));
        }

        // 7. 成功：エンコードされたデータを取得
        // writer.mem (ポインタ) と writer.size (長さ) から Rust の Vec<u8> を作成
        let output_data = std::slice::from_raw_parts(writer.mem, writer.size).to_vec();

        // 8. Writer が確保した C のメモリを解放 (必須)
        WebPFree(writer.mem as *mut c_void);

        output_data
    };

    println!("WebP: Successfully encoded WebP data (using Advanced API)");

    // ... (ICC プロファイルの処理は同じ) ...
    let final_webp_data = handle_icc_profile_embedding(webp_data, icc_profile, "WebP");

    // ... (ICC の推奨事項も同じ) ...
    provide_icc_recommendations("WebP", analysis.has_wide_gamut, analysis.has_hdr_content);

    Ok(final_webp_data)
}

/// WebPファイルサイズを推定
pub fn estimate_size(img: &HighBitDepthImage, options: &WebpOptions) -> usize {
    let (width, height) = match img {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };

    let channels = match img {
        HighBitDepthImage::Rgb(_) => 3,
        HighBitDepthImage::Rgba(_) => 4,
        HighBitDepthImage::Argb(_) => 4,
    };

    let uncompressed_size = (width * height * channels) as usize;

    // WebPの圧縮率推定
    let compression_ratio = if options.lossless {
        0.4 // ロスレスの場合は40%圧縮
    } else {
        // 品質に基づく圧縮率（品質が高いほど圧縮率は低い）
        let quality_factor = options.quality / 100.0;
        0.1 + (quality_factor * 0.5) // 10%-60%の範囲
    };

    (uncompressed_size as f64 * compression_ratio as f64) as usize
}
