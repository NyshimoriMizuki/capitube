/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate local_ip_address;

use local_ip_address::local_ip;

use server;

const PORT: u16 = 1425;
const ADDRESS: &'static str = "0.0.0.0";

fn main() {
    tauri::async_runtime::spawn(async move {
        let _server = match server::setup(ADDRESS, PORT, 1424).launch().await {
            Ok(s) => s,
            Err(e) => panic!("{}", e),
        };
    });
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_link])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_link() -> String {
    let ip = match local_ip() {
        Ok(ip) => format!("{:?}", ip),
        Err(e) => {
            eprintln!("Unexpected error in <get_link()>: {}", e);
            String::from("localhost")
        }
    };
    format!("http://{}:{}", ip, PORT)
}
