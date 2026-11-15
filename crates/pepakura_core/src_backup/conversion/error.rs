//! Ошибки конвертации PDO → PepaScene

use thiserror::Error;

/// Ошибки конвертации PDO → PepaScene
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("Ошибка парсинга PDO: {0}")]
    ParseError(#[from] crate::pdo_parser::PdoParseError),

    #[error("Некорректные данные вершины #{index}: {reason}")]
    InvalidVertexData {
        index: usize,
        reason: String,
    },

    #[error("Некорректные данные грани #{index}: {reason}")]
    InvalidFaceData {
        index: usize,
        reason: String,
    },

    #[error("Некорректный индекс вершины {vertex_index} в грани #{face_index} (max: {max_valid})")]
    InvalidVertexIndex {
        face_index: usize,
        vertex_index: usize,
        max_valid: usize,
    },

    #[error("Поврежденные данные текстуры #{id}: {reason}")]
    CorruptedTexture {
        id: u32,
        reason: String,
    },

    #[error("Ошибка валидации: {0}")]
    ValidationError(String),

    #[error("Ошибка выделения памяти: {0}")]
    MemoryError(String),

    #[error("Превышен лимит: {limit_name} = {value} (max: {max})")]
    LimitExceeded {
        limit_name: String,
        value: usize,
        max: usize,
    },

    #[error("Неподдерживаемая версия PDO: {version}")]
    UnsupportedPdoVersion {
        version: String,
    },

    #[error("Зашифрованный PDO файл (требуется дешифровка)")]
    EncryptedPdo,

    #[error("Внутренняя ошибка: {0}")]
    InternalError(String),

    #[error("IO ошибка: {0}")]
    IoError(#[from] std::io::Error),
}

/// Результат конвертации
pub type Result<T> = std::result::Result<T, ConversionError>;

/// Расширенная информация об ошибке с контекстом
#[derive(Debug)]
pub struct ErrorContext {
    pub error: ConversionError,
    pub file_path: Option<String>,
    pub byte_offset: Option<u64>,
    pub recovery_applied: bool,
    pub suggestions: Vec<String>,
}

impl ErrorContext {
    pub fn new(error: ConversionError) -> Self {
        Self {
            error,
            file_path: None,
            byte_offset: None,
            recovery_applied: false,
            suggestions: vec![],
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    pub fn with_offset(mut self, offset: u64) -> Self {
        self.byte_offset = Some(offset);
        self
    }

    pub fn with_recovery(mut self, applied: bool) -> Self {
        self.recovery_applied = applied;
        self
    }

    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}

impl std::fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)?;
        if let Some(path) = &self.file_path {
            write!(f, " (файл: {})", path)?;
        }
        if let Some(offset) = &self.byte_offset {
            write!(f, " (offset: 0x{:x})", offset)?;
        }
        if self.recovery_applied {
            write!(f, " [применено восстановление]")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = ConversionError::InvalidVertexData {
            index: 42,
            reason: "NaN coordinates".to_string(),
        };
        let ctx = ErrorContext::new(error)
            .with_path("test.pdo")
            .with_offset(0x1000);

        let msg = format!("{}", ctx);
        assert!(msg.contains("Некорректные данные вершины #42"));
        assert!(msg.contains("test.pdo"));
        assert!(msg.contains("0x1000"));
    }

    #[test]
    fn test_error_context_builder() {
        let error = ConversionError::EncryptedPdo;
        let ctx = ErrorContext::new(error)
            .with_path("encrypted.pdo")
            .with_suggestion("Используйте инструмент дешифровки")
            .with_suggestion("Проверьте лицензию");

        assert_eq!(ctx.suggestions.len(), 2);
        assert!(ctx.recovery_applied == false);
    }
}
