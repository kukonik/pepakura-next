use ai_bridge::{AiBridge, GenerationRequest};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use tokio::fs;

#[derive(Serialize, Deserialize)]
pub struct ImageTo3DPayload {
    pub image_path: String,
    #[serde(rename = "quality")]
    pub quality_param: Option<String>,
}

#[tauri::command]
pub async fn start_image_to_3d_generation(
    app: AppHandle,
    payload: ImageTo3DPayload,
) -> Result<ai_bridge::GenerationResponse, String> {
    let project_root = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to resolve resource dir: {}", e))?;

    let python_path = if cfg!(windows) {
        project_root.join("ai_worker/venv/Scripts/python.exe")
    } else {
        project_root.join("ai_worker/venv/bin/python")
    };

    let python_path = if python_path.exists() {
        python_path
    } else {
        PathBuf::from("python")
    };

    let script_path = project_root.join("ai_worker/triposr_runner.py");

    if !script_path.exists() {
        return Err("AI script not found".to_string());
    }

    let output_dir = std::env::temp_dir().join("pepakura_gen");

    fs::create_dir_all(&output_dir)
        .await
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;

    let bridge = AiBridge::new(python_path, script_path);

    let req = GenerationRequest {
        image_path: payload.image_path,
        output_dir: output_dir.to_string_lossy().to_string(),
        quality: payload.quality_param.unwrap_or("balanced".to_string()),
    };

    bridge.generate(req).await.map_err(|e| e.to_string())
}
