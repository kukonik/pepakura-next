//! Модуль экспорта развёрток в растровые форматы (PNG, JPG)
use std::path::Path;
use std::fs;
use image::{ImageFormat};
use crate::render::render_sheet_to_image;
use crate::unfold::types::{UnfoldParams, UnfoldResult};

/// Экспортирует результат развёртки в PNG файлы
pub fn export_to_png(result: &UnfoldResult, _params: &UnfoldParams, output_dir: &str) {
    for (i, sheet) in result.sheets.iter().enumerate() {
        let img = render_sheet_to_image(sheet);
        let path = Path::new(output_dir).join(format!("sheet_{:02}.png", i + 1));
        
        img.save(&path)
            .expect(&format!("Не удалось сохранить PNG файл: {:?}", path));
        println!("Сохранён PNG: {:?}", path);
    }
}

/// Экспортирует результат развёртки в JPG файлы
pub fn export_to_jpg(result: &UnfoldResult, _params: &UnfoldParams, output_dir: &str) {
    for (i, sheet) in result.sheets.iter().enumerate() {
        let img = render_sheet_to_image(sheet);
        let path = Path::new(output_dir).join(format!("sheet_{:02}.jpg", i + 1));
        
        img.save_with_format(&path, ImageFormat::Jpeg)
            .expect(&format!("Не удалось сохранить JPG файл: {:?}", path));
        println!("Сохранён JPG: {:?}", path);
    }
}

/// Экспортирует развёртку во все доступные форматы
pub fn export_to_all_formats(result: &crate::unfold::types::UnfoldResult,
                           params: &crate::unfold::types::UnfoldParams,
                           output_dir: &str) {
    println!("Начинаем экспорт в {}", output_dir);
    
    // Создаём директорию если её нет
    let _ = fs::create_dir_all(output_dir);
    
    // Экспорт во все форматы
    export_to_png(result, params, output_dir);
    export_to_jpg(result, params, output_dir);
    crate::export_svg::export_to_svg(result, params, output_dir);
    crate::export_pdf::export_to_pdf(result, params, output_dir);
    crate::export_3d::export_to_stl(result, params, output_dir);
    crate::export_3d::export_to_obj(result, params, output_dir);
    
    println!("Экспорт завершён!");
}