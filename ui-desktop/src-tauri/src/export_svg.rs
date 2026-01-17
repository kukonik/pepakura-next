//! Экспорт развёрток в SVG
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::unfold::types::{LineKind, Part2D, Sheet, UnfoldParams, UnfoldResult};

/// Экспортирует результат развёртки в SVG файлы
pub fn export_to_svg(result: &UnfoldResult, _params: &UnfoldParams, output_dir: &str) {
    for (i, sheet) in result.sheets.iter().enumerate() {
        let path = Path::new(output_dir).join(format!("sheet_{:02}.svg", i + 1));
        export_sheet_to_svg(sheet, &path);
        println!("Сохранён SVG: {:?}", path);
    }
}

/// Экспортирует один лист в SVG файл
fn export_sheet_to_svg(sheet: &Sheet, file_path: &Path) {
    let file = File::create(file_path).expect("Не удалось создать SVG файл");
    let mut w = BufWriter::new(file);

    // Заголовок SVG
    writeln!(
        w,
        r#"<?xml version="1.0" encoding="UTF-8"?>"#
    ).expect("Ошибка записи в SVG");
    
    writeln!(
        w,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{w}mm" height="{h}mm" viewBox="0 0 {w} {h}">"#,
        w = sheet.width_mm,
        h = sheet.height_mm
    ).expect("Ошибка записи в SVG");

    writeln!(w, r#"<g fill="none" stroke-linecap="round" stroke-linejoin="round">"#)
        .expect("Ошибка записи в SVG");

    // Рисуем все детали
    for part in &sheet.parts {
        for line in &part.lines {
            let (color, width) = match line.kind {
                LineKind::Cut => ("#000000", 0.5),
                LineKind::Mountain => ("#ff0000", 0.3),
                LineKind::Valley => ("#0000ff", 0.3),
                LineKind::GlueTab => ("#00ff00", 0.2),
            };

            writeln!(
                w,
                r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="{color}" stroke-width="{w}"/>"#,
                x1 = line.x1,
                y1 = line.y1,
                x2 = line.x2,
                y2 = line.y2,
                color = color,
                w = width
            ).expect("Ошибка записи линии в SVG");
        }
    }

    writeln!(w, "</g>").expect("Ошибка записи в SVG");
    writeln!(w, "</svg>").expect("Ошибка записи в SVG");
}