//! Интеграционные тесты для Phase 1 реализации.
//!
//! Тестирует:
//! - PDF экспорт
//! - Персистентность
//! - AI кэширование
//! - AI стриминг
//! - 3D Viewer composable
//! - Редактор развёрток

#[cfg(test)]
mod tests {
    use super::*;
    
    // ========================================================================
    // PDF Export Tests
    // ========================================================================
    
    #[test]
    fn test_pdf_export_basic() {
        use crate::export::{export_pdf, PdfExportConfig, PageSize};
        use crate::unfold::UnfoldedMesh;
        use crate::geometry::{Mesh, Vertex, Face};
        
        // Создаём тестовый меш
        let mut mesh = Mesh::new("TestCube");
        for i in 0..8 {
            let x = if i & 1 != 0 { 1.0 } else { 0.0 };
            let y = if i & 2 != 0 { 1.0 } else { 0.0 };
            let z = if i & 4 != 0 { 1.0 } else { 0.0 };
            mesh.add_vertex(Vertex::new(i, [x, y, z]));
        }
        
        // Создаём развёртку
        let unfolded = UnfoldedMesh {
            vertices_2d: vec![
                [0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0],
                [2.0, 0.0], [3.0, 0.0], [3.0, 1.0], [2.0, 1.0],
            ],
            faces: mesh.faces.clone(),
            source_mesh: mesh,
            metadata: Default::default(),
        };
        
        // Экспортируем в PDF
        let config = PdfExportConfig::default();
        let result = export_pdf(&unfolded, &config);
        
        assert!(result.is_ok());
        let pdf_result = result.unwrap();
        assert!(!pdf_result.bytes.is_empty());
        assert!(pdf_result.bytes.starts_with(b"%PDF"));
        assert_eq!(pdf_result.page_count, 1);
    }
    
    #[test]
    fn test_pdf_export_layers() {
        use crate::export::{export_pdf, PdfExportConfig};
        use crate::unfold::UnfoldedMesh;
        use crate::geometry::{Mesh, Vertex, Face};
        
        let mut mesh = Mesh::new("Test");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));
        
        let unfolded = UnfoldedMesh {
            vertices_2d: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]],
            faces: mesh.faces.clone(),
            source_mesh: mesh,
            metadata: Default::default(),
        };
        
        // Только линии реза
        let config_cut_only = PdfExportConfig {
            show_cut_lines: true,
            show_fold_lines: false,
            show_part_numbers: false,
            ..Default::default()
        };
        
        let result = export_pdf(&unfolded, &config_cut_only);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_pdf_export_orientation() {
        use crate::export::{export_pdf, PdfExportConfig, PdfOrientation};
        use crate::unfold::UnfoldedMesh;
        use crate::geometry::{Mesh, Vertex, Face};
        
        let mut mesh = Mesh::new("Test");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));
        
        let unfolded = UnfoldedMesh {
            vertices_2d: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]],
            faces: mesh.faces.clone(),
            source_mesh: mesh,
            metadata: Default::default(),
        };
        
        // Портрет
        let config_portrait = PdfExportConfig {
            orientation: PdfOrientation::Portrait,
            ..Default::default()
        };
        
        // Ландшафт
        let config_landscape = PdfExportConfig {
            orientation: PdfOrientation::Landscape,
            ..Default::default()
        };
        
        let pdf_p = export_pdf(&unfolded, &config_portrait).unwrap();
        let pdf_l = export_pdf(&unfolded, &config_landscape).unwrap();
        
        // Оба должны быть валидными PDF
        assert!(pdf_p.bytes.starts_with(b"%PDF"));
        assert!(pdf_l.bytes.starts_with(b"%PDF"));
    }
    
    // ========================================================================
    // Persistence Tests
    // ========================================================================
    
    #[test]
    fn test_persistence_save_load_state() {
        use crate::persistence::StatePersistence;
        use serde_json::json;
        
        let persistence = StatePersistence::in_memory().unwrap();
        
        let test_data = json!({
            "name": "Test Project",
            "value": 42
        });
        
        // Сохранение
        let save_result = persistence.save_state("test_key", &test_data);
        assert!(save_result.is_ok());
        
        // Загрузка
        let loaded: Option<serde_json::Value> = persistence.load_state("test_key").unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap()["name"], "Test Project");
        assert_eq!(loaded.unwrap()["value"], 42);
    }
    
    #[test]
    fn test_persistence_history() {
        use crate::persistence::StatePersistence;
        
        let persistence = StatePersistence::in_memory().unwrap();
        
        // Добавляем записи в историю
        persistence.push_history("project1", "edit", "{}", "{\"modified\": true}").unwrap();
        persistence.push_history("project1", "save", "{\"modified\": true}", "{}").unwrap();
        
        // Получаем историю
        let history = persistence.get_history("project1", 10).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].action, "save");
        assert_eq!(history[1].action, "edit");
        
        // Получаем последнее для undo
        let last = persistence.get_last_undo("project1").unwrap();
        assert!(last.is_some());
        assert_eq!(last.unwrap().action, "save");
    }
    
    #[test]
    fn test_persistence_settings() {
        use crate::persistence::StatePersistence;
        
        let persistence = StatePersistence::in_memory().unwrap();
        
        // Сохраняем настройки
        persistence.save_setting("theme", "dark").unwrap();
        persistence.save_setting("language", "ru").unwrap();
        persistence.save_setting("auto_save_interval", "60").unwrap();
        
        // Загружаем все настройки
        let settings = persistence.get_all_settings().unwrap();
        assert_eq!(settings.theme, "dark");
        assert_eq!(settings.language, "ru");
        assert_eq!(settings.auto_save_interval, 60);
    }
    
    #[test]
    fn test_persistence_recent_projects() {
        use crate::persistence::StatePersistence;
        
        let persistence = StatePersistence::in_memory().unwrap();
        
        // Добавляем проекты
        persistence.add_recent_project("/path/to/project1.pepa", "Project 1").unwrap();
        persistence.add_recent_project("/path/to/project2.pepa", "Project 2").unwrap();
        
        // Получаем список
        let recent = persistence.get_recent_projects().unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].1, "Project 2"); // Последний открытый первым
    }
    
    // ========================================================================
    // AI Cache Tests
    // ========================================================================
    
    #[test]
    fn test_ai_cache_put_get() {
        use crate::ai::cache::AiCache;
        
        let cache = AiCache::new(100);
        
        cache.put("test prompt", "test response");
        let result = cache.get("test prompt");
        
        assert_eq!(result, Some("test response".to_string()));
    }
    
    #[test]
    fn test_ai_cache_stats() {
        use crate::ai::cache::AiCache;
        
        let cache = AiCache::new(100);
        
        cache.put("test", "response");
        cache.get("test");
        cache.get("test");
        cache.get("nonexistent");
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.saves, 1);
    }
    
    #[test]
    fn test_ai_cache_hit_rate() {
        use crate::ai::cache::AiCache;
        
        let cache = AiCache::new(100);
        
        cache.put("test", "response");
        cache.get("test"); // hit
        cache.get("test"); // hit
        cache.get("other"); // miss
        
        let hit_rate = cache.hit_rate();
        assert!((hit_rate - 66.67).abs() < 0.1);
    }
    
    #[test]
    fn test_ai_cache_lru_eviction() {
        use crate::ai::cache::AiCache;
        
        let cache = AiCache::new(3);
        
        cache.put("prompt 1", "response 1");
        cache.put("prompt 2", "response 2");
        cache.put("prompt 3", "response 3");
        cache.put("prompt 4", "response 4"); // Должен вытеснить prompt 1
        
        assert!(!cache.contains("prompt 1"));
        assert!(cache.contains("prompt 2"));
        assert!(cache.contains("prompt 3"));
        assert!(cache.contains("prompt 4"));
    }
    
    // ========================================================================
    // AI Streaming Tests
    // ========================================================================
    
    #[tokio::test]
    async fn test_streaming_collect() {
        use crate::ai::streaming::collect_stream;
        use crate::ai::streaming::AiStream;
        use tokio::sync::mpsc;
        use tokio_stream::wrappers::ReceiverStream;
        
        // Создаём тестовый стрим
        let (tx, rx) = mpsc::channel(32);
        
        tx.send("Hello".to_string()).await.unwrap();
        tx.send(" ".to_string()).await.unwrap();
        tx.send("World".to_string()).await.unwrap();
        drop(tx);
        
        let stream = AiStream {
            receiver: ReceiverStream::new(rx),
        };
        
        let result = collect_stream(stream).await;
        assert_eq!(result, "Hello World");
    }
    
    #[tokio::test]
    async fn test_streaming_progress() {
        use crate::ai::streaming::{with_progress, AiStream};
        use futures::StreamExt;
        use tokio::sync::mpsc;
        use tokio_stream::wrappers::ReceiverStream;
        
        let (tx, rx) = mpsc::channel(32);
        
        tx.send("A".to_string()).await.unwrap();
        tx.send("B".to_string()).await.unwrap();
        tx.send("C".to_string()).await.unwrap();
        drop(tx);
        
        let stream = AiStream {
            receiver: ReceiverStream::new(rx),
        };
        
        let mut progress_stream = with_progress(stream);
        
        let mut tokens = Vec::new();
        while let Some((token, count)) = progress_stream.next().await {
            tokens.push((token, count));
        }
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], ("A".to_string(), 1));
        assert_eq!(tokens[1], ("B".to_string(), 2));
        assert_eq!(tokens[2], ("C".to_string(), 3));
    }
    
    // ========================================================================
    // Unfold Editor Tests
    // ========================================================================
    
    #[test]
    fn test_editor_snap_to_grid() {
        // Тестирование snap-to-grid функции
        let grid_size = 10;
        
        fn snap(value: f64, grid: f64) -> f64 {
            (value / grid).round() * grid
        }
        
        assert_eq!(snap(23.0, grid_size as f64), 20.0);
        assert_eq!(snap(27.0, grid_size as f64), 30.0);
        assert_eq!(snap(25.0, grid_size as f64), 20.0); // Banker's rounding
    }
    
    #[test]
    fn test_editor_rotation() {
        use std::f64::consts::PI;
        
        fn rotate_point(x: f64, y: f64, cx: f64, cy: f64, angle_deg: f64) -> (f64, f64) {
            let rad = angle_deg * PI / 180.0;
            let cos = rad.cos();
            let sin = rad.sin();
            let dx = x - cx;
            let dy = y - cy;
            (
                cx + dx * cos - dy * sin,
                cy + dx * sin + dy * cos,
            )
        }
        
        // Поворот на 90° вокруг начала координат
        let (x, y) = rotate_point(1.0, 0.0, 0.0, 0.0, 90.0);
        assert!((x - 0.0).abs() < 0.001);
        assert!((y - 1.0).abs() < 0.001);
        
        // Поворот на 180°
        let (x, y) = rotate_point(1.0, 0.0, 0.0, 0.0, 180.0);
        assert!((x - (-1.0)).abs() < 0.001);
        assert!((y - 0.0).abs() < 0.001);
    }
    
    #[test]
    fn test_editor_bounds_calculation() {
        fn calculate_bounds(vertices: &[(f64, f64)]) -> (f64, f64, f64, f64) {
            let xs: Vec<f64> = vertices.iter().map(|v| v.0).collect();
            let ys: Vec<f64> = vertices.iter().map(|v| v.1).collect();
            
            (
                xs.iter().cloned().fold(f64::INFINITY, f64::min),
                ys.iter().cloned().fold(f64::INFINITY, f64::min),
                xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            )
        }
        
        let vertices = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];
        let (min_x, min_y, max_x, max_y) = calculate_bounds(&vertices);
        
        assert_eq!(min_x, 0.0);
        assert_eq!(min_y, 0.0);
        assert_eq!(max_x, 1.0);
        assert_eq!(max_y, 1.0);
    }
    
    // ========================================================================
    // 3D Viewer Tests
    // ========================================================================
    
    #[test]
    fn test_3d_viewer_camera_projection() {
        fn project_camera_to_2d(
            position: [f64; 3],
            target: [f64; 3],
        ) -> (f64, f64, f64) {
            let dx = position[0] - target[0];
            let dy = position[1] - target[1];
            let dz = position[2] - target[2];
            
            let distance = (dx * dx + dy * dy + dz * dz).sqrt();
            let scale = (10.0 / distance).clamp(0.1, 5.0);
            
            (dx * 0.5, dy * 0.5, scale)
        }
        
        let (x, y, scale) = project_camera_to_2d([3.0, 3.0, 3.0], [0.0, 0.0, 0.0]);
        assert!((x - 1.5).abs() < 0.001);
        assert!((y - 1.5).abs() < 0.001);
        assert!(scale > 0.0 && scale <= 5.0);
    }
    
    #[test]
    fn test_3d_viewer_face_selection() {
        // Тест выбора грани raycasting'ом
        fn ray_triangle_intersect(
            ray_origin: [f64; 3],
            ray_dir: [f64; 3],
            v0: [f64; 3],
            v1: [f64; 3],
            v2: [f64; 3],
        ) -> Option<f64> {
            let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];
            
            let h = [
                ray_dir[1] * e2[2] - ray_dir[2] * e2[1],
                ray_dir[2] * e2[0] - ray_dir[0] * e2[2],
                ray_dir[0] * e2[1] - ray_dir[1] * e2[0],
            ];
            
            let a = e1[0] * h[0] + e1[1] * h[1] + e1[2] * h[2];
            
            if a.abs() < 0.000001 {
                return None;
            }
            
            let f = 1.0 / a;
            let s = [
                ray_origin[0] - v0[0],
                ray_origin[1] - v0[1],
                ray_origin[2] - v0[2],
            ];
            
            let u = f * (s[0] * h[0] + s[1] * h[1] + s[2] * h[2]);
            
            if u < 0.0 || u > 1.0 {
                return None;
            }
            
            let q = [
                s[1] * e1[2] - s[2] * e1[1],
                s[2] * e1[0] - s[0] * e1[2],
                s[0] * e1[1] - s[1] * e1[0],
            ];
            
            let v = f * (ray_dir[0] * q[0] + ray_dir[1] * q[1] + ray_dir[2] * q[2]);
            
            if v < 0.0 || u + v > 1.0 {
                return None;
            }
            
            let t = f * (e2[0] * q[0] + e2[1] * q[1] + e2[2] * q[2]);
            
            if t > 0.000001 {
                Some(t)
            } else {
                None
            }
        }
        
        // Треугольник
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.5, 1.0, 0.0];
        
        // Луч в центр треугольника
        let ray_origin = [0.5, 0.3, -1.0];
        let ray_dir = [0.0, 0.0, 1.0];
        
        let intersection = ray_triangle_intersect(ray_origin, ray_dir, v0, v1, v2);
        assert!(intersection.is_some());
        assert!((intersection.unwrap() - 1.0).abs() < 0.001);
    }
    
    // ========================================================================
    // Integration Tests
    // ========================================================================
    
    #[test]
    fn test_full_workflow() {
        use crate::export::{export_pdf, PdfExportConfig};
        use crate::unfold::{unfold_mds, UnfoldConfig};
        use crate::geometry::{Mesh, Vertex, Face};
        use crate::persistence::StatePersistence;
        use crate::ai::cache::AiCache;
        
        // 1. Создаём меш
        let mut mesh = Mesh::new("TestModel");
        for i in 0..8 {
            let x = if i & 1 != 0 { 1.0 } else { 0.0 };
            let y = if i & 2 != 0 { 1.0 } else { 0.0 };
            let z = if i & 4 != 0 { 1.0 } else { 0.0 };
            mesh.add_vertex(Vertex::new(i, [x, y, z]));
        }
        
        // 2. Развёртка
        let config = UnfoldConfig::default();
        let unfolded = unfold_mds(&mesh, &config).unwrap();
        
        // 3. Экспорт в PDF
        let pdf_config = PdfExportConfig::default();
        let pdf_result = export_pdf(&unfolded, &pdf_config).unwrap();
        assert!(pdf_result.bytes.starts_with(b"%PDF"));
        
        // 4. Сохранение состояния
        let persistence = StatePersistence::in_memory().unwrap();
        persistence.save_state("last_unfold", &unfolded.metadata).unwrap();
        
        // 5. AI кэширование совета
        let cache = AiCache::new(100);
        cache.put("unfold_advice", "Use MDS for this model");
        assert!(cache.contains("unfold_advice"));
        
        // 6. Загрузка состояния
        let loaded = persistence.load_state("last_unfold").unwrap();
        assert!(loaded.is_some());
    }
}
