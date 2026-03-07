
use commands::parse_pdo_to_pepa;
mod model_generator;
mod ai_service;

use std::collections::HashMap;
use ai_service::{AIService, GeneratedModel};
use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(serde::Serialize)]
struct GenerationResponse {
    model_id: String,
    vertices: usize,
    faces: usize,
    status: String,
    model_type: String,
    generation_method: String,
    processing_time_ms: u64,
}

static AI_SERVICE: Lazy<Mutex<Option<AIService>>> = Lazy::new(|| Mutex::new(None));

#[tauri::command]
fn generate_3d_model(description: String) -> Result<GeneratedModel, String> {
    if description.trim().is_empty() {
        return Err("Description cannot be empty".to_string());
    }

    // Initialize service if needed
    {
        let mut guard = AI_SERVICE.lock().unwrap();
        if guard.is_none() {
            *guard = Some(AIService::new());
        }
    }

    // Use service
    let guard = AI_SERVICE.lock().unwrap();
    let service = guard.as_ref().unwrap();
    service.generate_3d_model(&description)
}

#[tauri::command]
fn save_model(model_data: HashMap<String, serde_json::Value>) -> Result<(), String> {
    println!("Saving model: {:?}", model_data);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_3d_model,
            save_model
        , parse_pdo_to_pepa])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

