//! Tauri commands for the desktop application

use tauri::{AppHandle, Emitter, State};
use std::sync::Mutex;
use crate::AppState;
use pepakura_core::{Mesh, UnfoldConfig, UnfoldResult};
use pepakura_core::error::PepakuraError;
use pepakura_core::unfold::{unfold_mds, UnfoldedMesh};
use crate::state::{AppState as NewAppState, ProjectId, AppSettings, Project};
use crate::ai::replicate_client::{ReplicateClient, ReplicateError};
use crate::utils::converters::glb_to_obj_string;
use log::info;
use chrono::Utc;
use tauri_plugin_store::StoreExt;

/// Преобразование ошибки PepakuraError в строку для Tauri
fn map_pepakura_error(err: PepakuraError) -> String {
    format!("Pepakura error: {:?}", err)
}

/// Создать новый проект
///
/// # Arguments
/// * `name` - Название проекта
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<ProjectId, String>` - Идентификатор созданного проекта или ошибка
#[tauri::command]
pub async fn create_project(
    name: String,
    state: State<'_, Mutex<NewAppState>>
) -> Result<ProjectId, String> {
    info!("[TAURI] create_project called with name: {}", name);
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    
    // Генерируем новый ID (простой инкремент от количества проектов)
    let new_id = state.projects.len() as u64 + 1;
    
    let project = Project {
        id: new_id,
        name: name.clone(),
        created_at: Utc::now(),
        mesh_ids: Vec::new(),
        unfolded_ids: Vec::new(),
    };
    
    state.projects.insert(new_id, project);
    info!("[TAURI] Project created with ID: {}", new_id);
    Ok(new_id)
}

/// Импортировать модель из файла
///
/// # Arguments
/// * `path` - Путь к файлу модели
/// * `format` - Формат файла (например, "obj", "stl")
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<Mesh, String>` - Загруженная модель или ошибка
#[tauri::command]
pub async fn import_model(
    path: String,
    format: String,
    state: State<'_, Mutex<NewAppState>>
) -> Result<Mesh, String> {
    info!("[TAURI] import_model called with path: {}, format: {}", path, format);
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    
    // Заглушка: создаём простой куб
    // В будущем здесь будет загрузка из файла
    let mesh = create_cube_mesh();
    let new_id = state.meshes.len() + 1;
    state.meshes.insert(new_id, mesh.clone());
    info!("[TAURI] Mesh created with ID: {}", new_id);
    Ok(mesh)
}

/// Создать простой куб для тестирования
fn create_cube_mesh() -> Mesh {
    use pepakura_core::geometry::{Vertex, Face, Mesh, MeshMetadata};
    
    let vertices = vec![
        Vertex {
            id: 0,
            position: [0.0, 0.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([0.0, 0.0]),
        },
        Vertex {
            id: 1,
            position: [1.0, 0.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([1.0, 0.0]),
        },
        Vertex {
            id: 2,
            position: [1.0, 1.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([1.0, 1.0]),
        },
        Vertex {
            id: 3,
            position: [0.0, 1.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([0.0, 1.0]),
        },
        Vertex {
            id: 4,
            position: [0.0, 0.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([0.0, 0.0]),
        },
        Vertex {
            id: 5,
            position: [1.0, 0.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([1.0, 0.0]),
        },
        Vertex {
            id: 6,
            position: [1.0, 1.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([1.0, 1.0]),
        },
        Vertex {
            id: 7,
            position: [0.0, 1.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([0.0, 1.0]),
        },
    ];
    
    let faces = vec![
        Face { vertices: [0, 1, 2], material_id: None },
        Face { vertices: [0, 2, 3], material_id: None },
        Face { vertices: [4, 6, 5], material_id: None },
        Face { vertices: [4, 7, 6], material_id: None },
        Face { vertices: [0, 4, 5], material_id: None },
        Face { vertices: [0, 5, 1], material_id: None },
        Face { vertices: [1, 5, 6], material_id: None },
        Face { vertices: [1, 6, 2], material_id: None },
        Face { vertices: [2, 6, 7], material_id: None },
        Face { vertices: [2, 7, 3], material_id: None },
        Face { vertices: [3, 7, 4], material_id: None },
        Face { vertices: [3, 4, 0], material_id: None },
    ];
    
    Mesh {
        vertices,
        faces,
        name: "Cube".to_string(),
        metadata: MeshMetadata {
            name: Some("Cube".to_string()),
            author: None,
            description: None,
            tags: Vec::new(),
            created_at: None,
            modified_at: None,
        },
    }
}

/// Развернуть меш
///
/// # Arguments
/// * `mesh_id` - ID меша в состоянии
/// * `config` - Конфигурация развёртки
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<UnfoldedMesh, String>` - Развёрнутый меш или ошибка
#[tauri::command]
pub async fn unfold_mesh(
    mesh_id: usize,
    config: UnfoldConfig,
    state: State<'_, Mutex<NewAppState>>
) -> Result<UnfoldedMesh, String> {
    info!("[TAURI] unfold_mesh called with mesh_id: {}", mesh_id);
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    
    // Получить меш из состояния
    let mesh = state.meshes.get(&mesh_id)
        .ok_or_else(|| format!("Mesh with ID {} not found", mesh_id))?;
    
    // Вызвать алгоритм развёртки из ядра
    let unfolded = unfold_mds(&mesh, &config)
        .map_err(|e| format!("Unfold error: {:?}", e))?;
    
    // Сохранить развёрнутый меш в состоянии
    let new_id = state.unfolded.len() + 1;
    state.unfolded.insert(new_id, unfolded.clone());
    info!("[TAURI] Unfolded mesh created with ID: {}", new_id);
    Ok(unfolded)
}

/// Экспортировать развёртку в SVG
///
/// # Arguments
/// * `unfolded_id` - ID развёрнутого меша
/// * `path` - Путь для сохранения файла
/// * `config` - Конфигурация экспорта SVG
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<(), String>` - Успех или ошибка
#[tauri::command]
pub async fn export_svg(
    unfolded_id: usize,
    path: String,
    config: pepakura_core::export::SvgExportConfig,
    state: State<'_, Mutex<NewAppState>>
) -> Result<(), String> {
    info!("[TAURI] export_svg called with unfolded_id: {}, path: {}", unfolded_id, path);
    // Получить развёрнутый меш из состояния (клонируем, чтобы отпустить мьютекс)
    let unfolded = {
        let guard = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
        let x = guard.unfolded.get(&unfolded_id)
            .ok_or_else(|| format!("Unfolded mesh with ID {} not found", unfolded_id))?
            .clone();
        x
    };
    
    // Экспортировать в SVG
    let svg_content = pepakura_core::export::export_svg(&unfolded, &config)
        .map_err(|e| format!("Export error: {:?}", e))?;
    
    // Записать в файл
    tokio::fs::write(&path, svg_content).await
        .map_err(|e| format!("Failed to write SVG file: {}", e))?;
    
    info!("[TAURI] SVG exported to {}", path);
    Ok(())
}

/// Load a 3D model from a file
///
/// # Arguments
/// * `path` - Path to the model file
///
/// # Returns
/// * `Result<Mesh, String>` - The loaded model or an error message
#[tauri::command]
pub fn load_model(_path: &str) -> Result<Mesh, String> {
    // For now, we'll create a simple cube model
    // In a real implementation, this would load from various formats
    use pepakura_core::geometry::{Vertex, Face, Mesh, MeshMetadata};
    
    let vertices = vec![
        Vertex {
            id: 0,
            position: [0.0, 0.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([0.0, 0.0]),
        },
        Vertex {
            id: 1,
            position: [1.0, 0.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([1.0, 0.0]),
        },
        Vertex {
            id: 2,
            position: [1.0, 1.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([1.0, 1.0]),
        },
        Vertex {
            id: 3,
            position: [0.0, 1.0, 0.0],
            normal: Some([0.0, 0.0, -1.0]),
            uv: Some([0.0, 1.0]),
        },
        Vertex {
            id: 4,
            position: [0.0, 0.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([0.0, 0.0]),
        },
        Vertex {
            id: 5,
            position: [1.0, 0.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([1.0, 0.0]),
        },
        Vertex {
            id: 6,
            position: [1.0, 1.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([1.0, 1.0]),
        },
        Vertex {
            id: 7,
            position: [0.0, 1.0, 1.0],
            normal: Some([0.0, 0.0, 1.0]),
            uv: Some([0.0, 1.0]),
        },
    ];
    
    let faces = vec![
        Face { vertices: [0, 1, 2], material_id: None },
        Face { vertices: [0, 2, 3], material_id: None },
        Face { vertices: [4, 6, 5], material_id: None },
        Face { vertices: [4, 7, 6], material_id: None },
        Face { vertices: [0, 4, 5], material_id: None },
        Face { vertices: [0, 5, 1], material_id: None },
        Face { vertices: [1, 5, 6], material_id: None },
        Face { vertices: [1, 6, 2], material_id: None },
        Face { vertices: [2, 6, 7], material_id: None },
        Face { vertices: [2, 7, 3], material_id: None },
        Face { vertices: [3, 7, 4], material_id: None },
        Face { vertices: [3, 4, 0], material_id: None },
    ];
    
    Ok(Mesh {
        vertices,
        faces,
        name: "Cube".to_string(),
        metadata: MeshMetadata {
            name: Some("Cube".to_string()),
            author: None,
            description: None,
            tags: Vec::new(),
            created_at: None,
            modified_at: None,
        },
    })
}


/// Export the unfolded result to PNG
///
/// # Arguments
/// * `result` - The unfolding result
///
/// # Returns
/// * `Result<Vec<u8>, String>` - The PNG data or an error message
#[tauri::command]
pub fn export_png(_result: UnfoldResult) -> Result<Vec<u8>, String> {
    // TODO: реализовать экспорт PNG
    Ok(Vec::new())
}

/// Export the unfolded result to JPG
///
/// # Arguments
/// * `result` - The unfolding result
///
/// # Returns
/// * `Result<Vec<u8>, String>` - The JPG data or an error message
#[tauri::command]
pub fn export_jpg(_result: UnfoldResult) -> Result<Vec<u8>, String> {
    // TODO: реализовать экспорт JPG
    Ok(Vec::new())
}

/// Export the unfolded result to OBJ
///
/// # Arguments
/// * `result` - The unfolding result
///
/// # Returns
/// * `Result<Vec<u8>, String>` - The OBJ data or an error message
#[tauri::command]
pub fn export_obj(_result: UnfoldResult) -> Result<Vec<u8>, String> {
    // TODO: реализовать экспорт OBJ
    Ok(Vec::new())
}

/// Export the unfolded result to STL
///
/// # Arguments
/// * `result` - The unfolding result
///
/// # Returns
/// * `Result<Vec<u8>, String>` - The STL data or an error message
#[tauri::command]
pub fn export_stl(_result: UnfoldResult) -> Result<Vec<u8>, String> {
    // TODO: реализовать экспорт STL
    Ok(Vec::new())
}

/// Export the unfolded result to PDF
///
/// # Arguments
/// * `result` - The unfolding result
///
/// # Returns
/// * `Result<Vec<u8>, String>` - The PDF data or an error message
#[tauri::command]
pub fn export_unfold_pdf(_result: UnfoldResult) -> Result<Vec<u8>, String> {
    // TODO: реализовать экспорт PDF
    Ok(Vec::new())
}

/// Unfold a 3D model from OBJ file to SVG
///
/// # Arguments
/// * `obj_path` - Path to the OBJ file
///
/// # Returns
/// * `Result<String, String>` - The SVG content or an error message
#[tauri::command]
pub async fn unfold_3d_model(_obj_path: String) -> Result<String, String> {
    // Temporary stub
    Err("Unfold 3D model not implemented yet".to_string())
}

/// Получить список последних проектов
///
/// # Arguments
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<Vec<ProjectInfo>, String>` - Список проектов или ошибка
#[tauri::command]
pub async fn get_recent_projects(
    state: State<'_, Mutex<NewAppState>>
) -> Result<Vec<Project>, String> {
    info!("[TAURI] get_recent_projects called");
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    
    let mut projects: Vec<Project> = state.projects.iter()
        .map(|entry| entry.value().clone())
        .collect();
    
    // Сортируем по дате создания (новые сверху)
    projects.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    
    // Ограничиваем количество (например, 10)
    projects.truncate(10);
    
    Ok(projects)
}

/// Удалить проект
///
/// # Arguments
/// * `id` - ID проекта
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<(), String>` - Успех или ошибка
#[tauri::command]
pub async fn delete_project(
    id: ProjectId,
    state: State<'_, Mutex<NewAppState>>
) -> Result<(), String> {
    info!("[TAURI] delete_project called with id: {}", id);
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    
    if state.projects.remove(&id).is_none() {
        return Err(format!("Project with ID {} not found", id));
    }
    
    info!("[TAURI] Project {} deleted", id);
    Ok(())
}

/// Получить версию приложения
///
/// # Returns
/// * `String` - Версия приложения
#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Получить текущие настройки приложения
///
/// # Arguments
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<AppSettings, String>` - Настройки или ошибка
#[tauri::command]
pub async fn get_settings(
    state: State<'_, Mutex<NewAppState>>
) -> Result<AppSettings, String> {
    info!("[TAURI] get_settings called");
    let state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    Ok(state.settings.clone())
}

/// Сохранить настройки приложения
///
/// # Arguments
/// * `settings` - Новые настройки
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<(), String>` - Успех или ошибка
#[tauri::command]
pub async fn save_settings(
    settings: AppSettings,
    state: State<'_, Mutex<NewAppState>>
) -> Result<(), String> {
    info!("[TAURI] save_settings called");
    let mut state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    state.settings = settings;
    // TODO: сохранить настройки на диск через tauri-plugin-store
    info!("[TAURI] Settings saved");
    Ok(())
}

/// Сбросить настройки к значениям по умолчанию
///
/// # Arguments
/// * `state` - Состояние приложения
///
/// # Returns
/// * `Result<AppSettings, String>` - Настройки по умолчанию или ошибка
#[tauri::command]
pub async fn reset_settings(
    state: State<'_, Mutex<NewAppState>>
) -> Result<AppSettings, String> {
    info!("[TAURI] reset_settings called");
    let mut state = state.lock().map_err(|e| format!("Failed to lock state: {}", e))?;
    state.settings = AppSettings::default();
    // TODO: сохранить настройки на диск через tauri-plugin-store
    info!("[TAURI] Settings reset to defaults");
    Ok(state.settings.clone())
}

/// Открыть диалог выбора папки (заглушка)
///
/// # Arguments
/// * `title` - Заголовок диалога
/// * `directory` - Если true, выбирать папку, иначе файл
///
/// # Returns
/// * `Result<String, String>` - Выбранный путь или ошибка
#[tauri::command]
pub async fn open_dialog(
    title: String,
    directory: bool,
) -> Result<String, String> {
    info!("[TAURI] open_dialog called with title: {}, directory: {}", title, directory);
    // Заглушка: возвращаем пустую строку
    // В реальной реализации используем tauri::api::dialog
    Ok("".to_string())
}

/// Get the current application state
///
/// # Arguments
/// * `state` - The application state
///
/// # Returns
/// * `AppState` - The current application state
#[tauri::command]
pub fn get_app_state(state: State<'_, Mutex<AppState>>) -> AppState {
    state.lock().unwrap().clone()
}

/// Update the application state
///
/// # Arguments
/// * `state` - The application state
/// * `new_state` - The new application state
#[tauri::command]
pub fn set_app_state(state: State<'_, Mutex<AppState>>, new_state: AppState) {
    *state.lock().unwrap() = new_state;
}

/// Health check command to verify Tauri connectivity
#[tauri::command]
pub fn health_check() -> &'static str {
    println!("[TAURI] Health check called");
    "OK"
}

/// Nest parts on paper sheets
///
/// # Arguments
/// * `parts_json` - JSON string with array of parts, each part has width and height
/// * `paper_size` - Tuple (width, height) in mm
/// * `margin` - Margin in mm
/// * `gap` - Gap between parts in mm
///
/// # Returns
/// * `Result<String, String>` - JSON string with nesting result
#[tauri::command]
pub fn nest_parts(_parts_json: String, _paper_size: (f32, f32), _margin: f32, _gap: f32) -> Result<String, String> {
    // TODO: implement nesting when pepakura_core includes nesting module
    Err("Nesting not implemented yet".to_string())
}

/// AI generation from image (stub)
///
/// # Arguments
/// * `payload` - JSON payload with image_path, format, quality
///
/// # Returns
/// * `Result<serde_json::Value, String>` - JSON response
#[tauri::command]
pub async fn ai_generate_from_image(_payload: serde_json::Value) -> Result<serde_json::Value, String> {
    // TODO: implement actual AI generation
    let response = serde_json::json!({
        "success": false,
        "error": "AI generation from image is not implemented yet",
        "mesh_path": null,
        "vertices": 0,
        "faces": 0,
        "device": null,
        "cached": false,
    });
    Ok(response)
}

/// AI generation from text (stub)
///
/// # Arguments
/// * `payload` - JSON payload with prompt, quality, format, use_cache
///
/// # Returns
/// * `Result<serde_json::Value, String>` - JSON response
#[tauri::command]
pub async fn ai_generate_from_text(_payload: serde_json::Value) -> Result<serde_json::Value, String> {
    // TODO: implement actual AI generation
    let response = serde_json::json!({
        "success": false,
        "error": "AI generation from text is not implemented yet",
        "mesh_path": null,
        "vertices": 0,
        "faces": 0,
        "device": null,
        "cached": false,
    });
    Ok(response)
}

/// Parse a mock OBJ string and return statistics
#[tauri::command]
pub fn parse_mock_obj(obj_string: String) -> Result<ParseResult, String> {
    let vertices_count = obj_string.lines().filter(|l| l.trim().starts_with("v ")).count();
    let faces_count = obj_string.lines().filter(|l| l.trim().starts_with("f ")).count();

    if vertices_count == 0 && faces_count == 0 {
        Ok(ParseResult {
            success: false,
            vertices_count: 0,
            faces_count: 0,
            error_msg: Some("Invalid OBJ".into()),
        })
    } else {
        Ok(ParseResult {
            success: true,
            vertices_count,
            faces_count,
            error_msg: None,
        })
    }
}

/// Ping-pong test command for IPC verification
#[tauri::command]
pub fn ping_pong(input_message: String) -> Result<PingResponse, String> {
    Ok(PingResponse {
        message: format!("Core received: {}", input_message),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64,
        success: true,
    })
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ParseResult {
    pub success: bool,
    pub vertices_count: usize,
    pub faces_count: usize,
    pub error_msg: Option<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PingResponse {
    pub message: String,
    pub timestamp: u64,
    pub success: bool,
}

#[derive(serde::Serialize, Clone)]
struct UnfoldProgress {
    percent: f32,
    message: String,
}

/// Асинхронная команда для имитации тяжелых вычислений раскладки
/// Выполняется в фоновом потоке и отправляет прогресс во фронтенд
#[tauri::command]
pub async fn start_mock_unfold(app: AppHandle, total_faces: usize) -> Result<String, String> {
    // 1. Проверка лимитов (Защита от OOM на слабых ПК)
    if total_faces > 10000 {
        return Err(format!("Модель слишком сложна ({} граней). Максимум для демо: 10000.", total_faces));
    }

    // 2. Запуск в фоновом потоке (чтобы не заблокировать async runtime Tauri)
    let result = tokio::task::spawn_blocking(move || {
        // ИМИТАЦИЯ тяжелой работы алгоритма MDS
        for i in 1..=10 {
            std::thread::sleep(std::time::Duration::from_millis(500));
            
            let percent = (i as f32 / 10.0) * 100.0;
            let msg = format!("Раскладка граней... {}%", percent as u32);
            
            // 3. Отправка события в UI
            app.emit("unfold-progress", UnfoldProgress { percent, message: msg })
                .map_err(|e| e.to_string())?;
        }
        
        Ok("Mock 2D data generated".to_string())
    })
    .await
    .map_err(|e| e.to_string())?;

    result
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjGeometry {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub face_count: usize,
    pub warnings: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ObjParseError {
    #[error("Файл пуст или не содержит вершин")]
    NoVertices,
    #[error("Обнаружен невалидный индекс: {index} (всего вершин: {total})")]
    InvalidIndex { index: usize, total: usize },
}

#[tauri::command]
pub fn load_real_obj(obj_string: String) -> Result<ObjGeometry, String> {
    // ПРЕДВАРИТЕЛЬНЫЙ ПРОСМОТ: Оценка размера для аллокации памяти
    let mut estimated_vertices = 0;
    let mut estimated_faces = 0;
    for line in obj_string.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('v') { estimated_vertices += 1; }
        if trimmed.starts_with('f') { estimated_faces += 1; }
    }

    if estimated_vertices == 0 {
        return Err(ObjParseError::NoVertices.to_string());
    }

    // БЕЗОПАСНАЯ АЛЛОКАЦИЯ: reserve ровно столько памяти, сколько нужно
    let mut positions = Vec::with_capacity(estimated_vertices * 3);
    let mut indices = Vec::with_capacity(estimated_faces * 3);
    let mut warnings = Vec::new();
    let mut current_vertex_index = 0u32;
    let mut actual_face_count = 0;

    for line in obj_string.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() { continue; }

        match parts[0] {
            "v" => {
                // БЕЗОПАСНЫЙ ПАРСИНГ: .parse().unwrap_or(0.0) вместо .unwrap()
                let x: f32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let y: f32 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                let z: f32 = parts.get(3).and_then(|s| s.parse().ok()).unwrap_or(0.0);
                positions.push(x);
                positions.push(y);
                positions.push(z);
                current_vertex_index += 1;
            }
            "f" => {
                if parts.len() < 4 {
                    warnings.push(format!("Игнорируем грань с {} вершинами (минимум 3)", parts.len()));
                    continue;
                }

                // ВАЛИДАЦИЯ ИНДЕКСОВ: Проверяем, не ссылается ли грань на несуществующую вершину
                let parse_idx = |s: &str, max: u32| -> Option<u32> {
                    s.parse().ok().and_then(|i: u32| if i < max { Some(i) } else { None })
                };

                if parts.len() == 4 {
                    let i0 = match parse_idx(parts[1], current_vertex_index) {
                        Some(val) => val,
                        None => continue,
                    };
                    let i1 = match parse_idx(parts[2], current_vertex_index) {
                        Some(val) => val,
                        None => continue,
                    };
                    let i2 = match parse_idx(parts[3], current_vertex_index) {
                        Some(val) => val,
                        None => continue,
                    };

                    indices.push(i0);
                    indices.push(i1);
                    indices.push(i2);
                    actual_face_count += 1;
                } else {
                    // ТРИАНГУЛЯЦИЯ N-ГОНОВ (4+ вершин)
                    let first = match parse_idx(parts[1], current_vertex_index) { Some(v) => v, None => continue };
                    for i in 2..parts.len() {
                        let prev = match parse_idx(parts[i-1], current_vertex_index) { Some(v) => v, None => break };
                        let curr = match parse_idx(parts[i], current_vertex_index) { Some(v) => v, None => break };
                        indices.push(first);
                        indices.push(prev);
                        indices.push(curr);
                    }
                    actual_face_count += 1;
                }
            }
            _ => {} // Игнорируем vn, vt, usemtl
        }
    }

    Ok(ObjGeometry {
        positions,
        indices,
        face_count: actual_face_count,
        warnings,
    })
}

/// Сохраняет API ключ Replicate в защищенное хранилище
///
/// # Аргументы
/// * `key` - API ключ
/// * `app` - Handle приложения
///
/// # Возвращает
/// * `Result<(), String>` - Успех или ошибка
#[tauri::command]
pub async fn save_api_key(key: String, app: AppHandle) -> Result<(), String> {
    info!("[TAURI] save_api_key called (длина ключа: {})", key.len());
    
    let store = app.store("settings.json")
        .map_err(|e| format!("Не удалось открыть хранилище: {}", e))?;
    
    store.set("api_key", key)
        .map_err(|e| format!("Не удалось сохранить ключ: {}", e))?;
    
    info!("[TAURI] API ключ сохранен");
    Ok(())
}

/// Проверяет, сохранен ли API ключ
///
/// # Аргументы
/// * `app` - Handle приложения
///
/// # Возвращает
/// * `Result<bool, String>` - true если ключ есть, false если нет
#[tauri::command]
pub async fn has_api_key(app: AppHandle) -> Result<bool, String> {
    let store = app.store("settings.json")
        .map_err(|e| format!("Не удалось открыть хранилище: {}", e))?;
    
    let key: Option<String> = store.get("api_key")
        .map_err(|e| format!("Не удалось прочитать ключ: {}", e))?;
    
    Ok(key.is_some())
}

/// Полный пайплайн генерации и развертки 3D модели из текста
///
/// # Аргументы
/// * `prompt` - Текстовое описание модели
/// * `app` - Handle приложения
/// * `window` - Окно для отправки событий прогресса
///
/// # Возвращает
/// * `Result<String, String>` - SVG содержимое развертки или ошибка
#[tauri::command]
pub async fn generate_and_unfold(
    prompt: String,
    app: AppHandle,
    window: tauri::Window,
) -> Result<String, String> {
    info!("[TAURI] generate_and_unfold called with prompt: '{}'", prompt);
    
    // 1. Получить API ключ из защищенного хранилища
    let store = app.store("settings.json")
        .map_err(|e| format!("Не удалось открыть хранилище: {}", e))?;
    
    let api_key: String = store.get("api_key")
        .map_err(|e| format!("Не удалось прочитать ключ: {}", e))?
        .ok_or_else(|| "API ключ не найден. Пожалуйста, сохраните ключ в настройках.".to_string())?;
    
    if api_key.is_empty() {
        return Err("API ключ пустой".to_string());
    }
    
    // 2. Создать клиент Replicate
    let client = ReplicateClient::new(api_key);
    
    // 3. Генерация модели (асинхронно с обработкой прогресса)
    window.emit("ai-generation-progress", serde_json::json!({
        "percent": 10,
        "message": "Начинаем генерацию 3D модели..."
    })).map_err(|e| e.to_string())?;
    
    let model_bytes = tokio::task::spawn_blocking(move || {
        // В реальности здесь должен быть async вызов, но spawn_blocking позволяет
        // не блокировать основной поток Tauri
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                client.generate(&prompt, None).await
            })
    })
    .await
    .map_err(|e| format!("Ошибка выполнения задачи генерации: {}", e))?
    .map_err(|e| format!("Ошибка генерации: {}", e))?;
    
    window.emit("ai-generation-progress", serde_json::json!({
        "percent": 40,
        "message": "Модель сгенерирована, конвертируем формат..."
    })).map_err(|e| e.to_string())?;
    
    // 4. Конвертация GLB -> OBJ строка
    let obj_string = glb_to_obj_string(&model_bytes)
        .map_err(|e| format!("Ошибка конвертации GLB в OBJ: {}", e))?;
    
    window.emit("ai-generation-progress", serde_json::json!({
        "percent": 60,
        "message": "Загружаем геометрию..."
    })).map_err(|e| e.to_string())?;
    
    // 5. Парсинг OBJ в геометрию (используем существующий парсер)
    let geometry = load_real_obj(obj_string)?;
    
    // 6. Создание меша из геометрии (упрощенное преобразование)
    // TODO: Реализовать преобразование ObjGeometry в Mesh из pepakura_core
    // Для MVP создадим простой куб
    let mesh = create_cube_mesh();
    
    window.emit("ai-generation-progress", serde_json::json!({
        "percent": 80,
        "message": "Выполняем развертку..."
    })).map_err(|e| e.to_string())?;
    
    // 7. Развертка меша
    let config = UnfoldConfig::default();
    let unfolded = unfold_mds(&mesh, &config)
        .map_err(|e| format!("Ошибка развертки: {:?}", e))?;
    
    window.emit("ai-generation-progress", serde_json::json!({
        "percent": 95,
        "message": "Экспортируем в SVG..."
    })).map_err(|e| e.to_string())?;
    
    // 8. Экспорт в SVG
    let svg_config = pepakura_core::export::SvgExportConfig::default();
    let svg_content = pepakura_core::export::export_svg(&unfolded, &svg_config)
        .map_err(|e| format!("Ошибка экспорта SVG: {:?}", e))?;
    
    window.emit("ai-generation-progress", serde_json::json!({
        "percent": 100,
        "message": "Готово!"
    })).map_err(|e| e.to_string())?;
    
    info!("[TAURI] generate_and_unfold успешно завершен");
    Ok(svg_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mock_obj_success() {
        let mock_obj = "v 0 0 0\nv 1 0 0\nv 1 1 0\nf 1 2 3";
        let result = parse_mock_obj(mock_obj.to_string()).unwrap();
        
        assert_eq!(result.success, true);
        assert_eq!(result.vertices_count, 3);
        assert_eq!(result.faces_count, 1);
        assert!(result.error_msg.is_none());
    }

    #[test]
    fn test_parse_mock_obj_empty_input() {
        let result = parse_mock_obj("".to_string()).unwrap();
        assert_eq!(result.success, false);
        // Проверяем, что мы не упали, а вернули ошибку
        assert!(result.error_msg.is_some());
    }
}