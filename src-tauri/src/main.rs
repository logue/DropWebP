mod command;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        // Vue から呼び出せるコマンド関数を登録
        .invoke_handler(tauri::generate_handler![
            command::list_full_paths,
            command::convert_image,
            command::convert_u8i
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
