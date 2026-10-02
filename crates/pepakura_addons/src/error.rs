//! Ошибки системы аддонов

use thiserror::Error;

/// Ошибки аддонов
#[derive(Debug, Error)]
pub enum AddonError {
    #[error("Аддон не найден: {0}")]
    NotFound(String),

    #[error("Ошибка инициализации аддона {0}: {1}")]
    InitializationError(String, String),

    #[error("Ошибка деинициализации аддона {0}: {1}")]
    DeinitializationError(String, String),

    #[error("Конфликт аддонов: {0}")]
    Conflict(String),

    #[error("Несовместимая версия аддона {0}: требуется {1}, получено {2}")]
    VersionMismatch(String, String, String),

    #[error("Ошибка загрузки аддона: {0}")]
    LoadError(String),

    #[error("Ошибка выгрузки аддона: {0}")]
    UnloadError(String),

    #[error("Аддон {0} не реализует требуемый интерфейс: {1}")]
    MissingInterface(String, String),

    #[error("Ошибка конфигурации аддона {0}: {1}")]
    ConfigError(String, String),

    #[error("Внутренняя ошибка: {0}")]
    Internal(String),
}

impl AddonError {
    /// Создать ошибку NotFound
    pub fn not_found(name: impl Into<String>) -> Self {
        Self::NotFound(name.into())
    }

    /// Создать ошибку InitializationError
    pub fn init_error(name: impl Into<String>, message: impl Into<String>) -> Self {
        Self::InitializationError(name.into(), message.into())
    }

    /// Создать ошибку Internal
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}
