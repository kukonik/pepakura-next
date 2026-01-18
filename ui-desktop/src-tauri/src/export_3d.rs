//! Экспорт 3D моделей для печати
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::unfold::types::{UnfoldResult, UnfoldParams};

/// Экспортирует исходную 3D модель в STL файл
pub fn export_to_stl(_result: &UnfoldResult, _params: &UnfoldParams, output_dir: &str) {
    let path = Path::new(output_dir).join("model.stl");
    let file = File::create(&path).expect("Не удалось создать STL файл");
    let mut writer = BufWriter::new(file);
    
    // Заголовок STL (ASCII формат)
    writeln!(writer, "solid pepakura_model").expect("Ошибка записи STL");
    
    // Здесь должна быть логика экспорта треугольников из исходной модели
    // Пока что оставляем заглушку
    writeln!(writer, "facet normal 0 0 0").expect("Ошибка записи STL");
    writeln!(writer, "  outer loop").expect("Ошибка записи STL");
    writeln!(writer, "    vertex 0 0 0").expect("Ошибка записи STL");
    writeln!(writer, "    vertex 10 0 0").expect("Ошибка записи STL");
    writeln!(writer, "    vertex 0 10 0").expect("Ошибка записи STL");
    writeln!(writer, "  endloop").expect("Ошибка записи STL");
    writeln!(writer, "endfacet").expect("Ошибка записи STL");
    
    writeln!(writer, "endsolid pepakura_model").expect("Ошибка записи STL");
    println!("Сохранён STL: {:?}", path);
}

/// Экспортирует исходную 3D модель в OBJ файл
pub fn export_to_obj(_result: &UnfoldResult, _params: &UnfoldParams, output_dir: &str) {
    let path = Path::new(output_dir).join("model.obj");
    let file = File::create(&path).expect("Не удалось создать OBJ файл");
    let mut writer = BufWriter::new(file);
    
    // Заголовок OBJ
    writeln!(writer, "# Pepakura Next 3D Model").expect("Ошибка записи OBJ");
    writeln!(writer, "o pepakura_model").expect("Ошибка записи OBJ");
    
    // Здесь должна быть логика экспорта вершин и граней из исходной модели
    // Пока что оставляем заглушку
    writeln!(writer, "v 0.0 0.0 0.0").expect("Ошибка записи OBJ");
    writeln!(writer, "v 10.0 0.0 0.0").expect("Ошибка записи OBJ");
    writeln!(writer, "v 0.0 10.0 0.0").expect("Ошибка записи OBJ");
    writeln!(writer, "f 1 2 3").expect("Ошибка записи OBJ");
    
    println!("Сохранён OBJ: {:?}", path);
}
