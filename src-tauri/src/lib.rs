mod commands;
mod ai_commands; // <-- добавляем модуль
pub mod pdo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::parse_pdo_to_pepa,
            commands::import_pdo,
            commands::load_project,
            commands::save_project,
            commands::nest_project,
            commands::export_nest_result_to_svg,
            commands::export_sheet_to_svg_cmd,
            commands::apply_nest_overrides,
            commands::unfold_3d_model, // <-- добавляем новую команду
            ai_commands::start_image_to_3d_generation, // <-- регистрируем команду
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
