//! Базовые трейты для плагинов.

use crate::geometry::Mesh;
use crate::unfold::{UnfoldedMesh, UnfoldConfig};
use crate::PepakuraError;
use std::path::Path;

/// Плагин импорта моделей.
/// 
/// Реализуется для поддержки различных форматов файлов.
/// 
/// ## Пример
/// 
/// ```rust
/// pub struct ObjImporter;
/// 
/// impl ImportPlugin for ObjImporter {
///     fn name(&self) -> &str { "Wavefront OBJ" }
///     fn supported_extensions(&self) -> &[&str] { &["obj"] }
///     fn import(&self, path: &Path) -> Result<Mesh, PepakuraError> {
///         // реализация импорта
///         todo!()
///     }
/// }
/// ```
pub trait ImportPlugin: Send + Sync {
    /// Возвращает имя плагина.
    fn name(&self) -> &str;
    
    /// Поддерживаемые расширения файлов (без точки).
    fn supported_extensions(&self) -> &[&str];
    
    /// Импортирует модель из файла.
    /// 
    /// # Аргументы
    /// * `path` - путь к файлу
    /// 
    /// # Возвращает
    /// * `Ok(Mesh)` - загруженный меш
    /// * `Err(PepakuraError)` - ошибка импорта
    fn import(&self, path: &Path) -> Result<Mesh, PepakuraError>;
    
    /// Проверяет, поддерживает ли плагин данное расширение.
    fn supports_extension(&self, ext: &str) -> bool {
        self.supported_extensions()
            .iter()
            .any(|&e| e.eq_ignore_ascii_case(ext))
    }
}

/// Плагин экспорта.
/// 
/// Реализуется для поддержки различных форматов экспорта.
pub trait ExportPlugin: Send + Sync {
    /// Возвращает имя плагина.
    fn name(&self) -> &str;
    
    /// Поддерживаемые расширения файлов (без точки).
    fn supported_extensions(&self) -> &[&str];
    
    /// Экспортирует развёрнутый меш в файл.
    /// 
    /// # Аргументы
    /// * `unfolded` - развёрнутый меш
    /// * `path` - путь к файлу
    /// 
    /// # Возвращает
    /// * `Ok(())` - успех
    /// * `Err(PepakuraError)` - ошибка экспорта
    fn export(&self, unfolded: &UnfoldedMesh, path: &Path) -> Result<(), PepakuraError>;
    
    /// Проверяет, поддерживает ли плагин данное расширение.
    fn supports_extension(&self, ext: &str) -> bool {
        self.supported_extensions()
            .iter()
            .any(|&e| e.eq_ignore_ascii_case(ext))
    }
}

/// Плагин развёртки.
/// 
/// Реализуется для альтернативных алгоритмов развёртки.
pub trait UnfoldPlugin: Send + Sync {
    /// Возвращает имя плагина.
    fn name(&self) -> &str;
    
    /// Описание алгоритма.
    fn description(&self) -> &str {
        "Пользовательский алгоритм развёртки"
    }
    
    /// Разворачивает меш.
    /// 
    /// # Аргументы
    /// * `mesh` - исходный 3D-меш
    /// * `config` - параметры развёртки
    /// 
    /// # Возвращает
    /// * `Ok(UnfoldedMesh)` - развёрнутый меш
    /// * `Err(PepakuraError)` - ошибка развёртки
    fn unfold(&self, mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh, PepakuraError>;
}

/// Конфигурация плагина.
#[derive(Debug, Clone)]
pub struct PluginConfig {
    /// Включён ли плагин
    pub enabled: bool,
    /// Приоритет плагина (чем выше, тем важнее)
    pub priority: i32,
    /// Пользовательские настройки
    pub settings: std::collections::HashMap<String, String>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            priority: 0,
            settings: std::collections::HashMap::new(),
        }
    }
}

/// Метаданные плагина.
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    /// Уникальный идентификатор
    pub id: String,
    /// Название
    pub name: String,
    /// Версия
    pub version: String,
    /// Автор
    pub author: Option<String>,
    /// Описание
    pub description: Option<String>,
    /// Конфигурация
    pub config: PluginConfig,
}

impl PluginMetadata {
    /// Создаёт новые метаданные плагина.
    pub fn new(id: &str, name: &str, version: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            version: version.to_string(),
            author: None,
            description: None,
            config: PluginConfig::default(),
        }
    }
    
    /// Устанавливает автора.
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }
    
    /// Устанавливает описание.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }
}
