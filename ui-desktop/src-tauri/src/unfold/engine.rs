//! Основной движок развёртки 3D-моделей

use crate::unfold::types::*;

/// Основной движок развёртки
pub struct UnfoldEngine;

impl UnfoldEngine {
    /// Создаёт новый экземпляр движка развёртки
    pub fn new() -> Self {
        Self
    }

    /// Выполняет развёртку 3D-модели в 2D-шаблоны
    pub fn unfold_model(&self, model: &Model3D, params: &UnfoldParams) -> UnfoldResult {
        // TODO: Реализовать реальный алгоритм развёртки
        // Пока используем заглушку для демонстрации
        
        // Создаём тестовую развёртку с несколькими листами
        let (width_mm, height_mm) = Self::get_paper_dimensions(¶ms.paper_format);
        
        let sheet_count = params.max_sheets.min(4).max(1);
        let mut sheets = Vec::new();

        for i in 0..sheet_count {
            let id = i + 1;
            let mut parts = Vec::new();

            // Создаём несколько тестовых деталей на листе
            let part_count = 6 + (i as u32) * 2;
            for j in 0..part_count {
                let part_id = j + 1;
                let bounds = Rect {
                    x: 10.0 + (j as f32 * 8.0) % (width_mm * 0.6),
                    y: 10.0 + (j as f32 * 12.0) % (height_mm * 0.6),
                    width: 30.0 + (j % 3) as f32 * 5.0,
                    height: 20.0 + (j % 2) as f32 * 5.0,
                };

                let x1 = bounds.x;
                let y1 = bounds.y;
                let x2 = bounds.x + bounds.width;
                let y2 = bounds.y + bounds.height;

                // Создаём линии контура детали
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
                width_mm,
                height_mm,
                margin_mm: params.margin_mm,
                parts,
            });
        }

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