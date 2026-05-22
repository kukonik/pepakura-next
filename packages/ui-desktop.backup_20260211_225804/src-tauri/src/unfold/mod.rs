//! Модуль для интеграции с ядром разворачивания

pub mod commands;
pub mod paper_optimize;
pub mod bridge;

use serde::{Deserialize, Serialize};

/// Параметры разворачивания
#[derive(Serialize, Deserialize, Clone)]
pub struct UnfoldParams {
    /// Минимальный угол для создания шва (в градусах)
    pub min_seam_angle: f32,
    /// Максимальная длина шва
    pub max_seam_length: f32,
    /// Использовать ли автоматическое создание швов
    pub auto_seams: bool,
}

impl Default for UnfoldParams {
    fn default() -> Self {
        Self {
            min_seam_angle: 75.0,
            max_seam_length: 10.0,
            auto_seams: true,
        }
    }
}

/// Результат разворачивания
#[derive(Serialize, Deserialize, Clone)]
pub struct UnfoldResult {
    /// Развернутые грани
    pub unfolded_faces: Vec<UnfoldedFace>,
    /// Швы развертки
    pub seams: Vec<Seam>,
    /// Ширина развертки
    pub width: f32,
    /// Высота развертки
    pub height: f32,
}

/// Развернутая грань
#[derive(Serialize, Deserialize, Clone)]
pub struct UnfoldedFace {
    /// Индекс грани в оригинальной модели
    pub face_index: usize,
    /// Вершины грани в 2D пространстве
    pub vertices_2d: Vec<[f32; 2]>,
    /// Нормаль грани
    pub normal: [f32; 3],
}

/// Шов развертки
#[derive(Serialize, Deserialize, Clone)]
pub struct Seam {
    /// Начальная точка шва
    pub start_point: [f32; 2],
    /// Конечная точка шва
    pub end_point: [f32; 2],
    /// Индекс первой грани
    pub face_a_index: usize,
    /// Индекс второй грани
    pub face_b_index: usize,
}