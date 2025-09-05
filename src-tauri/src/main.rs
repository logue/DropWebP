mod command;
mod decoder;
mod encoder;
mod options;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // Vue から呼び出せるコマンド関数を登録
        .invoke_handler(tauri::generate_handler![
            command::convert,
            command::parse_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
