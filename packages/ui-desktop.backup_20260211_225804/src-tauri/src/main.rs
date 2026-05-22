// Предventing запуск консоли в Windows
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod ai;
mod unfold;
mod export;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .manage(ai::commands::AiState {
            config: std::sync::Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            // Команды для работы с 3D моделями
            commands::import_model,
            commands::save_project,
            commands::load_project,
            
            // Команды для разворачивания
            unfold::commands::unfold_model,
            unfold::commands::layout_unfolded_pieces,
            
            // Команды для оптимизации бумаги
            commands::optimize_model_for_paper,
            commands::get_default_paper_optimize_params,
            
            // Команды для экспорта
            export::export_to_svg,
            export::export_to_png,
            export::export_to_jpg,
            export::export_to_obj,
            export::export_to_stl,
            
            // Команды для AI
            ai::commands::send_ai_message,
            ai::commands::get_ai_suggestions,
            ai::commands::configure_ai_backend,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}