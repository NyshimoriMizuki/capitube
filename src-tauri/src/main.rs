// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(unused_imports)]
use tauri::Manager;

const COMPILE_MODE: &str = "dev";

fn main() {
    #[allow(unused_variables)]
    tauri::Builder::default()
        .setup(|app| {
            if COMPILE_MODE == "dev" {
                #[cfg(debug_assertions)]
                {
                    let window = app.get_window("main").unwrap();
                    window.open_devtools();
                    window.close_devtools();
                }
            }
            Ok(())
        })
        // .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
