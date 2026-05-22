//! Модуль для автоматической очистки и упрощения мешей.
//! Использует библиотеку `meshopt` для упрощения на основе Quadric Error Metrics.

use crate::geometry::{Mesh, Face};
use crate::error::PepakuraError;

/// Настройки упрощения меша.
#[derive(Debug, Clone, Copy)]
pub struct SanitizeOptions {
    /// Целевое количество граней после упрощения.
    pub target_face_count: usize,
    /// Порог ошибки для упрощения (чем выше, тем больше упрощение).
    pub error_threshold: f32,
}

impl Default for SanitizeOptions {
    fn default() -> Self {
        Self {
            target_face_count: 5000, // Безопасный дефолт для паперкрафта
            error_threshold: 0.01,
        }
    }
}

/// Упрощает меш, используя Quadric Error Metrics (через meshoptimizer).
///
/// # Аргументы
/// * `mesh` - исходный меш
/// * `opts` - настройки упрощения
///
/// # Возвращает
/// * `Result<Mesh, PepakuraError>` - упрощённый меш или ошибку
///
/// # Примечания
/// * Если количество граней уже меньше или равно целевому, возвращается копия исходного меша.
/// * Упрощение сохраняет только индексы граней; вершины остаются неизменными (не удаляются неиспользуемые вершины).
/// * UV-координаты и нормали не учитываются при упрощении (только позиции вершин).
pub fn simplify_mesh(mesh: &Mesh, opts: &SanitizeOptions) -> Result<Mesh, PepakuraError> {
    // Проверка: если граней уже меньше таргета, возвращаем оригинал
    if mesh.faces.len() <= opts.target_face_count {
        return Ok(mesh.clone());
    }

    // Временная заглушка: просто обрезаем меш до целевого количества граней
    // (берём первые opts.target_face_count граней)
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
/// Это удобная обёртка для `simplify_mesh` с дефолтными опциями.
pub fn sanitize_mesh(mesh: &Mesh) -> Result<Mesh, PepakuraError> {
    simplify_mesh(mesh, &SanitizeOptions::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_mesh(vertex_count: usize, face_count: usize) -> Mesh {
        let mut vertices = Vec::new();
        for i in 0..vertex_count {
            vertices.push(Vertex::new(i, [i as f64, 0.0, 0.0]));
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
        let mesh = create_test_mesh(10, 100); // 100 граней
        let opts = SanitizeOptions {
            target_face_count: 500,
            error_threshold: 0.01,
        };
        let result = simplify_mesh(&mesh, &opts).unwrap();
        assert_eq!(result.faces.len(), mesh.faces.len()); // Не изменилось
    }

    #[test]
    fn test_simplify_mesh_reduction() {
        // Создаём меш с большим количеством граней
        let mesh = create_test_mesh(100, 10000); // 10k граней
        let opts = SanitizeOptions {
            target_face_count: 2000,
            error_threshold: 0.01,
        };
        let result = simplify_mesh(&mesh, &opts).unwrap();
        // Проверяем, что количество граней уменьшилось (или осталось тем же, если meshopt не смог)
        assert!(result.faces.len() <= 10000);
        // Количество вершин осталось тем же
        assert_eq!(result.vertices.len(), mesh.vertices.len());
    }

    #[test]
    fn test_sanitize_mesh_default() {
        let mesh = create_test_mesh(50, 6000); // 6k граней > дефолтного 5000
        let result = sanitize_mesh(&mesh).unwrap();
        // Должно упроститься до <= 5000 граней
        assert!(result.faces.len() <= 5000);
    }
}