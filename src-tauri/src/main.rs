mod command;

fn main() {
    tauri::Builder::default()
        // Vue から呼び出せるコマンド関数を登録
        .invoke_handler(tauri::generate_handler![
            command::list_full_paths,
            command::convert_image
        ])
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
