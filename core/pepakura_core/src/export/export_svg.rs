//! Экспорт развёрток в формат SVG

use crate::unfold::layout::LayoutResult;
use super::ExportOptions;
use std::fmt::Write;

/// Экспортирует укладку в формат SVG
/// 
/// # Аргументы
/// * `layout` - Результат укладки для экспорта
/// * `options` - Опции экспорта
/// 
/// # Возвращает
/// Вектор байтов с SVG данными или ошибку
pub fn export_to_svg(
    layout: &LayoutResult,
    options: &ExportOptions,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut svg_content = String::new();
    
    // Вычисляем смещение для отрицательных координат
    let offset_x = if layout.bounds.min_x < 0.0 {
        layout.bounds.min_x.abs()
    } else {
        0.0
    };
    
    let offset_y = if layout.bounds.min_y < 0.0 {
        layout.bounds.min_y.abs()
    } else {
        0.0
    };
    
    // Вычисляем масштаб для вписывания в заданные размеры
    let scale_x = options.width as f64 / (layout.bounds.max_x - layout.bounds.min_x);
    let scale_y = options.height as f64 / (layout.bounds.max_y - layout.bounds.min_y);
    let scale = scale_x.min(scale_y) * 0.9; // Добавляем небольшой отступ
    
    // Заголовок SVG
    writeln!(svg_content, "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>")?;
    writeln!(svg_content, "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" width=\"{}\" height=\"{}\">", options.width, options.height)?;
    
    // Стили
    writeln!(svg_content, "<style>")?;
    writeln!(svg_content, "polygon {{ fill: none; stroke: {}; stroke-width: {}; }}", options.line_color, options.line_width)?;
    writeln!(svg_content, "</style>")?;
    
    // Рисуем каждую грань
    for face in &layout.faces {
        // Преобразуем координаты для вписывания в заданные размеры
        let points: Vec<String> = face.vertices_2d.iter().map(|v| {
            let x = (v.x + offset_x) * scale;
            let y = (v.y + offset_y) * scale;
            format!("{:.2},{:.2}", x, y)
        }).collect();
        
        writeln!(svg_content, "<polygon points=\"{}\" />", points.join(" "))?;
    }
    
    // Закрываем SVG
    writeln!(svg_content, "</svg>")?;
    
    Ok(svg_content.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unfold::{LayoutResult, PlacedFace, Bounds, Point2D};
    
    #[test]
    fn test_export_simple_svg() {
        // Создаем простую укладку
        let face = PlacedFace {
            unfold_index: 0,
            vertices_2d: [
                Point2D::new(0.0, 0.0),
                Point2D::new(10.0, 0.0),
                Point2D::new(0.0, 10.0),
            ],
            position: Point2D::new(0.0, 0.0),
        };
        
        let layout = LayoutResult {
            faces: vec![face],
            bounds: Bounds {
                min_x: 0.0,
                max_x: 10.0,
                min_y: 0.0,
                max_y: 10.0,
            },
            overlaps: vec![],
        };
        
        let options = ExportOptions::default();
        let result = export_to_svg(&layout, &options);
        
        // Проверяем, что экспорт успешен
        assert!(result.is_ok());
        
        let svg_data = result.unwrap();
        let svg_string = String::from_utf8(svg_data).unwrap();
        
        // Проверяем, что SVG содержит ожидаемые элементы
        assert!(svg_string.contains("<svg"));
        assert!(svg_string.contains("<polygon"));
        assert!(svg_string.contains("</svg>"));
    }
}

