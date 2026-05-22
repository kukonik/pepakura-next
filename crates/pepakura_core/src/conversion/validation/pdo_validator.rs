//! Валидатор структуры PDO

use crate::pdo_parser::PdoModel;
use crate::conversion::traits::ValidationResult;

/// Валидатор PDO структуры
pub struct PdoValidator;

impl PdoValidator {
    /// Валидирует PDO модель
    pub fn validate(pdo: &PdoModel) -> ValidationResult {
        let mut result = ValidationResult::ok();

        // Проверка header
        if pdo.header.is_empty() {
            result.add_warning("Empty PDO header");
        }

        // Проверка вершин
        if pdo.vertices.is_empty() {
            result.add_warning("No vertices in PDO model");
        }

        // Проверка граней
        if pdo.faces.is_empty() {
            result.add_warning("No faces in PDO model");
        }

        // Проверка текстур
        for (idx, texture) in pdo.textures.iter().enumerate() {
            if texture.width == 0 || texture.height == 0 {
                result.add_error(format!(
                    "Texture {} has invalid dimensions: {}x{}",
                    idx, texture.width, texture.height
                ));
            }

            // Проверка размера данных
            let expected_size = texture.width * texture.height * 4;
            if texture.data.len() != expected_size as usize {
                result.add_error(format!(
                    "Texture {} has invalid data size: expected {}, got {}",
                    idx,
                    expected_size,
                    texture.data.len()
                ));
            }
        }

        // Проверка индексов в гранях
        let vertex_count = pdo.vertices.len();
        for (idx, face) in pdo.faces.iter().enumerate() {
            if face.indices.len() < 3 {
                result.add_error(format!(
                    "Face {} has too few vertices: {}",
                    idx,
                    face.indices.len()
                ));
            }

            for &vertex_idx in &face.indices {
                if vertex_idx as usize >= vertex_count {
                    result.add_error(format!(
                        "Face {} references non-existent vertex {}",
                        idx, vertex_idx
                    ));
                }
            }
        }

        result
    }

    /// Быстрая проверка валидности
    pub fn is_valid_quick(pdo: &PdoModel) -> bool {
        !pdo.vertices.is_empty() && !pdo.faces.is_empty()
    }

    /// Проверка на наличие потенциальных проблем
    pub fn get_warnings(pdo: &PdoModel) -> Vec<String> {
        let mut warnings = Vec::new();

        if pdo.header.is_empty() {
            warnings.push("Empty PDO header".to_string());
        }

        if pdo.vertices.len() < 3 {
            warnings.push(format!(
                "Very few vertices: {} (minimum for a valid mesh is 3)",
                pdo.vertices.len()
            ));
        }

        if pdo.faces.is_empty() {
            warnings.push("No faces - model has no geometry".to_string());
        }

        warnings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdo_parser::{PdoVertex, PdoFace, PdoTexture};

    #[test]
    fn test_validate_valid_model() {
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

        let result = PdoValidator::validate(&pdo);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_face_index() {
        let pdo = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![PdoVertex { x: 0.0, y: 0.0, z: 0.0 }],
            faces: vec![PdoFace {
                indices: vec![0, 1, 2], // Индексы 1 и 2 не существуют
                material_id: 0,
            }],
            textures: vec![],
        };

        let result = PdoValidator::validate(&pdo);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_texture_dimensions() {
        let pdo = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![PdoVertex { x: 0.0, y: 0.0, z: 0.0 }],
            faces: vec![],
            textures: vec![PdoTexture {
                id: 0,
                width: 0,
                height: 0,
                data: vec![],
                name: "Test".to_string(),
            }],
        };

        let result = PdoValidator::validate(&pdo);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_is_valid_quick() {
        let valid = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![PdoVertex { x: 0.0, y: 0.0, z: 0.0 }],
            faces: vec![PdoFace {
                indices: vec![0, 0, 0],
                material_id: 0,
            }],
            textures: vec![],
        };
        assert!(PdoValidator::is_valid_quick(&valid));

        let invalid = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![],
            faces: vec![],
            textures: vec![],
        };
        assert!(!PdoValidator::is_valid_quick(&invalid));
    }

    #[test]
    fn test_get_warnings() {
        let pdo = PdoModel {
            header: "".to_string(),
            vertices: vec![],
            faces: vec![],
            textures: vec![],
        };

        let warnings = PdoValidator::get_warnings(&pdo);
        assert!(!warnings.is_empty());
        assert!(warnings.iter().any(|w| w.contains("header")));
    }
}
