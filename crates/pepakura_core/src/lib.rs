//! # Pepakura Core
//! 
//! Ядро для генерации развёрток бумажных моделей (papercraft).
//! 
//! ## Пример использования
//! 
//! ```rust
//! use pepakura_core::geometry::{Mesh, Vertex, Face};
//! use pepakura_core::unfold::{unfold_mds, UnfoldConfig};
//! use pepakura_core::export::{export_svg, SvgExportConfig};
//! use pepakura_core::plugins::{PluginRegistry, create_builtin_registry};
//! use pepakura_core::ai::{AiConfig, PepakuraAssistant};
//! 
//! let mut mesh = Mesh::new("Cube");
//! // Добавление вершин и граней...
//! 
//! let config = UnfoldConfig::default();
//! let unfolded = unfold_mds(&mesh, &config).unwrap();
//! 
//! let svg_config = SvgExportConfig::default();
//! let svg = export_svg(&unfolded, &svg_config).unwrap();
//! 
//! // Использование плагинов
//! let registry = create_builtin_registry();
//! 
//! // AI-помощник (если Ollama доступен)
//! let ai_config = AiConfig::default();
//! // let assistant = PepakuraAssistant::new(&ai_config).unwrap();
//! ```
//! 
//! ## Модули
//!
//! - [`geometry`] — структуры для работы с мешами (Vertex, Face, Mesh)
//! - [`unfold`] — алгоритмы развёртки (MDS, projection)
//! - [`export`] — экспорт в различные форматы (SVG)
//! - [`plugins`] — система плагинов (ImportPlugin, ExportPlugin, UnfoldPlugin)
//! - [`ai`] — AI-интеграция (Ollama, OpenAI)
//! - [`pdo_parser`] — парсер PDO файлов
//! - [`conversion`] — конвертация PDO → PepaScene
//! - [`project`] — управление проектами
//! - [`nesting`] — раскладка деталей на листе

pub mod compat;
pub mod error;
pub mod geometry;
pub mod import;
pub mod unfold;
pub mod export;
pub mod plugins;
#[cfg(feature = "native")]
pub mod ai;
pub mod analysis;
#[cfg(not(target_arch = "wasm32"))]
pub mod persistence;
pub mod pdo_parser;
pub mod pepa_scene_adapter;
pub mod pdo_to_pepa;
pub mod project;
pub mod nesting;
pub mod conversion;
pub mod sanitize;

// Ре-экспорт основных типов
pub use import::parse_obj_str;
pub use error::{PepakuraError, Result};
pub use geometry::{Mesh, Vertex, Face, BoundingBox, MeshMetadata, MeshError};
pub use unfold::{UnfoldedMesh as UnfoldResult, UnfoldConfig, UnfoldMetadata, UnfoldError, UnfoldAlgorithm};
pub use unfold::lscm::unfold_lscm;
pub use unfold::mds_optimized::{mds_parallel, mds_sparse};
pub use export::{SvgExportConfig, PageSize, ExportError};
pub use export::{PdfExportConfig, PdfOrientation, export_pdf, export_pdf_to_file};
pub use export::{DxfExportConfig, DxfUnits, export_dxf, export_dxf_to_file, DxfExportError};
pub use export::{DxfExportResult};
pub use export::{TextureExportConfig, TextureExportResult, export_textures, TextureExportError};
pub use export::{UvData, UvVertex, UvFace, extract_uv_from_mesh, generate_uv_from_position};
pub use plugins::{PluginRegistry, ImportPlugin, ExportPlugin, UnfoldPlugin, PluginMetadata};
pub use plugins::builtin::create_builtin_registry;
#[cfg(feature = "native")]
pub use ai::{AiConfig, AiProvider, AiStatus, PepakuraAssistant, create_assistant};
pub use analysis::{MeshAnalyzer, AnalysisResult, AiAnalysisResult, MeshStats, LlmStatusInfo, MeshProvider};
#[cfg(not(target_arch = "wasm32"))]
pub use persistence::Persistence;
pub use pdo_parser::*;
pub use pepa_scene_adapter::*;
pub use pdo_to_pepa::*;
pub use project::*;
pub use nesting::*;
pub use nesting::genetic::{GeneticConfig, GeneticNesting, optimize_nesting_genetic};
pub use conversion::{
    convert_pdo_to_scene,
    convert_model_to_scene,
    convert_pdo_to_scene_with_progress,
    ConversionConfig,
    ConversionError,
    FromPdoModel,
    SceneConversionProgress,
    ConversionStage,
    // Валидация
    ValidationResult,
    PdoValidator,
    GeometryValidator,
    RecoveryStrategy,
    validate_pdo_model,
    try_recover_pdo,
    is_encrypted_pdo,
};

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


