//! Модуль валидации PDO данных

mod pdo_validator;
mod geometry_validator;
mod error_recovery;

pub use pdo_validator::PdoValidator;
pub use geometry_validator::GeometryValidator;
pub use error_recovery::RecoveryStrategy;
pub use super::traits::ValidationResult;

use crate::pdo_parser::PdoModel;
use crate::conversion::error::ConversionError;

/// Валидирует PDO модель
pub fn validate_pdo_model(pdo: &PdoModel) -> ValidationResult {
    let mut result = ValidationResult::ok();

    // Валидация структуры PDO
    let pdo_validation = PdoValidator::validate(pdo);
    result.merge(&pdo_validation);

    // Валидация геометрии
    let geometry_validation = GeometryValidator::validate(pdo);
    result.merge(&geometry_validation);

    result
}

/// Проверяет, зашифрован ли PDO файл
pub fn is_encrypted_pdo(data: &[u8]) -> bool {
    // Проверка магических байт зашифрованного PDO
    // PDOE = Pepakura Data Encrypted
    data.starts_with(b"PDOE")
}

/// Пытается восстановить поврежденную PDO модель
pub fn try_recover_pdo(data: &[u8]) -> Result<PdoModel, ConversionError> {
    let mut model = PdoModel::parse_from_bytes(data)?;

    // Применяем стратегии восстановления
    RecoveryStrategy::recover_model(&mut model);

    Ok(model)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pdo_parser::{PdoVertex, PdoFace, PdoTexture};

    #[test]
    fn test_validate_empty_model() {
        let pdo = PdoModel {
            header: "TEST".to_string(),
            vertices: vec![],
            faces: vec![],
            textures: vec![],
        };

        let result = validate_pdo_model(&pdo);
        assert!(result.is_valid);
    }

    #[test]
    fn test_is_encrypted_pdo() {
        let encrypted = b"PDOE\x00\x00\x00\x00";
        assert!(is_encrypted_pdo(encrypted));

        let normal = b"PDO_TEST\x00\x00\x00\x00";
        assert!(!is_encrypted_pdo(normal));
    }
}
