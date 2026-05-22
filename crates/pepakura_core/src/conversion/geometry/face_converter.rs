//! Конвертер граней PDO → Pepa

use crate::pdo_parser::PdoFace;
use crate::geometry::Face;
use crate::conversion::config::{ConversionConfig, ErrorHandlingMode};
use crate::conversion::error::{Result, ConversionError};
use crate::conversion::progress::ProgressTrackerBuilder;
use crate::conversion::traits::FaceProgress;
use crate::compat::*;

/// Конвертер граней PDO → Pepa
#[derive(Debug, Clone)]
pub struct FaceConverter {
    config: ConversionConfig,
}

impl FaceConverter {
    pub fn new(config: ConversionConfig) -> Self {
        Self { config }
    }

    /// Конвертирует все грани с триангуляцией
    pub fn convert_faces(&self, pdo_faces: &[PdoFace], vertex_count: usize) -> Result<Vec<Face>> {
        // Проверка лимитов
        self.config
            .validate_limits(0, pdo_faces.len() * 2) // Максимум 2 треугольника на грань
            .map_err(|e| {
                if let ConversionError::LimitExceeded { .. } = e {
                    e
                } else {
                    ConversionError::InternalError("Unexpected error type".to_string())
                }
            })?;

        let results: Vec<Result<Vec<Face>>> = pdo_faces
            .par_iter()
            .enumerate()
            .map(|(idx, face)| self.triangulate_face(face, vertex_count, idx))
            .collect();

        // Объединяем результаты
        let mut all_faces = Vec::new();
        for result in results {
            all_faces.extend(result?);
        }

        Ok(all_faces)
    }

    /// Конвертирует грани с отслеживанием прогресса
    pub fn convert_faces_with_progress(
        &self,
        pdo_faces: &[PdoFace],
        vertex_count: usize,
        mut callback: impl FnMut(&FaceProgress) + Send + Sync + 'static,
    ) -> Result<Vec<Face>> {
        let total = pdo_faces.len();
        let tracker = ProgressTrackerBuilder::new(total)
            .with_description("Конвертация граней")
            .with_callback(move |percent, desc| {
                callback(&FaceProgress {
                    processed: (percent / 100.0 * total as f32) as usize,
                    total,
                    triangulated: 0,
                    description: desc.to_string(),
                });
            })
            .build();

        let results: Vec<Result<Vec<Face>>> = pdo_faces
            .par_iter()
            .enumerate()
            .map(|(idx, face)| {
                let triangulated = self.triangulate_face(face, vertex_count, idx)?;
                tracker.tracker().increment();
                tracker.maybe_report();
                Ok(triangulated)
            })
            .collect();

        tracker.report_final();

        // Объединяем результаты
        let mut all_faces = Vec::new();
        for result in results {
            let faces = result?;
            all_faces.extend(faces);
        }

        Ok(all_faces)
    }

    /// Триангуляция полигона (fan triangulation)
    pub fn triangulate_face(
        &self,
        pdo_face: &PdoFace,
        vertex_count: usize,
        face_index: usize,
    ) -> Result<Vec<Face>> {
        if pdo_face.indices.len() < 3 {
            return match self.config.error_handling {
                ErrorHandlingMode::FailFast => {
                    Err(ConversionError::InvalidFaceData {
                        index: face_index,
                        reason: format!(
                            "Face has {} vertices, minimum is 3",
                            pdo_face.indices.len()
                        ),
                    })
                }
                ErrorHandlingMode::Recover | ErrorHandlingMode::Skip => {
                    Ok(vec![]) // Пропускаем грань
                }
            };
        }

        // Валидация индексов
        for &idx in &pdo_face.indices {
            if idx as usize >= vertex_count {
                return match self.config.error_handling {
                    ErrorHandlingMode::FailFast => {
                        Err(ConversionError::InvalidVertexIndex {
                            face_index,
                            vertex_index: idx as usize,
                            max_valid: vertex_count - 1,
                        })
                    }
                    ErrorHandlingMode::Recover | ErrorHandlingMode::Skip => {
                        Ok(vec![]) // Пропускаем грань
                    }
                };
            }
        }

        // Fan triangulation
        let mut triangulated = Vec::new();
        let first_idx = pdo_face.indices[0] as usize;

        for i in 1..pdo_face.indices.len() - 1 {
            let v1 = pdo_face.indices[i] as usize;
            let v2 = pdo_face.indices[i + 1] as usize;

            let face = Face::with_material(first_idx, v1, v2, pdo_face.material_id as usize);

            triangulated.push(face);
        }

        Ok(triangulated)
    }

    /// Валидация всех граней
    pub fn validate_faces(&self, pdo_faces: &[PdoFace], vertex_count: usize) -> Vec<(usize, String)> {
        pdo_faces
            .iter()
            .enumerate()
            .filter_map(|(idx, face)| {
                // Проверка количества вершин
                if face.indices.len() < 3 {
                    return Some((
                        idx,
                        format!("Too few vertices: {} (minimum 3)", face.indices.len()),
                    ));
                }

                // Проверка индексов
                for &vtx_idx in &face.indices {
                    if vtx_idx as usize >= vertex_count {
                        return Some((
                            idx,
                            format!(
                                "Invalid vertex index {} (max valid: {})",
                                vtx_idx,
                                vertex_count - 1
                            ),
                        ));
                    }
                }

                None
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_face() {
        let converter = FaceConverter::new(ConversionConfig::default());
        let pdo_faces = vec![PdoFace {
            indices: vec![0, 1, 2],
            material_id: 0,
        }];

        let faces = converter.convert_faces(&pdo_faces, 3).unwrap();

        assert_eq!(faces.len(), 1);
        assert_eq!(faces[0].vertices, [0, 1, 2]);
    }

    #[test]
    fn test_quad_triangulation() {
        let converter = FaceConverter::new(ConversionConfig::default());
        let pdo_faces = vec![PdoFace {
            indices: vec![0, 1, 2, 3], // Quad
            material_id: 1,
        }];

        let faces = converter.convert_faces(&pdo_faces, 4).unwrap();

        assert_eq!(faces.len(), 2); // 2 треугольника
        assert_eq!(faces[0].vertices, [0, 1, 2]);
        assert_eq!(faces[1].vertices, [0, 2, 3]);
        assert_eq!(faces[0].material_id, Some(1));
    }

    #[test]
    fn test_pentagon_triangulation() {
        let converter = FaceConverter::new(ConversionConfig::default());
        let pdo_faces = vec![PdoFace {
            indices: vec![0, 1, 2, 3, 4], // Pentagon
            material_id: 0,
        }];

        let faces = converter.convert_faces(&pdo_faces, 5).unwrap();

        assert_eq!(faces.len(), 3); // n-2 треугольника для n-угольника
        assert_eq!(faces[0].vertices, [0, 1, 2]);
        assert_eq!(faces[1].vertices, [0, 2, 3]);
        assert_eq!(faces[2].vertices, [0, 3, 4]);
    }

    #[test]
    fn test_invalid_face_too_few_vertices() {
        let converter = FaceConverter::new(ConversionConfig {
            error_handling: ErrorHandlingMode::FailFast,
            ..Default::default()
        });
        let pdo_faces = vec![PdoFace {
            indices: vec![0, 1], // Только 2 вершины
            material_id: 0,
        }];

        let result = converter.convert_faces(&pdo_faces, 3);
        assert!(matches!(result, Err(ConversionError::InvalidFaceData { .. })));
    }

    #[test]
    fn test_invalid_vertex_index() {
        let converter = FaceConverter::new(ConversionConfig {
            error_handling: ErrorHandlingMode::FailFast,
            ..Default::default()
        });
        let pdo_faces = vec![PdoFace {
            indices: vec![0, 1, 10], // Индекс 10 не существует
            material_id: 0,
        }];

        let result = converter.convert_faces(&pdo_faces, 3);
        assert!(matches!(
            result,
            Err(ConversionError::InvalidVertexIndex { .. })
        ));
    }

    #[test]
    fn test_skip_invalid_face() {
        let converter = FaceConverter::new(ConversionConfig {
            error_handling: ErrorHandlingMode::Skip,
            ..Default::default()
        });
        let pdo_faces = vec![
            PdoFace {
                indices: vec![0, 1], // Invalid
                material_id: 0,
            },
            PdoFace {
                indices: vec![0, 1, 2], // Valid
                material_id: 0,
            },
        ];

        let faces = converter.convert_faces(&pdo_faces, 3).unwrap();
        assert_eq!(faces.len(), 1); // Только валидная грань
    }

    #[test]
    fn test_validate_faces() {
        let converter = FaceConverter::new(ConversionConfig::default());
        let pdo_faces = vec![
            PdoFace {
                indices: vec![0, 1, 2],
                material_id: 0,
            },
            PdoFace {
                indices: vec![0, 1], // Invalid
                material_id: 0,
            },
            PdoFace {
                indices: vec![0, 1, 10], // Invalid index
                material_id: 0,
            },
        ];

        let errors = converter.validate_faces(&pdo_faces, 3);
        assert_eq!(errors.len(), 2);
    }
}
