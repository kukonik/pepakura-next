//! Стратегии восстановления поврежденных данных

use crate::pdo_parser::{PdoModel, PdoVertex};
use crate::conversion::validation::GeometryValidator;

/// Стратегии восстановления данных
pub struct RecoveryStrategy;

impl RecoveryStrategy {
    /// Восстанавливает поврежденную PDO модель
    pub fn recover_model(model: &mut PdoModel) {
        // Восстановление вершин
        Self::recover_vertices(&mut model.vertices);

        // Восстановление граней
        Self::recover_faces(&mut model.faces, model.vertices.len());

        // Удаление дубликатов вершин
        Self::remove_duplicate_vertices(model);

        // Удаление изолированных вершин
        Self::remove_isolated_vertices(model);
    }

    /// Восстанавливает поврежденные вершины
    pub fn recover_vertices(vertices: &mut Vec<PdoVertex>) {
        for vertex in vertices.iter_mut() {
            Self::recover_vertex(vertex);
        }
    }

    /// Восстанавливает одну вершину
    pub fn recover_vertex(vertex: &mut PdoVertex) {
        if !vertex.x.is_finite() {
            vertex.x = 0.0;
        }
        if !vertex.y.is_finite() {
            vertex.y = 0.0;
        }
        if !vertex.z.is_finite() {
            vertex.z = 0.0;
        }
    }

    /// Восстанавливает поврежденные грани
    pub fn recover_faces(faces: &mut Vec<crate::pdo_parser::PdoFace>, vertex_count: usize) {
        faces.retain(|face| {
            // Проверка количества вершин
            if face.indices.len() < 3 {
                return false;
            }

            // Проверка индексов
            for &idx in &face.indices {
                if idx as usize >= vertex_count {
                    return false;
                }
            }

            true
        });
    }

    /// Удаляет дублирующиеся вершины
    pub fn remove_duplicate_vertices(model: &mut PdoModel) {
        let epsilon = 1e-6;
        let mut unique_vertices: Vec<PdoVertex> = Vec::new();
        let mut index_map = std::collections::HashMap::new();

        for (new_idx, vertex) in model.vertices.iter().enumerate() {
            // Поиск существующей похожей вершины
            let mut found_idx: Option<usize> = None;

            for (existing_idx, existing_vertex) in unique_vertices.iter().enumerate() {
                let dx = vertex.x - existing_vertex.x;
                let dy = vertex.y - existing_vertex.y;
                let dz = vertex.z - existing_vertex.z;

                if dx.abs() < epsilon && dy.abs() < epsilon && dz.abs() < epsilon {
                    found_idx = Some(existing_idx);
                    break;
                }
            }

            if let Some(idx) = found_idx {
                index_map.insert(new_idx, idx as u32);
            } else {
                index_map.insert(new_idx, unique_vertices.len() as u32);
                unique_vertices.push(vertex.clone());
            }
        }

        // Обновление индексов в гранях
        for face in &mut model.faces {
            for idx in &mut face.indices {
                if let Some(&new_idx) = index_map.get(&(*idx as usize)) {
                    *idx = new_idx;
                }
            }
        }

        model.vertices = unique_vertices;
    }

    /// Удаляет изолированные вершины (не используемые в гранях)
    pub fn remove_isolated_vertices(model: &mut PdoModel) {
        // Сбор используемых вершин
        let mut used_vertices = std::collections::HashSet::new();
        for face in &model.faces {
            for &idx in &face.indices {
                used_vertices.insert(idx as usize);
            }
        }

        // Если все вершины используются, ничего не делаем
        if used_vertices.len() == model.vertices.len() {
            return;
        }

        // Создание маппинга старых индексов в новые
        let mut index_map = Vec::with_capacity(model.vertices.len());
        let mut new_vertices = Vec::new();
        let mut new_index = 0u32;

        for (old_idx, vertex) in model.vertices.iter().enumerate() {
            if used_vertices.contains(&old_idx) {
                index_map.push(new_index);
                new_vertices.push(vertex.clone());
                new_index += 1;
            } else {
                index_map.push(u32::MAX); // Пометка на удаление
            }
        }

        // Обновление индексов в гранях
        for face in &mut model.faces {
            for idx in &mut face.indices {
                *idx = index_map[*idx as usize];
            }
        }

        model.vertices = new_vertices;
    }

    /// Масштабирует модель к единичному размеру
    pub fn scale_to_unit_size(model: &mut PdoModel) {
        if model.vertices.is_empty() {
            return;
        }

        // Вычисление bounding box
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        for vertex in &model.vertices {
            min[0] = min[0].min(vertex.x);
            min[1] = min[1].min(vertex.y);
            min[2] = min[2].min(vertex.z);
            max[0] = max[0].max(vertex.x);
            max[1] = max[1].max(vertex.y);
            max[2] = max[2].max(vertex.z);
        }

        // Вычисление центра и размера
        let center = [
            (min[0] + max[0]) / 2.0,
            (min[1] + max[1]) / 2.0,
            (min[2] + max[2]) / 2.0,
        ];

        let size = [
            (max[0] - min[0]).max(1e-6),
            (max[1] - min[1]).max(1e-6),
            (max[2] - min[2]).max(1e-6),
        ];

        let max_size = size[0].max(size[1]).max(size[2]);
        let scale = 1.0 / max_size;

        // Применение трансформации
        for vertex in &mut model.vertices {
            vertex.x = (vertex.x - center[0]) * scale;
            vertex.y = (vertex.y - center[1]) * scale;
            vertex.z = (vertex.z - center[2]) * scale;
        }
    }

    /// Центрирует модель в начале координат
    pub fn center_model(model: &mut PdoModel) {
        if model.vertices.is_empty() {
            return;
        }

        // Вычисление центроида
        let mut sum = [0.0f32; 3];
        for vertex in &model.vertices {
            sum[0] += vertex.x;
            sum[1] += vertex.y;
            sum[2] += vertex.z;
        }

        let count = model.vertices.len() as f32;
        let centroid = [sum[0] / count, sum[1] / count, sum[2] / count];

        // Сдвиг к началу координат
        for vertex in &mut model.vertices {
            vertex.x -= centroid[0];
            vertex.y -= centroid[1];
            vertex.z -= centroid[2];
        }
    }

    /// Инвертирует нормали (меняет порядок вершин в гранях)
    pub fn flip_normals(model: &mut PdoModel) {
        for face in &mut model.faces {
            if face.indices.len() >= 3 {
                // Меняем местами первые две вершины
                face.indices.swap(0, 1);
            }
        }
    }

    /// Упрощает модель (удаление вырожденных граней)
    pub fn simplify_model(model: &mut PdoModel) {
        let original_face_count = model.faces.len();

        model.faces.retain(|face| {
            if face.indices.len() < 3 {
                return false;
            }

            // Проверка на вырожденность
            if face.indices.len() == 3 {
                let v0 = &model.vertices[face.indices[0] as usize];
                let v1 = &model.vertices[face.indices[1] as usize];
                let v2 = &model.vertices[face.indices[2] as usize];

                if GeometryValidator::is_degenerate_triangle(v0, v1, v2) {
                    return false;
                }
            }

            true
        });

        let removed = original_face_count - model.faces.len();
        if removed > 0 {
            log::info!("Removed {} degenerate faces", removed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdo_parser::{PdoFace, PdoVertex};

    #[test]
    fn test_recover_nan_vertex() {
        let mut vertex = PdoVertex {
            x: f32::NAN,
            y: f32::INFINITY,
            z: 5.0,
        };

        RecoveryStrategy::recover_vertex(&mut vertex);

        assert!(vertex.x.is_finite());
        assert!(vertex.y.is_finite());
        assert_eq!(vertex.z, 5.0);
        assert_eq!(vertex.x, 0.0);
        assert_eq!(vertex.y, 0.0);
    }

    #[test]
    fn test_recover_faces_invalid_indices() {
        let mut faces = vec![
            PdoFace {
                indices: vec![0, 1, 2],
                material_id: 0,
            },
            PdoFace {
                indices: vec![0, 5, 10], // Несуществующие индексы
                material_id: 0,
            },
        ];

        RecoveryStrategy::recover_faces(&mut faces, 3);

        assert_eq!(faces.len(), 1);
    }

    #[test]
    fn test_remove_duplicate_vertices() {
        let mut model = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![
                PdoVertex { x: 0.0, y: 0.0, z: 0.0 },
                PdoVertex { x: 0.0, y: 0.0, z: 0.0 }, // Дубликат
                PdoVertex { x: 1.0, y: 0.0, z: 0.0 },
            ],
            faces: vec![
                PdoFace {
                    indices: vec![0, 1, 2],
                    material_id: 0,
                },
            ],
            textures: vec![],
        };

        RecoveryStrategy::remove_duplicate_vertices(&mut model);

        assert_eq!(model.vertices.len(), 2);
        assert_eq!(model.faces[0].indices, vec![0, 0, 1]);
    }

    #[test]
    fn test_scale_to_unit_size() {
        let mut model = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![
                PdoVertex { x: 0.0, y: 0.0, z: 0.0 },
                PdoVertex { x: 10.0, y: 10.0, z: 10.0 },
            ],
            faces: vec![],
            textures: vec![],
        };

        RecoveryStrategy::scale_to_unit_size(&mut model);

        // После масштабирования размер должен быть около 1
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];

        for vertex in &model.vertices {
            min[0] = min[0].min(vertex.x);
            max[0] = max[0].max(vertex.x);
        }

        let size = max[0] - min[0];
        assert!((size - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_flip_normals() {
        let mut model = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![
                PdoVertex { x: 0.0, y: 0.0, z: 0.0 },
                PdoVertex { x: 1.0, y: 0.0, z: 0.0 },
                PdoVertex { x: 0.0, y: 1.0, z: 0.0 },
            ],
            faces: vec![PdoFace {
                indices: vec![0, 1, 2],
                material_id: 0,
            }],
            textures: vec![],
        };

        RecoveryStrategy::flip_normals(&mut model);

        assert_eq!(model.faces[0].indices, vec![1, 0, 2]);
    }
}
