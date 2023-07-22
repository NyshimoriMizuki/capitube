// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use server;

fn main() {
    tauri::async_runtime::spawn(async move {
        let _server = match server::setup("0.0.0.0", 1425).launch().await {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };
    });
    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
