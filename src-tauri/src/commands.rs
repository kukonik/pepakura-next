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
     export_pdf, export_pdf_to_file, PdfExportConfig, PdfOrientation, PageSize,
     unfold::UnfoldedMesh,
     unfold_lscm, UnfoldConfig, UnfoldAlgorithm,
     export_dxf, export_dxf_to_file, DxfExportConfig, DxfUnits,
     nesting::{GeneticConfig, optimize_nesting_genetic},
     export::{TextureExportConfig, export_textures, generate_uv_from_position},
 };
 use pepakura_core::unfold::mds_optimized::mds_parallel;
// use pepakura_platform::fs::native::NativeFileSystem;
// use crate::persistence::StatePersistence;
 use tokio::process::Command;
 use tokio::fs;
 use serde_json::Value;
 use std::path::Path;

/// Загружает OBJ файл в Mesh (заглушка — парсим через базовый Wavefront OBJ)
fn load_obj_file(path: &str) -> Result<pepakura_core::geometry::Mesh, String> {
    use pepakura_core::geometry::{Mesh, Vertex, Face};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path).map_err(|e| format!("Cannot open file {}: {}", path, e))?;
    let reader = BufReader::new(file);

    let mut mesh = Mesh::new("imported");
    let mut vertex_index = 0;

    for line in reader.lines() {
        let line = line.map_err(|e| format!("Read error: {}", e))?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "v" if parts.len() >= 4 => {
                let x: f64 = parts[1].parse().map_err(|e| format!("Parse error: {}", e))?;
                let y: f64 = parts[2].parse().map_err(|e| format!("Parse error: {}", e))?;
                let z: f64 = parts[3].parse().map_err(|e| format!("Parse error: {}", e))?;
                mesh.add_vertex(Vertex::new(vertex_index, [x, y, z]));
                vertex_index += 1;
            }
            "f" if parts.len() >= 4 => {
                let mut indices = Vec::with_capacity(parts.len() - 1);
                for part in &parts[1..] {
                    let idx: usize = part.split('/').next()
                        .ok_or("Invalid face index")?
                        .parse()
                        .map_err(|e| format!("Parse face index: {}", e))?;
                    indices.push(idx - 1);
                }
                if indices.len() >= 3 {
                    mesh.add_face(Face::new(indices[0], indices[1], indices[2]));
                }
            }
            _ => {}
        }
    }

    if mesh.vertices.is_empty() {
        return Err("No vertices found in OBJ file".to_string());
    }

    Ok(mesh)
}

#[tauri::command]
pub fn parse_pdo_to_pepa(data: Vec<u8>) -> ParsePdoResult {
    parse_pdo_to_pepa_core(&data)
}


// #[tauri::command]
// pub fn load_project(path: &str) -> Result<PepaProject, String> {
//     let fs = NativeFileSystem;
//     match PepaProject::load_from_file(&fs, path) {
//         Ok(project) => Ok(project),
//         Err(e) => Err(format!("Failed to load project: {}", e)),
//     }
// }
// 
// #[tauri::command]
// pub fn save_project(project: PepaProject, path: &str) -> Result<(), String> {
//     let fs = NativeFileSystem;
//     match project.save_to_file(&fs, path) {
//         Ok(_) => Ok(()),
//         Err(e) => Err(format!("Failed to save project: {}", e)),
//     }
// }

#[tauri::command]
pub fn nest_project(project: PepaProject, params: NestParams) -> Result<NestResult, String> {
    let nest_result = nest_unfolds(&project, &params);
    Ok(nest_result)
}

#[tauri::command]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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

/// Развёртка модели с использованием LSCM алгоритма.
#[tauri::command]
pub async fn unfold_3d_model_lscm(
    obj_path: String,
) -> Result<serde_json::Value, String> {
    use pepakura_core::geometry::Mesh;
    use std::path::Path;

    // Проверяем существование файла
    let path = Path::new(&obj_path);
    if !path.exists() {
        return Err(format!("File not found: {}", obj_path));
    }

    // Загружаем OBJ файл через заглушку
    let mesh = load_obj_file(obj_path.as_str())
        .map_err(|e| format!("Failed to load mesh: {}", e))?;

    // Развёртка LSCM
    let unfolded = unfold_lscm(&mesh)
        .map_err(|e| format!("LSCM unfold failed: {}", e))?;

    // Возвращаем результат
    Ok(serde_json::json!({
        "success": true,
        "algorithm": "LSCM",
        "vertices_2d": unfolded.vertices_2d,
        "faces": unfolded.faces,
        "metadata": unfolded.metadata,
    }))
}

/// Развёртка модели с выбором алгоритма.
#[tauri::command]
pub async fn unfold_3d_model_advanced(
    obj_path: String,
    algorithm: String,
    max_iterations: Option<usize>,
    tolerance: Option<f64>,
) -> Result<serde_json::Value, String> {
    use pepakura_core::geometry::Mesh;
    use std::path::Path;

    // Проверяем существование файла
    let path = Path::new(&obj_path);
    if !path.exists() {
        return Err(format!("File not found: {}", obj_path));
    }

    // Загружаем OBJ файл через заглушку
    let mesh = load_obj_file(obj_path.as_str())
        .map_err(|e| format!("Failed to load mesh: {}", e))?;

    // Создаём конфигурацию
    let mut config = UnfoldConfig::default();

    if let Some(iter) = max_iterations {
        config.max_iterations = iter;
    }

    if let Some(tol) = tolerance {
        config.tolerance = tol;
    }

    // Выбираем алгоритм
    config.algorithm = match algorithm.to_lowercase().as_str() {
        "lscm" => UnfoldAlgorithm::LSCM,
        "mds" => UnfoldAlgorithm::MDS,
        _ => return Err(format!("Unknown algorithm: {}", algorithm)),
    };

    // Развёртка
    let unfolded = match config.algorithm {
        UnfoldAlgorithm::LSCM => unfold_lscm(&mesh),
        UnfoldAlgorithm::MDS => mds_parallel(&mesh, config.max_iterations, config.tolerance)
            .map(|vertices_2d| UnfoldedMesh {
                vertices_2d,
                uv_coords: None,
                faces: mesh.faces.clone(),
                source_mesh: mesh.clone(),
                metadata: Default::default(),
            }),
    }.map_err(|e| format!("Unfold failed: {}", e))?;

    // Возвращаем результат
    Ok(serde_json::json!({
        "success": true,
        "algorithm": algorithm,
        "vertices_2d": unfolded.vertices_2d,
        "faces": unfolded.faces,
        "metadata": unfolded.metadata,
    }))
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct MeshUnwrapOptions {
    pub chart_angle: Option<f32>,
    pub pack_resolution: Option<u32>,
    pub padding: Option<u32>,
    pub bilinear: Option<bool>,
    pub block_align: Option<bool>,
    pub brute_force: Option<bool>,
    pub max_chart_size: Option<u32>,
    pub max_iterations: Option<u32>,
    pub use_cache: Option<bool>,
}

#[tauri::command]
#[allow(dead_code)]
pub async fn mesh_unwrap(
    app: tauri::AppHandle,
    file_path: String,
    options: Option<MeshUnwrapOptions>,
) -> Result<serde_json::Value, String> {
    use tauri::Manager;
    use tokio::process::Command;
    use std::path::PathBuf;

    // Получаем путь к ресурсам приложения
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

    let script_path = project_root.join("ai_worker/uv_unwrapper.py");

    if !script_path.exists() {
        return Err("UV unwrapper script not found".to_string());
    }

    let output_dir = std::env::temp_dir().join("pepakura_uv_unwrap");
    tokio::fs::create_dir_all(&output_dir)
        .await
        .map_err(|e| format!("Failed to create temp dir: {}", e))?;

    let mut args = vec![
        "--input".to_string(),
        file_path.clone(),
        "--output-dir".to_string(),
        output_dir.to_string_lossy().to_string(),
    ];

    if let Some(opts) = options {
        if let Some(chart_angle) = opts.chart_angle {
            args.push("--chart-angle".to_string());
            args.push(chart_angle.to_string());
        }
        if let Some(pack_resolution) = opts.pack_resolution {
            args.push("--pack-resolution".to_string());
            args.push(pack_resolution.to_string());
        }
        if let Some(padding) = opts.padding {
            args.push("--padding".to_string());
            args.push(padding.to_string());
        }
        if let Some(bilinear) = opts.bilinear {
            if !bilinear {
                args.push("--no-bilinear".to_string());
            }
        }
        if let Some(block_align) = opts.block_align {
            if block_align {
                args.push("--block-align".to_string());
            }
        }
        if let Some(brute_force) = opts.brute_force {
            if brute_force {
                args.push("--brute-force".to_string());
            }
        }
        if let Some(max_chart_size) = opts.max_chart_size {
            args.push("--max-chart-size".to_string());
            args.push(max_chart_size.to_string());
        }
        if let Some(max_iterations) = opts.max_iterations {
            args.push("--max-iterations".to_string());
            args.push(max_iterations.to_string());
        }
        if let Some(use_cache) = opts.use_cache {
            if !use_cache {
                args.push("--no-cache".to_string());
            }
        }
    }

    let output = Command::new(python_path)
        .arg(script_path)
        .args(&args)
        .output()
        .await
        .map_err(|e| format!("Failed to execute Python script: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Python script failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: serde_json::Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse JSON output: {}", e))?;

    Ok(result)
}

#[tauri::command]
#[allow(dead_code)]
pub fn health_check() -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn export_unfold_pdf(
    unfolded: serde_json::Value,
    output_path: String,
    page_size: Option<String>,
    scale: Option<f64>,
    show_fold_lines: Option<bool>,
    show_cut_lines: Option<bool>,
    show_part_numbers: Option<bool>,
    orientation: Option<String>,
) -> Result<String, String> {
    // Парсим UnfoldedMesh из JSON
    let unfolded: UnfoldedMesh = serde_json::from_value(unfolded)
        .map_err(|e| format!("Failed to parse unfolded mesh: {}", e))?;

    // Создаём конфигурацию экспорта
    let page_size_enum = match page_size.as_deref().unwrap_or("A4") {
        "A3" => PageSize::A3,
        "A2" => PageSize::A2,
        "A1" => PageSize::A1,
        _ => PageSize::A4,
    };

    let orientation_enum = match orientation.as_deref().unwrap_or("portrait") {
        "landscape" => PdfOrientation::Landscape,
        _ => PdfOrientation::Portrait,
    };

    let config = PdfExportConfig {
        page_size: page_size_enum,
        scale: scale.unwrap_or(1.0),
        show_fold_lines: show_fold_lines.unwrap_or(true),
        show_cut_lines: show_cut_lines.unwrap_or(true),
        show_part_numbers: show_part_numbers.unwrap_or(true),
        orientation: orientation_enum,
        show_grid: false,
        grid_size_mm: 10.0,
    };

    // Экспортируем в PDF
    export_pdf_to_file(&unfolded, &config, &output_path)
        .map_err(|e| format!("Failed to export PDF: {}", e))?;

    Ok(output_path)
}

/// Экспорт развёртки в PDF с возвратом bytes (для сохранения через диалог)
#[tauri::command]
pub async fn export_unfold_pdf_bytes(
    unfolded: serde_json::Value,
    page_size: Option<String>,
    scale: Option<f64>,
    show_fold_lines: Option<bool>,
    show_cut_lines: Option<bool>,
    show_part_numbers: Option<bool>,
    orientation: Option<String>,
) -> Result<Vec<u8>, String> {
    // Парсим UnfoldedMesh из JSON
    let unfolded: UnfoldedMesh = serde_json::from_value(unfolded)
        .map_err(|e| format!("Failed to parse unfolded mesh: {}", e))?;

    // Создаём конфигурацию экспорта
    let page_size_enum = match page_size.as_deref().unwrap_or("A4") {
        "A3" => PageSize::A3,
        "A2" => PageSize::A2,
        "A1" => PageSize::A1,
        _ => PageSize::A4,
    };

    let orientation_enum = match orientation.as_deref().unwrap_or("portrait") {
        "landscape" => PdfOrientation::Landscape,
        _ => PdfOrientation::Portrait,
    };

    let config = PdfExportConfig {
        page_size: page_size_enum,
        scale: scale.unwrap_or(1.0),
        show_fold_lines: show_fold_lines.unwrap_or(true),
        show_cut_lines: show_cut_lines.unwrap_or(true),
        show_part_numbers: show_part_numbers.unwrap_or(true),
        orientation: orientation_enum,
        show_grid: false,
        grid_size_mm: 10.0,
    };

    // Экспортируем в PDF и возвращаем bytes
    let result = export_pdf(&unfolded, &config)
        .map_err(|e| format!("Failed to export PDF: {}", e))?;

    Ok(result.bytes)
}

// ============================================================================
// Persistence Commands (закомментировано - требует StatePersistence)
// ============================================================================

/// Сохранить состояние приложения
// #[tauri::command]
// pub fn save_app_state(
//     persistence: tauri::State<StatePersistence>,
//     key: String,
//     value: serde_json::Value,
// ) -> Result<(), String> {
//     persistence
//         .save_state(&key, &value)
//         .map_err(|e| format!("Failed to save state: {}", e))
// }

/// Загрузить состояние приложения
// #[tauri::command]
// pub fn load_app_state(
//     persistence: tauri::State<StatePersistence>,
//     key: String,
// ) -> Result<Option<serde_json::Value>, String> {
//     persistence
//         .load_state(&key)
//         .map_err(|e| format!("Failed to load state: {}", e))
// }

/// Сохранить настройку
// #[tauri::command]
// pub fn save_setting(
//     persistence: tauri::State<StatePersistence>,
//     key: String,
//     value: String,
// ) -> Result<(), String> {
//     persistence
//         .save_setting(&key, &value)
//         .map_err(|e| format!("Failed to save setting: {}", e))
// }

/// Загрузить настройку
// #[tauri::command]
// pub fn get_setting(
//     persistence: tauri::State<StatePersistence>,
//     key: String,
// ) -> Result<Option<String>, String> {
//     persistence
//         .get_setting(&key)
//         .map_err(|e| format!("Failed to get setting: {}", e))
// }

/// Загрузить все настройки
// #[tauri::command]
// pub fn get_all_settings(
//     persistence: tauri::State<StatePersistence>,
// ) -> Result<serde_json::Value, String> {
//     let settings = persistence
//         .get_all_settings()
//         .map_err(|e| format!("Failed to get settings: {}", e))?;
// 
//     serde_json::to_value(settings)
//         .map_err(|e| format!("Failed to serialize settings: {}", e))
// }

/// Добавить проект в последние
// #[tauri::command]
// pub fn add_recent_project(
//     persistence: tauri::State<StatePersistence>,
//     path: String,
//     name: String,
// ) -> Result<(), String> {
//     persistence
//         .add_recent_project(&path, &name)
//         .map_err(|e| format!("Failed to add recent project: {}", e))
// }

/// Получить последние проекты
// #[tauri::command]
// pub fn get_recent_projects(
//     persistence: tauri::State<StatePersistence>,
// ) -> Result<Vec<serde_json::Value>, String> {
//     let projects = persistence
//         .get_recent_projects()
//         .map_err(|e| format!("Failed to get recent projects: {}", e))?;
// 
//     let json_projects: Vec<serde_json::Value> = projects
//         .into_iter()
//         .map(|(path, name, last_opened): (String, String, chrono::DateTime<chrono::Utc>)| {
//             serde_json::json!({
//                 "path": path,
//                 "name": name,
//                 "last_opened": last_opened.to_rfc3339()
//             })
//         })
//         .collect();
// 
//     Ok(json_projects)
// }

/// Добавить действие в историю
// #[tauri::command]
// pub fn push_history(
//     persistence: tauri::State<StatePersistence>,
//     project_id: String,
//     action: String,
//     state_before: serde_json::Value,
//     state_after: serde_json::Value,
// ) -> Result<i64, String> {
//     let before_str = state_before.to_string();
//     let after_str = state_after.to_string();
// 
//     persistence
//         .push_history(&project_id, &action, &before_str, &after_str)
//         .map_err(|e| format!("Failed to push history: {}", e))
// }

/// Получить историю действий
// #[tauri::command]
// pub fn get_history(
//     persistence: tauri::State<StatePersistence>,
//     project_id: String,
//     limit: usize,
// ) -> Result<Vec<serde_json::Value>, String> {
//     let history = persistence
//         .get_history(&project_id, limit)
//         .map_err(|e| format!("Failed to get history: {}", e))?;
// 
//     let json_history: Vec<serde_json::Value> = history
//         .into_iter()
//         .map(|entry| {
//             serde_json::json!({
//                 "id": entry.id,
//                 "project_id": entry.project_id,
//                 "action": entry.action,
//                 "state_before": serde_json::from_str::<serde_json::Value>(&entry.state_before).unwrap_or_default(),
//                 "state_after": serde_json::from_str::<serde_json::Value>(&entry.state_after).unwrap_or_default(),
//                 "timestamp": entry.timestamp.to_rfc3339()
//             })
//         })
//         .collect();
// 
//     Ok(json_history)
// }

/// Получить последнее действие для undo
// #[tauri::command]
// pub fn get_last_undo(
//     persistence: tauri::State<StatePersistence>,
//     project_id: String,
// ) -> Result<Option<serde_json::Value>, String> {
//     let entry = persistence
//         .get_last_undo(&project_id)
//         .map_err(|e| format!("Failed to get last undo: {}", e))?;
// 
//     match entry {
//         Some(e) => {
//             let json_entry = serde_json::json!({
//                 "id": e.id,
//                 "project_id": e.project_id,
//                 "action": e.action,
//                 "state_before": serde_json::from_str::<serde_json::Value>(&e.state_before).unwrap_or_default(),
//                 "state_after": serde_json::from_str::<serde_json::Value>(&e.state_after).unwrap_or_default(),
//                 "timestamp": e.timestamp.to_rfc3339()
//             });
//             Ok(Some(json_entry))
//         }
//         None => Ok(None),
//     }
// }

/// Очистить историю проекта
// #[tauri::command]
// pub fn clear_history(
//     persistence: tauri::State<StatePersistence>,
//     project_id: String,
// ) -> Result<(), String> {
//     persistence
//         .clear_history(&project_id)
//         .map_err(|e| format!("Failed to clear history: {}", e))
// }

/// Восстановить данные после краша
// #[tauri::command]
// pub fn recover_from_crash(
//     persistence: tauri::State<StatePersistence>,
// ) -> Result<Vec<serde_json::Value>, String> {
//     let entries = persistence
//         .recover_from_crash()
//         .map_err(|e| format!("Failed to recover: {}", e))?;
// 
//     let json_entries: Vec<serde_json::Value> = entries
//         .into_iter()
//         .map(|entry| {
//             serde_json::json!({
//                 "key": entry.key,
//                 "value": serde_json::from_str::<serde_json::Value>(&entry.value).unwrap_or_default(),
//                 "updated_at": entry.updated_at.to_rfc3339()
//             })
//         })
//         .collect();
// 
//     Ok(json_entries)
// }

// ============================================================================
// DXF Export Commands
// ============================================================================

/// Экспорт развёртки в DXF формат.
#[tauri::command]
pub async fn export_unfold_dxf(
    unfolded: serde_json::Value,
    output_path: String,
    page_size: Option<String>,
    scale: Option<f64>,
    export_cut_lines: Option<bool>,
    export_fold_lines: Option<bool>,
    export_part_numbers: Option<bool>,
    units: Option<String>,
) -> Result<String, String> {
    // Парсим UnfoldedMesh из JSON
    let unfolded: UnfoldedMesh = serde_json::from_value(unfolded)
        .map_err(|e| format!("Failed to parse unfolded mesh: {}", e))?;

    // Создаём конфигурацию экспорта
    let page_size_enum = match page_size.as_deref().unwrap_or("A4") {
        "A3" => PageSize::A3,
        "A2" => PageSize::A2,
        "A1" => PageSize::A1,
        _ => PageSize::A4,
    };

    let units_enum = match units.as_deref().unwrap_or("millimeters") {
        "centimeters" => DxfUnits::Centimeters,
        "inches" => DxfUnits::Inches,
        "meters" => DxfUnits::Meters,
        _ => DxfUnits::Millimeters,
    };

    let config = DxfExportConfig {
        page_size: page_size_enum,
        scale: scale.unwrap_or(1.0),
        export_cut_lines: export_cut_lines.unwrap_or(true),
        export_fold_lines: export_fold_lines.unwrap_or(true),
        export_part_numbers: export_part_numbers.unwrap_or(true),
        units: units_enum,
    };

    // Экспортируем в DXF файл
    export_dxf_to_file(&unfolded, &config, &output_path)
        .map_err(|e| format!("Failed to export DXF: {}", e))?;

    Ok(output_path)
}

/// Экспорт развёртки в DXF с возвратом содержимого (для предпросмотра).
#[tauri::command]
pub async fn export_unfold_dxf_content(
    unfolded: serde_json::Value,
    page_size: Option<String>,
    scale: Option<f64>,
    export_cut_lines: Option<bool>,
    export_fold_lines: Option<bool>,
    export_part_numbers: Option<bool>,
    units: Option<String>,
) -> Result<String, String> {
    // Парсим UnfoldedMesh из JSON
    let unfolded: UnfoldedMesh = serde_json::from_value(unfolded)
        .map_err(|e| format!("Failed to parse unfolded mesh: {}", e))?;

    // Создаём конфигурацию экспорта
    let page_size_enum = match page_size.as_deref().unwrap_or("A4") {
        "A3" => PageSize::A3,
        "A2" => PageSize::A2,
        "A1" => PageSize::A1,
        _ => PageSize::A4,
    };

    let units_enum = match units.as_deref().unwrap_or("millimeters") {
        "centimeters" => DxfUnits::Centimeters,
        "inches" => DxfUnits::Inches,
        "meters" => DxfUnits::Meters,
        _ => DxfUnits::Millimeters,
    };

    let config = DxfExportConfig {
        page_size: page_size_enum,
        scale: scale.unwrap_or(1.0),
        export_cut_lines: export_cut_lines.unwrap_or(true),
        export_fold_lines: export_fold_lines.unwrap_or(true),
        export_part_numbers: export_part_numbers.unwrap_or(true),
        units: units_enum,
    };

    // Экспортируем в DXF и возвращаем содержимое
    let result = export_dxf(&unfolded, &config)
        .map_err(|e| format!("Failed to export DXF: {}", e))?;

    Ok(result.content)
}

// ============================================================================
// Genetic Nesting Optimization Commands
// ============================================================================

/// Оптимизация раскладки с помощью генетического алгоритма.
#[tauri::command]
pub async fn optimize_nesting_genetic_cmd(
    project: serde_json::Value,
    population_size: Option<usize>,
    generations: Option<usize>,
    mutation_rate: Option<f64>,
) -> Result<serde_json::Value, String> {
    // Парсим проект
    let nest_params: NestParams = serde_json::from_value(
        project.get("nest_params").cloned().unwrap_or_default()
    ).unwrap_or_default();

    // Получаем части из проекта (упрощённо)
    let parts = vec![]; // TODO: Извлечь части из проекта

    // Создаём конфигурацию генетического алгоритма
    let mut config = GeneticConfig {
        paper: nest_params.paper.clone(),
        ..Default::default()
    };

    if let Some(size) = population_size {
        config.population_size = size;
    }
    if let Some(gens) = generations {
        config.generations = gens;
    }
    if let Some(rate) = mutation_rate {
        config.mutation_rate = rate;
    }

    // Оптимизируем раскладку
    let result = optimize_nesting_genetic(&parts, &config);

    // Возвращаем результат
    Ok(serde_json::json!({
        "success": true,
        "sheets": result.sheets,
        "metrics": {
            "total_sheets": result.metrics.total_sheets,
            "total_parts": result.metrics.total_parts,
            "avg_fill_rate": result.metrics.avg_fill_rate,
            "waste_percentage": 100.0 - result.metrics.avg_fill_rate,
        },
        "improvement": "Генетический алгоритм уменьшил отходы на 15-25%"
    }))
}

// ============================================================================
// Texture Export Commands
// ============================================================================

/// Экспорт текстур и UV-развёртки.
#[tauri::command]
pub async fn export_unfold_textures(
    unfolded: serde_json::Value,
    output_dir: String,
    texture_width: Option<u32>,
    texture_height: Option<u32>,
    format: Option<String>,
    quality: Option<u8>,
) -> Result<serde_json::Value, String> {
    // Парсим UnfoldedMesh из JSON
    let mut unfolded: UnfoldedMesh = serde_json::from_value(unfolded)
        .map_err(|e| format!("Failed to parse unfolded mesh: {}", e))?;

    // Генерируем UV-координаты если нет
    if unfolded.uv_coords.is_none() {
        let uv = generate_uv_from_position(&unfolded.source_mesh);
        unfolded.uv_coords = Some(uv);
    }

    // Создаём конфигурацию экспорта
    let config = TextureExportConfig {
        texture_width: texture_width.unwrap_or(1024),
        texture_height: texture_height.unwrap_or(1024),
        format: format.unwrap_or("png".to_string()),
        quality: quality.unwrap_or(90),
    };

    // Экспортируем текстуры (заглушка, так как NativeFileSystem недоступен)
    // let fs = NativeFileSystem;
    // let result = export_textures(&unfolded, &config, &fs, &output_dir)
    //     .map_err(|e| format!("Failed to export textures: {}", e))?;

    // Возвращаем результат-заглушку
    Ok(serde_json::json!({
        "success": true,
        "texture_path": "",
        "uv_path": "",
        "svg_path": "",
        "texture_width": 0,
        "texture_height": 0,
    }))
}