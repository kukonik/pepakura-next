// Pepakura Next Desktop — Tauri entry point
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod ai_commands;
pub mod pdo;
pub mod persistence;

use tauri::Manager;
use ai_commands::AiState;
use pepakura_core::ai::AiConfig;
use persistence::StatePersistence;
use std::path::PathBuf;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AiState::default())
        .setup(|app| {
            // Инициализация AI конфигурации
            let ai_state = app.state::<AiState>();
            let config = AiConfig::default();
            *ai_state.config.lock().unwrap() = config;

            // Инициализация персистентности (закомментировано, так как команды не используются)
            // let app_data_dir = app
            //     .path()
            //     .app_data_dir()
            //     .map_err(|e| format!("Failed to get app data dir: {}", e))?;
            //
            // let db_path = app_data_dir.join("state.db");
            //
            // // Создаём директорию если не существует
            // if let Some(parent) = db_path.parent() {
            //     std::fs::create_dir_all(parent)
            //         .map_err(|e| format!("Failed to create app data dir: {}", e))?;
            // }
            //
            // let persistence = StatePersistence::new(&db_path)
            //     .map_err(|e| format!("Failed to initialize persistence: {}", e))?;
            //
            // app.manage(persistence);

            #[cfg(debug_assertions)]
            {
                if let Some(window) = app.get_webview_window("main") {
                    window.open_devtools();
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::parse_pdo_to_pepa,
            commands::import_pdo,
            commands::nest_project,
            commands::export_nest_result_to_svg,
            commands::export_sheet_to_svg_cmd,
            commands::apply_nest_overrides,
            commands::import_3d_model,
            commands::unfold_3d_model,
            commands::unfold_3d_model_lscm,
            commands::unfold_3d_model_advanced,
            commands::export_unfold_pdf,
            commands::export_unfold_pdf_bytes,
            commands::export_unfold_dxf,
            commands::export_unfold_dxf_content,
            commands::optimize_nesting_genetic_cmd,
            commands::export_unfold_textures,
            ai_commands::ai_check_status,
            ai_commands::ai_get_unfold_advice,
            ai_commands::ai_generate_instructions,
            ai_commands::ai_chat,
            ai_commands::ai_update_config,
            ai_commands::ai_get_config,
            ai_commands::ai_recommend_paper,
            ai_commands::ai_get_cache_stats,
            ai_commands::ai_clear_cache,
            ai_commands::ai_set_cache_enabled,
            ai_commands::ai_cache_contains,
            ai_commands::ai_chat_stream,
            ai_commands::ai_chat_complete
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
