//! Оптимизатор раскладки деталей на листах

use crate::unfold::types::*;
use std::f32::consts::PI;

/// Оптимизатор раскладки деталей
pub struct LayoutOptimizer;

impl LayoutOptimizer {
    /// Создаёт новый экземпляр оптимизатора
    pub fn new() -> Self {
        Self
    }

    /// Оптимизирует раскладку деталей на листах
    pub fn optimize_layout(&self, result: &mut UnfoldResult) {
        // Оптимизируем каждую деталь на каждом листе
        for sheet in &mut result.sheets {
            self.optimize_sheet_parts(sheet);
        }
        
        // Оптимизируем распределение по листам
        self.optimize_sheet_distribution(result);
    }

    /// Оптимизирует детали на одном листе
    fn optimize_sheet_parts(&self, sheet: &mut Sheet) {
        // Сортируем детали по убыванию площади для лучшей упаковки
        sheet.parts.sort_by(|a, b| {
            let area_a = a.bounds.width * a.bounds.height;
            let area_b = b.bounds.width * b.bounds.height;
            area_b.partial_cmp(&area_a).unwrap()
        });
        
        // Применяем простую упаковку с поворотами
        self.pack_parts(sheet);
    }

    /// Применяет алгоритм упаковки к деталям на листе
    fn pack_parts(&self, sheet: &mut Sheet) {
        let margin = sheet.margin_mm;
        let max_width = sheet.width_mm - 2.0 * margin;
        let max_height = sheet.height_mm - 2.0 * margin;
        
        let mut x_cursor = margin;
        let mut y_cursor = margin;
        let mut max_y_in_row = 0.0;
        let row_spacing = 10.0;
        let part_spacing = 5.0;
        
        for part in &mut sheet.parts {
            // Определяем лучшую ориентацию (исходная или повёрнутая на 90 градусов)
            let (width, height) = (part.bounds.width, part.bounds.height);
            let (rotated_width, rotated_height) = (height, width);
            
            let use_rotation = rotated_width <= max_width && rotated_height <= max_height 
                && (rotated_width * rotated_height > width * height);
            
            // Применяем поворот, если он улучшает размещение
            if use_rotation {
                self.rotate_part(part, 90.0);
            }
            
            // Проверяем, помещается ли деталь в текущий ряд
            let part_width = part.bounds.width;
            let part_height = part.bounds.height;
            
            if x_cursor + part_width > max_width + margin {
                // Переходим к следующей строке
                x_cursor = margin;
                y_cursor += max_y_in_row + row_spacing;
                max_y_in_row = 0.0;
            }
            
            // Позиционируем деталь
            let offset_x = x_cursor - part.bounds.x;
            let offset_y = y_cursor - part.bounds.y;
            
            // Обновляем координаты линий
            for line in &mut part.lines {
                line.x1 += offset_x;
                line.y1 += offset_y;
                line.x2 += offset_x;
                line.y2 += offset_y;
            }
            
            // Обновляем границы
            part.bounds.x = x_cursor;
            part.bounds.y = y_cursor;
            
            // Обновляем позицию курсора
            x_cursor += part_width + part_spacing;
            max_y_in_row = max_y_in_row.max(part_height);
        }
    }

    /// Поворачивает деталь на заданный угол
    fn rotate_part(&self, part: &mut Part2D, angle_deg: f32) {
        let angle_rad = angle_deg.to_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        
        // Центр детали
        let center_x = part.bounds.x + part.bounds.width / 2.0;
        let center_y = part.bounds.y + part.bounds.height / 2.0;
        
        // Поворачиваем все линии
        for line in &mut part.lines {
            // Переносим точку в начало координат
            let x1_rel = line.x1 - center_x;
            let y1_rel = line.y1 - center_y;
            let x2_rel = line.x2 - center_x;
            let y2_rel = line.y2 - center_y;
            
            // Поворачиваем
            let x1_rot = x1_rel * cos_a - y1_rel * sin_a;
            let y1_rot = x1_rel * sin_a + y1_rel * cos_a;
            let x2_rot = x2_rel * cos_a - y2_rel * sin_a;
            let y2_rot = x2_rel * sin_a + y2_rel * cos_a;
            
            // Переносим обратно
            line.x1 = x1_rot + center_x;
            line.y1 = y1_rot + center_y;
            line.x2 = x2_rot + center_x;
            line.y2 = y2_rot + center_y;
        }
        
        // Обновляем границы (упрощённо)
        let new_width = part.bounds.height;
        let new_height = part.bounds.width;
        part.bounds.width = new_width;
        part.bounds.height = new_height;
    }

    /// Оптимизирует распределение деталей по листам
    fn optimize_sheet_distribution(&self, result: &mut UnfoldResult) {
        // Пока просто оставляем структуру для будущей реализации
        // Здесь можно реализовать алгоритмы перераспределения деталей
        // между листами для лучшего использования пространства
    }
}

impl Default for LayoutOptimizer {
    fn default() -> Self {
        Self::new()
    }
}