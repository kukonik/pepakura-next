use tauri::{AppHandle, Manager, Wry};
use pepakura_core::{parse_pdo_to_pepa_core, ParsePdoResult};

#[tauri::command]
pub fn parse_pdo_to_pepa(data: Vec<u8>) -> ParsePdoResult {
    parse_pdo_to_pepa_core(&data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            parse_pdo_to_pepa,
            // ... другие команды
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
