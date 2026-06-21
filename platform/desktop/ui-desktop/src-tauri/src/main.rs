#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod ai_runner;
mod orchestrator;
mod commands;
mod export;
mod commands_mesh;

use commands::*;

fn main() {
    ai_runner::start_ai_orchestrator();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            unfold_mesh,
            commands_mesh::optimize_mesh_for_unfold,
            export::export_pdf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

