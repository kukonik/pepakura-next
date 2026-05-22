// Stub commands for Pepakura Next (Final Fix)
use tauri::State;
use std::sync::{Mutex, OnceLock};
use serde_json::Value;

// --- Basic State ---

pub struct AppState {
    pub value: Mutex<String>,
}

// --- Core Functions ---

#[tauri::command]
pub fn load_model(path: String) -> Result<String, String> {
    println!("(Stub) Loading model: {}", path);
    Ok("Model Loaded".into())
}

#[tauri::command]
pub fn load_real_obj() -> Result<String, String> {
    println!("(Stub) load_real_obj called");
    Ok("Obj Loaded".into())
}

// Исправлено: unused variable warning (obj_data -> _obj_data)
#[tauri::command]
pub fn unfold_3d_model(_obj_data: String) -> Result<Value, String> {
    let result = serde_json::json!({
        "vertices_2d": [0.0, 0.0, 100.0, 0.0, 100.0, 100.0],
        "faces": [[0, 1, 2]],
        "error_msg": null
    });
    Ok(result)
}

#[tauri::command]
pub fn unfold_lscm(_mesh_data: String, _config: String) -> Result<Value, String> {
    let result = serde_json::json!({
        "vertices_2d": [10.0, 10.0, 50.0, 10.0, 50.0, 50.0, 10.0, 50.0],
        "faces": [[0, 1, 2, 3]],
        "error_msg": null
    });
    Ok(result)
}

// --- AI & Workflow ---

// Недостающая функция 1
#[tauri::command]
pub async fn generate_and_unfold(_prompt: String) -> Result<Value, String> {
    println!("(Stub) generate_and_unfold");
    Ok(serde_json::json!({ "status": "mock_generated" }))
}

// --- Export Functions ---

#[tauri::command]
pub fn export_svg() -> Result<String, String> { Ok("SVG Export Stub".into()) }
#[tauri::command]
pub fn export_png() -> Result<String, String> { Ok("PNG Export Stub".into()) }
#[tauri::command]
pub fn export_jpg() -> Result<String, String> { Ok("JPG Export Stub".into()) }
#[tauri::command]
pub fn export_obj() -> Result<String, String> { Ok("OBJ Export Stub".into()) }
#[tauri::command]
pub fn export_stl() -> Result<String, String> { Ok("STL Export Stub".into()) }
#[tauri::command]
pub fn export_unfold_pdf() -> Result<String, String> { Ok("PDF Export Stub".into()) }

// --- State & Settings ---

#[tauri::command]
pub fn get_app_state(state: State<AppState>) -> Result<String, String> {
    Ok(state.value.lock().unwrap().clone())
}

#[tauri::command]
pub fn set_app_state(val: String, state: State<AppState>) -> Result<(), String> {
    *state.value.lock().unwrap() = val;
    Ok(())
}

#[tauri::command]
pub fn get_app_version() -> String { "0.2.0 Alpha".to_string() }
#[tauri::command]
pub fn get_settings() -> Result<String, String> { Ok("{}".into()) }
#[tauri::command]
pub fn save_settings() -> Result<(), String> { Ok(()) }
#[tauri::command]
pub fn reset_settings() -> Result<(), String> { Ok(()) }

// --- System / Health ---

#[tauri::command]
pub fn health_check() -> String { "OK".to_string() }
#[tauri::command]
pub fn ping_pong() -> String { "Pong".to_string() }
#[tauri::command]
pub fn open_dialog() -> Result<String, String> { Ok("Dialog Stub".into()) }

// --- Project Management ---

#[tauri::command]
pub fn create_project() -> Result<(), String> { Ok(()) }
#[tauri::command]
pub fn import_model() -> Result<(), String> { Ok(()) }
#[tauri::command]
pub fn unfold_mesh() -> Result<(), String> { Ok(()) }
#[tauri::command]
pub fn get_recent_projects() -> Result<Vec<String>, String> { Ok(vec![]) }
#[tauri::command]
pub fn delete_project() -> Result<(), String> { Ok(()) }

// --- AI (Stubs) ---

#[tauri::command]
pub async fn ai_generate_from_image(_img: Vec<u8>) -> Result<String, String> { Ok("AI Stub".into()) }
#[tauri::command]
pub async fn ai_generate_from_text(_text: String) -> Result<String, String> { Ok("AI Stub".into()) }
#[tauri::command]
pub fn nest_parts() -> Result<(), String> { Ok(()) }

// --- Mock / Debug ---

#[tauri::command]
pub fn parse_mock_obj() -> Result<String, String> { Ok("Mock Obj".into()) }
#[tauri::command]
pub fn start_mock_unfold() -> Result<(), String> { Ok(()) }

// --- Store / API Key ---

static API_KEY: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_api_key_storage() -> &'static Mutex<Option<String>> {
    API_KEY.get_or_init(|| Mutex::new(None))
}

#[tauri::command]
pub fn save_api_key(key: String) -> Result<(), String> {
    let storage = get_api_key_storage();
    let mut api_key = storage.lock().unwrap();
    *api_key = Some(key);
    Ok(())
}

#[tauri::command]
pub fn get_api_key() -> Result<Option<String>, String> {
    let storage = get_api_key_storage();
    let api_key = storage.lock().unwrap();
    Ok(api_key.clone())
}

// Недостающая функция 2
#[tauri::command]
pub fn has_api_key() -> bool {
    let storage = get_api_key_storage();
    let api_key = storage.lock().unwrap();
    api_key.is_some()
}
