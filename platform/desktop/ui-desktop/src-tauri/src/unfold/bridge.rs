//! Bridge between Tauri and the unfolding core

use pepakura_core::{Mesh, UnfoldResult};

/// Convert a Tauri mesh to a core mesh
///
/// # Arguments
/// * `tauri_mesh` - The Tauri mesh to convert
///
/// # Returns
/// * `Mesh` - The converted core mesh
pub fn tauri_mesh_to_core(tauri_mesh: TauriMesh) -> Mesh {
    use pepakura_core::geometry::{Vertex, Face, Mesh, MeshMetadata};
    
    let vertices: Vec<Vertex> = tauri_mesh.vertices.iter().enumerate().map(|(i, v)| Vertex {
        id: i,
        position: [v.x, v.y, v.z],
        normal: None,
        uv: None,
    }).collect();
    
    let faces: Vec<Face> = tauri_mesh.faces.iter().map(|f| {
        // Преобразуем Vec<usize> в [usize; 3], предполагаем треугольники
        let mut arr = [0; 3];
        for (i, &idx) in f.iter().take(3).enumerate() {
            arr[i] = idx;
        }
        Face { vertices: arr, material_id: None }
    }).collect();
    
    Mesh {
        vertices,
        faces,
        name: "Converted".to_string(),
        metadata: MeshMetadata::default(),
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
        vertices: core_mesh.vertices.iter().map(|v| Vertex3D { x: v.position[0], y: v.position[1], z: v.position[2] }).collect(),
        faces: core_mesh.faces.iter().map(|f| f.vertices.to_vec()).collect(),
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
            layout_count: 1, // фиктивное значение
            seam_count: 0,   // швы не поддерживаются
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
pub fn tauri_result_to_core(_tauri_result: &TauriUnfoldResult) -> UnfoldResult {
    // Заглушка: возвращаем пустой результат
    // Создаём минимальный UnfoldedMesh
    use pepakura_core::geometry::{Mesh, MeshMetadata};
    use pepakura_core::unfold::{UnfoldedMesh, UnfoldMetadata};
    
    let source_mesh = Mesh {
        vertices: Vec::new(),
        faces: Vec::new(),
        name: "Empty".to_string(),
        metadata: MeshMetadata::default(),
    };
    
    UnfoldedMesh {
        vertices_2d: Vec::new(),
        uv_coords: None,
        faces: Vec::new(),
        source_mesh,
        metadata: UnfoldMetadata::default(),
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