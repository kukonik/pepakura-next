// Импорт из ядра
use pepakura_core::{parse_pdo_to_pepa_core, ParsePdoResult};

/// Обёртка для вызова парсера PDO из Tauri
#[tauri::command]
pub fn parse_pdo_to_pepa(data: Vec<u8>) -> ParsePdoResult {
    // Передаем данные в ядро (ссылка на массив байт)
    parse_pdo_to_pepa_core(&data)
}
