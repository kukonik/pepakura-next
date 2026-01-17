//! Тестовый скрипт для проверки экспорта
use std::path::Path;

fn main() {
    // Создаём тестовую структуру развёртки
    let result = crate::unfold::types::UnfoldResult {
        sheets: vec![
            crate::unfold::types::Sheet {
                width_mm: 210.0,  // A4
                height_mm: 297.0,
                parts: vec![
                    crate::unfold::types::Part2D {
                        lines: vec![
                            crate::unfold::types::Line2D {
                                x1: 20.0,
                                y1: 20.0,
                                x2: 190.0,
                                y2: 20.0,
                                kind: crate::unfold::types::LineKind::Cut,
                            },
                            crate::unfold::types::Line2D {
                                x1: 20.0,
                                y1: 40.0,
                                x2: 190.0,
                                y2: 80.0,
                                kind: crate::unfold::types::LineKind::Mountain,
                            },
                        ],
                    },
                ],
            },
        ],
    };
    
    let params = crate::unfold::types::UnfoldParams {
        // параметры по умолчанию
    };
    
    let output_dir = "D:\\Dev\\pepakura-next\\output_test";
    
    // Экспортируем во все форматы
    crate::export::export_to_all_formats(&result, &params, output_dir);
}