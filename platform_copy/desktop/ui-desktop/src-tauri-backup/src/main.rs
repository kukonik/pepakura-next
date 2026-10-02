#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::State;
use std::sync::Mutex;

mod commands;
mod unfold;
mod export;
mod ai;

/// Application state
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppState {
    pub project_path: Option<String>,
    pub model_loaded: bool,
    pub unfold_completed: bool,
}

/// Main function for the Tauri application
fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            project_path: None,
            model_loaded: false,
            unfold_completed: false,
        }))
        .invoke_handler(tauri::generate_handler![
            commands::load_model,
            commands::unfold_model,
            commands::unfold_3d_model,
            commands::export_svg,
            commands::export_png,
            commands::export_jpg,
            commands::export_obj,
            commands::export_stl,
            commands::get_app_state,
            commands::set_app_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}