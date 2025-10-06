#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod command;
mod decoder;
mod encoder;
mod error;
mod logging;
mod options;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    println!("[{:.2?}] App start", start_time.elapsed());
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_notification::init())
        // Vue から呼び出せるコマンド関数を登録
        .invoke_handler(tauri::generate_handler![
            command::convert,
            command::get_path_info,
            command::delete_path
        ])
        .setup(|app| {
            // Initialize logging system
            logging::init_logging(app.handle().clone());
            logging::send_log(logging::LogLevel::Info, "Application started successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    println!("[{:.2?}] App exit", start_time.elapsed());
}
