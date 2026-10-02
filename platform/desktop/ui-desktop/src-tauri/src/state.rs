//! Глобальное состояние приложения

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use pepakura_core::{Mesh, UnfoldConfig};
use pepakura_core::unfold::{UnfoldAlgorithm, UnfoldedMesh};

/// Идентификатор проекта
pub type ProjectId = u64;

/// Конфигурация приложения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfiguration {
    pub default_export_path: String,
    pub auto_save: bool,
    pub language: String,
}

/// Настройки приложения (полные)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: String,
    pub theme: String,
    pub default_export_path: String,
    pub unfold_config: UnfoldConfig,
    pub ai_config: AiConfig,
}

/// AI конфигурация
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: "ru".to_string(),
            theme: "system".to_string(),
            default_export_path: "".to_string(),
            unfold_config: UnfoldConfig {
                preserve_detail: true,
                max_iterations: 100,
                tolerance: 1e-6,
                algorithm: UnfoldAlgorithm::MDS,
            },
            ai_config: AiConfig {
                provider: "ollama".to_string(),
                model: "llama3.2".to_string(),
                api_key: None,
            },
        }
    }
}

/// Состояние приложения
#[derive(Debug)]
pub struct AppState {
    /// Проекты, загруженные в приложение
    pub projects: DashMap<ProjectId, Project>,
    /// Загруженные меши
    pub meshes: DashMap<usize, Mesh>,
    /// Развёрнутые меши
    pub unfolded: DashMap<usize, UnfoldedMesh>,
    /// Конфигурация приложения (устаревшая, для обратной совместимости)
    pub config: AppConfiguration,
    /// Полные настройки приложения
    pub settings: AppSettings,
}

/// Проект
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub mesh_ids: Vec<usize>,
    pub unfolded_ids: Vec<usize>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            projects: DashMap::new(),
            meshes: DashMap::new(),
            unfolded: DashMap::new(),
            config: AppConfiguration {
                default_export_path: "".to_string(),
                auto_save: true,
                language: "ru".to_string(),
            },
            settings: AppSettings::default(),
        }
    }
}