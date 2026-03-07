use pepakura_core::{
     parse_pdo_to_pepa_core,
     ParsePdoResult,
     PepaProject,
     nest_unfolds,
     NestParams,
     PartOverride,
     NestResult,
     export_nest_result_to_svgs,
     apply_overrides_to_nest_result,
     export_sheet_to_svg as core_export_sheet_to_svg,
     pdo_parser::PdoModel,
     pepa_scene_adapter::{PepaScene, FromPdoModel},
 };
 use tokio::process::Command;
 use tokio::fs;
 use serde_json::Value;
 use std::path::Path;

#[tauri::command]
pub fn parse_pdo_to_pepa(data: Vec<u8>) -> ParsePdoResult {
    parse_pdo_to_pepa_core(&data)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Warning {
    pub code: String,
    pub message: String,
    pub part_id: Option<u32>,
    pub severity: Severity,
}

#[tauri::command]
pub fn load_project(path: &str) -> Result<PepaProject, String> {
    match PepaProject::load_from_file(path) {
        Ok(project) => Ok(project),
        Err(e) => Err(format!("Failed to load project: {}", e)),
    }
}

#[tauri::command]
pub fn save_project(project: PepaProject, path: &str) -> Result<(), String> {
    match project.save_to_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to save project: {}", e)),
    }
}

#[tauri::command]
pub fn nest_project(project: PepaProject, params: NestParams) -> Result<NestResult, String> {
    let nest_result = nest_unfolds(&project, &params);
    Ok(nest_result)
}

#[tauri::command]
pub fn export_nest_result_to_svg(
    project: PepaProject,
    params: NestParams,
) -> Result<Vec<String>, String> {
    let nest_result = nest_unfolds(&project, &params);
    let unfold_result = project.get_unfold_result();
    let svgs = export_nest_result_to_svgs(&nest_result, &unfold_result);
    Ok(svgs)
}

#[tauri::command]
pub fn apply_nest_overrides(
    project: PepaProject,
    params: NestParams,
    overrides: Vec<PartOverride>,
) -> Result<NestResult, String> {
    let nest_result = nest_unfolds(&project, &params);
    let adjusted_result = apply_overrides_to_nest_result(&nest_result, &overrides);
    Ok(adjusted_result)
}

// Внутреннее имя функции другое, наружу остаётся export_sheet_to_svg
#[tauri::command(rename = "export_sheet_to_svg")]
pub fn export_sheet_to_svg_cmd(
    project: PepaProject,
    sheet_index: usize,
) -> Result<String, String> {
    let nest_params = project.settings.nest_params.clone().unwrap_or_default();
    let nest_result = nest_unfolds(&project, &nest_params);

    if sheet_index >= nest_result.sheets.len() {
        return Err(format!("Sheet index {} is out of range", sheet_index));
    }

    let unfold_result = project.get_unfold_result();
    let sheet = &nest_result.sheets[sheet_index];
    let svg = core_export_sheet_to_svg(sheet, &unfold_result);
    Ok(svg)
}

#[tauri::command]
pub async fn import_3d_model(file_path: String) -> Result<serde_json::Value, String> {
    // Проверяем, что файл существует
    if !Path::new(&file_path).exists() {
        return Err(format!("File not found: {}", file_path));
    }
    
    // Получаем расширение файла
    let extension = Path::new(&file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    // Проверяем поддерживаемые форматы
    let supported_formats = ["obj", "stl", "ply"];
    if !supported_formats.contains(&extension.as_str()) {
        return Err(format!("Unsupported file format: {}. Supported formats: {:?}", extension, supported_formats));
    }
    
    // Возвращаем информацию о модели
    let result = serde_json::json!({
        "success": true,
        "modelPath": file_path,
        "info": {
            "format": extension,
            "name": Path::new(&file_path).file_stem().and_then(|s| s.to_str()).unwrap_or("model"),
            "vertices": 0,  // Будет заполнено позже
            "faces": 0,     // Будет заполнено позже
            "edges": 0,     // Будет заполнено позже
            "surfaceArea": 0.0,  // Будет заполнено позже
            "volume": 0.0        // Будет заполнено позже
        }
    });
    
    Ok(result)
}

#[tauri::command]
pub async fn unfold_3d_model(obj_path: String) -> Result<String, String> {
    // Получаем путь к ресурсам приложения
    let resource_path = std::env::current_exe()
        .map_err(|e| format!("Failed to get executable path: {}", e))?
        .parent()
        .ok_or("Failed to get parent directory")?
        .to_path_buf();
    
    // Определяем путь к Python скрипту
    let script_path = resource_path.join("addon_server.py");
    
    // Проверяем существование скрипта
    if !script_path.exists() {
        return Err(format!("Python script not found at: {:?}", script_path));
    }
    
    // Создаем временные пути для SVG файла
    let output_svg_path = std::env::temp_dir().join("unfolded_model.svg");
    
    // Выполняем Python скрипт для развёртки
    let output = Command::new("python")
        .arg(&script_path)
        .arg(&obj_path)
        .arg(&output_svg_path)
        .output()
        .await
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;
    
    // Проверяем результат выполнения
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python script failed: {}", stderr));
    }
    
    // Читаем результат из stdout
    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse Python script output: {}", e))?;
    
    // Проверяем статус результата
    let status = result["status"].as_str().unwrap_or("error");
    if status != "success" {
        let message = result["message"].as_str().unwrap_or("Unknown error");
        return Err(format!("Python script error: {}", message));
    }
    
    // Добавляем логирование для отладки
    println!("Unfold result: {:?}", result);
    
    // Проверяем, что SVG файл был создан
    if !output_svg_path.exists() {
        return Err(format!("SVG file was not created at: {:?}", output_svg_path));
    }
    
    // Читаем содержимое SVG файла
    let svg_content = fs::read_to_string(&output_svg_path)
        .await
        .map_err(|e| format!("Failed to read SVG file: {}", e))?;
    
    // Дополнительное логирование содержимого SVG
    println!("SVG content length: {}", svg_content.len());
    
    Ok(svg_content)
}

#[tauri::command]
pub fn import_pdo(file_path: String) -> Result<PepaScene, String> {
    let data = std::fs::read(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let model = PdoModel::parse_from_bytes(&data)
        .map_err(|e| format!("Failed to parse PDO: {}", e))?;
    
    let scene = PepaScene::from_pdo_model(&model);
    Ok(scene)
}

#[tauri::command]
pub fn health_check() -> Result<(), String> {
    Ok(())
}