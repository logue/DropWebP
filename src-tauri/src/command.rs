use crate::decoder::decode;
use crate::encoder::encode;
use crate::options::EncodeOptions;

/// Uint8Arrayバイナリデータを WebP にエンコードします。
/// # 引数
/// - `data`: 変換対象の画像データ
/// - `output`: 出力先ファイルパス
/// - `options`: エンコードオプション
///
/// # 戻り値
/// - 成功した場合は WebP のバイト列を `Vec<u8>` として返します。
/// - 失敗した場合は `Box<dyn Error>` を返します。
#[tauri::command]
pub async fn convert(data: Vec<u8>, options: EncodeOptions) -> Result<Vec<u8>, String> {
    // spawn_blocking でUIをフリーズさせずに重い処理を実行
    let converted_data = tauri::async_runtime::spawn_blocking(move || {
        // 画像デコード
        let img = decode(data).map_err(|e| format!("Failed to decode image: {}", e))?;
        // 画像エンコード
        let data = encode(&img, options).map_err(|e| format!("Failed to encode image: {}", e))?;

        Ok(data)
    })
    .await
    .map_err(|e| e.to_string())?;
    converted_data
}
