pub mod layout;
// pub mod paper_optimize; // temporarily disabled
// pub mod unwrap3d; // temporarily disabled
pub mod mds;

// Re-export types from mds
pub use mds::{UnfoldConfig, UnfoldedMesh, UnfoldMetadata};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldResult {
    pub faces: Vec<UnfoldedFace>,
    pub seams: Vec<Seam>,
    pub layout: LayoutResult,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldedFace {
    pub vertices_2d: Vec<Point2D>,
    pub center: Point2D,
    pub original_face_index: usize,
    pub tabs: Vec<GlueTab>,
}

impl UnfoldedFace {
    pub fn new(vertices_2d: Vec<Point2D>, center: Point2D, original_face_index: usize) -> Self {
        Self {
            vertices_2d,
            center,
            original_face_index,
            tabs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Seam {
    pub id: usize,
    pub start: Point2D,
    pub end: Point2D,
    pub face1_index: usize,
    pub face2_index: usize,
    pub angle_degrees: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LayoutResult {
    pub faces: Vec<PlacedFace>,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlacedFace {
    pub face: UnfoldedFace,
    pub position: Point2D,
    pub rotation: f64,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Структура для представления вкладыша
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlueTab {
    /// Точки вкладыша
    pub points: Vec<Point2D>,
    /// ID шва, к которому относится вкладыш
    pub seam_id: usize,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OverlapInfo {
    pub face1_index: usize,
    pub face2_index: usize,
    pub area: f64,
}

// Temporarily commented out because it depends on old Model
/*
pub fn unfold_model(model: &Model) -> UnfoldResult {
    // Создаем развернутые грани
    let faces: Vec<UnfoldedFace> = model.faces.iter().enumerate().map(|(index, _)| {
        UnfoldedFace::new(Vec::new(), Point2D { x: 0.0, y: 0.0 }, index)
    }).collect();
    
    // Создаем швы между гранями
    let seams = create_seams(model);
    
    // Выполняем укладку
    let layout = layout::arrange_faces(faces.clone());
    
    UnfoldResult {
        faces,
        seams,
        layout,
    }
}

/// Создает швы между гранями модели
fn create_seams(model: &Model) -> Vec<Seam> {
    let mut seams = Vec::new();
    let mut seam_id = 0;
    
    // Для простоты создаем швы между всеми соседними гранями
    // В реальной реализации здесь будет более сложная логика
    for i in 0..model.faces.len() {
        for j in (i + 1)..model.faces.len() {
            // Проверяем, являются ли грани соседними
            // Пока используем простую эвристику
            if are_faces_adjacent(&model.faces[i], &model.faces[j]) {
                let seam = Seam {
                    id: seam_id,
                    start: Point2D { x: 0.0, y: 0.0 },
                    end: Point2D { x: 10.0, y: 0.0 }, // Примерные координаты
                    face1_index: i,
                    face2_index: j,
                    angle_degrees: 90.0, // Примерный угол
                };
                seams.push(seam);
                seam_id += 1;
            }
        }
    }
    
    seams
}

/// Проверяет, являются ли две грани соседними
fn are_faces_adjacent(face1: &crate::model::Face, face2: &crate::model::Face) -> bool {
    // Проверяем, есть ли общие вершины между гранями
    let mut common_vertices = 0;
    for &idx1 in &face1.vertices {
        for &idx2 in &face2.vertices {
            if idx1 == idx2 {
                common_vertices += 1;
            }
        }
    }
    
    // Грани считаются соседними, если у них есть общие вершины
    common_vertices > 0
}
*/