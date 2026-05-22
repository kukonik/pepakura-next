//! Bridge between Tauri and the unfolding core

use pepakura_core::{Model, UnfoldResult};
use pepakura_core::unfold::{UnfoldedFace, Seam, LayoutResult, PlacedFace, Point2D};

/// Convert a Tauri mesh to a core model
///
/// # Arguments
/// * `tauri_mesh` - The Tauri mesh to convert
///
/// # Returns
/// * `Model` - The converted core model
pub fn tauri_mesh_to_core(tauri_mesh: TauriMesh) -> Model {
    Model {
        vertices: tauri_mesh.vertices.iter().map(|v| pepakura_core::model::Vertex { x: v.x, y: v.y, z: v.z }).collect(),
        faces: tauri_mesh.faces.iter().map(|f| pepakura_core::model::Face { vertices: f.clone() }).collect(),
    }
}

/// Convert a core model to a Tauri mesh
///
/// # Arguments
/// * `core_model` - The core model to convert
///
/// # Returns
/// * `TauriMesh` - The converted Tauri mesh
pub fn core_mesh_to_tauri(core_model: &Model) -> TauriMesh {
    TauriMesh {
        vertices: core_model.vertices.iter().map(|v| Vertex3D { x: v.x, y: v.y, z: v.z }).collect(),
        faces: core_model.faces.iter().map(|f| f.vertices.clone()).collect(),
    }
}

/// Convert a core unfold result to a Tauri unfold result
///
/// # Arguments
/// * `core_result` - The core unfold result to convert
///
/// # Returns
/// * `TauriUnfoldResult` - The converted Tauri unfold result
pub fn core_result_to_tauri(core_result: &UnfoldResult) -> TauriUnfoldResult {
    // Заглушка: возвращаем пустые данные
    TauriUnfoldResult {
        layouts: Vec::new(),
        seams: Vec::new(),
        stats: TauriUnfoldStats {
            face_count: core_result.faces.len(),
            layout_count: 1, // core_result.layout - единственный макет
            seam_count: core_result.seams.len(),
            processing_time: 0.0,
        },
    }
}

/// Convert a Tauri unfold result to a core unfold result
///
/// # Arguments
/// * `tauri_result` - The Tauri unfold result to convert
///
/// # Returns
/// * `UnfoldResult` - The converted core unfold result
pub fn tauri_result_to_core(tauri_result: &TauriUnfoldResult) -> UnfoldResult {
    // Заглушка: возвращаем пустой результат
    UnfoldResult {
        faces: Vec::new(),
        seams: Vec::new(),
        layout: LayoutResult {
            faces: Vec::new(),
            width: 0.0,
            height: 0.0,
        },
    }
}

/// Tauri representation of a 3D vertex
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Vertex3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Tauri representation of a 2D vertex
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Vertex2D {
    pub x: f64,
    pub y: f64,
}

/// Tauri representation of a mesh
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TauriMesh {
    pub vertices: Vec<Vertex3D>,
    pub faces: Vec<Vec<usize>>,
}

/// Tauri representation of a 2D face
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TauriFace2D {
    pub vertex_indices: Vec<usize>,
    pub original_face_index: usize,
}

/// Tauri representation of a layout
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TauriLayout {
    pub vertices: Vec<Vertex2D>,
    pub faces: Vec<TauriFace2D>,
    pub bounds: [f64; 4],
}

/// Tauri representation of seam information
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TauriSeamInfo {
    pub face_indices: [usize; 2],
    pub length: f64,
}

/// Tauri representation of unfolding statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TauriUnfoldStats {
    pub face_count: usize,
    pub layout_count: usize,
    pub seam_count: usize,
    pub processing_time: f64,
}

/// Tauri representation of an unfold result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TauriUnfoldResult {
    pub layouts: Vec<TauriLayout>,
    pub seams: Vec<TauriSeamInfo>,
    pub stats: TauriUnfoldStats,
}