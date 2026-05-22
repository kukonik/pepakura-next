#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;

mod commands;
mod unfold;
// mod export; // TODO: Временно изолировано из-за рассинхрона API с pepakura_core (Alpha 0.1)
mod ai;
mod state;
mod utils;

// Tauri plugins

/// Application state (legacy)
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
        .manage(Mutex::new(state::AppState::default()))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::init())
        .invoke_handler(tauri::generate_handler![
            commands::load_model,
            commands::unfold_3d_model,
            commands::export_svg,
            commands::export_png,
            commands::export_jpg,
            commands::export_obj,
            commands::export_stl,
            commands::export_unfold_pdf,
            commands::get_app_state,
            commands::set_app_state,
            commands::health_check,
            commands::ping_pong,
            commands::parse_mock_obj,
            commands::start_mock_unfold,
            commands::nest_parts,
            commands::ai_generate_from_image,
            commands::ai_generate_from_text,
            commands::create_project,
            commands::import_model,
            commands::unfold_mesh,
            commands::export_svg,
            commands::get_recent_projects,
            commands::delete_project,
            commands::get_app_version,
            commands::get_settings,
            commands::save_settings,
            commands::reset_settings,
            commands::open_dialog,
            commands::load_real_obj,
            commands::save_api_key,
            commands::has_api_key,
            commands::generate_and_unfold,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}