//! Основные команды Tauri для работы с проектами

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tauri::api::dialog;

// Импортируем команды оптимизации
use crate::unfold::paper_optimize::*;

/// Структура проекта
#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub model_path: String,
    pub created_at: String,
    pub settings: ProjectSettings,
}

/// Настройки проекта
#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectSettings {
    pub ai_backend: String,
    pub ai_model: String,
    pub export_format: String,
}

/// Импортирует 3D модель
#[tauri::command]
pub fn import_model(app_handle: tauri::AppHandle) -> Result<String, String> {
    // Открываем диалог выбора файла
    let file_path = dialog::blocking::FileDialogBuilder::new()
        .add_filter("3D Models", &["obj", "stl", "ply"])
        .pick_file()
        .map_err(|_| "Failed to open file dialog".to_string())?;
    
    match file_path {
        Some(path) => {
            // Проверяем, что файл существует
            if !Path::new(&path).exists() {
                return Err("Selected file does not exist".to_string());
            }
            
            // Возвращаем путь к файлу
            Ok(path.to_string_lossy().to_string())
        }
        None => Err("No file selected".to_string()),
    }
}

// Реэкспортируем команды оптимизации
pub use crate::unfold::paper_optimize::{
    optimize_model_for_paper,
    get_default_paper_optimize_params,
};

/// Сохраняет проект
#[tauri::command]
pub fn save_project(project: Project, file_path: String) -> Result<(), String> {
    // Сериализуем проект в JSON
    let json = serde_json::to_string_pretty(&project)
        .map_err(|e| format!("Failed to serialize project: {}", e))?;
    
    // Сохраняем в файл
    fs::write(&file_path, json)
        .map_err(|e| format!("Failed to save project: {}", e))?;
    
    Ok(())
}

/// Загружает проект
#[tauri::command]
pub fn load_project(file_path: String) -> Result<Project, String> {
    // Читаем файл
    let json = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read project file: {}", e))?;
    
    // Десериализуем проект
    let project: Project = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse project file: {}", e))?;
    
    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_project_serialization() {
        let project = Project {
            name: "Test Project".to_string(),
            model_path: "/path/to/model.obj".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
            settings: ProjectSettings {
                ai_backend: "ollama".to_string(),
                ai_model: "llama2".to_string(),
                export_format: "svg".to_string(),
            },
        };
        
        let json = serde_json::to_string_pretty(&project).unwrap();
        let loaded_project: Project = serde_json::from_str(&json).unwrap();
        
        assert_eq!(project.name, loaded_project.name);
        assert_eq!(project.model_path, loaded_project.model_path);
    }
}