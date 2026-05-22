//! Конфигурация конвертации PDO → PepaScene

use serde::{Deserialize, Serialize};
use crate::ConversionError;

/// Конфигурация конвертации PDO → PepaScene
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionConfig {
    /// Извлекать 3D геометрию
    pub extract_3d_geometry: bool,

    /// Извлекать 2D развертки
    pub extract_2d_unfolds: bool,

    /// Генерировать клапаны автоматически (если отсутствуют)
    pub auto_generate_tabs: bool,

    /// Вычислять нормали
    pub compute_normals: bool,

    /// Вычислять UV координаты
    pub compute_uvs: bool,

    /// Создавать текстуры атласы
    pub create_texture_atlas: bool,

    /// Максимальный размер чанка для асинхронной обработки
    pub chunk_size: usize,

    /// Порог для переключения на асинхронный режим
    pub async_threshold_vertices: usize,

    /// Строгость валидации
    pub validation_mode: ValidationMode,

    /// Обработка ошибок
    pub error_handling: ErrorHandlingMode,

    /// Лимит на количество вершин (0 = без лимита)
    pub max_vertices: usize,

    /// Лимит на количество граней (0 = без лимита)
    pub max_faces: usize,
}

impl Default for ConversionConfig {
    fn default() -> Self {
        Self {
            extract_3d_geometry: true,
            extract_2d_unfolds: true,
            auto_generate_tabs: true,
            compute_normals: true,
            compute_uvs: true,
            create_texture_atlas: true,
            chunk_size: 10_000,
            async_threshold_vertices: 100_000,
            validation_mode: ValidationMode::Strict,
            error_handling: ErrorHandlingMode::Recover,
            max_vertices: 10_000_000,
            max_faces: 50_000_000,
        }
    }
}

impl ConversionConfig {
    /// Конфигурация для быстрой конвертации (только геометрия)
    pub fn fast() -> Self {
        Self {
            extract_3d_geometry: true,
            extract_2d_unfolds: false,
            auto_generate_tabs: false,
            compute_normals: true,
            compute_uvs: false,
            create_texture_atlas: false,
            chunk_size: 50_000,
            async_threshold_vertices: 500_000,
            validation_mode: ValidationMode::Lenient,
            error_handling: ErrorHandlingMode::Skip,
            max_vertices: 0,
            max_faces: 0,
            ..Default::default()
        }
    }

    /// Конфигурация для полной конвертации (все данные)
    pub fn full() -> Self {
        Self {
            extract_3d_geometry: true,
            extract_2d_unfolds: true,
            auto_generate_tabs: true,
            compute_normals: true,
            compute_uvs: true,
            create_texture_atlas: true,
            chunk_size: 5_000,
            async_threshold_vertices: 50_000,
            validation_mode: ValidationMode::Strict,
            error_handling: ErrorHandlingMode::Recover,
            ..Default::default()
        }
    }

    /// Конфигурация для отладки (строгая валидация, FailFast)
    pub fn debug() -> Self {
        Self {
            validation_mode: ValidationMode::Strict,
            error_handling: ErrorHandlingMode::FailFast,
            ..Default::default()
        }
    }

    /// Проверка необходимости асинхронной обработки
    pub fn needs_async_processing(&self, vertex_count: usize) -> bool {
        vertex_count >= self.async_threshold_vertices
    }

    /// Проверка лимитов
    pub fn validate_limits(&self, vertices: usize, faces: usize) -> Result<(), ConversionError> {
        if self.max_vertices > 0 && vertices > self.max_vertices {
            return Err(ConversionError::LimitExceeded {
                limit_name: "vertices".to_string(),
                value: vertices,
                max: self.max_vertices,
            });
        }

        if self.max_faces > 0 && faces > self.max_faces {
            return Err(ConversionError::LimitExceeded {
                limit_name: "faces".to_string(),
                value: faces,
                max: self.max_faces,
            });
        }

        Ok(())
    }
}

/// Режим валидации
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidationMode {
    /// Пропускать проверки
    None,
    /// Предупреждать о проблемах, но продолжать
    Lenient,
    /// Останавливаться при критических ошибках
    Strict,
}

/// Режим обработки ошибок
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorHandlingMode {
    /// Немедленно возвращать ошибку
    FailFast,
    /// Пытаться восстановить данные
    Recover,
    /// Пропускать проблемные элементы
    Skip,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ConversionConfig::default();
        assert!(config.extract_3d_geometry);
        assert!(config.compute_normals);
        assert_eq!(config.chunk_size, 10_000);
    }

    #[test]
    fn test_fast_config() {
        let config = ConversionConfig::fast();
        assert!(!config.extract_2d_unfolds);
        assert!(!config.compute_uvs);
        assert_eq!(config.validation_mode, ValidationMode::Lenient);
    }

    #[test]
    fn test_full_config() {
        let config = ConversionConfig::full();
        assert!(config.extract_2d_unfolds);
        assert!(config.compute_uvs);
        assert_eq!(config.validation_mode, ValidationMode::Strict);
    }

    #[test]
    fn test_async_threshold() {
        let config = ConversionConfig::default();
        assert!(!config.needs_async_processing(50_000));
        assert!(config.needs_async_processing(100_000));
        assert!(config.needs_async_processing(200_000));
    }

    #[test]
    fn test_validate_limits_success() {
        let config = ConversionConfig {
            max_vertices: 1000,
            max_faces: 5000,
            ..Default::default()
        };
        assert!(config.validate_limits(500, 2000).is_ok());
    }

    #[test]
    fn test_validate_limits_exceeded() {
        let config = ConversionConfig {
            max_vertices: 1000,
            max_faces: 5000,
            ..Default::default()
        };
        assert!(matches!(
            config.validate_limits(2000, 100),
            Err(ConversionError::LimitExceeded { .. })
        ));
    }
}
