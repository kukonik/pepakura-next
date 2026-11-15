//! Модуль для автоматической очистки и упрощения мешей.
//! В нативной сборке (Tauri) использует библиотеку meshopt (Quadric Error Metrics).
//! В WASM сборке использует заглушку (транкация), так как meshopt не компилируется в WASM.

use crate::geometry::{Mesh, Face};
use crate::error::PepakuraError;

/// Настройки упрощения меша.
#[derive(Debug, Clone, Copy)]
pub struct SanitizeOptions {
    /// Целевое количество граней после упрощения.
    pub target_face_count: usize,
    /// Порог ошибки для упрощения (0.0 - максимальное качество, 1.0 - агрессивное).
    pub error_threshold: f32,
}

impl Default for SanitizeOptions {
    fn default() -> Self {
        Self {
            target_face_count: 5000,
            error_threshold: 0.01,
        }
    }
}

// ==========================================
// Нативная реализация (Tauri / Desktop) с meshopt
// ==========================================
#[cfg(all(feature = "native", not(feature = "wasm")))]
pub fn simplify_mesh(mesh: &Mesh, opts: &SanitizeOptions) -> Result<Mesh, PepakuraError> {
    if mesh.faces.len() <= opts.target_face_count {
        return Ok(mesh.clone());
    }

    // Конвертируем позиции в f32
    let vertex_positions_f32: Vec<f32> = mesh.vertices
        .iter()
        .flat_map(|v| v.position.iter().map(|&c| c as f32))
        .collect();

    // Конвертируем индексы граней в u32
    let mut indices: Vec<u32> = mesh.faces
        .iter()
        .flat_map(|f| f.vertices.iter().map(|&i| i as u32))
        .collect();

    // Подготавливаем данные для meshopt 0.3 (нужен слайс байтов)
    let vertex_bytes: Vec<u8> = vertex_positions_f32.iter()
        .flat_map(|f| f.to_ne_bytes())
        .collect();

    let vertex_data = meshopt::VertexDataAdapter::new(
        &vertex_bytes,
        std::mem::size_of::<[f32; 3]>(), // Страйд: 3 флоата = 12 байт
        0
    ).expect("Failed to create VertexDataAdapter for meshopt");

    let target_index_count = (opts.target_face_count * 3) as usize;
    let options = meshopt::SimplifyOptions::LockBorder;

    let mut result_error: f32 = 0.0;

    // Вызов нового API meshopt 0.3.0
    meshopt::simplify(
        &mut indices,
        &vertex_data,
        target_index_count,
        opts.error_threshold,
        options,
        Some(&mut result_error),
    );

    log::info!("Meshopt simplification completed with error: {}", result_error);

    // Собираем новый список граней
    let mut new_faces = Vec::with_capacity(indices.len() / 3);
    for chunk in indices.chunks(3) {
        if chunk.len() == 3 {
            new_faces.push(Face {
                vertices: [chunk[0] as usize, chunk[1] as usize, chunk[2] as usize],
                material_id: None,
            });
        }
    }

    // Очистка неиспользуемых вершин
    let mut used_vertex_indices = std::collections::HashSet::new();
    for face in &new_faces {
        used_vertex_indices.insert(face.vertices[0]);
        used_vertex_indices.insert(face.vertices[1]);
        used_vertex_indices.insert(face.vertices[2]);
    }

    let mut new_vertices = Vec::with_capacity(used_vertex_indices.len());
    let mut index_remap = vec![0usize; mesh.vertices.len()];

    for (old_idx, vertex) in mesh.vertices.iter().enumerate() {
        if used_vertex_indices.contains(&old_idx) {
            let new_idx = new_vertices.len();
            index_remap[old_idx] = new_idx;
            new_vertices.push(vertex.clone());
        }
    }

    for face in &mut new_faces {
        face.vertices[0] = index_remap[face.vertices[0]];
        face.vertices[1] = index_remap[face.vertices[1]];
        face.vertices[2] = index_remap[face.vertices[2]];
    }

    let mut new_mesh = Mesh::with_data(&mesh.name, new_vertices, new_faces);
    new_mesh.metadata = mesh.metadata.clone();

    Ok(new_mesh)
}

// ==========================================
// WASM реализация (Заглушка - транкация)
// ==========================================
#[cfg(feature = "wasm")]
pub fn simplify_mesh(mesh: &Mesh, opts: &SanitizeOptions) -> Result<Mesh, PepakuraError> {
    if mesh.faces.len() <= opts.target_face_count {
        return Ok(mesh.clone());
    }

    log::warn!("WASM build: meshopt is not available. Using fallback truncation.");

    let new_faces: Vec<Face> = mesh.faces
        .iter()
        .take(opts.target_face_count)
        .cloned()
        .collect();

    let mut new_mesh = mesh.clone();
    new_mesh.faces = new_faces;
    Ok(new_mesh)
}

/// Автоматически санитизирует меш с настройками по умолчанию.
pub fn sanitize_mesh(mesh: &Mesh) -> Result<Mesh, PepakuraError> {
    simplify_mesh(mesh, &SanitizeOptions::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_mesh(vertex_count: usize, face_count: usize) -> Mesh {
        let mut vertices = Vec::new();
        for i in 0..vertex_count {
            vertices.push(crate::geometry::Vertex { id: i, position: [i as f64, 0.0, 0.0] });
        }

        let mut faces = Vec::new();
        for i in 0..face_count {
            let v1 = (i * 3) % vertex_count;
            let v2 = (i * 3 + 1) % vertex_count;
            let v3 = (i * 3 + 2) % vertex_count;
            faces.push(Face {
                vertices: [v1, v2, v3],
                material_id: None,
            });
        }

        Mesh {
            vertices,
            faces,
            name: "Test Mesh".to_string(),
            metadata: Default::default(),
        }
    }

    #[test]
    fn test_simplify_mesh_already_small() {
        let mesh = create_test_mesh(10, 100);
        let opts = SanitizeOptions {
            target_face_count: 500,
            error_threshold: 0.01,
        };
        let result = simplify_mesh(&mesh, &opts).unwrap();
        assert_eq!(result.faces.len(), mesh.faces.len());
    }

    #[test]
    fn test_sanitize_mesh_default() {
        let mesh = create_test_mesh(50, 6000);
        let result = sanitize_mesh(&mesh).unwrap();
        assert!(result.faces.len() <= 5000);
    }
}
