#![allow(dead_code)]
//! Оптимизатор размещения деталей на листах

use crate::unfold::types::{Part2D, Rect, Sheet};

/// Оптимизатор размещения деталей на листах
pub struct LayoutOptimizer;

impl LayoutOptimizer {
    /// Создает новый экземпляр оптимизатора
    pub fn new() -> Self {
        Self
    }

    /// Размещает детали на листах с минимальными отходами
    pub fn optimize_layout(&self, parts: Vec<Part2D>, sheet_width: f32, sheet_height: f32, margin: f32) -> Vec<Sheet> {
        let mut sheets = Vec::new();
        let mut remaining_parts = parts;
        let mut sheet_index = 1u32;

        while !remaining_parts.is_empty() {
            let mut sheet_parts = Vec::new();
            let mut occupied_areas = Vec::new();

            // Пытаемся разместить детали на текущем листе
            let mut i = 0;
            while i < remaining_parts.len() {
                let part = &remaining_parts[i];
                let part_width = part.bounds.width + 2.0 * margin;
                let part_height = part.bounds.height + 2.0 * margin;

                // Ищем свободное место на листе
                if let Some((x, y)) = self.find_free_space(&occupied_areas, part_width, part_height, sheet_width, sheet_height) {
                    // Создаем новую деталь с обновленными координатами
                    let adjusted_part = Part2D {
                        id: part.id,
                        name: part.name.clone(),
                        bounds: Rect {
                            x: x + margin,
                            y: y + margin,
                            width: part.bounds.width,
                            height: part.bounds.height,
                        },
                        lines: part.lines.clone(),
                    };

                    sheet_parts.push(adjusted_part);
                    occupied_areas.push(Rect { x, y, width: part_width, height: part_height });

                    // Удаляем размещённую деталь
                    remaining_parts.remove(i);
                } else {
                    i += 1;
                }
            }

            // Создаем лист с размещёнными деталями
            let sheet = Sheet {
                id: sheet_index,
                index: sheet_index,
                width_mm: sheet_width,
                height_mm: sheet_height,
                margin_mm: margin,
                parts: sheet_parts.clone(),
            };

            sheets.push(sheet);
            sheet_index += 1;

            // Если не удалось разместить ни одной детали, прерываем цикл
            if sheet_parts.is_empty() && !remaining_parts.is_empty() {
                // Это может произойти, если деталь больше листа
                // В реальной реализации здесь должна быть обработка ошибок
                break;
            }
        }

        sheets
    }

    /// Находит свободное место на листе для размещения детали
    fn find_free_space(&self, occupied_areas: &[Rect], part_width: f32, part_height: f32, sheet_width: f32, sheet_height: f32) -> Option<(f32, f32)> {
        // Простой алгоритм размещения - первый подходящий
        for y in 0..=((sheet_height - part_height) as i32) {
            for x in 0..=((sheet_width - part_width) as i32) {
                let x = x as f32;
                let y = y as f32;

                // Проверяем, не пересекается ли с уже размещенными деталями
                let mut intersects = false;
                for area in occupied_areas {
                    if self.rects_intersect(x, y, part_width, part_height, area.x, area.y, area.width, area.height) {
                        intersects = true;
                        break;
                    }
                }

                if !intersects {
                    return Some((x, y));
                }
            }
        }

        None
    }

    /// Проверяет, пересекаются ли два прямоугольника
    fn rects_intersect(&self, x1: f32, y1: f32, w1: f32, h1: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }
}

impl Default for LayoutOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unfold::types::{Line2D, LineKind};

    #[test]
    fn test_optimize_layout() {
        let parts = vec![
            Part2D {
                id: 1,
                name: Some("Test Part 1".to_string()),
                bounds: Rect { x: 0.0, y: 0.0, width: 50.0, height: 50.0 },
                lines: vec![],
            },
            Part2D {
                id: 2,
                name: Some("Test Part 2".to_string()),
                bounds: Rect { x: 0.0, y: 0.0, width: 30.0, height: 30.0 },
                lines: vec![],
            },
        ];

        let optimizer = LayoutOptimizer::new();
        let sheets = optimizer.optimize_layout(parts, 210.0, 297.0, 5.0);

        assert!(!sheets.is_empty());
        assert_eq!(sheets[0].parts.len(), 2);
    }
}
