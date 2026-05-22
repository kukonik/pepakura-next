#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Model, Face, Vertex, Vector3D};
    
    #[test]
    fn test_paper_optimize_params_default() {
        let params = PaperOptimizeParams::default();
        assert_eq!(params.sheet_width, 210.0);
        assert_eq!(params.sheet_height, 297.0);
        assert_eq!(params.min_gap, 2.0);
        assert_eq!(params.min_tab_width, 5.0);
        assert_eq!(params.max_auto_tab_angle, 60.0);
        assert_eq!(params.add_print_margins, true);
        assert_eq!(params.margin_size, 5.0);
    }
    
    #[test]
    fn test_calculate_face_area() {
        // Создаем простую треугольную грань
        let face = UnfoldedFace {
            vertex_indices: vec![0, 1, 2],
            vertices_2d: vec![
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 1.0, y: 0.0 },
                Point2D { x: 0.0, y: 1.0 },
            ],
            normal: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
            tabs: vec![],
        };
        
        let area = calculate_face_area(&face);
        // Площадь треугольника с основанием 1 и высотой 1 должна быть 0.5
        assert!((area - 0.5).abs() < 0.0001);
    }
    
    #[test]
    fn test_calculate_face_area_degenerate() {
        // Создаем вырожденную грань (все точки совпадают)
        let face = UnfoldedFace {
            vertex_indices: vec![0, 1, 2],
            vertices_2d: vec![
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 0.0, y: 0.0 },
            ],
            normal: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
            tabs: vec![],
        };
        
        let area = calculate_face_area(&face);
        // Площадь вырожденного треугольника должна быть 0
        assert_eq!(area, 0.0);
    }
    
    #[test]
    fn test_calculate_face_area_insufficient_vertices() {
        // Создаем грань с недостаточным количеством вершин
        let face = UnfoldedFace {
            vertex_indices: vec![0, 1],
            vertices_2d: vec![
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 1.0, y: 0.0 },
            ],
            normal: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
            tabs: vec![],
        };
        
        let area = calculate_face_area(&face);
        // Площадь грани с недостаточным количеством вершин должна быть 0
        assert_eq!(area, 0.0);
    }
    
    #[test]
    fn test_create_glue_tab() {
        let seam = Seam {
            id: 0,
            face1_index: 0,
            face2_index: 1,
            start: Point2D { x: 0.0, y: 0.0 },
            end: Point2D { x: 10.0, y: 0.0 },
            angle_degrees: 45.0,
        };
        
        let params = PaperOptimizeParams {
            min_gap: 2.0,
            sheet_width: 210.0,
            sheet_height: 297.0,
            min_tab_width: 5.0,
            max_auto_tab_angle: 60.0,
            add_print_margins: true,
            margin_size: 5.0,
        };
        
        let tab = create_glue_tab(&seam, &params);
        
        // Проверяем, что созданы правильные точки вкладыша
        assert_eq!(tab.points.len(), 3);
        assert_eq!(tab.seam_id, 0);
        
        // Проверяем, что средняя точка находится на середине шва
        let midpoint = &tab.points[1];
        assert!((midpoint.x - 5.0).abs() < 0.0001);
        assert!((midpoint.y - 5.0).abs() < 0.0001); // 5.0 - это min_tab_width
    }
    
    #[test]
    fn test_find_face_for_seam() {
        let faces = vec![
            UnfoldedFace {
                vertex_indices: vec![0, 1, 2],
                vertices_2d: vec![
                    Point2D { x: 0.0, y: 0.0 },
                    Point2D { x: 1.0, y: 0.0 },
                    Point2D { x: 0.0, y: 1.0 },
                ],
                normal: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
                tabs: vec![],
            },
            UnfoldedFace {
                vertex_indices: vec![3, 4, 5],
                vertices_2d: vec![
                    Point2D { x: 1.0, y: 0.0 },
                    Point2D { x: 2.0, y: 0.0 },
                    Point2D { x: 1.0, y: 1.0 },
                ],
                normal: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
                tabs: vec![],
            },
        ];
        
        let seam1 = Seam {
            id: 0,
            face1_index: 0,
            face2_index: 1,
            start: Point2D { x: 1.0, y: 0.0 },
            end: Point2D { x: 1.0, y: 1.0 },
            angle_degrees: 90.0,
        };
        
        let seam2 = Seam {
            id: 1,
            face1_index: 2,
            face2_index: 3,
            start: Point2D { x: 2.0, y: 0.0 },
            end: Point2D { x: 2.0, y: 1.0 },
            angle_degrees: 90.0,
        };
        
        // Проверяем, что находим грань для шва, который к ней принадлежит
        let face_index1 = find_face_for_seam(&faces, &seam1);
        assert_eq!(face_index1, Some(0));
        
        // Проверяем, что не находим грань для шва, который ни к одной не принадлежит
        let face_index2 = find_face_for_seam(&faces, &seam2);
        assert_eq!(face_index2, None);
    }
    
    #[test]
    fn test_calculate_paper_usage() {
        let layout = LayoutResult {
            faces: vec![],
            width: 210.0,
            height: 297.0,
        };
        
        let params = PaperOptimizeParams::default();
        
        let usage = calculate_paper_usage(&layout, &params);
        
        // Для пустой развертки использование бумаги должно быть 0
        assert_eq!(usage.model_area, 0.0);
        assert_eq!(usage.sheet_count, 0);
        assert_eq!(usage.usage_percentage, 0.0);
    }
    
    #[test]
    fn test_analyze_model_for_paper() {
        let model = Model {
            vertices: vec![
                Vertex { position: Vector3D { x: 0.0, y: 0.0, z: 0.0 } },
                Vertex { position: Vector3D { x: 1.0, y: 0.0, z: 0.0 } },
                Vertex { position: Vector3D { x: 0.0, y: 1.0, z: 0.0 } },
            ],
            faces: vec![
                Face {
                    vertex_indices: vec![0, 1, 2],
                    normal: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
                    material_id: None,
                }
            ],
        };
        
        let analysis = analyze_model_for_paper(&model);
        
        // Проверяем, что анализ возвращает правильные значения
        assert_eq!(analysis.face_count, 1);
        assert_eq!(analysis.estimated_sheet_count, 1);
        assert_eq!(analysis.complexity_score, 1.0);
    }
    
    #[test]
    fn test_generate_assembly_tips() {
        let layout = LayoutResult {
            faces: vec![],
            width: 210.0,
            height: 297.0,
        };
        
        let analysis = ModelAnalysis {
            face_count: 50,
            estimated_sheet_count: 5,
            complexity_score: 50.0,
        };
        
        let tips = generate_assembly_tips(&layout, &analysis);
        
        // Проверяем, что генерируются базовые рекомендации
        assert!(tips.contains(&"Начните сборку с центральных элементов".to_string()));
        assert!(tips.contains(&"Склейте все вкладыши перед финальной сборкой".to_string()));
        assert!(tips.contains(&"Используйте линейку для точного сгибания".to_string()));
    }
    
    #[test]
    fn test_generate_assembly_tips_complex_model() {
        let layout = LayoutResult {
            faces: vec![],
            width: 210.0,
            height: 297.0,
        };
        
        let analysis = ModelAnalysis {
            face_count: 150,
            estimated_sheet_count: 15,
            complexity_score: 80.0,
        };
        
        let tips = generate_assembly_tips(&layout, &analysis);
        
        // Проверяем, что для сложной модели генерируются дополнительные рекомендации
        assert!(tips.contains(&"Рекомендуется собирать модель по частям".to_string()));
        assert!(tips.contains(&"Используйте клей-карандаш для точечной фиксации".to_string()));
    }
    
    #[test]
    fn test_generate_assembly_tips_many_sheets() {
        let layout = LayoutResult {
            faces: vec![],
            width: 210.0,
            height: 297.0,
        };
        
        let analysis = ModelAnalysis {
            face_count: 200,
            estimated_sheet_count: 25,
            complexity_score: 90.0,
        };
        
        let tips = generate_assembly_tips(&layout, &analysis);
        
        // Проверяем, что для модели с большим количеством листов генерируется рекомендация по нумерации
        assert!(tips.contains(&"Рекомендуется нумеровать элементы перед сборкой".to_string()));
    }
}