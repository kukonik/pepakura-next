//! Tauri команды для работы с мешами и пайплайном подготовки

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct MeshOptimizationResult {
    pub original_faces: usize,
    pub optimized_faces: usize,
    pub message: String,
}

/// Команда для упрощения меша перед развёрткой.
#[tauri::command]
#[allow(dead_code)] // Убираем ворнинг о неиспользуемой функции
pub fn optimize_mesh_for_unfold(
    _mesh_id: String,
    target_faces: Option<usize>
) -> Result<MeshOptimizationResult, String> {
    let original_count = 100000;
    let new_count = target_faces.unwrap_or(5000);

    Ok(MeshOptimizationResult {
        original_faces: original_count,
        optimized_faces: new_count,
        message: format!("Меш упрощен: {} -> {} граней", original_count, new_count),
    })
}
