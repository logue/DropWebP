use drop_webp_lib::{correct_orientation, encode_webp, load_image};
use image::DynamicImage;
use std::{fs, io::Cursor, path::PathBuf};

/// ディレクトリ内のファイル一覧を取得します。
/// # 引数
/// - `path`: 対象ディレクトリのパス
/// # 戻り値
/// - 成功した場合はファイル名のベクターを返します。
/// - 失敗した場合はエラーメッセージを返します。
/// # 注意
/// - サブディレクトリ内のファイルは含まれません。
///
#[tauri::command]
pub fn list_full_paths(path: String) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;

    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            paths.push(path_str.to_string());
        }
    }

    Ok(paths)
}

/// 単一の画像を WebP にエンコードします。
/// # 引数
/// - `input`: 対象ファイルパス
/// - `output`: 出力先ファイルパス (None の場合は元ファイルと同じディレクトリに保存されます)
/// - `overwrite`: 上書きフラグ
/// - `quality`: 品質 (0〜100)。100 の場合はロスレスになります。
///
/// # 戻り値
/// - 成功した場合は `true` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
#[tauri::command]
pub async fn convert_image(
    input: PathBuf,
    output: Option<PathBuf>,
    overwrite: bool,
    quality: i32,
) -> Result<bool, String> {
    // ブロッキングタスクとして実行
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        // 画像を開く
        let img: DynamicImage = load_image(&input).map_err(|e| e.to_string())?;

        // 出力パス決定ロジック
        let out_path = match output {
            // Some(path) の場合: pathがファイルかディレクトリかを判定
            Some(path) => {
                if path.is_dir() {
                    // pathがディレクトリの場合:
                    // 元のファイル名を使って、そのディレクトリ内に保存するパスを生成
                    let file_name = input
                        .file_name()
                        .ok_or_else(|| "Could not get file name from input path".to_string())?;

                    // 新しいファイル名 (例: image.png -> image.webp)
                    let new_file_name = PathBuf::from(file_name).with_extension("webp");

                    path.join(new_file_name)
                } else {
                    // pathがファイルの場合:
                    // そのまま出力パスとして使用
                    path
                }
            }
            // None の場合: 元のファイルと同じディレクトリに保存
            None => {
                let parent = input.parent().ok_or("Invalid file path".to_string())?;
                let filename = input
                    .file_stem()
                    .ok_or("Invalid file name".to_string())?
                    .to_string_lossy();
                parent.join(format!("{}.webp", filename))
            }
        };

        // overwriteのチェック (このロジックは元から正しい)
        if out_path.exists() && !overwrite {
            return Err(format!(
                "File '{}' already exists.",
                out_path.to_string_lossy()
            ));
        }

        // WebP に変換
        let encoded = encode_webp(&img, quality).map_err(|e| e.to_string())?;

        // 書き込み
        std::fs::write(&out_path, &encoded).map_err(|e| e.to_string())?;

        Ok(())
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(true)
}

/// Uint8Arrayバイナリデータを WebP にエンコードします。
/// # 引数
/// - `data`: 変換対象の画像データ
/// - `output`: 出力先ファイルパス
/// - `quality`: 品質 (0〜100)。100 の場合はロスレスになります。
///
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
#[tauri::command]
pub async fn convert_u8i(data: Vec<u8>, output: PathBuf, quality: i32) -> Result<bool, String> {
    // ブロッキングタスクとして実行
    tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
        // まずは image crate で読み込み
        let img = image::ImageReader::new(Cursor::new(&data))
            .with_guessed_format()
            .map_err(|e| e.to_string())?
            .decode()
            .map_err(|e| e.to_string())?;

        // ICCプロファイルがある場合はsRGBに変換（image crateでは自動変換されないので単純化）
        let img = img.to_rgba8();

        // EXIFのOrientationを補正（JPEG/HEIC向け）
        let img = correct_orientation(&img, &data);

        // WebP に変換
        let encoded =
            encode_webp(&DynamicImage::ImageRgba8(img), quality).map_err(|e| e.to_string())?;
        // 書き込み
        std::fs::write(&output, &encoded).map_err(|e| e.to_string())?;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())??;
    Ok(true)
}
