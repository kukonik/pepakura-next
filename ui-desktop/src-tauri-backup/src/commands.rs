//! Tauri commands for the desktop application

use tauri::State;
use crate::AppState;
use pepakura_core::model::Mesh;
use pepakura_core::unfold::{unfold_mesh, UnfoldParams, UnfoldResult};
use pepakura_core::export::{export_to_svg, export_to_png, export_to_jpg, export_to_obj, export_to_stl};
use std::path::PathBuf;
use tokio::process::Command;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use serde_json::Value;

/// Load a 3D model from a file
/// 
/// # Arguments
/// * `path` - Path to the model file
/// 
/// # Returns
/// * `Result<Mesh, String>` - The loaded mesh or an error message
#[tauri::command]
pub fn load_model(path: &str) -> Result<Mesh, String> {
    // For now, we'll create a simple cube mesh
    // In a real implementation, this would load from various formats
    let vertices = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ];
    
    let faces = vec![
        vec![0, 1, 2, 3], // bottom
        vec![4, 7, 6, 5], // top
        vec![0, 4, 5, 1], // front
        vec![2, 6, 7, 3], // back
        vec![0, 3, 7, 4], // left
        vec![1, 5, 6, 2], // right
    ];
    
    Ok(Mesh { vertices, faces })
}

/// Unfold a 3D model into 2D layouts
/// 
/// # Arguments
/// * `mesh` - The 3D mesh to unfold
/// * `params` - Unfolding parameters
/// 
/// # Returns
/// * `Result<UnfoldResult, String>` - The unfolding result or an error message
#[tauri::command]
pub fn unfold_model(mesh: Mesh, params: UnfoldParams) -> Result<UnfoldResult, String> {
    unfold_mesh(&mesh, &params).map_err(|e| e.to_string())
}

/// Export the unfolded result to SVG
/// 
/// # Arguments
/// * `result` - The unfolding result
/// 
/// # Returns
/// * `Result<Vec<u8>, String>` - The SVG data or an error message
#[tauri::command]
pub fn export_svg(result: UnfoldResult) -> Result<Vec<u8>, String> {
    Ok(export_to_svg(&result))
}

/// Export the unfolded result to PNG
/// 
/// # Arguments
/// * `result` - The unfolding result
/// 
/// # Returns
/// * `Result<Vec<u8>, String>` - The PNG data or an error message
#[tauri::command]
pub fn export_png(result: UnfoldResult) -> Result<Vec<u8>, String> {
    Ok(export_to_png(&result))
}

/// Export the unfolded result to JPG
/// 
/// # Arguments
/// * `result` - The unfolding result
/// 
/// # Returns
/// * `Result<Vec<u8>, String>` - The JPG data or an error message
#[tauri::command]
pub fn export_jpg(result: UnfoldResult) -> Result<Vec<u8>, String> {
    Ok(export_to_jpg(&result))
}

/// Export the unfolded result to OBJ
/// 
/// # Arguments
/// * `result` - The unfolding result
/// 
/// # Returns
/// * `Result<Vec<u8>, String>` - The OBJ data or an error message
#[tauri::command]
pub fn export_obj(result: UnfoldResult) -> Result<Vec<u8>, String> {
    Ok(export_to_obj(&result))
}

/// Export the unfolded result to STL
/// 
/// # Arguments
/// * `result` - The unfolding result
/// 
/// # Returns
/// * `Result<Vec<u8>, String>` - The STL data or an error message
#[tauri::command]
pub fn export_stl(result: UnfoldResult) -> Result<Vec<u8>, String> {
    Ok(export_to_stl(&result))
}

/// Unfold a 3D model from OBJ file to SVG
///
/// # Arguments
/// * `obj_path` - Path to the OBJ file
///
/// # Returns
/// * `Result<String, String>` - The SVG content or an error message
#[tauri::command]
pub async fn unfold_3d_model(obj_path: String) -> Result<String, String> {
    // Получаем путь к ресурсам приложения
    let resource_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .ok_or("Failed to get parent directory")?
        .to_path_buf();
    
    // Определяем путь к Python скрипту
    let script_path = resource_path.parent().unwrap().parent().unwrap().join("addons").join("addon_server.py");
    
    // Проверяем существование скрипта
    if !script_path.exists() {
        return Err(format!("Python script not found at: {:?}", script_path));
    }
    
    // Проверяем существование файла модели
    let mut model_path = std::path::Path::new(&obj_path).to_path_buf();
    if !model_path.exists() {
        // Если файл не найден по указанному пути, ищем его в текущей директории
        let current_dir = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {}", e))?;
        model_path = current_dir.join(&obj_path);
        if !model_path.exists() {
            return Err(format!("Model file not found: {}", obj_path));
        }
    }
    
    // Подготавливаем JSON запрос для Python скрипта
    let request = serde_json::json!({
        "tool": "model_tools",
        "op": "unfold_model",
        "payload": {
            "path": obj_path
        }
    });
    
    // Выполняем Python скрипт для развёртки
    let output = Command::new("python")
        .arg(&script_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start Python script: {}", e))?;
    
    // Отправляем JSON запрос в stdin
    let mut stdin = output.stdin.unwrap();
    let request_str = request.to_string();
    use tokio::io::AsyncWriteExt;
    stdin.write_all(request_str.as_bytes()).await
        .map_err(|e| format!("Failed to write to Python script stdin: {}", e))?;
    stdin.flush().await
        .map_err(|e| format!("Failed to flush Python script stdin: {}", e))?;
    drop(stdin); // Закрываем stdin
    
    // Читаем результат из stdout
    let output_result = output.wait_with_output()
        .await
        .map_err(|e| format!("Failed to read Python script output: {}", e))?;
    
    // Проверяем результат выполнения
    if !output_result.status.success() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        return Err(format!("Python script failed: {}", stderr));
    }
    
    // Парсим JSON ответ
    let response: Value = serde_json::from_slice(&output_result.stdout)
        .map_err(|e| format!("Failed to parse Python script output: {}", e))?;
    
    // Проверяем статус результата
    let ok = response["ok"].as_bool().unwrap_or(false);
    if !ok {
        let error = response["error"].as_str().unwrap_or("Unknown error");
        return Err(format!("Python script error: {}", error));
    }
    
    // Извлекаем SVG контент
    let svg_content = response["result"]["svg"].as_str()
        .ok_or("SVG content not found in Python script response")?
        .to_string();
    
    Ok(svg_content)
}

/// Get the current application state
/// 
/// # Arguments
/// * `state` - The application state
/// 
/// # Returns
/// * `AppState` - The current application state
#[tauri::command]
pub fn get_app_state(state: State<AppState>) -> AppState {
    state.inner().clone()
}

/// Update the application state
/// 
/// # Arguments
/// * `state` - The application state
/// * `new_state` - The new application state
#[tauri::command]
pub fn set_app_state(state: State<AppState>, new_state: AppState) {
    *state.inner() = new_state;
}