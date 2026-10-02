mod commands;

#[tauri::command]
fn unfold_lscm(obj_data: String) -> Result<String, String> {
    println!("[Rust DIAG] Получены данные OBJ, длина: {}", obj_data.len());
    if obj_data.contains("ERROR") {
        return Err("Модель содержит ошибки геометрии".to_string());
    }
    Ok(format!(
        "Успешно! Получено {} символов OBJ. Развертка готова (заглушка).",
        obj_data.len()
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            unfold_lscm,
            commands::parse_pdo_to_pepa,
            commands::nest_project,
            commands::export_nest_result_to_svg,
            commands::apply_nest_overrides,
            commands::import_3d_model,
            commands::unfold_3d_model,
            commands::unfold_3d_model_lscm,
            commands::unfold_3d_model_advanced,
            commands::import_pdo,
            commands::mesh_unwrap,
            commands::health_check,
            commands::export_unfold_pdf,
            commands::export_unfold_pdf_bytes,
            commands::export_unfold_dxf,
            commands::export_unfold_dxf_content,
            commands::optimize_nesting_genetic_cmd,
            commands::export_unfold_textures
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}