//! Типы данных для движка развёртки

use serde::{Deserialize, Serialize};

/// Тип линии на развёртке.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Вершина в 3D-пространстве
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Грань (треугольник) в 3D-модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face3D {
    pub vertices: [u32; 3], // индексы вершин
    pub normal: [f32; 3],    // нормаль грани
}

/// 3D-модель
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model3D {
    pub vertices: Vec<Vertex3D>,
    pub faces: Vec<Face3D>,
}