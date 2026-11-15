//! Валидатор геометрии

use crate::pdo_parser::PdoModel;
use crate::conversion::traits::ValidationResult;

/// Валидатор геометрии PDO модели
pub struct GeometryValidator;

impl GeometryValidator {
    /// Валидирует геометрию PDO модели
    pub fn validate(pdo: &PdoModel) -> ValidationResult {
        let mut result = ValidationResult::ok();

        // Проверка вершин на NaN и Infinity
        for (idx, vertex) in pdo.vertices.iter().enumerate() {
            if !vertex.x.is_finite() || !vertex.y.is_finite() || !vertex.z.is_finite() {
                result.add_error(format!(
                    "Vertex {} has invalid coordinates: [{}, {}, {}]",
                    idx, vertex.x, vertex.y, vertex.z
                ));
            }
        }

        // Проверка на вырожденные грани
        for (idx, face) in pdo.faces.iter().enumerate() {
            if face.indices.len() < 3 {
                continue; // Уже проверено в PdoValidator
            }

            // Проверка на коллинеарные вершины
            if face.indices.len() == 3 {
                let v0 = &pdo.vertices[face.indices[0] as usize];
                let v1 = &pdo.vertices[face.indices[1] as usize];
                let v2 = &pdo.vertices[face.indices[2] as usize];

                if Self::is_degenerate_triangle(v0, v1, v2) {
                    result.add_warning(format!(
                        "Face {} is degenerate (collinear or coincident vertices)",
                        idx
                    ));
                }
            }
        }

        // Проверка на изолированные вершины (не используемые ни в одной грани)
        let mut used_vertices = std::collections::HashSet::new();
        for face in &pdo.faces {
            for &idx in &face.indices {
                used_vertices.insert(idx as usize);
            }
        }

        let isolated_vertices: Vec<usize> = (0..pdo.vertices.len())
            .filter(|i| !used_vertices.contains(i))
            .collect();

        if !isolated_vertices.is_empty() && isolated_vertices.len() < pdo.vertices.len() {
            result.add_warning(format!(
                "Model has {} isolated vertices (not used in any face)",
                isolated_vertices.len()
            ));
        }

        // Проверка на дублирующиеся вершины
        let duplicates = Self::find_duplicate_vertices(pdo);
        if !duplicates.is_empty() {
            result.add_warning(format!(
                "Model has {} duplicate vertices (may cause rendering issues)",
                duplicates.len()
            ));
        }

        result
    }

    /// Проверяет, является ли треугольник вырожденным
    pub fn is_degenerate_triangle(
        v0: &crate::pdo_parser::PdoVertex,
        v1: &crate::pdo_parser::PdoVertex,
        v2: &crate::pdo_parser::PdoVertex,
    ) -> bool {
        // Векторы сторон
        let edge1 = [
            v1.x - v0.x,
            v1.y - v0.y,
            v1.z - v0.z,
        ];
        let edge2 = [
            v2.x - v0.x,
            v2.y - v0.y,
            v2.z - v0.z,
        ];

        // Векторное произведение
        let cross = [
            edge1[1] * edge2[2] - edge1[2] * edge2[1],
            edge1[2] * edge2[0] - edge1[0] * edge2[2],
            edge1[0] * edge2[1] - edge1[1] * edge2[0],
        ];

        // Длина векторного произведения
        let len_squared = cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2];

        // Если длина близка к нулю, треугольник вырожденный
        len_squared < 1e-10
    }

    /// Находит дублирующиеся вершины
    fn find_duplicate_vertices(pdo: &PdoModel) -> Vec<(usize, usize)> {
        let mut duplicates = Vec::new();
        let epsilon = 1e-6;

        for i in 0..pdo.vertices.len() {
            for j in (i + 1)..pdo.vertices.len() {
                let v1 = &pdo.vertices[i];
                let v2 = &pdo.vertices[j];

                let dx = v1.x - v2.x;
                let dy = v1.y - v2.y;
                let dz = v1.z - v2.z;

                if dx.abs() < epsilon && dy.abs() < epsilon && dz.abs() < epsilon {
                    duplicates.push((i, j));
                }
            }
        }

        duplicates
    }

    /// Вычисляет площадь поверхности модели
    pub fn compute_surface_area(pdo: &PdoModel) -> f64 {
        let mut total_area = 0.0;

        for face in &pdo.faces {
            if face.indices.len() < 3 {
                continue;
            }

            // Триангуляция и вычисление площади
            let v0 = &pdo.vertices[face.indices[0] as usize];

            for i in 1..face.indices.len() - 1 {
                let v1 = &pdo.vertices[face.indices[i] as usize];
                let v2 = &pdo.vertices[face.indices[i + 1] as usize];

                let edge1 = [
                    v1.x - v0.x,
                    v1.y - v0.y,
                    v1.z - v0.z,
                ];
                let edge2 = [
                    v2.x - v0.x,
                    v2.y - v0.y,
                    v2.z - v0.z,
                ];

                let cross = [
                    edge1[1] * edge2[2] - edge1[2] * edge2[1],
                    edge1[2] * edge2[0] - edge1[0] * edge2[2],
                    edge1[0] * edge2[1] - edge1[1] * edge2[0],
                ];

                let area = (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() / 2.0;
                total_area += area;
            }
        }

        total_area as f64
    }

    /// Вычисляет объем модели (если замкнута)
    pub fn compute_volume(pdo: &PdoModel) -> f64 {
        let mut volume = 0.0;

        for face in &pdo.faces {
            if face.indices.len() < 3 {
                continue;
            }

            // Используем первую вершину как опорную
            let v0 = &pdo.vertices[face.indices[0] as usize];

            for i in 1..face.indices.len() - 1 {
                let v1 = &pdo.vertices[face.indices[i] as usize];
                let v2 = &pdo.vertices[face.indices[i + 1] as usize];

                // Смешанное произведение для объема тетраэдра
                volume += v0.x * (v1.y * v2.z - v1.z * v2.y)
                    - v0.y * (v1.x * v2.z - v1.z * v2.x)
                    + v0.z * (v1.x * v2.y - v1.y * v2.x);
            }
        }

        (volume.abs() / 6.0) as f64
    }

    /// Проверяет, является ли модель замкнутой (водонепроницаемой)
    pub fn is_watertight(pdo: &PdoModel) -> bool {
        // Подсчет количества использований каждого ребра
        let mut edge_counts = std::collections::HashMap::new();

        for face in &pdo.faces {
            if face.indices.len() < 3 {
                continue;
            }

            for i in 0..face.indices.len() {
                let j = (i + 1) % face.indices.len();
                let edge = if face.indices[i] < face.indices[j] {
                    (face.indices[i], face.indices[j])
                } else {
                    (face.indices[j], face.indices[i])
                };

                *edge_counts.entry(edge).or_insert(0) += 1;
            }
        }

        // В замкнутой модели каждое ребро используется ровно 2 раза
        edge_counts.values().all(|&count| count == 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdo_parser::{PdoVertex, PdoFace};

    #[test]
    fn test_validate_valid_triangle() {
        let pdo = PdoModel {
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

        let result = GeometryValidator::validate(&pdo);
        assert!(result.is_valid);
    }

    #[test]
    fn test_validate_nan_vertex() {
        let pdo = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![
                PdoVertex { x: f32::NAN, y: 0.0, z: 0.0 },
                PdoVertex { x: 1.0, y: 0.0, z: 0.0 },
            ],
            faces: vec![],
            textures: vec![],
        };

        let result = GeometryValidator::validate(&pdo);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("NaN")));
    }

    #[test]
    fn test_degenerate_triangle() {
        let v0 = PdoVertex { x: 0.0, y: 0.0, z: 0.0 };
        let v1 = PdoVertex { x: 1.0, y: 0.0, z: 0.0 };
        let v2 = PdoVertex { x: 2.0, y: 0.0, z: 0.0 }; // Коллинеарные точки

        assert!(GeometryValidator::is_degenerate_triangle(&v0, &v1, &v2));
    }

    #[test]
    fn test_surface_area() {
        // Простой треугольник в XY плоскости
        let pdo = PdoModel {
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

        let area = GeometryValidator::compute_surface_area(&pdo);
        assert!((area - 0.5).abs() < 0.0001); // Площадь = 0.5
    }

    #[test]
    fn test_duplicate_vertices() {
        let pdo = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![
                PdoVertex { x: 0.0, y: 0.0, z: 0.0 },
                PdoVertex { x: 0.0, y: 0.0, z: 0.0 }, // Дубликат
                PdoVertex { x: 1.0, y: 0.0, z: 0.0 },
            ],
            faces: vec![],
            textures: vec![],
        };

        let duplicates = GeometryValidator::find_duplicate_vertices(&pdo);
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0], (0, 1));
    }
}
