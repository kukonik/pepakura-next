//! Модуль конвертации геометрии PDO → Pepa

mod vertex_converter;
mod face_converter;
mod normal_calculator;
mod uv_mapper;

pub use vertex_converter::VertexConverter;
pub use face_converter::FaceConverter;
pub use normal_calculator::NormalCalculator;
pub use uv_mapper::UvMapper;

use crate::pdo_parser::{PdoVertex, PdoFace};
use crate::geometry::{Vertex, Face};
use crate::conversion::config::ConversionConfig;
use crate::conversion::error::Result;
use crate::conversion::VertexProgress;
use crate::conversion::FaceProgress;

/// Конвертирует вершины PDO в формат Pepa
pub fn convert_vertices(
    pdo_vertices: &[PdoVertex],
    config: &ConversionConfig,
) -> Result<Vec<Vertex>> {
    let converter = VertexConverter::new(config.clone());
    converter.convert_vertices(pdo_vertices)
}

/// Конвертирует грани PDO в формат Pepa с триангуляцией
pub fn convert_faces(
    pdo_faces: &[PdoFace],
    vertex_count: usize,
    config: &ConversionConfig,
) -> Result<Vec<Face>> {
    let converter = FaceConverter::new(config.clone());
    converter.convert_faces(pdo_faces, vertex_count)
}

/// Конвертация с отслеживанием прогресса
pub fn convert_vertices_with_progress(
    pdo_vertices: &[PdoVertex],
    config: &ConversionConfig,
    callback: impl FnMut(&VertexProgress) + Send + Sync + 'static,
) -> Result<Vec<Vertex>> {
    let converter = VertexConverter::new(config.clone());
    converter.convert_vertices_with_progress(pdo_vertices, callback)
}

/// Конвертация граней с отслеживанием прогресса
pub fn convert_faces_with_progress(
    pdo_faces: &[PdoFace],
    vertex_count: usize,
    config: &ConversionConfig,
    callback: impl FnMut(&FaceProgress) + Send + Sync + 'static,
) -> Result<Vec<Face>> {
    let converter = FaceConverter::new(config.clone());
    converter.convert_faces_with_progress(pdo_faces, vertex_count, callback)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_vertices_basic() {
        let pdo_vertices = vec![
            PdoVertex { x: 0.0, y: 0.0, z: 0.0 },
            PdoVertex { x: 1.0, y: 2.0, z: 3.0 },
        ];
        let config = ConversionConfig::default();

        let vertices = convert_vertices(&pdo_vertices, &config).unwrap();

        assert_eq!(vertices.len(), 2);
        assert_eq!(vertices[0].position, [0.0, 0.0, 0.0]);
        assert_eq!(vertices[1].position, [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_convert_faces_triangle() {
        let pdo_faces = vec![PdoFace {
            indices: vec![0, 1, 2],
            material_id: 0,
        }];
        let config = ConversionConfig::default();

        let faces = convert_faces(&pdo_faces, 3, &config).unwrap();

        assert_eq!(faces.len(), 1);
        assert_eq!(faces[0].vertices, [0, 1, 2]);
    }

    #[test]
    fn test_convert_faces_quad_triangulation() {
        let pdo_faces = vec![PdoFace {
            indices: vec![0, 1, 2, 3],
            material_id: 1,
        }];
        let config = ConversionConfig::default();

        let faces = convert_faces(&pdo_faces, 4, &config).unwrap();

        assert_eq!(faces.len(), 2);
        assert_eq!(faces[0].material_id, Some(1));
    }
}
