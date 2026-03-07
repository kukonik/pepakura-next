mod pdo_parser;
mod pepa_scene_adapter;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsePdoResult {
    pub success: bool,
    pub error: Option<String>,
    pub scene: Option<pepa_scene_adapter::PepaScene>,
}

/// Parse a PDO file and convert to PepaScene (core function without Tauri)
pub fn parse_pdo_to_pepa_core(data: &[u8]) -> ParsePdoResult {
    match pdo_parser::PdoModel::parse_from_bytes(data) {
        Ok(pdo_model) => {
            let pepa_scene: pepa_scene_adapter::PepaScene = pdo_model.into();
            ParsePdoResult {
                success: true,
                error: None,
                scene: Some(pepa_scene),
            }
        }
        Err(e) => {
            ParsePdoResult {
                success: false,
                error: Some(e.to_string()),
                scene: None,
            }
        }
    }
}

// Re-export everything from submodules
pub use pdo_parser::*;
pub use pepa_scene_adapter::*;
