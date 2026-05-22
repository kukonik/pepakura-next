use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaMaterial {
    pub id: u32,
    pub name: String,
    pub diffuse_color: [f32; 4],
    pub texture_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaMesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub material_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaBoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaScene {
    /// Версия сцены
    pub scene_version: String,
    pub meshes: Vec<PepaMesh>,
    pub materials: Vec<PepaMaterial>,
    pub bounding_box: Option<PepaBoundingBox>,
}

use crate::pdo_parser::PdoModel;
use crate::conversion::{convert_model_to_scene, ConversionConfig};

/// Trait для конвертации PDO → PepaScene
/// 
/// Устаревший метод, используйте `convert_model_to_scene` из модуля `conversion`
pub trait FromPdoModel {
    fn from_pdo_model(pdo: &PdoModel) -> Self;
}

/// Реализация через новый конвертер с config по умолчанию
impl FromPdoModel for PepaScene {
    fn from_pdo_model(pdo: &PdoModel) -> Self {
        // Используем новый конвертер с config по умолчанию
        match convert_model_to_scene(pdo, &ConversionConfig::default()) {
            Ok(scene) => scene,
            Err(_) => {
                // Fallback на минимальную сцену при ошибке
                PepaScene {
                    scene_version: "1.0".to_string(),
                    meshes: vec![],
                    materials: vec![],
                    bounding_box: None,
                }
            }
        }
    }
}

impl PepaScene {
    /// Создает пустую сцену
    pub fn empty() -> Self {
        Self {
            scene_version: "1.0".to_string(),
            meshes: vec![],
            materials: vec![],
            bounding_box: None,
        }
    }

    /// Создает сцену из PDO модели с заданной конфигурацией
    pub fn from_pdo_model_with_config(pdo: &PdoModel, config: &ConversionConfig) -> Result<Self, String> {
        convert_model_to_scene(pdo, config)
            .map_err(|e| e.to_string())
    }
}
