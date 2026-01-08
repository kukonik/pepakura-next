#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};
use tauri_plugin_dialog::DialogExt;

use printpdf::{
  Color, Line, LineCapStyle, LineJoinStyle, Mm, PdfDocument, PdfLayerReference, Pt,
};

/// Тип линии на развёртке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineKind {
  Cut,
  Valley,
  Mountain,
  GlueTab,
}

/// Отрезок линии в координатах листа, миллиметры.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line2D {
  pub x1: f32,
  pub y1: f32,
  pub x2: f32,
  pub y2: f32,
  pub kind: LineKind,
}

/// Прямоугольный бокс детали на листе, миллиметры.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rect {
  pub x: f32,
  pub y: f32,
  pub width: f32,
  pub height: f32,
}

/// Деталь на листе.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Part2D {
  pub id: u32,
  pub name: Option<String>,
  pub bounds: Rect,
  pub lines: Vec<Line2D>,
}

/// Лист бумаги с деталями.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
  pub id: u32,
  pub index: u32,
  pub width_mm: f32,
  pub height_mm: f32,
  pub margin_mm: f32,
  pub parts: Vec<Part2D>,
}

/// Результат развёртки: набор листов.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldResult {
  pub sheets: Vec<Sheet>,
}

/// Параметры бумаги и развёртки, приходящие с фронта.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldParams {
  pub paper_format: String, // "A4" | "A3" | "Letter" и т.п.
  pub margin_mm: f32,
  pub max_sheets: u32,
  pub scale: f32,
}

/// Простая заглушка движка развёртки: делает фиктивные листы,
/// но в финальном формате UnfoldResult.
fn compute_unfold_stub(_model_path: &str, params: &UnfoldParams) -> UnfoldResult {
  let (w, h) = match params.paper_format.as_str() {
    "A3" => (297.0_f32, 420.0_f32),
    "Letter" => (215.9_f32, 279.4_f32),
    _ => (210.0_f32, 297.0_f32),
  };

  let sheet_count = params.max_sheets.min(4).max(1);
  let mut sheets = Vec::new();

  for i in 0..sheet_count {
    let id = i + 1;
    let mut parts = Vec::new();

    let part_count = 6 + (i as u32) * 2;
    for j in 0..part_count {
      let part_id = j + 1;
      let bounds = Rect {
        x: 10.0 + (j as f32 * 8.0) % (w * 0.6),
        y: 10.0 + (j as f32 * 12.0) % (h * 0.6),
        width: 30.0 + (j % 3) as f32 * 5.0,
        height: 20.0 + (j % 2) as f32 * 5.0,
      };

      let x1 = bounds.x;
      let y1 = bounds.y;
      let x2 = bounds.x + bounds.width;
      let y2 = bounds.y + bounds.height;

      let lines = vec![
        Line2D {
          x1,
          y1,
          x2,
          y2: y1,
          kind: LineKind::Cut,
        },
        Line2D {
          x1: x2,
          y1,
          x2,
          y2,
          kind: LineKind::Cut,
        },
        Line2D {
          x1: x2,
          y1: y2,
          x2: x1,
          y2,
          kind: LineKind::Cut,
        },
        Line2D {
          x1,
          y1: y2,
          x2: x1,
          y2: y1,
          kind: LineKind::Cut,
        },
      ];

      parts.push(Part2D {
        id: part_id,
        name: Some(format!("Part {}-{}", id, part_id)),
        bounds,
        lines,
      });
    }

    sheets.push(Sheet {
      id,
      index: id,
      width_mm: w,
      height_mm: h,
      margin_mm: params.margin_mm,
      parts,
    });
  }

  UnfoldResult { sheets }
}

#[tauri::command]
async fn import_3d_model(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
  use tauri_plugin_dialog::DialogExt;

  println!("[PepakuraNext] import_3d_model called");

  let (tx, rx) = std::sync::mpsc::channel::<Option<String>>();

  app_handle
    .dialog()
    .file()
    .set_title("Импорт 3D модели")
    .add_filter("3D Models", &["obj", "stl", "gltf", "glb"])
    .add_filter("Все файлы", &["*"])
    .pick_file(move |file_path| {
      let path_str = file_path.map(|p| p.to_string());
      tx.send(path_str).ok();
    });

  let file_opt = rx.recv().map_err(|e| e.to_string())?;
  let Some(path) = file_opt else {
    return Err("Импорт отменён пользователем".into());
  };

  println!("[PepakuraNext] 3D model selected: {}", path);

  Ok(serde_json::json!({
    "path": path,
    "format": "auto"
  }))
}

#[tauri::command]
async fn save_project(
  app_handle: tauri::AppHandle,
  project: serde_json::Value,
) -> Result<(), String> {
  let (tx, rx) = std::sync::mpsc::channel::<Option<String>>();

  app_handle
    .dialog()
    .file()
    .set_title("Сохранить проект Pepakura Next")
    .add_filter("Pepakura Next Project", &["pnx"])
    .add_filter("JSON", &["json"])
    .save_file(move |file_path| {
      let path_str = file_path.map(|p| p.to_string());
      tx.send(path_str).ok();
    });

  let file_opt = rx.recv().map_err(|e| e.to_string())?;
  let Some(path) = file_opt else {
    return Err("Сохранение отменено пользователем".into());
  };

  let pretty = serde_json::to_string_pretty(&project).map_err(|e| e.to_string())?;
  fs::write(&path, pretty).map_err(|e| e.to_string())?;

  println!("[PepakuraNext] Project saved to {}", path);
  Ok(())
}

#[tauri::command]
async fn load_project(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
  let (tx, rx) = std::sync::mpsc::channel::<Option<String>>();

  app_handle
    .dialog()
    .file()
    .set_title("Открыть проект Pepakura Next")
    .add_filter("Pepakura Next Project", &["pnx"])
    .add_filter("JSON", &["json"])
    .pick_file(move |file_path| {
      let path_str = file_path.map(|p| p.to_string());
      tx.send(path_str).ok();
    });

  let file_opt = rx.recv().map_err(|e| e.to_string())?;
  let Some(path) = file_opt else {
    return Err("Открытие отменено пользователем".into());
  };

  let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
  let json: serde_json::Value = serde_json::from_str(&data)
    .map_err(|e| format!("Ошибка парсинга JSON: {e}"))?;

  println!("[PepakuraNext] Project loaded from {}", path);
  Ok(json)
}

#[tauri::command]
async fn run_addon(
  tool: String,
  op: String,
  payload: serde_json::Value,
) -> Result<serde_json::Value, String> {
  let mut child = Command::new("D:/Dev/pepakura-next/addons/.venv/Scripts/python.exe")
    .arg("addon_server.py")
    .current_dir("D:/Dev/pepakura-next/addons")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    .map_err(|e| format!("failed to spawn addon server: {e}"))?;

  let req = serde_json::json!({
    "tool": tool,
    "op": op,
    "payload": payload
  });

  {
    let stdin = child.stdin.as_mut().ok_or("failed to open stdin")?;
    let data = serde_json::to_vec(&req).map_err(|e| e.to_string())?;
    stdin.write_all(&data).map_err(|e| e.to_string())?;
  }

  let output = child.wait_with_output().map_err(|e| e.to_string())?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr);
    return Err(format!("addon server failed: {stderr}"));
  }

  let resp_str = String::from_utf8_lossy(&output.stdout);
  let resp_json: serde_json::Value = serde_json::from_str(&resp_str)
    .map_err(|e| format!("invalid json from addon: {e}, raw={resp_str}"))?;

  Ok(resp_json)
}

#[tauri::command]
async fn read_model_file(path: String) -> Result<Vec<u8>, String> {
  use std::io::Read;

  let mut file = std::fs::File::open(&path).map_err(|e| e.to_string())?;
  let mut buf = Vec::new();
  file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
  println!("[PepakuraNext] read_model_file: {} ({} bytes)", path, buf.len());
  Ok(buf)
}

#[tauri::command]
async fn unfold_3d_model(
  model_path: String,
  params: UnfoldParams,
) -> Result<UnfoldResult, String> {
  println!(
    "[PepakuraNext] unfold_3d_model called: path={}, format={}, margin={}mm, max_sheets={}, scale={}",
    model_path,
    params.paper_format,
    params.margin_mm,
    params.max_sheets,
    params.scale
  );

  let result = compute_unfold_stub(&model_path, &params);
  Ok(result)
}

/// Нарисовать все линии одного листа на заданном PDF‑слое.
/// Координаты: UnfoldResult использует миллиметры с началом в левом верхнем углу,
/// а printpdf — пункты с началом в левом нижнем, поэтому инвертируем Y.
fn draw_sheet_on_layer(sheet: &Sheet, layer: &PdfLayerReference, color_cut: Color) {
  let page_width_mm = sheet.width_mm as f64;
  let page_height_mm = sheet.height_mm as f64;

  for part in &sheet.parts {
    for line in &part.lines {
      // Выбор цвета и толщины по типу линии.
      let (color, width_pt) = match line.kind {
        LineKind::Cut => (color_cut, 0.8),
        LineKind::Valley => (Color::Rgb(0.2, 0.6, 1.0, None), 0.4),
        LineKind::Mountain => (Color::Rgb(1.0, 0.6, 0.2, None), 0.4),
        LineKind::GlueTab => (Color::Rgb(0.4, 1.0, 0.4, None), 0.5),
      };

      let x1_mm = line.x1 as f64;
      let y1_mm = line.y1 as f64;
      let x2_mm = line.x2 as f64;
      let y2_mm = line.y2 as f64;

      // Инвертируем Y: 0 сверху → 0 снизу.
      let y1_mm_pdf = page_height_mm - y1_mm;
      let y2_mm_pdf = page_height_mm - y2_mm;

      let start = (Mm(x1_mm), Mm(y1_mm_pdf));
      let end = (Mm(x2_mm), Mm(y2_mm_pdf));

      let line_vec = Line {
        points: vec![(start, false), (end, false)],
        is_closed: false,
        has_fill: false,
        has_stroke: true,
        is_clipping_path: false,
      };

      layer.set_outline_color(color);
      layer.set_outline_thickness(Pt(width_pt));
      layer.set_line_cap_style(LineCapStyle::Round);
      layer.set_line_join_style(LineJoinStyle::Round);

      layer.add_line(line_vec);
    }
  }
}

#[tauri::command]
async fn export_unfold_pdf(
  app_handle: tauri::AppHandle,
  unfold: UnfoldResult,
) -> Result<(), String> {
  if unfold.sheets.is_empty() {
    return Err("Нет листов для экспорта. Выполните развёртку перед сохранением PDF.".into());
  }

  let (tx, rx) = std::sync::mpsc::channel::<Option<String>>();

  app_handle
    .dialog()
    .file()
    .set_title("Сохранить PDF развёртки")
    .add_filter("PDF", &["pdf"])
    .save_file(move |file_path| {
      let path_str = file_path.map(|p| p.to_string());
      tx.send(path_str).ok();
    });

  let file_opt = rx.recv().map_err(|e| e.to_string())?;
  let Some(path) = file_opt else {
    return Err("Сохранение PDF отменено пользователем".into());
  };

  println!("[PepakuraNext] export_unfold_pdf: {}", path);

  // Используем размер первого листа как базу, остальные страницы будут
  // со своими размерами.
  let first_sheet = &unfold.sheets[0];
  let doc_title = "Pepakura Next Unfold";
  let (mut doc, page1, layer1) = PdfDocument::new(
    doc_title,
    Mm(first_sheet.width_mm as f64),
    Mm(first_sheet.height_mm as f64),
    "Layer 1",
  );

  let color_cut = Color::Rgb(0.0, 0.0, 0.0, None);

  // Первый лист на первой странице.
  {
    let layer = doc.get_page(page1).get_layer(layer1);
    draw_sheet_on_layer(first_sheet, &layer, color_cut);
  }

  // Остальные листы — отдельные страницы.
  for sheet in unfold.sheets.iter().skip(1) {
    let (page, layer) = doc.add_page(
      Mm(sheet.width_mm as f64),
      Mm(sheet.height_mm as f64),
      format!("Layer {}", sheet.index),
    );
    let layer_ref = doc.get_page(page).get_layer(layer);
    draw_sheet_on_layer(sheet, &layer_ref, color_cut);
  }

  doc
    .save(&mut fs::File::create(&path).map_err(|e| e.to_string())?)
    .map_err(|e| e.to_string())?;

  println!("[PepakuraNext] PDF exported to {}", path);
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
      import_3d_model,
      save_project,
      load_project,
      run_addon,
      read_model_file,
      unfold_3d_model,
      export_unfold_pdf
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
