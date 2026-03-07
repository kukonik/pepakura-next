use pepakura_core::{parse_pdo_to_pepa_core, ParsePdoResult};

#[tauri::command]
pub fn parse_pdo_to_pepa(data: Vec<u8>) -> ParsePdoResult {
    parse_pdo_to_pepa_core(&data)
}
