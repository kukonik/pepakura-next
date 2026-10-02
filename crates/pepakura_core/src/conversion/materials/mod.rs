//! Модуль конвертации материалов и текстур

mod material_converter;
mod texture_extractor;

pub use material_converter::MaterialConverter;
pub use texture_extractor::{TextureExtractor, TextureAtlasRegion};

use crate::pepa_scene_adapter::PepaMaterial;
use crate::conversion::error::Result;

/// Конвертирует текстуры PDO в материалы Pepa
pub fn convert_materials(textures: &[crate::pdo_parser::PdoTexture]) -> Result<Vec<PepaMaterial>> {
    MaterialConverter::convert_materials(textures)
}

/// Конвертирует текстуры PDO в материалы Pepa, возвращая пустой вектор при ошибке
pub fn convert_materials_or_default(textures: &[crate::pdo_parser::PdoTexture]) -> Vec<PepaMaterial> {
    convert_materials(textures).unwrap_or_else(|_| vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdo_parser::PdoTexture;

    #[test]
    fn test_convert_materials_empty() {
        let materials = convert_materials(&[]).unwrap();
        assert!(materials.is_empty());
    }

    #[test]
    fn test_convert_materials_single() {
        let textures = vec![PdoTexture {
            id: 0,
            width: 256,
            height: 256,
            data: vec![],
            name: "TestTexture".to_string(),
        }];

        let materials = convert_materials(&textures).unwrap();
        assert_eq!(materials.len(), 1);
        assert_eq!(materials[0].name, "TestTexture");
        assert_eq!(materials[0].id, 0);
    }
}
