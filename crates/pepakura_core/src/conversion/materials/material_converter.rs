//! Конвертер материалов PDO → Pepa

use crate::pdo_parser::PdoTexture;
use crate::pepa_scene_adapter::PepaMaterial;
use crate::conversion::error::Result;

/// Конвертер материалов PDO → Pepa
pub struct MaterialConverter;

impl MaterialConverter {
    /// Конвертирует текстуры PDO в материалы Pepa
    pub fn convert_materials(textures: &[PdoTexture]) -> Result<Vec<PepaMaterial>> {
        let materials = textures
            .iter()
            .map(|texture| {
                PepaMaterial {
                    id: texture.id,
                    name: if texture.name.is_empty() {
                        format!("Material_{}", texture.id)
                    } else {
                        texture.name.clone()
                    },
                    diffuse_color: [1.0, 1.0, 1.0, 1.0], // Default white
                    texture_id: Some(texture.id),
                }
            })
            .collect();

        Ok(materials)
    }

    /// Создает материал по умолчанию (если текстур нет)
    pub fn create_default_material() -> PepaMaterial {
        PepaMaterial {
            id: 0,
            name: "Default".to_string(),
            diffuse_color: [0.8, 0.8, 0.8, 1.0],
            texture_id: None,
        }
    }

    /// Конвертирует текстуры с материалами по умолчанию (если список пуст)
    pub fn convert_materials_or_default(textures: &[PdoTexture]) -> Vec<PepaMaterial> {
        if textures.is_empty() {
            vec![Self::create_default_material()]
        } else {
            textures
                .iter()
                .map(|texture| PepaMaterial {
                    id: texture.id,
                    name: if texture.name.is_empty() {
                        format!("Material_{}", texture.id)
                    } else {
                        texture.name.clone()
                    },
                    diffuse_color: [1.0, 1.0, 1.0, 1.0],
                    texture_id: Some(texture.id),
                })
                .collect()
        }
    }

    /// Извлекает уникальный ID для нового материала
    pub fn next_material_id(materials: &[PepaMaterial]) -> u32 {
        materials.iter().map(|m| m.id).max().unwrap_or(0) + 1
    }

    /// Находит материал по ID
    pub fn find_material_by_id(
        materials: &[PepaMaterial],
        id: u32,
    ) -> Option<&PepaMaterial> {
        materials.iter().find(|m| m.id == id)
    }

    /// Находит материал по имени
    pub fn find_material_by_name<'a>(
        materials: &'a [PepaMaterial],
        name: &'a str,
    ) -> Option<&'a PepaMaterial> {
        materials.iter().find(|m| m.name == name)
    }

    /// Проверяет, есть ли материал с текстурой
    pub fn has_textured_materials(materials: &[PepaMaterial]) -> bool {
        materials.iter().any(|m| m.texture_id.is_some())
    }

    /// Возвращает только текстурированные материалы
    pub fn get_textured_materials(materials: &[PepaMaterial]) -> Vec<&PepaMaterial> {
        materials
            .iter()
            .filter(|m| m.texture_id.is_some())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_materials() {
        let textures = vec![
            PdoTexture {
                id: 0,
                width: 256,
                height: 256,
                data: vec![],
                name: "Texture1".to_string(),
            },
            PdoTexture {
                id: 1,
                width: 512,
                height: 512,
                data: vec![],
                name: "".to_string(), // Empty name
            },
        ];

        let materials = MaterialConverter::convert_materials(&textures).unwrap();

        assert_eq!(materials.len(), 2);
        assert_eq!(materials[0].name, "Texture1");
        assert_eq!(materials[1].name, "Material_1");
    }

    #[test]
    fn test_default_material() {
        let material = MaterialConverter::create_default_material();
        assert_eq!(material.id, 0);
        assert_eq!(material.name, "Default");
        assert_eq!(material.diffuse_color, [0.8, 0.8, 0.8, 1.0]);
        assert!(material.texture_id.is_none());
    }

    #[test]
    fn test_convert_materials_or_default_empty() {
        let textures: Vec<PdoTexture> = vec![];
        let materials = MaterialConverter::convert_materials_or_default(&textures);

        assert_eq!(materials.len(), 1);
        assert_eq!(materials[0].name, "Default");
    }

    #[test]
    fn test_convert_materials_or_default_with_data() {
        let textures = vec![PdoTexture {
            id: 0,
            width: 256,
            height: 256,
            data: vec![],
            name: "Test".to_string(),
        }];

        let materials = MaterialConverter::convert_materials_or_default(&textures);
        assert_eq!(materials.len(), 1);
        assert_eq!(materials[0].name, "Test");
    }

    #[test]
    fn test_next_material_id() {
        let materials = vec![
            PepaMaterial {
                id: 0,
                name: "M1".to_string(),
                diffuse_color: [1.0; 4],
                texture_id: None,
            },
            PepaMaterial {
                id: 5,
                name: "M2".to_string(),
                diffuse_color: [1.0; 4],
                texture_id: None,
            },
        ];

        assert_eq!(MaterialConverter::next_material_id(&materials), 6);
    }

    #[test]
    fn test_find_material_by_id() {
        let materials = vec![PepaMaterial {
            id: 42,
            name: "Test".to_string(),
            diffuse_color: [1.0; 4],
            texture_id: None,
        }];

        let found = MaterialConverter::find_material_by_id(&materials, 42);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");

        let not_found = MaterialConverter::find_material_by_id(&materials, 99);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_has_textured_materials() {
        let materials_no_texture = vec![PepaMaterial {
            id: 0,
            name: "Test".to_string(),
            diffuse_color: [1.0; 4],
            texture_id: None,
        }];
        assert!(!MaterialConverter::has_textured_materials(&materials_no_texture));

        let materials_with_texture = vec![PepaMaterial {
            id: 0,
            name: "Test".to_string(),
            diffuse_color: [1.0; 4],
            texture_id: Some(0),
        }];
        assert!(MaterialConverter::has_textured_materials(&materials_with_texture));
    }
}
