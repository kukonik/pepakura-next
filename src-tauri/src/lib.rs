use tauri::command;

#[command]
fn unfold_lscm(obj_data: String) -> Result<String, String> {
    println!("[Rust DIAG] Получены данные OBJ, длина: {}", obj_data.len());
    if obj_data.contains("ERROR") { return Err("Модель содержит ошибки геометрии".to_string()); }
    Ok(format!("Успешно! Получено {} символов OBJ. Развертка готова (заглушка).", obj_data.len()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![unfold_lscm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
