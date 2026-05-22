//! Модуль конвертации PDO → PepaScene
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::conversion::{convert_pdo_to_scene, ConversionConfig};
//!
//! let config = ConversionConfig::full();
//! let scene = convert_pdo_to_scene(&pdo_bytes, &config)?;
//! ```

mod traits;
mod config;
mod error;
mod progress;

mod geometry;
mod materials;
mod validation;

pub use traits::*;
pub use config::*;
pub use error::*;
pub use progress::*;

pub use geometry::{VertexConverter, FaceConverter, NormalCalculator, UvMapper};
pub use materials::{MaterialConverter, TextureExtractor, TextureAtlasRegion};
pub use validation::{
    PdoValidator,
    GeometryValidator,
    RecoveryStrategy,
    validate_pdo_model,
    try_recover_pdo,
    is_encrypted_pdo,
    ValidationResult,
};

use crate::pdo_parser::PdoModel;
use crate::pepa_scene_adapter::{PepaScene, PepaMesh, PepaBoundingBox, PepaMaterial};
use crate::conversion::geometry::{convert_vertices, convert_faces, convert_vertices_with_progress, convert_faces_with_progress};
use crate::conversion::materials::convert_materials_or_default;

/// Основная функция конвертации PDO → PepaScene
pub fn convert_pdo_to_scene(data: &[u8], config: &ConversionConfig) -> Result<PepaScene> {
    // Парсинг PDO
    let pdo_model = PdoModel::parse_from_bytes(data)?;

    // Конвертация
    convert_model_to_scene(&pdo_model, config)
}

/// Конвертирует распарсенную модель в сцену
pub fn convert_model_to_scene(
    pdo_model: &PdoModel,
    config: &ConversionConfig,
) -> Result<PepaScene> {
    // Проверка лимитов
    config
        .validate_limits(pdo_model.vertices.len(), pdo_model.faces.len())
        .map_err(|e| {
            if let ConversionError::LimitExceeded { .. } = e {
                e
            } else {
                ConversionError::InternalError("Unexpected error type".to_string())
            }
        })?;

    // Проверка порога для асинхронной обработки
    if config.needs_async_processing(pdo_model.vertices.len()) {
        // В реальной реализации здесь был бы async runtime
        // Для синхронного случая используем параллельную обработку
        convert_model_parallel(pdo_model, config)
    } else {
        convert_model_sync(pdo_model, config)
    }
}

/// Синхронная конвертация
fn convert_model_sync(
    pdo_model: &PdoModel,
    config: &ConversionConfig,
) -> Result<PepaScene> {
    // 1. Конвертация вершин
    let vertices = convert_vertices(&pdo_model.vertices, config)?;

    // 2. Конвертация граней
    let faces = convert_faces(&pdo_model.faces, vertices.len(), config)?;

    // 3. Вычисление нормалей
    let normals = if config.compute_normals {
        NormalCalculator::compute_normals(&vertices, &faces)
    } else {
        vec![]
    };

    // 4. UV маппинг
    let uvs = if config.compute_uvs {
        UvMapper::compute_uvs(pdo_model, &vertices, &faces)
    } else {
        vec![]
    };

    // 5. Материалы
    let materials = convert_materials_or_default(&pdo_model.textures);

    // 6. Bounding box
    let vertex_converter = VertexConverter::new(config.clone());
    let bounding_box = vertex_converter.compute_bounding_box(&vertices);

    // 7. Формирование PepaMesh
    let mut positions = Vec::with_capacity(vertices.len() * 3);
    let mut normal_data = Vec::with_capacity(normals.len() * 3);
    let mut uv_data = Vec::with_capacity(uvs.len() * 2);

    for vertex in &vertices {
        positions.extend_from_slice(&[
            vertex.position[0] as f32,
            vertex.position[1] as f32,
            vertex.position[2] as f32,
        ]);
    }

    for normal in &normals {
        normal_data.extend_from_slice(&[
            normal[0] as f32,
            normal[1] as f32,
            normal[2] as f32,
        ]);
    }

    for uv in &uvs {
        uv_data.extend_from_slice(&[uv[0] as f32, uv[1] as f32]);
    }

    let indices: Vec<u32> = faces
        .iter()
        .flat_map(|face| face.vertices.iter().map(|&v| v as u32))
        .collect();

    let mesh = PepaMesh {
        positions,
        indices,
        normals: normal_data,
        uvs: uv_data,
        material_id: if materials.is_empty() {
            None
        } else {
            Some(0)
        },
    };

    // 8. Формирование PepaScene
    Ok(PepaScene {
        scene_version: "1.0".to_string(),
        meshes: vec![mesh],
        materials: materials
            .into_iter()
            .map(|m| PepaMaterial {
                id: m.id,
                name: m.name,
                diffuse_color: m.diffuse_color,
                texture_id: m.texture_id,
            })
            .collect(),
        bounding_box: Some(PepaBoundingBox {
            min: [
                bounding_box.min[0] as f32,
                bounding_box.min[1] as f32,
                bounding_box.min[2] as f32,
            ],
            max: [
                bounding_box.max[0] as f32,
                bounding_box.max[1] as f32,
                bounding_box.max[2] as f32,
            ],
        }),
    })
}

/// Параллельная конвертация с использованием Rayon
fn convert_model_parallel(
    pdo_model: &PdoModel,
    config: &ConversionConfig,
) -> Result<PepaScene> {
    // Аналогично sync, но с параллельной обработкой
    // Rayon уже используется внутри convert_vertices и convert_faces
    convert_model_sync(pdo_model, config)
}

/// Конвертация с отслеживанием прогресса
pub fn convert_pdo_to_scene_with_progress(
    data: &[u8],
    config: &ConversionConfig,
    callback: impl FnMut(&SceneConversionProgress) + Send + Sync + 'static,
) -> Result<PepaScene> {
    let pdo_model = PdoModel::parse_from_bytes(data)?;
    convert_model_to_scene_with_progress(&pdo_model, config, callback)
}

/// Конвертация модели с отслеживанием прогресса
pub fn convert_model_to_scene_with_progress(
    pdo_model: &PdoModel,
    config: &ConversionConfig,
    callback: impl FnMut(&SceneConversionProgress) + Send + Sync + 'static,
) -> Result<PepaScene> {
    use std::sync::{Arc, Mutex};

    // Обёртываем callback в Arc<Mutex<...>> для безопасного разделения между замыканиями
    let callback = Arc::new(Mutex::new(Box::new(callback) as Box<dyn FnMut(&SceneConversionProgress) + Send + Sync>));

    // Этап 1: Конвертация вершин
    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::VertexConversion,
            0.0,
        ));
    }

    let vertices = {
        let callback = callback.clone();
        convert_vertices_with_progress(
            &pdo_model.vertices,
            config,
            move |progress: &VertexProgress| {
                let mut cb = callback.lock().unwrap();
                cb(&SceneConversionProgress::new(
                    ConversionStage::VertexConversion,
                    progress.percent_complete(),
                ));
            },
        )?
    };

    // Этап 2: Конвертация граней
    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::FaceConversion,
            0.0,
        ));
    }

    let faces = {
        let callback = callback.clone();
        convert_faces_with_progress(
            &pdo_model.faces,
            vertices.len(),
            config,
            move |progress: &FaceProgress| {
                let mut cb = callback.lock().unwrap();
                cb(&SceneConversionProgress::new(
                    ConversionStage::FaceConversion,
                    progress.percent_complete(),
                ));
            },
        )?
    };

    // Этап 3: Вычисление нормалей
    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::NormalCalculation,
            50.0,
        ));
    }

    let normals = if config.compute_normals {
        NormalCalculator::compute_normals(&vertices, &faces)
    } else {
        vec![]
    };

    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::NormalCalculation,
            100.0,
        ));
    }

    // Этап 4: UV маппинг
    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::UvMapping,
            0.0,
        ));
    }

    let uvs = if config.compute_uvs {
        UvMapper::compute_uvs(pdo_model, &vertices, &faces)
    } else {
        vec![]
    };

    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::UvMapping,
            100.0,
        ));
    }

    // Этап 5: Материалы
    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::MaterialConversion,
            0.0,
        ));
    }

    let materials = convert_materials_or_default(&pdo_model.textures);

    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::MaterialConversion,
            100.0,
        ));
    }

    // Этап 6: Формирование сцены
    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::Finalization,
            0.0,
        ));
    }

    // Формируем меш
    let vertex_converter = VertexConverter::new(config.clone());
    let bounding_box = vertex_converter.compute_bounding_box(&vertices);

    let mut positions = Vec::with_capacity(vertices.len() * 3);
    let mut normal_data = Vec::with_capacity(normals.len() * 3);
    let mut uv_data = Vec::with_capacity(uvs.len() * 2);

    for vertex in &vertices {
        positions.extend_from_slice(&[
            vertex.position[0] as f32,
            vertex.position[1] as f32,
            vertex.position[2] as f32,
        ]);
    }

    for normal in &normals {
        normal_data.extend_from_slice(&[
            normal[0] as f32,
            normal[1] as f32,
            normal[2] as f32,
        ]);
    }

    for uv in &uvs {
        uv_data.extend_from_slice(&[uv[0] as f32, uv[1] as f32]);
    }

    let indices: Vec<u32> = faces
        .iter()
        .flat_map(|face| face.vertices.iter().map(|&v| v as u32))
        .collect();

    let mesh = PepaMesh {
        positions,
        indices,
        normals: normal_data,
        uvs: uv_data,
        material_id: if materials.is_empty() { None } else { Some(0) },
    };

    {
        let callback = callback.clone();
        let mut cb = callback.lock().unwrap();
        cb(&SceneConversionProgress::new(
            ConversionStage::Finalization,
            100.0,
        ));
    }

    Ok(PepaScene {
        scene_version: "1.0".to_string(),
        meshes: vec![mesh],
        materials: materials
            .into_iter()
            .map(|m| PepaMaterial {
                id: m.id,
                name: m.name,
                diffuse_color: m.diffuse_color,
                texture_id: m.texture_id,
            })
            .collect(),
        bounding_box: Some(PepaBoundingBox {
            min: [
                bounding_box.min[0] as f32,
                bounding_box.min[1] as f32,
                bounding_box.min[2] as f32,
            ],
            max: [
                bounding_box.max[0] as f32,
                bounding_box.max[1] as f32,
                bounding_box.max[2] as f32,
            ],
        }),
    })
}

// Ре-экспорт для удобства
pub use config::ConversionConfig;
pub use error::ConversionError;
pub use traits::FromPdoModel;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        // Создаем тестовые PDO данные
        let test_data = create_test_pdo_bytes();

        let config = ConversionConfig::default();
        let scene = convert_pdo_to_scene(&test_data, &config).unwrap();

        assert_eq!(scene.scene_version, "1.0");
        assert!(!scene.meshes.is_empty());
        assert!(scene.bounding_box.is_some());
    }

    #[test]
    fn test_conversion_with_materials() {
        let test_data = create_test_pdo_bytes_with_materials();

        let config = ConversionConfig::full();
        let scene = convert_pdo_to_scene(&test_data, &config).unwrap();

        assert!(!scene.materials.is_empty());
    }

    fn create_test_pdo_bytes() -> Vec<u8> {
        // Минимальный валидный PDO для тестов
        // Header + vertex count + vertices + face count + faces
        let mut data = Vec::new();

        // Header (C-string)
        data.extend_from_slice(b"PDO_TEST\0");

        // Vertex count (2)
        data.extend_from_slice(&2u32.to_le_bytes());

        // Vertices
        data.extend_from_slice(&0.0f32.to_le_bytes());
        data.extend_from_slice(&0.0f32.to_le_bytes());
        data.extend_from_slice(&0.0f32.to_le_bytes());

        data.extend_from_slice(&1.0f32.to_le_bytes());
        data.extend_from_slice(&1.0f32.to_le_bytes());
        data.extend_from_slice(&1.0f32.to_le_bytes());

        // Face count (0 - нет граней для простоты)
        data.extend_from_slice(&0u32.to_le_bytes());

        // Texture count (0)
        data.extend_from_slice(&0u32.to_le_bytes());

        data
    }

    fn create_test_pdo_bytes_with_materials() -> Vec<u8> {
        let mut data = Vec::new();

        // Header
        data.extend_from_slice(b"PDO_TEST\0");

        // Vertex count (3)
        data.extend_from_slice(&3u32.to_le_bytes());

        // Vertices (triangle)
        for _ in 0..3 {
            data.extend_from_slice(&0.0f32.to_le_bytes());
            data.extend_from_slice(&0.0f32.to_le_bytes());
            data.extend_from_slice(&0.0f32.to_le_bytes());
        }

        // Face count (1)
        data.extend_from_slice(&1u32.to_le_bytes());

        // Face (triangle with 3 indices)
        data.extend_from_slice(&3u16.to_le_bytes()); // indices count
        data.extend_from_slice(&0u32.to_le_bytes());
        data.extend_from_slice(&1u32.to_le_bytes());
        data.extend_from_slice(&2u32.to_le_bytes());
        data.extend_from_slice(&0u16.to_le_bytes()); // material_id

        // Texture count (1)
        data.extend_from_slice(&1u32.to_le_bytes());

        // Texture
        data.extend_from_slice(&0u32.to_le_bytes()); // id
        data.extend_from_slice(&4u32.to_le_bytes()); // width
        data.extend_from_slice(&4u32.to_le_bytes()); // height
        data.extend_from_slice(&vec![0u8; 4 * 4 * 4][..]); // RGBA data
        data.extend_from_slice(b"TestTexture\0");

        data
    }
}
