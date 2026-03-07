pub mod pdo_parser;
pub mod pepa_scene_adapter;
pub mod pdo_to_pepa;
pub mod project;
pub mod nesting;
pub mod unfold;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsePdoResult {
    pub success: bool,
    pub error: Option<String>,
    pub scene: Option<pepa_scene_adapter::PepaScene>,
}

/// Parse a PDO file and convert to PepaScene (core API, без Tauri)
pub fn parse_pdo_to_pepa_core(data: &[u8]) -> ParsePdoResult {
    match pdo_parser::PdoModel::parse_from_bytes(data) {
        Ok(pdo_model) => {
            let pepa_scene = pdo_to_pepa::convert_pdo_to_pepa_scene(&pdo_model);
            ParsePdoResult {
                success: true,
                error: None,
                scene: Some(pepa_scene),
            }
        }
        Err(e) => ParsePdoResult {
            success: false,
            error: Some(e.to_string()),
            scene: None,
        },
    }
}

pub use pdo_parser::*;
pub use pepa_scene_adapter::*;
pub use pdo_to_pepa::*;
pub use project::*;
pub use nesting::*;
pub use unfold::*;
