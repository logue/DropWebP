use crate::{encoder::extract_pixel_data, error::AppError};
use image::DynamicImage;
use libwebp_sys::{
    WebPConfigInit, WebPEncode, WebPFree, WebPMemoryWrite, WebPMemoryWriterInit, WebPPictureFree,
    WebPPictureImportRGB, WebPPictureImportRGBA, WebPPictureInit, WebPValidateConfig,
};
use serde::{Deserialize, Serialize};
use std::{ffi::c_void, slice::from_raw_parts};

/// WebP形式のオプション
/// quality: 0-100 (0は最低品質、100は最高品質)
/// lossless: true/false (可逆圧縮を使うかどうか
/// method: 0-6 (0は高速、6は高品質)
/// autofilter: true/false (自動フィルタリングを使うかどうか)
/// hint: 画像のヒント (WebPImageHint列挙型)
/// 注意: losslessがtrueの場合、qualityは無視される)
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WebpOptions {
    pub quality: f32,
    pub lossless: bool,
    pub method: u8,
    pub autofilter: bool,
    pub hint: WebPImageHint,
    pub preset: WebPPreset,
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(isize)]
pub enum WebPPreset {
    Default = libwebp_sys::WEBP_PRESET_DEFAULT as isize,
    Picture = libwebp_sys::WEBP_PRESET_PICTURE as isize,
    Photo = libwebp_sys::WEBP_PRESET_PHOTO as isize,
    Drawing = libwebp_sys::WEBP_PRESET_DRAWING as isize,
    Icon = libwebp_sys::WEBP_PRESET_ICON as isize,
    Text = libwebp_sys::WEBP_PRESET_TEXT as isize,
}

/// 画像を WebP にエンコードします。
/// # 引数
/// - `img`: 変換対象の画像 (DynamicImage)
/// - `options`: WebPエンコードオプション (WebpOptions)
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `AppError` を返します。
/// # 注意
/// - `libwebp-sys` クレートを使用して WebP エンコードを行います。ビルド時に `libwebp` ライブラリがシステムにインストールされている必要があります。
pub fn encode(img: &DynamicImage, options: &WebpOptions) -> Result<Vec<u8>, AppError> {
    let width = img.width() as i32;
    let height = img.height() as i32;

    // 1. データ準備
    let (raw, is_rgba) = extract_pixel_data(img);

    // libwebpはCライブラリなので、unsafeブロックで囲む
    unsafe {
        // --------------------------------------------------------------------
        // 2. 高度なAPI: WebPConfig の設定
        // --------------------------------------------------------------------
        let mut config = std::mem::MaybeUninit::uninit();
        if WebPConfigInit(config.as_mut_ptr()) == 0 {
            return Err(AppError::Encode(
                "WebPConfig initialization failure.".into(),
            ));
        }
        let mut config = config.assume_init();

        // オプションからロスレス設定を反映
        if options.lossless {
            config.lossless = 1;
        }

        // オプションからヒントを反映
        config.image_hint = options.hint as u32;
        config.method = options.method.clamp(0, 6) as i32;
        config.autofilter = if options.autofilter { 1 } else { 0 };

        // (推奨) 設定が有効か検証
        if WebPValidateConfig(&config) == 0 {
            return Err(AppError::Encode("Invalid WebPConfig.".into()));
        }

        // --------------------------------------------------------------------
        // 3. 高度なAPI: WebPPicture の設定とピクセルデータのインポート
        // --------------------------------------------------------------------
        let mut picture = std::mem::MaybeUninit::uninit();
        if WebPPictureInit(picture.as_mut_ptr()) == 0 {
            return Err(AppError::Encode(
                "WebPPicture initialization failure.".into(),
            ));
        }
        let mut picture = picture.assume_init();
        picture.width = width as i32;
        picture.height = height as i32;

        // RGBAかRGBかに応じて、適切なインポート関数を呼び出す
        let stride = if is_rgba {
            width.checked_mul(4).ok_or(AppError::Encode(
                "Stride calculation overflowed".to_string(),
            ))? as i32
        } else {
            width.checked_mul(3).ok_or(AppError::Encode(
                "Stride calculation overflowed".to_string(),
            ))? as i32
        };

        let import_ok = if is_rgba {
            WebPPictureImportRGBA(&mut picture, raw.as_ptr(), stride)
        } else {
            WebPPictureImportRGB(&mut picture, raw.as_ptr(), stride)
        };

        if import_ok == 0 {
            WebPPictureFree(&mut picture); // 失敗時も解放が必要
            return Err(AppError::Encode("Failed to import pixel data.".into()));
        }

        // --------------------------------------------------------------------
        // 4. エンコードの実行と結果の取得
        // --------------------------------------------------------------------
        let mut writer = std::mem::MaybeUninit::<libwebp_sys::WebPMemoryWriter>::uninit();
        WebPMemoryWriterInit(writer.as_mut_ptr());
        let mut writer = writer.assume_init();
        picture.writer = Some(std::mem::transmute(WebPMemoryWrite as usize));
        picture.custom_ptr = &mut writer as *mut _ as *mut std::ffi::c_void;

        let result = if WebPEncode(&config, &mut picture) == 1 {
            // 成功した場合
            let slice = from_raw_parts(writer.mem, writer.size);
            Ok(slice.to_vec())
        } else {
            // 失敗した場合
            Err(AppError::Encode("WebP encoding failure.".into()))
        };

        // --------------------------------------------------------------------
        // 5. C側で確保された全てのメモリを解放
        // --------------------------------------------------------------------
        WebPPictureFree(&mut picture);
        WebPFree(writer.mem as *mut c_void); // WebPMemoryWriterが確保したメモリも解放

        println!("Finished encoding WebP.");

        result
    }
}
