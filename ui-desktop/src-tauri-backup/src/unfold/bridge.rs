//! Bridge between Tauri and the unfolding core

use pepakura_core::model::{Mesh, Vertex, Face};
use pepakura_core::unfold::{UnfoldResult, Layout, Face2D, SeamInfo, UnfoldStats};

/// Convert a Tauri mesh to a core mesh
/// 
/// # Arguments
/// * `tauri_mesh` - The Tauri mesh to convert
/// 
/// # Returns
/// * `Mesh` - The converted core mesh
pub fn tauri_mesh_to_core(tauri_mesh: TauriMesh) -> Mesh {
    Mesh {
        vertices: tauri_mesh.vertices.iter().map(|v| [v.x, v.y, v.z]).collect(),
        faces: tauri_mesh.faces.clone(),
    }
}

/// Convert a core mesh to a Tauri mesh
/// 
/// # Arguments
/// * `core_mesh` - The core mesh to convert
/// 
/// # Returns
/// * `TauriMesh` - The converted Tauri mesh
pub fn core_mesh_to_tauri(core_mesh: &Mesh) -> TauriMesh {
    TauriMesh {
        vertices: core_mesh.vertices.iter().map(|v| Vertex3D { x: v[0], y: v[1], z: v[2] }).collect(),
        faces: core_mesh.faces.clone(),
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
    TauriUnfoldResult {
        layouts: core_result.layouts.iter().map(|l| TauriLayout {
            vertices: l.vertices.iter().map(|v| Vertex2D { x: v[0], y: v[1] }).collect(),
            faces: l.faces.iter().map(|f| TauriFace2D {
                vertex_indices: f.vertex_indices.clone(),
                original_face_index: f.original_face_index,
            }).collect(),
            bounds: l.bounds,
        }).collect(),
        seams: core_result.seams.iter().map(|s| TauriSeamInfo {
            face_indices: s.face_indices,
            length: s.length,
        }).collect(),
        stats: TauriUnfoldStats {
            face_count: core_result.stats.face_count,
            layout_count: core_result.stats.layout_count,
            seam_count: core_result.stats.seam_count,
            processing_time: core_result.stats.processing_time,
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
    UnfoldResult {
        layouts: tauri_result.layouts.iter().map(|l| Layout {
            vertices: l.vertices.iter().map(|v| [v.x, v.y]).collect(),
            faces: l.faces.iter().map(|f| Face2D {
                vertex_indices: f.vertex_indices.clone(),
                original_face_index: f.original_face_index,
            }).collect(),
            bounds: l.bounds,
        }).collect(),
        seams: tauri_result.seams.iter().map(|s| SeamInfo {
            face_indices: s.face_indices,
            length: s.length,
        }).collect(),
        stats: UnfoldStats {
            face_count: tauri_result.stats.face_count,
            layout_count: tauri_result.stats.layout_count,
            seam_count: tauri_result.stats.seam_count,
            processing_time: tauri_result.stats.processing_time,
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