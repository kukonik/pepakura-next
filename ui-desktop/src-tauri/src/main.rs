#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

mod model_loader;
mod unfold;
mod export;
mod export_svg;
mod export_pdf;
mod export_3d;
mod render;

use model_loader::ModelLoader;
use unfold::types::{UnfoldParams, UnfoldResult};
use unfold::engine::UnfoldEngine;
use unfold::layout_optimizer::LayoutOptimizer;

fn main() {
    println!("Pepakura Next - 3D to 2D unfolding tool");
    
    // Параметры развертки
    let params = UnfoldParams {
        paper_format: "A4".to_string(),
        margin_mm: 10.0,
        max_sheets: 10,
        scale: 1.0,
    };

    // Загрузка модели
    let loader = ModelLoader::new();
    let model_path = "assets/models/cube.obj";
    println!("Loading model from: {}", model_path);
    
    let model = match loader.load_obj(model_path) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error loading model: {}", e);
            return;
        }
    };

    println!("Model loaded successfully!");
    println!("Vertices: {}", model.vertices.len());
    println!("Faces: {}", model.faces.len());

    // Развертка модели
    let unfolder = UnfoldEngine::new();
    println!("Unfolding model...");
    
    let unfold_result = unfolder.unfold_model(&model, &params);

    println!("Unfolding completed successfully!");
    println!("Sheets generated: {}", unfold_result.sheets.len());
    
    for (i, sheet) in unfold_result.sheets.iter().enumerate() {
        println!("Sheet {}: {} parts", i + 1, sheet.parts.len());
    }

    // Оптимизация расположения
    let optimizer = LayoutOptimizer::new();
    println!("Optimizing layout...");
    
    // Получаем размеры бумаги
    let (width_mm, height_mm) = match params.paper_format.as_str() {
        "A3" => (297.0, 420.0),
        "Letter" => (215.9, 279.4),
        _ => (210.0, 297.0), // A4 по умолчанию
    };
    
    // Собираем все детали из всех листов
    let mut all_parts = Vec::new();
    for sheet in unfold_result.sheets {
        all_parts.extend(sheet.parts);
    }
    
    // Оптимизируем размещение деталей
    let optimized_sheets = optimizer.optimize_layout(all_parts, width_mm, height_mm, params.margin_mm);
    let optimized_result = UnfoldResult {
        sheets: optimized_sheets,
    };

    // Экспорт во все форматы
    let output_dir = "output";
    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    
    println!("Exporting to all formats...");
    export::export_to_all_formats(&optimized_result, &params, output_dir);

    println!("Export completed successfully!");
    println!("Files saved to: {}", output_dir);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        assert_eq!(2 + 2, 4);
    }
}