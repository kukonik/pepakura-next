//! Манифест аддона - метаданные и возможности

use serde::{Deserialize, Serialize};

/// Тип аддона
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AddonType {
    /// Плагин импорта (новые форматы файлов)
    Importer,
    /// Плагин экспорта (новые форматы файлов)
    Exporter,
    /// Алгоритм развёртки
    Unfolder,
    /// Инструмент оптимизации
    Optimizer,
    /// Интеграция с внешним сервисом
    Integration,
    /// Утилита или инструмент
    Utility,
    /// Пользовательский тип
    Custom(String),
}

/// Возможности аддона
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddonCapabilities {
    /// Поддерживаемые форматы импорта
    pub import_formats: Vec<String>,
    /// Поддерживаемые форматы экспорта
    pub export_formats: Vec<String>,
    /// Требуемые разрешения
    pub permissions: Vec<String>,
    /// Зависимости от других аддонов
    pub dependencies: Vec<String>,
    /// Минимальная версия Pepakura Core
    pub min_core_version: Option<String>,
}

/// Манифест аддона
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddonManifest {
    /// Уникальное имя аддона
    pub name: String,
    /// Версия аддона (SemVer)
    pub version: String,
    /// Описание аддона
    pub description: String,
    /// Автор аддона
    pub author: Option<String>,
    /// Тип аддона
    pub addon_type: AddonType,
    /// Возможности аддона
    pub capabilities: AddonCapabilities,
    /// URL репозитория
    pub repository: Option<String>,
    /// Лицензия
    pub license: Option<String>,
}

impl AddonManifest {
    /// Создать новый манифест
    pub fn new(name: impl Into<String>, version: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            author: None,
            addon_type: AddonType::Custom("unknown".to_string()),
            capabilities: AddonCapabilities::default(),
            repository: None,
            license: None,
        }
    }

    /// Установить тип аддона
    pub fn with_type(mut self, addon_type: AddonType) -> Self {
        self.addon_type = addon_type;
        self
    }

    /// Добавить поддерживаемый формат импорта
    pub fn with_import_format(mut self, format: impl Into<String>) -> Self {
        self.capabilities.import_formats.push(format.into());
        self
    }

    /// Добавить поддерживаемый формат экспорта
    pub fn with_export_format(mut self, format: impl Into<String>) -> Self {
        self.capabilities.export_formats.push(format.into());
        self
    }

    /// Установить автора
    pub fn with_author(mut self, author: impl Into<String>) -> Self {
        self.author = Some(author.into());
        self
    }

    /// Установить лицензию
    pub fn with_license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }
}

impl Default for AddonManifest {
    fn default() -> Self {
        Self::new("unknown", "0.1.0", "Unknown addon")
    }
}
