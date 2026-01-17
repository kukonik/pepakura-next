//! Ядро рендеринга развёрток
use image::{ImageBuffer, Rgb as ImageRgb};
use crate::unfold::types::{LineKind, Part2D, Sheet};

/// Создаёт изображение листа с белым фоном
pub fn render_sheet_to_image(sheet: &Sheet) -> ImageBuffer<ImageRgb<u8>, Vec<u8>> {
    let dpi = 150;
    let width_px = (sheet.width_mm * dpi as f32 / 25.4) as u32;
    let height_px = (sheet.height_mm * dpi as f32 / 25.4) as u32;
    
    let mut img = ImageBuffer::from_pixel(width_px, height_px, ImageRgb([255, 255, 255]));
    
    for part in &sheet.parts {
        draw_part_on_image(
            part,
            &mut img,
            sheet.width_mm,
            sheet.height_mm,
            width_px,
            height_px,
        );
    }
    img
}

/// Рисует деталь на изображении
fn draw_part_on_image(
    part: &Part2D,
    img: &mut ImageBuffer<ImageRgb<u8>, Vec<u8>>,
    sheet_width_mm: f32,
    sheet_height_mm: f32,
    img_width_px: u32,
    img_height_px: u32,
) {
    let scale_x = img_width_px as f32 / sheet_width_mm;
    let scale_y = img_height_px as f32 / sheet_height_mm;

    for line in &part.lines {
        let x1 = (line.x1 * scale_x) as i32;
        let y1 = (line.y1 * scale_y) as i32;
        let x2 = (line.x2 * scale_x) as i32;
        let y2 = (line.y2 * scale_y) as i32;

        draw_line(img, x1, y1, x2, y2, line.kind.clone());
    }
}

/// Рисует линию на изображении (алгоритм Брезенхэма)
fn draw_line(
    img: &mut ImageBuffer<ImageRgb<u8>, Vec<u8>>,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    kind: LineKind,
) {
    let color = match kind {
        LineKind::Cut => ImageRgb([0, 0, 0]),
        LineKind::Mountain => ImageRgb([255, 0, 0]),
        LineKind::Valley => ImageRgb([0, 0, 255]),
        LineKind::GlueTab => ImageRgb([0, 255, 0]),
    };

    let (width, height) = img.dimensions();

    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();
    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };
    let mut err = dx - dy;

    let mut x = x1;
    let mut y = y1;

    while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
        *img.get_pixel_mut(x as u32, y as u32) = color;

        if x == x2 && y == y2 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x += sx;
        }
        if e2 < dx {
            err += dx;
            y += sy;
        }
    }
}