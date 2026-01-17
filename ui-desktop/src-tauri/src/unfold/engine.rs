//! Основной движок развёртки 3D-моделей
 
use crate::unfold::types::*;
use crate::unfold::face_unfolder::FaceUnfolder;
use crate::unfold::layout_optimizer::LayoutOptimizer;
 
/// Основной движок развёртки
pub struct UnfoldEngine;

impl UnfoldEngine {
    /// Создаёт новый экземпляр движка развёртки
    pub fn new() -> Self {
        Self
    }

    /// Выполняет развёртку 3D-модели в 2D-шаблоны
    pub fn unfold_model(&self, model: &Model3D, params: &UnfoldParams) -> UnfoldResult {
        let face_unfolder = FaceUnfolder::new();
        let mut parts = Vec::new();
        
        // Развертываем каждую грань модели
        for (face_index, _face) in model.faces.iter().enumerate() {
            if let Ok(unfolded) = face_unfolder.unfold_face(model, face_index as u32) {
                // Добавляем клапаны к линиям
                let mut lines_with_tabs = Vec::new();
                for line in &unfolded.lines {
                    let mut tab_lines = face_unfolder.add_glue_tab(line, 5.0); // Ширина клапана 5мм
                    lines_with_tabs.append(&mut tab_lines);
                }
                
                let part = Part2D {
                    id: unfolded.face_id,
                    name: Some(format!("Face {}", unfolded.face_id)),
                    bounds: unfolded.bounds,
                    lines: lines_with_tabs,
                };
                
                parts.push(part);
            }
        }
        
        // Оптимизируем размещение деталей на листах
        let (width_mm, height_mm) = Self::get_paper_dimensions(&params.paper_format);
        let optimizer = LayoutOptimizer::new();
        let sheets = optimizer.optimize_layout(parts, width_mm, height_mm, params.margin_mm);
        
        UnfoldResult { sheets }
    }

    /// Возвращает размеры бумаги в миллиметрах
    fn get_paper_dimensions(paper_format: &str) -> (f32, f32) {
        match paper_format {
            "A3" => (297.0, 420.0),
            "Letter" => (215.9, 279.4),
            _ => (210.0, 297.0), // A4 по умолчанию
        }
    }
}

impl Default for UnfoldEngine {
    fn default() -> Self {
        Self::new()
    }
}