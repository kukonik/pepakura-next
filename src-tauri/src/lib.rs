mod commands;
mod ai_commands;
mod persistence;
pub mod pdo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::parse_pdo_to_pepa,
            commands::import_pdo,
            // commands::load_project,
            // commands::save_project,
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
            // commands::save_app_state,
            // commands::load_app_state,
            // commands::save_setting,
            // commands::get_setting,
            // commands::get_all_settings,
            // commands::add_recent_project,
            // commands::get_recent_projects,
            // commands::push_history,
            // commands::get_history,
            // commands::get_last_undo,
            // commands::clear_history,
            // commands::recover_from_crash,
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
            ai_commands::ai_chat_complete,
            ai_commands::ai_chat_stream_native,
            ai_commands::ai_chat_stream_with_cancel,
            ai_commands::ai_analyze_distortion,
            ai_commands::ai_analyze_nesting,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
