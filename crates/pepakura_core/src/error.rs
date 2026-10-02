//! Типы ошибок pepakura_core.

use thiserror::Error;

/// Основная ошибка pepakura_core.
#[derive(Debug, Error)]
pub enum PepakuraError {
    /// Ошибка ввода/вывода
    #[error("Ошибка ввода/вывода: {0}")]
    IoError(#[from] std::io::Error),

    /// Ошибка парсинга
    #[error("Ошибка парсинга: {0}")]
    ParseError(String),

    /// Пустой меш
    #[error("Меш пуст")]
    EmptyMesh,

    /// Некорректная геометрия
    #[error("Некорректная геометрия: {0}")]
    InvalidGeometry(String),

    /// Неподдерживаемый формат
    #[error("Неподдерживаемый формат: {0}")]
    UnsupportedFormat(String),

    /// Ошибка развёртки
    #[error("Ошибка развёртки: {0}")]
    UnfoldError(#[from] crate::unfold::UnfoldError),

    /// Ошибка экспорта
    #[error("Ошибка экспорта: {0}")]
    ExportError(#[from] crate::export::ExportError),

    /// Ошибка геометрии
    #[error("Ошибка геометрии: {0}")]
    GeometryError(#[from] crate::geometry::MeshError),

    /// Ошибка конвертации PDO → PepaScene
    #[error("Ошибка конвертации: {0}")]
    ConversionError(#[from] crate::conversion::ConversionError),

    /// Плагин не найден
    #[error("Плагин не найден: {0}")]
    PluginNotFound(String),

    /// Ошибка валидации
    #[error("Ошибка валидации: {0}")]
    ValidationError(String),

    /// AI ошибка
    #[error("AI ошибка: {0}")]
    AiError(String),
}

/// Результат операций pepakura_core.
pub type Result<T> = std::result::Result<T, PepakuraError>;

// Ре-экспорт специфичных ошибок для удобства
pub use crate::conversion::ConversionError;
pub use crate::geometry::MeshError;
pub use crate::unfold::UnfoldError;
pub use crate::export::ExportError;
