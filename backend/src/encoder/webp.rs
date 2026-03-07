use super::common::{
    EncodingAnalysis, ToneMappingType, apply_tone_mapping, convert_f32_to_u8,
    get_encoding_recommendations, handle_icc_profile_embedding, log_encoding_analysis,
    provide_icc_recommendations,
};
use super::progress::ProgressCallback;
use crate::{encoder::extract_pixel_data, error::AppError, options::HighBitDepthImage};
use serde::{Deserialize, Serialize};
// WebPエンコーディング: ロスレス時はSimple API、ロッシー時はAdvanced API
use libwebp_sys::{
    WebPConfig, WebPEncode, WebPEncodeLosslessRGB, WebPEncodeLosslessRGBA, WebPFree,
    WebPImageHint as LibWebPImageHint, WebPMemoryWrite, WebPMemoryWriter, WebPMemoryWriterInit,
    WebPPicture, WebPPictureFree, WebPPictureImportRGB, WebPPictureImportRGBA, WebPValidateConfig,
};
use std::ffi::{c_int, c_void};
use std::sync::Arc;

/// WebP format encoding options
/// quality: 0-100 (0 is lowest quality, 100 is highest quality)
/// lossless: true/false (whether to use lossless compression)
/// method: 0-6 (0 is fast, 6 is high quality)
/// autofilter: true/false (whether to use automatic filtering)
/// hint: Image hint (WebPImageHint enumeration)
/// preset: WebP preset for different image types
/// filter_strength: 0-100 (filter strength for reducing artifacts)
/// filter_sharpness: 0-7 (sharpness of the filtering)
/// sns_strength: 0-100 (spatial noise shaping strength)
/// alpha_quality: 0-100 (alpha channel compression quality)
/// Note: When lossless is true, quality is ignored)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebpOptions {
    pub quality: f32,
    pub lossless: bool,
    pub method: u8,
    pub autofilter: bool,
    pub hint: WebPImageHint,
    pub preset: WebPPreset,
    pub filter_strength: u8,
    pub filter_sharpness: u8,
    pub sns_strength: u8,
    pub alpha_quality: u8,
}

/// WebPの画像ヒント
/// - Default: 標準的な用途
/// - Picture: 写真やリアルな画像向け
/// - Photo: 写真向け
/// - Graph: 図やイラスト向け
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebPImageHint {
    Default,
    Picture,
    Photo,
    Graph,
    Last,
}

impl WebPImageHint {
    fn to_libwebp_hint(&self) -> LibWebPImageHint {
        match self {
            WebPImageHint::Default => LibWebPImageHint::WEBP_HINT_DEFAULT,
            WebPImageHint::Picture => LibWebPImageHint::WEBP_HINT_PICTURE,
            WebPImageHint::Photo => LibWebPImageHint::WEBP_HINT_PHOTO,
            WebPImageHint::Graph => LibWebPImageHint::WEBP_HINT_GRAPH,
            WebPImageHint::Last => LibWebPImageHint::WEBP_HINT_LAST,
        }
    }
}

/// WebPプリセット
/// 異なる画像タイプに最適化された設定
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebPPreset {
    Default, // 標準設定
    Picture, // デジタル写真（ポートレートやインドアショット）
    Photo,   // 写真（アウトドア、自然光）
    Drawing, // 描画やライン画像
    Icon,    // アイコンやファビコン
    Text,    // テキストライクな画像
}

impl WebPPreset {
    /// プリセットに基づいて品質を調整
    fn adjust_quality(&self, base_quality: f32) -> f32 {
        match self {
            WebPPreset::Photo => (base_quality * 1.05).min(100.0), // 写真は少し高品質
            WebPPreset::Picture => base_quality,
            WebPPreset::Drawing => (base_quality * 0.9).max(50.0), // 描画は少し品質下げてサイズ重視
            WebPPreset::Icon => (base_quality * 0.8).max(40.0),    // アイコンはさらにサイズ重視
            WebPPreset::Text => 100.0, // テキストは最高品質（通常ロスレス）
            WebPPreset::Default => base_quality,
        }
    }

    /// プリセットに基づいてロスレス推奨かどうか
    fn prefers_lossless(&self) -> bool {
        matches!(self, WebPPreset::Text | WebPPreset::Icon)
    }

    /// プリセットに基づいたデフォルト設定を適用
    fn apply_preset_defaults(&self, config: &mut WebPConfig) {
        match self {
            WebPPreset::Photo => {
                config.sns_strength = 80;
                config.filter_sharpness = 4;
                config.filter_strength = 35;
                config.autofilter = 1;
            }
            WebPPreset::Picture => {
                config.sns_strength = 80;
                config.filter_sharpness = 3;
                config.filter_strength = 30;
                config.autofilter = 1;
            }
            WebPPreset::Drawing => {
                config.sns_strength = 25;
                config.filter_sharpness = 6;
                config.filter_strength = 10;
                config.autofilter = 0;
            }
            WebPPreset::Icon => {
                config.sns_strength = 0;
                config.filter_strength = 0;
                config.autofilter = 0;
            }
            WebPPreset::Text => {
                config.sns_strength = 0;
                config.filter_strength = 0;
                config.autofilter = 0;
                config.lossless = 1;
            }
            WebPPreset::Default => {
                // デフォルト設定はWebPConfigInitで設定済み
            }
        }
    }
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
        "WebP: Encoding settings - Quality: {}, Lossless: {}, Method: {}, Hint: {:?}, Preset: {:?}",
        options.quality, options.lossless, options.method, options.hint, options.preset
    );
    println!(
        "WebP: Advanced settings - Filter: {}, Sharpness: {}, SNS: {}, Alpha Quality: {}",
        options.filter_strength,
        options.filter_sharpness,
        options.sns_strength,
        options.alpha_quality
    );

    // プリセットがロスレス推奨かつユーザーがロスレス指定していない場合の調整
    let use_lossless = options.lossless || options.preset.prefers_lossless();

    // ロスレス時はSimple API（高速）、ロッシー時はAdvanced API（全オプション使用）
    let webp_data = if use_lossless {
        println!("WebP: Using Simple API for lossless encoding");

        unsafe {
            let mut output_ptr: *mut u8 = std::ptr::null_mut();

            let result_size = if is_rgba {
                // ロスレスRGBA
                WebPEncodeLosslessRGBA(
                    pixels_u8.as_ptr(),
                    width as i32,
                    height as i32,
                    (width * 4) as i32, // stride
                    &mut output_ptr,
                )
            } else {
                // ロスレスRGB
                WebPEncodeLosslessRGB(
                    pixels_u8.as_ptr(),
                    width as i32,
                    height as i32,
                    (width * 3) as i32, // stride
                    &mut output_ptr,
                )
            };

            if result_size == 0 || output_ptr.is_null() {
                return Err(AppError::Encode("WebP lossless encoding failed".into()));
            }

            // ポインタからVec<u8>を作成
            let output_data = std::slice::from_raw_parts(output_ptr, result_size).to_vec();

            // libwebpが確保したメモリを解放
            WebPFree(output_ptr as *mut c_void);

            output_data
        }
    } else {
        println!("WebP: Using Advanced API for lossy encoding with full options");

        unsafe {
            // 1. Config (エンコード設定) の初期化
            let mut config: WebPConfig = std::mem::zeroed();

            // libwebpの推奨する安全なデフォルト値で初期化
            config.lossless = 0;
            config.quality = 75.0;
            config.method = 4;
            config.image_hint = LibWebPImageHint::WEBP_HINT_DEFAULT;
            config.target_size = 0;
            config.target_PSNR = 0.0;
            config.segments = 4;
            config.sns_strength = 50;
            config.filter_strength = 60;
            config.filter_sharpness = 0;
            config.filter_type = 1;
            config.autofilter = 0;
            config.alpha_compression = 1;
            config.alpha_filtering = 1;
            config.alpha_quality = 100;
            config.pass = 1;
            config.show_compressed = 0;
            config.preprocessing = 0;
            config.partitions = 0;
            config.partition_limit = 0;
            config.emulate_jpeg_size = 0;
            config.thread_level = 0;
            config.low_memory = 0;
            config.near_lossless = 100;
            config.exact = 0;
            config.use_delta_palette = 0;
            config.use_sharp_yuv = 0;
            config.qmin = 0;
            config.qmax = 100;

            println!("WebP: Config initialized with safe defaults");

            // プリセットに基づいたデフォルト設定を適用
            options.preset.apply_preset_defaults(&mut config);

            // プリセットに基づく品質調整
            let adjusted_quality = options.preset.adjust_quality(options.quality);

            // ユーザー指定の詳細設定を適用（プリセットをオーバーライド）
            // WebPの有効範囲内にクランプ
            config.quality = adjusted_quality.clamp(0.0, 100.0);
            config.lossless = 0; // ロッシーエンコーディング
            config.method = (options.method as c_int).clamp(0, 6);
            config.image_hint = options.hint.to_libwebp_hint();
            config.autofilter = if options.autofilter { 1 } else { 0 };
            config.filter_strength = (options.filter_strength as c_int).clamp(0, 100);
            config.filter_sharpness = (options.filter_sharpness as c_int).clamp(0, 7);
            config.sns_strength = (options.sns_strength as c_int).clamp(0, 100);
            config.alpha_quality = (options.alpha_quality as c_int).clamp(0, 100);

            println!(
                "WebP: Final config - Quality: {:.1}, Method: {}, Hint: {:?}",
                adjusted_quality, options.method, options.hint
            );
            println!(
                "WebP: Filter settings - Strength: {}, Sharpness: {}, SNS: {}, Alpha: {}",
                options.filter_strength,
                options.filter_sharpness,
                options.sns_strength,
                options.alpha_quality
            );

            // 設定が妥当か検証（戻り値は0が失敗、1が成功）
            let validation_result = WebPValidateConfig(&config);
            println!("WebP: Config validation result: {}", validation_result);

            if validation_result == 0 {
                println!("WebP: Invalid config detected! Full config dump:");
                println!("  - lossless: {}", config.lossless);
                println!("  - quality: {}", config.quality);
                println!("  - method: {}", config.method);
                println!("  - image_hint: {:?}", config.image_hint);
                println!("  - target_size: {}", config.target_size);
                println!("  - target_PSNR: {}", config.target_PSNR);
                println!("  - segments: {}", config.segments);
                println!("  - sns_strength: {}", config.sns_strength);
                println!("  - filter_strength: {}", config.filter_strength);
                println!("  - filter_sharpness: {}", config.filter_sharpness);
                println!("  - filter_type: {}", config.filter_type);
                println!("  - autofilter: {}", config.autofilter);
                println!("  - alpha_compression: {}", config.alpha_compression);
                println!("  - alpha_filtering: {}", config.alpha_filtering);
                println!("  - alpha_quality: {}", config.alpha_quality);
                println!("  - pass: {}", config.pass);
                println!("  - preprocessing: {}", config.preprocessing);
                println!("  - partitions: {}", config.partitions);
                println!("  - partition_limit: {}", config.partition_limit);
                println!("  - near_lossless: {}", config.near_lossless);
                println!("  - qmin: {}", config.qmin);
                println!("  - qmax: {}", config.qmax);
                return Err(AppError::Encode(format!(
                    "Invalid WebPConfig - validation failed. Quality: {:.1}, Method: {}, Lossless: {}",
                    config.quality, config.method, config.lossless
                )));
            }

            // 2. Picture (画像データ) の初期化
            let mut picture: WebPPicture = std::mem::zeroed();
            if !libwebp_sys::WebPPictureInit(&mut picture) {
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
            picture.writer = Some(WebPMemoryWrite);
            picture.custom_ptr = &mut writer as *mut _ as *mut c_void;

            // 5. エンコード実行
            let encode_result = WebPEncode(&config, &mut picture);

            // 6. Picture リソースを解放 (必須)
            WebPPictureFree(&mut picture);

            if encode_result == 0 {
                // エラー時も Writer のメモリを解放
                if !writer.mem.is_null() {
                    WebPFree(writer.mem as *mut c_void);
                }
                return Err(AppError::Encode(format!(
                    "WebPEncode failed (error code: {:?})",
                    picture.error_code
                )));
            }

            // 7. 成功：エンコードされたデータを取得
            let output_data = std::slice::from_raw_parts(writer.mem, writer.size).to_vec();

            // 8. Writer が確保した C のメモリを解放 (必須)
            WebPFree(writer.mem as *mut c_void);

            output_data
        }
    };

    println!(
        "WebP: Successfully encoded WebP data (Advanced API with all options applied: Preset: {:?})",
        options.preset
    ); // ... (ICC プロファイルの処理は同じ) ...
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

/// Encode image to WebP format with progress callback support
///
/// # Arguments
/// - `pixel_data`: Source image to encode (HighBitDepthImage)
/// - `icc_profile`: ICC profile for color management
/// - `options`: WebP encoding options (WebpOptions)
/// - `progress_callback`: Progress callback implementation
///
/// # Returns
/// - Success: WebP format byte data as Vec<u8>
/// - Failure: AppError
///
/// # Notes
/// - This function supports progress reporting during encoding
/// - Only available for Advanced API (lossy encoding)
/// - Lossless encoding does not support progress callbacks
pub fn encode_with_progress(
    pixel_data: &HighBitDepthImage,
    icc_profile: Option<Vec<u8>>,
    options: &WebpOptions,
    progress_callback: Arc<dyn ProgressCallback>,
) -> Result<Vec<u8>, AppError> {
    progress_callback.on_progress(0.0, "Starting WebP encoding");

    // Perform content analysis for optimal encoding
    let analysis = EncodingAnalysis::analyze(pixel_data, icc_profile.as_deref());
    log_encoding_analysis(&analysis, "WebP");
    get_encoding_recommendations(&analysis, "WebP");

    progress_callback.on_progress(10.0, "Analysis complete");

    // Get image dimensions and pixel data
    let (width, height) = match pixel_data {
        HighBitDepthImage::Rgb(buf) => buf.dimensions(),
        HighBitDepthImage::Rgba(buf) => buf.dimensions(),
        HighBitDepthImage::Argb(buf) => buf.dimensions(),
    };
    let (pixels_f32, is_rgba) = extract_pixel_data(pixel_data);

    progress_callback.on_progress(20.0, "Extracting pixel data");

    // Apply tone mapping if needed
    let processed_pixels = if analysis.tone_mapping_required {
        progress_callback.on_progress(30.0, "Applying tone mapping");
        apply_tone_mapping(&pixels_f32, is_rgba, ToneMappingType::Reinhard, 1.0)
    } else {
        pixels_f32.to_vec()
    };

    progress_callback.on_progress(40.0, "Converting pixel format");
    let pixels_u8 = convert_f32_to_u8(&processed_pixels);

    let use_lossless = options.lossless || options.preset.prefers_lossless();

    if use_lossless {
        // Lossless encoding doesn't support progress callbacks
        progress_callback.on_progress(50.0, "Encoding (lossless - no intermediate progress)");

        let webp_data = unsafe {
            let mut output_ptr: *mut u8 = std::ptr::null_mut();

            let result_size = if is_rgba {
                WebPEncodeLosslessRGBA(
                    pixels_u8.as_ptr(),
                    width as i32,
                    height as i32,
                    (width * 4) as i32,
                    &mut output_ptr,
                )
            } else {
                WebPEncodeLosslessRGB(
                    pixels_u8.as_ptr(),
                    width as i32,
                    height as i32,
                    (width * 3) as i32,
                    &mut output_ptr,
                )
            };

            if result_size == 0 || output_ptr.is_null() {
                progress_callback.on_error("WebP lossless encoding failed");
                return Err(AppError::Encode("WebP lossless encoding failed".into()));
            }

            let output_data = std::slice::from_raw_parts(output_ptr, result_size).to_vec();
            WebPFree(output_ptr as *mut c_void);
            output_data
        };

        progress_callback.on_progress(90.0, "Finalizing");
        let final_webp_data = handle_icc_profile_embedding(webp_data, icc_profile, "WebP");
        progress_callback.on_complete();
        return Ok(final_webp_data);
    }

    // Lossy encoding with progress callback support
    progress_callback.on_progress(50.0, "Configuring encoder");

    unsafe {
        // Setup WebPConfig
        let mut config: WebPConfig = std::mem::zeroed();
        config.lossless = 0;
        config.quality = options.preset.adjust_quality(options.quality);
        config.method = options.method as c_int;
        config.image_hint = options.hint.to_libwebp_hint();
        config.target_size = 0;
        config.target_PSNR = 0.0;
        config.segments = 4;
        config.sns_strength = options.sns_strength as c_int;
        config.filter_strength = options.filter_strength as c_int;
        config.filter_sharpness = options.filter_sharpness as c_int;
        config.filter_type = 1;
        config.autofilter = if options.autofilter { 1 } else { 0 };
        config.alpha_compression = 1;
        config.alpha_filtering = 1;
        config.alpha_quality = options.alpha_quality as c_int;
        config.pass = 1;
        config.show_compressed = 0;
        config.preprocessing = 0;
        config.partitions = 0;
        config.partition_limit = 0;
        config.emulate_jpeg_size = 0;
        config.thread_level = 1;
        config.low_memory = 0;
        config.near_lossless = 100;
        config.exact = 0;
        config.use_delta_palette = 0;
        config.use_sharp_yuv = 0;
        config.qmin = 0;
        config.qmax = 100;

        options.preset.apply_preset_defaults(&mut config);

        if WebPValidateConfig(&config) == 0 {
            progress_callback.on_error("Invalid WebP configuration");
            return Err(AppError::Encode("Invalid WebP configuration".into()));
        }

        progress_callback.on_progress(60.0, "Preparing image data");

        // Setup WebPPicture with progress hook
        let mut picture: WebPPicture = std::mem::zeroed();
        picture.use_argb = 0;
        picture.width = width as c_int;
        picture.height = height as c_int;

        // Setup progress hook
        // Note: We use user_data field to store callback pointer
        extern "C" fn progress_hook(percent: c_int, picture: *const WebPPicture) -> c_int {
            unsafe {
                if picture.is_null() {
                    return 1;
                }
                let user_data = (*picture).user_data;
                if !user_data.is_null() {
                    let callback = &*(user_data as *const Arc<dyn ProgressCallback>);
                    let adjusted_percent = 60.0 + (percent as f32 * 0.30); // 60-90%
                    callback.on_progress(adjusted_percent, "Encoding");
                }
            }
            1 // Continue encoding
        }

        // Store callback pointer in picture's user_data field
        let callback_ptr = &progress_callback as *const Arc<dyn ProgressCallback>;
        picture.progress_hook = Some(progress_hook);
        picture.user_data = callback_ptr as *mut c_void;

        // Import RGB/RGBA data
        let import_result = if is_rgba {
            WebPPictureImportRGBA(&mut picture, pixels_u8.as_ptr(), (width * 4) as c_int)
        } else {
            WebPPictureImportRGB(&mut picture, pixels_u8.as_ptr(), (width * 3) as c_int)
        };

        if import_result == 0 {
            WebPPictureFree(&mut picture);
            progress_callback.on_error("Failed to import pixel data");
            return Err(AppError::Encode("Failed to import WebP pixel data".into()));
        }

        // Setup memory writer
        let mut writer: WebPMemoryWriter = std::mem::zeroed();
        WebPMemoryWriterInit(&mut writer);
        picture.writer = Some(WebPMemoryWrite);
        picture.custom_ptr = &mut writer as *mut _ as *mut c_void;

        // Encode
        let encode_result = WebPEncode(&config, &mut picture);

        WebPPictureFree(&mut picture);

        if encode_result == 0 {
            if !writer.mem.is_null() {
                WebPFree(writer.mem as *mut c_void);
            }
            progress_callback.on_error("WebP encoding failed");
            return Err(AppError::Encode(format!(
                "WebPEncode failed (error code: {:?})",
                picture.error_code
            )));
        }

        progress_callback.on_progress(90.0, "Finalizing");

        let output_data = std::slice::from_raw_parts(writer.mem, writer.size).to_vec();
        WebPFree(writer.mem as *mut c_void);

        let final_webp_data = handle_icc_profile_embedding(output_data, icc_profile, "WebP");
        provide_icc_recommendations("WebP", analysis.has_wide_gamut, analysis.has_hdr_content);

        progress_callback.on_complete();
        Ok(final_webp_data)
    }
}
