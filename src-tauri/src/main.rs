// Pepakura Next Desktop — Tauri entry point
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod ai_commands; // <--- 1. ДОБАВЛЕНО: Объявление модуля AI команд

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::parse_pdo_to_pepa,
            commands::load_project,
            commands::save_project,
            commands::nest_project,
            commands::apply_nest_overrides,
            commands::unfold_3d_model,
            commands::import_3d_model,
            commands::import_pdo,
            commands::health_check,
            // FIXME: implement export_unfold in commands.rs
            // FIXME: implement optimize_model_for_paper in commands.rs
            ai_commands::start_image_to_3d_generation
        ])
                .plugin(tauri_plugin_dialog::init())
                .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
