#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod commands;
mod export;

use commands::unfold_mesh;
use export::export_pdf;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            unfold_mesh,
            export_pdf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}