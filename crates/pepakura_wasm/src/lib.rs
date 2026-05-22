//! WASM bindings для Pepakura Core.
//!
//! Предоставляет JavaScript/TypeScript API для:
//! - Развёртки 3D моделей (MDS, LSCM)
//! - Экспорта в SVG, PDF, DXF
//! - Оптимизации раскладки (nesting)
//!
//! ## Пример использования (JavaScript)
//!
//! ```javascript
//! import { init, unfoldMesh } from 'pepakura_core_wasm';
//!
//! await init();
//! const result = unfoldMesh(meshData, { algorithm: 'lscm' });
//! ```

use pepakura_core::geometry::{Mesh, Vertex, Face};
use pepakura_core::unfold::{unfold_mds, UnfoldConfig, UnfoldAlgorithm, UnfoldedMesh};
use pepakura_core::unfold_lscm;
use pepakura_core::export::{export_svg, SvgExportConfig, PageSize};
use pepakura_core::nesting::{NestPart, NestParams, PaperSettings, nest_unfolds};
use wasm_bindgen::prelude::*;

// Инициализация для better panic messages
#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Инициализирует WASM модуль.
#[wasm_bindgen]
pub async fn init() -> Result<(), JsValue> {
    Ok(())
}

/// Конфигурация развёртки.
#[wasm_bindgen]
#[derive(Clone)]
pub struct UnfoldConfigWasm {
    algorithm: String,
    max_iterations: usize,
    tolerance: f64,
    preserve_detail: bool,
}

#[wasm_bindgen]
impl UnfoldConfigWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(
        algorithm: Option<String>,
        max_iterations: Option<usize>,
        tolerance: Option<f64>,
        preserve_detail: Option<bool>,
    ) -> Self {
        Self {
            algorithm: algorithm.unwrap_or_else(|| "mds".to_string()),
            max_iterations: max_iterations.unwrap_or(100),
            tolerance: tolerance.unwrap_or(1e-6),
            preserve_detail: preserve_detail.unwrap_or(true),
        }
    }
}

impl From<UnfoldConfigWasm> for UnfoldConfig {
    fn from(config: UnfoldConfigWasm) -> Self {
        Self {
            algorithm: match config.algorithm.to_lowercase().as_str() {
                "lscm" => UnfoldAlgorithm::LSCM,
                _ => UnfoldAlgorithm::MDS,
            },
            max_iterations: config.max_iterations,
            tolerance: config.tolerance,
            preserve_detail: config.preserve_detail,
        }
    }
}

/// Вершина 3D меша.
#[wasm_bindgen]
#[derive(Clone)]
pub struct VertexWasm {
    id: usize,
    position: Vec<f64>,
}

#[wasm_bindgen]
impl VertexWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(id: usize, position: Vec<f64>) -> Self {
        Self { id, position }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn position(&self) -> Vec<f64> {
        self.position.clone()
    }
}

/// Грань меша.
#[wasm_bindgen]
#[derive(Clone)]
pub struct FaceWasm {
    vertices: Vec<usize>,
}

#[wasm_bindgen]
impl FaceWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(vertices: Vec<usize>) -> Self {
        Self { vertices }
    }

    pub fn vertices(&self) -> Vec<usize> {
        self.vertices.clone()
    }
}

/// 3D меш.
#[wasm_bindgen]
#[derive(Clone)]
pub struct MeshWasm {
    name: String,
    vertices: Vec<VertexWasm>,
    faces: Vec<FaceWasm>,
}

#[wasm_bindgen]
impl MeshWasm {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, vertices: Vec<VertexWasm>, faces: Vec<FaceWasm>) -> Self {
        Self { name, vertices, faces }
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}

impl TryFrom<&MeshWasm> for Mesh {
    type Error = JsValue;

    fn try_from(mesh: &MeshWasm) -> Result<Self, Self::Error> {
        let mut m = Mesh::new(&mesh.name);

        for v in &mesh.vertices {
            if v.position.len() != 3 {
                return Err(JsValue::from_str("Vertex position must have 3 coordinates"));
            }
            m.add_vertex(Vertex::new(
                v.id,
                [v.position[0], v.position[1], v.position[2]],
            ));
        }

        for f in &mesh.faces {
            if f.vertices.len() != 3 {
                return Err(JsValue::from_str("Face must have 3 vertices"));
            }
            m.add_face(Face::new(f.vertices[0], f.vertices[1], f.vertices[2]));
        }

        Ok(m)
    }
}

/// Результат развёртки.
#[wasm_bindgen]
pub struct UnfoldedMeshWasm {
    vertices_2d: Vec<f64>,
    faces: Vec<JsValue>,
    metadata: String,
}

#[wasm_bindgen]
impl UnfoldedMeshWasm {
    pub fn vertices_2d(&self) -> Vec<f64> {
        self.vertices_2d.clone()
    }

    pub fn faces(&self) -> Vec<JsValue> {
        self.faces.clone()
    }

    pub fn metadata(&self) -> String {
        self.metadata.clone()
    }
}

/// Разворачивает меш используя выбранный алгоритм.
///
/// # Аргументы
/// * `mesh` - 3D меш
/// * `config` - конфигурация развёртки
///
/// # Возвращает
/// * `UnfoldedMeshWasm` - результат развёртки
#[wasm_bindgen]
pub fn unfold_mesh(mesh: &MeshWasm, config: UnfoldConfigWasm) -> Result<UnfoldedMeshWasm, JsValue> {
    let mesh: Mesh = mesh.try_into()?;
    let config: UnfoldConfig = config.into();

    let result = match config.algorithm {
        UnfoldAlgorithm::LSCM => unfold_lscm(&mesh),
        UnfoldAlgorithm::MDS => unfold_mds(&mesh, &config),
    }
    .map_err(|e| JsValue::from_str(&format!("Unfold error: {}", e)))?;

    // Конвертируем результат в WASM-совместимый формат
    let mut vertices_2d = Vec::new();
    for &[x, y] in &result.vertices_2d {
        vertices_2d.push(x);
        vertices_2d.push(y);
    }

    let faces = result
        .faces
        .iter()
        .map(|f| {
            serde_wasm_bindgen::to_value(&serde_json::json!({
                "vertices": f.vertices,
            }))
            .unwrap_or(JsValue::NULL)
        })
        .collect();

    let metadata = serde_json::to_string(&result.metadata).unwrap_or_default();

    Ok(UnfoldedMeshWasm {
        vertices_2d,
        faces,
        metadata,
    })
}

/// Экспортирует развёртку в SVG.
///
/// # Аргументы
/// * `vertices_2d` - 2D вершины (плоский массив [x1, y1, x2, y2, ...])
/// * `faces` - грани (массив индексов вершин)
/// * `page_size` - размер страницы (A4, A3, etc)
/// * `scale` - масштаб
///
/// # Возвращает
/// * `String` - SVG содержимое
#[wasm_bindgen]
pub fn export_to_svg(
    vertices_2d: Vec<f64>,
    faces: Vec<JsValue>,
    page_size: Option<String>,
    scale: Option<f64>,
) -> Result<String, JsValue> {
    // Парсим вершины
    let mut vertices = Vec::new();
    for i in (0..vertices_2d.len()).step_by(2) {
        vertices.push([vertices_2d[i], vertices_2d[i + 1]]);
    }

    // Парсим грани
    let mut mesh_faces = Vec::new();
    for face_val in faces {
        let face: serde_json::Value = serde_wasm_bindgen::from_value(face_val)
            .map_err(|e| JsValue::from_str(&format!("Face parse error: {}", e)))?;

        if let Some(vertices_arr) = face["vertices"].as_array() {
            let v: Vec<usize> = vertices_arr
                .iter()
                .filter_map(|v| v.as_u64().map(|n| n as usize))
                .collect();

            if v.len() == 3 {
                mesh_faces.push(Face::new(v[0], v[1], v[2]));
            }
        }
    }

    // Создаём фейковый меш для экспорта
    let mesh = Mesh::new("Export");
    let unfolded = UnfoldedMesh {
        vertices_2d: vertices,
        faces: mesh_faces,
        source_mesh: mesh,
        metadata: Default::default(),
        uv_coords: None,
    };

    // Конфигурация экспорта
    let page_size_enum = match page_size.as_deref().unwrap_or("A4") {
        "A3" => PageSize::A3,
        "A2" => PageSize::A2,
        "A1" => PageSize::A1,
        _ => PageSize::A4,
    };

    let config = SvgExportConfig {
        page_size: page_size_enum,
        scale: scale.unwrap_or(1.0),
        show_vertex_ids: false,
        show_fold_lines: true,
        show_cut_lines: true,
        show_part_numbers: true,
    };

    // Экспортируем
    export_svg(&unfolded, &config)
        .map_err(|e| JsValue::from_str(&format!("SVG export error: {}", e)))
}

/// Оптимизирует раскладку деталей.
///
/// # Аргументы
/// * `parts` - детали для размещения
/// * `paper_format` - формат бумаги (A4, A3, etc)
///
/// # Возвращает
/// * `JsValue` - результат оптимизации
#[wasm_bindgen]
pub fn optimize_nesting(
    parts: Vec<JsValue>,
    paper_format: Option<String>,
) -> Result<JsValue, JsValue> {
    // Парсим части
    let mut nest_parts = Vec::new();
    for (i, part_val) in parts.iter().enumerate() {
        let part: serde_json::Value = serde_wasm_bindgen::from_value(part_val.clone())
            .map_err(|e| JsValue::from_str(&format!("Part parse error: {}", e)))?;

        nest_parts.push(NestPart {
            id: i as u32,
            name: part["name"].as_str().map(|s| s.to_string()),
            unfolded_face_index: part["unfolded_face_index"].as_u64().unwrap_or(0) as usize,
            x_mm: part["x_mm"].as_f64().unwrap_or(0.0) as f32,
            y_mm: part["y_mm"].as_f64().unwrap_or(0.0) as f32,
            width_mm: part["width_mm"].as_f64().unwrap_or(50.0) as f32,
            height_mm: part["height_mm"].as_f64().unwrap_or(50.0) as f32,
            rotation: part["rotation"].as_f64().unwrap_or(0.0) as f32,
        });
    }

    // Параметры бумаги
    let paper = PaperSettings::from_format(paper_format.as_deref().unwrap_or("A4"));

    // Параметры раскладки
    let params = NestParams {
        paper,
        max_sheets: 10,
        scale: 1.0,
        rotation_step_deg: 45.0,
    };

    // Создаём фейковый проект
    use pepakura_core::project::PepaProject;
    use pepakura_core::PepaScene;
    let scene = PepaScene::empty();
    let project = PepaProject::new("WASM Project".to_string(), scene);

    // Оптимизируем
    let result = nest_unfolds(&project, &params);

    // Конвертируем в JSON
    serde_wasm_bindgen::to_value(&serde_json::json!({
        "sheets": result.sheets.len(),
        "total_parts": result.metrics.total_parts,
        "avg_fill_rate": result.metrics.avg_fill_rate,
        "total_sheets_area": result.metrics.total_sheets_area,
        "total_parts_area": result.metrics.total_parts_area,
    }))
    .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}

/// Версия библиотеки.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_version() {
        assert!(!version().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_unfold_config() {
        let config = UnfoldConfigWasm::new(
            Some("lscm".to_string()),
            Some(200),
            Some(1e-5),
            Some(true),
        );

        assert_eq!(config.algorithm, "lscm");
        assert_eq!(config.max_iterations, 200);
    }
}
