//! Конвертер вершин PDO → Pepa

use crate::pdo_parser::PdoVertex;
use crate::geometry::Vertex;
use crate::conversion::config::{ConversionConfig, ErrorHandlingMode};
use crate::conversion::error::{Result, ConversionError};
use crate::conversion::progress::ProgressTrackerBuilder;
use crate::conversion::traits::VertexProgress;
use crate::compat::*;

/// Конвертер вершин PDO → Pepa
#[derive(Debug, Clone)]
pub struct VertexConverter {
    config: ConversionConfig,
}

impl VertexConverter {
    pub fn new(config: ConversionConfig) -> Self {
        Self { config }
    }

    /// Конвертирует все вершины из PDO модели
    pub fn convert_vertices(&self, pdo_vertices: &[PdoVertex]) -> Result<Vec<Vertex>> {
        // Проверка лимитов
        self.config
            .validate_limits(pdo_vertices.len(), 0)
            .map_err(|e| {
                if let ConversionError::LimitExceeded { .. } = e {
                    e
                } else {
                    ConversionError::InternalError("Unexpected error type".to_string())
                }
            })?;

        let vertices: Result<Vec<Vertex>> = pdo_vertices
            .par_iter()
            .enumerate()
            .map(|(idx, pdo_vertex)| self.convert_single_vertex(idx, pdo_vertex))
            .collect();

        vertices
    }

    /// Конвертирует вершины с отслеживанием прогресса
    pub fn convert_vertices_with_progress(
        &self,
        pdo_vertices: &[PdoVertex],
        mut callback: impl FnMut(&VertexProgress) + Send + Sync + 'static,
    ) -> Result<Vec<Vertex>> {
        let total = pdo_vertices.len();
        let tracker = ProgressTrackerBuilder::new(total)
            .with_description("Конвертация вершин")
            .with_callback(move |percent, desc| {
                callback(&VertexProgress {
                    processed: (percent / 100.0 * total as f32) as usize,
                    total,
                    description: desc.to_string(),
                });
            })
            .build();

        let vertices: Result<Vec<Vertex>> = pdo_vertices
            .par_iter()
            .enumerate()
            .map(|(idx, pdo_vertex)| {
                let vertex = self.convert_single_vertex(idx, pdo_vertex)?;
                tracker.tracker().increment();
                tracker.maybe_report();
                Ok(vertex)
            })
            .collect();

        tracker.report_final();
        vertices
    }

    /// Конвертирует одну вершину
    pub fn convert_single_vertex(
        &self,
        index: usize,
        pdo_vertex: &PdoVertex,
    ) -> Result<Vertex> {
        // Валидация координат
        if !self.is_coordinate_valid(pdo_vertex) {
            match self.config.error_handling {
                ErrorHandlingMode::FailFast => {
                    return Err(ConversionError::InvalidVertexData {
                        index,
                        reason: "NaN or Infinity coordinates".to_string(),
                    });
                }
                ErrorHandlingMode::Recover | ErrorHandlingMode::Skip => {
                    // Заменяем на нулевую вершину
                    return Ok(Vertex::new(index, [0.0, 0.0, 0.0]));
                }
            }
        }

        Ok(Vertex::new(
            index,
            [
                pdo_vertex.x as f64,
                pdo_vertex.y as f64,
                pdo_vertex.z as f64,
            ],
        ))
    }

    /// Вычисляет bounding box для вершин
    pub fn compute_bounding_box(&self, vertices: &[Vertex]) -> crate::geometry::BoundingBox {
        if vertices.is_empty() {
            return crate::geometry::BoundingBox::empty();
        }

        let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for vertex in vertices {
            for i in 0..3 {
                min[i] = min[i].min(vertex.position[i]);
                max[i] = max[i].max(vertex.position[i]);
            }
        }

        crate::geometry::BoundingBox::new(min, max)
    }

    /// Проверка валидности координат
    fn is_coordinate_valid(&self, vertex: &PdoVertex) -> bool {
        vertex.x.is_finite() && vertex.y.is_finite() && vertex.z.is_finite()
    }

    /// Валидация всех вершин
    pub fn validate_vertices(&self, pdo_vertices: &[PdoVertex]) -> Vec<(usize, String)> {
        pdo_vertices
            .iter()
            .enumerate()
            .filter_map(|(idx, v)| {
                if !self.is_coordinate_valid(v) {
                    Some((
                        idx,
                        format!(
                            "Invalid coordinates: [{}, {}, {}]",
                            v.x, v.y, v.z
                        ),
                    ))
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_conversion() {
        let converter = VertexConverter::new(ConversionConfig::default());
        let pdo_vertices = vec![
            PdoVertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            PdoVertex {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        ];

        let vertices = converter.convert_vertices(&pdo_vertices).unwrap();

        assert_eq!(vertices.len(), 2);
        assert_eq!(vertices[0].position, [0.0, 0.0, 0.0]);
        assert_eq!(vertices[1].position, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_bounding_box_computation() {
        let converter = VertexConverter::new(ConversionConfig::default());
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 2.0, 3.0]),
        ];

        let bbox = converter.compute_bounding_box(&vertices);

        assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox.max, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_invalid_vertex_recovery() {
        let converter = VertexConverter::new(ConversionConfig {
            error_handling: ErrorHandlingMode::Recover,
            ..Default::default()
        });

        let pdo_vertices = vec![
            PdoVertex {
                x: f32::NAN,
                y: 0.0,
                z: 0.0,
            },
            PdoVertex {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
        ];

        let vertices = converter.convert_vertices(&pdo_vertices).unwrap();

        assert_eq!(vertices.len(), 2);
        assert_eq!(vertices[0].position, [0.0, 0.0, 0.0]); // Recovered
        assert_eq!(vertices[1].position, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_invalid_vertex_failfast() {
        let converter = VertexConverter::new(ConversionConfig {
            error_handling: ErrorHandlingMode::FailFast,
            ..Default::default()
        });

        let pdo_vertices = vec![PdoVertex {
            x: f32::NAN,
            y: 0.0,
            z: 0.0,
        }];

        let result = converter.convert_vertices(&pdo_vertices);
        assert!(matches!(result, Err(ConversionError::InvalidVertexData { .. })));
    }

    #[test]
    fn test_validate_vertices() {
        let converter = VertexConverter::new(ConversionConfig::default());
        let pdo_vertices = vec![
            PdoVertex {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            PdoVertex {
                x: f32::NAN,
                y: 0.0,
                z: 0.0,
            },
        ];

        let errors = converter.validate_vertices(&pdo_vertices);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].0, 1);
    }
}
