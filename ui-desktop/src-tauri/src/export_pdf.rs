//! Экспорт развёрток в PDF через img2pdf
use std::fs;
use std::path::Path;
use image::DynamicImage;
use img2pdf;

use crate::render::render_sheet_to_image;
use crate::unfold::types::{UnfoldResult, UnfoldParams};

/// Экспортирует развёртки в PDF файлы
pub fn export_to_pdf(result: &UnfoldResult, _params: &UnfoldParams, output_dir: &str) {
    for (i, sheet) in result.sheets.iter().enumerate() {
        // 1. Рендерим изображение
        let img = render_sheet_to_image(sheet);
        
        // 2. Конвертируем в DynamicImage
        let dynamic_img = DynamicImage::ImageRgb8(img);
        
        // 3. Конвертируем в PDF с правильным размером
        let pdf_bytes = img2pdf::convert_image_with_size(
            &dynamic_img,
            img2pdf::Size::Millimeters(sheet.width_mm as f64, sheet.height_mm as f64)
        ).expect("Не удалось конвертировать в PDF");
        
        // 4. Сохраняем PDF
        let path = Path::new(output_dir).join(format!("sheet_{:02}.pdf", i + 1));
        fs::write(&path, pdf_bytes).expect("Не удалось записать PDF файл");
        
        println!("Сохранён PDF: {:?}", path);
    }
}