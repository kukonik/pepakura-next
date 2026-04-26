//! Pepakura Core - library for papercraft model unfolding.
//!
//! This library provides data structures and algorithms for converting 3D meshes
//! into 2D unfoldings suitable for papercraft.

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

pub mod errors;
pub mod geometry;
pub mod unfold;
pub mod export;
pub mod util;

// Re-export public API
pub use errors::{PepakuraError, Result};
pub use geometry::{Mesh, Vertex, Face};
pub use unfold::{UnfoldConfig, UnfoldedMesh, UnfoldMetadata};
pub use export::{SvgExportConfig, PageSize, export_svg};

/// Supported export formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExportFormat {
    Svg,
    Pdf,
    Png,
    Obj,
    Stl,
}

/// Options for unfolding.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldOptions {
    pub preserve_proportions: bool,
    pub auto_rotate: bool,
    pub spacing: f64,
}

impl Default for UnfoldOptions {
    fn default() -> Self {
        UnfoldOptions {
            preserve_proportions: true,
            auto_rotate: true,
            spacing: 10.0,
        }
    }
}

/// Unfold a mesh using default MDS algorithm.
pub fn unfold_mesh(mesh: &Mesh, options: &UnfoldOptions) -> Result<UnfoldedMesh> {
    let config = UnfoldConfig {
        preserve_detail: options.preserve_proportions,
        max_iterations: 100,
        tolerance: 1e-6,
    };
    unfold::mds::unfold_mds(mesh, &config)
}

/// WASM-friendly version that takes JSON string and returns JSON string.
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub fn unfold_mesh_json(mesh_json: &str, options_json: &str) -> std::result::Result<String, String> {
    let mesh: Mesh = serde_json::from_str(mesh_json)
        .map_err(|e| format!("Failed to parse mesh JSON: {}", e))?;
    let options: UnfoldOptions = serde_json::from_str(options_json)
        .map_err(|e| format!("Failed to parse options JSON: {}", e))?;
    
    let unfolded = unfold_mesh(&mesh, &options)
        .map_err(|e| format!("Unfolding failed: {}", e))?;
    
    serde_json::to_string(&unfolded)
        .map_err(|e| format!("Failed to serialize result: {}", e))
}

/// Dummy function for testing.
pub fn dummy_function() -> String {
    "Hello from Pepakura Core".to_string()
}

// DXF module only for desktop (native) builds with dxf feature
#[cfg(all(feature = "dxf", not(target_family = "wasm")))]
pub mod dxf;