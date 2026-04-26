use pepakura_platform::fs::{FileSystem, FileError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono;

use crate::pepa_scene_adapter::{PepaScene};
use crate::nesting::{PaperSettings, NestParams};

/// Версия схемы проекта
pub const PROJECT_SCHEMA_VERSION: &str = "1.0";

/// Метаданные проекта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    /// Название проекта
    pub name: String,
    /// Описание проекта
    pub description: Option<String>,
    /// Автор проекта
    pub author: Option<String>,
    /// Дата создания
    pub created_at: String,
    /// Дата последнего изменения
    pub updated_at: String,
}

/// Настройки проекта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    /// Формат листа для печати
    pub paper_format: String,
    /// Ширина поля в мм
    pub margin_mm: f32,
    /// Масштаб
    pub scale: f32,
    /// Настройки бумаги для размещения
    pub paper_settings: Option<PaperSettings>,
    /// Параметры размещения
    pub nest_params: Option<NestParams>,
    /// Дополнительные настройки (для расширений)
    pub extensions: HashMap<String, serde_json::Value>,
}

/// Основная структура проекта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaProject {
    /// Версия схемы
    pub schema_version: String,
    /// Метаданные проекта
    pub project_meta: ProjectMeta,
    /// Сцена проекта
    pub scene: PepaScene,
    /// Настройки проекта
    pub settings: ProjectSettings,
    /// Расширения проекта
    pub extensions: HashMap<String, serde_json::Value>,
}

// ==========================================
// ПЕРВЫЙ БЛОК IMPL
// ==========================================
impl PepaProject {
    /// Создание нового проекта
    pub fn new(name: String, scene: PepaScene) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            schema_version: PROJECT_SCHEMA_VERSION.to_string(),
            project_meta: ProjectMeta {
                name,
                description: None,
                author: None,
                created_at: now.clone(),
                updated_at: now,
            },
            scene,
            settings: ProjectSettings {
                paper_format: "A4".to_string(),
                margin_mm: 5.0,
                scale: 1.0,
                paper_settings: None,
                nest_params: None,
                extensions: HashMap::new(),
            },
            extensions: HashMap::new(),
        }
    }
} // <--- ВОТ ТУТ НЕ ХВАТАЛО ЗАКРЫВАЮЩЕЙ СКОБКИ!

// ==========================================
// ВТОРОЙ БЛОК IMPL
// ==========================================
impl PepaProject {
    /// Загрузка проекта из файла
    pub fn load_from_file<F: FileSystem>(fs: &F, path: &str) -> Result<PepaProject, FileError> {
        let file_data = fs.read_file(path)?;
        let contents = file_data.as_string()?;
        let project: PepaProject = serde_json::from_str(&contents)
            .map_err(|e| FileError::ReadError(format!("JSON deserialize error: {}", e)))?;
        Ok(project)
    }

    /// Сохранение проекта в файл
    pub fn save_to_file<F: FileSystem>(&self, fs: &F, path: &str) -> Result<(), FileError> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| FileError::WriteError(format!("JSON serialize error: {}", e)))?;
        fs.write_text(path, &json)?;
        Ok(())
    }
    
    /// Получить результат развертки модели
    pub fn get_unfold_result(&self) -> crate::unfold::UnfoldResult {
        // Пока возвращаем пустой результат
        // В реальной реализации здесь будет логика получения результата развертки
        crate::unfold::UnfoldResult {
            faces: vec![],
            seams: vec![],
            layout: crate::unfold::LayoutResult {
                faces: vec![],
                width: 0.0,
                height: 0.0,
            },
        }
    }
}