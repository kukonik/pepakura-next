//! Integration-тесты для Tauri команд.
//! 
//! Тестируют взаимодействие между frontend и Rust-бэкендом.

use pepakura_core::ai::{AiConfig, AiProvider};
use pepakura_core::geometry::{Face, Mesh, Vertex};
use pepakura_core::unfold::{unfold_mds, UnfoldConfig};
use pepakura_core::export::{export_svg, SvgExportConfig};

/// Создаёт тестовый куб.
fn create_test_cube() -> Mesh {
    let mut mesh = Mesh::new("TestCube");
    
    // 8 вершин куба
    let vertices = [
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
    ];
    
    for (i, &pos) in vertices.iter().enumerate() {
        mesh.add_vertex(Vertex::new(i, pos));
    }
    
    // 12 граней (по 2 на каждую сторону)
    let faces = [
        [0, 1, 2], [0, 2, 3],
        [4, 6, 5], [4, 7, 6],
        [0, 5, 1], [0, 4, 5],
        [1, 6, 2], [1, 5, 6],
        [2, 7, 3], [2, 6, 7],
        [3, 4, 0], [3, 7, 4],
    ];
    
    for &[a, b, c] in &faces {
        mesh.add_face(Face::new(a, b, c));
    }
    
    mesh
}

/// Тест: развёртка куба через MDS.
#[test]
fn test_unfold_cube_mds() {
    let mesh = create_test_cube();
    let config = UnfoldConfig::default();
    
    let result = unfold_mds(&mesh, &config);
    assert!(result.is_ok(), "Развёртка куба должна succeed");
    
    let unfolded = result.unwrap();
    assert_eq!(unfolded.vertices_2d.len(), 8);
    assert_eq!(unfolded.faces.len(), 12);
}

/// Тест: экспорт развёртки в SVG.
#[test]
fn test_export_svg_cube() {
    let mesh = create_test_cube();
    let config = UnfoldConfig::default();
    let unfolded = unfold_mds(&mesh, &config).unwrap();
    
    let svg_config = SvgExportConfig::default();
    let svg_result = export_svg(&unfolded, &svg_config);
    
    assert!(svg_result.is_ok(), "Экспорт в SVG должен succeed");
    
    let svg = svg_result.unwrap();
    assert!(svg.contains("<?xml version=\"1.0\""));
    assert!(svg.contains("<svg"));
    assert!(svg.contains("</svg>"));
    assert!(svg.contains("class=\"cut-line\""));
}

/// Тест: AI конфигурация по умолчанию.
#[test]
fn test_ai_config_default() {
    let config = AiConfig::default();
    
    assert_eq!(config.provider, AiProvider::Ollama);
    assert_eq!(config.ollama_url, "http://localhost:11434");
    assert_eq!(config.model, "llama3.2");
    assert!((config.temperature - 0.7).abs() < 0.001);
}

/// Тест: геометрия куба.
#[test]
fn test_cube_geometry() {
    let mesh = create_test_cube();
    
    assert_eq!(mesh.vertex_count(), 8);
    assert_eq!(mesh.face_count(), 12);
    
    let bbox = mesh.bounding_box();
    assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
    assert_eq!(bbox.max, [1.0, 1.0, 1.0]);
    
    let centroid = mesh.centroid();
    assert_eq!(centroid, [0.5, 0.5, 0.5]);
}

/// Тест: валидация меша.
#[test]
fn test_mesh_validation() {
    let mesh = create_test_cube();
    let result = mesh.validate();
    assert!(result.is_ok(), "Куб должен быть валидным мешем");
}

/// Тест: площадь поверхности куба.
#[test]
fn test_cube_surface_area() {
    let mesh = create_test_cube();
    let area = mesh.total_area();
    
    // Площадь поверхности куба со стороной 1 = 6
    assert!((area - 6.0).abs() < 0.1, "Площадь куба должна быть ~6.0");
}

/// Тест: масштабирование меша.
#[test]
fn test_mesh_scale() {
    let mut mesh = create_test_cube();
    
    mesh.scale(2.0);
    
    let bbox = mesh.bounding_box();
    assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
    assert_eq!(bbox.max, [2.0, 2.0, 2.0]);
}

/// Тест: центрирование меша.
#[test]
fn test_mesh_center() {
    let mut mesh = create_test_cube();
    
    mesh.center();
    
    let centroid = mesh.centroid();
    assert!(centroid.iter().all(|&v| v.abs() < 0.0001), "Центроид должен быть в начале координат");
}

/// Тест: SVG с разными настройками.
#[test]
fn test_svg_export_options() {
    let mesh = create_test_cube();
    let unfolded = unfold_mds(&mesh, &UnfoldConfig::default()).unwrap();
    
    // Экспорт без линий сгиба
    let config_no_folds = SvgExportConfig {
        show_fold_lines: false,
        ..Default::default()
    };
    
    let svg = export_svg(&unfolded, &config_no_folds).unwrap();
    assert!(!svg.contains("class=\"fold-line\""), "Не должно быть линий сгиба");
    
    // Экспорт без номеров деталей
    let config_no_numbers = SvgExportConfig {
        show_part_numbers: false,
        ..Default::default()
    };
    
    let svg = export_svg(&unfolded, &config_no_numbers).unwrap();
    assert!(!svg.contains("class=\"part-number\""), "Не должно быть номеров деталей");
}

/// Тест: MDS с разными параметрами.
#[test]
fn test_mds_different_params() {
    let mesh = create_test_cube();
    
    // С малым количеством итераций
    let config_low_iter = UnfoldConfig {
        max_iterations: 10,
        ..Default::default()
    };
    
    let result = unfold_mds(&mesh, &config_low_iter);
    assert!(result.is_ok(), "MDS должен работать с малым числом итераций");
    
    // С высокой точностью
    let config_high_precision = UnfoldConfig {
        max_iterations: 500,
        tolerance: 1e-10,
        ..Default::default()
    };
    
    let result = unfold_mds(&mesh, &config_high_precision);
    assert!(result.is_ok(), "MDS должен работать с высокой точностью");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_integration_full_workflow() {
        // Полный рабочий процесс: создание -> развёртка -> экспорт
        let mesh = create_test_cube();
        
        // Развёртка
        let config = UnfoldConfig::default();
        let unfolded = unfold_mds(&mesh, &config).expect("Развёртка должна succeed");
        
        // Экспорт
        let svg_config = SvgExportConfig::default();
        let svg = export_svg(&unfolded, &svg_config).expect("Экспорт должен succeed");
        
        // Проверка результата
        assert!(svg.len() > 1000, "SVG должен быть достаточно большим");
        assert!(svg.contains("Pepakura Next"), "SVG должен содержать метаданные");
    }
}
