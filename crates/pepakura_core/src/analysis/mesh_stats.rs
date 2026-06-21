//! # Mesh Statistics
//!
//! Вычисление статистики 3D меша для анализа.
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::analysis::mesh_stats::MeshStats;
//! use pepakura_core::geometry::Mesh;
//!
//! let mesh = Mesh::new("test");
//! let stats = MeshStats::from_mesh(&mesh);
//! println!("{}", stats.summary());
//! ```

use crate::geometry::{Mesh, Face};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// Статистика меша
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStats {
    /// Количество вершин
    pub vertex_count: u32,
    /// Количество граней
    pub face_count: u32,
    /// Количество рёбер (вычисляется)
    pub edge_count: u32,
    /// Размеры bounding box (x, y, z)
    pub bbox_size: [f64; 3],
    /// Центр bounding box
    pub bbox_center: [f64; 3],
    /// Площадь поверхности
    pub surface_area: f64,
    /// Объём (если замкнутая модель, вычисляется через дивергенцию)
    pub volume: Option<f64>,
    /// Средняя площадь грани
    pub avg_face_area: f64,
    /// Минимальная площадь грани
    pub min_face_area: f64,
    /// Максимальная площадь грани
    pub max_face_area: f64,
    /// Количество изолированных частей (компонент связности)
    pub isolated_parts: u32,
    /// Среднее количество рёбер на грань
    pub avg_edges_per_face: f64,
    /// Соотношение сторон bounding box
    pub aspect_ratio: f64,
}

impl MeshStats {
    /// Вычислить статистику из меша
    pub fn from_mesh(mesh: &Mesh) -> Self {
        let vertex_count = mesh.vertices.len() as u32;
        let face_count = mesh.faces.len() as u32;

        // Вычисляем bounding box
        let (min, max) = Self::compute_bbox(mesh);
        let bbox_size = [
            max[0] - min[0],
            max[1] - min[1],
            max[2] - min[2],
        ];
        let bbox_center = [
            (min[0] + max[0]) / 2.0,
            (min[1] + max[1]) / 2.0,
            (min[2] + max[2]) / 2.0,
        ];

        // Вычисляем площади граней
        let mut face_areas: Vec<f64> = Vec::with_capacity(mesh.faces.len());
        let mut total_area = 0.0;
        let mut min_area = f64::MAX;
        let mut max_area = f64::MIN;

        for face in &mesh.faces {
            let area = Self::compute_face_area(mesh, face);
            face_areas.push(area);
            total_area += area;
            if area < min_area {
                min_area = area;
            }
            if area > max_area {
                max_area = area;
            }
        }

        let avg_face_area = if face_count > 0 {
            total_area / face_count as f64
        } else {
            0.0
        };

        // Вычисляем количество рёбер (каждое ребро принадлежит 2 граням в среднем)
        let edge_count = Self::count_edges(mesh);

        // Вычисляем объём (если меш замкнутый)
        let volume = Self::compute_volume(mesh);

        // Вычисляем количество изолированных частей
        let isolated_parts = Self::count_isolated_parts(mesh);

        // Среднее количество рёбер на грань
        let total_edges_in_faces: u32 = mesh.faces.iter()
            .map(|f| f.vertices.len() as u32)
            .sum();
        let avg_edges_per_face = if face_count > 0 {
            total_edges_in_faces as f64 / face_count as f64
        } else {
            0.0
        };

        // Соотношение сторон
        let aspect_ratio = if bbox_size.iter().all(|&s| s > 0.0) {
            bbox_size.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                / bbox_size.iter().cloned().fold(f64::INFINITY, f64::min)
        } else {
            1.0
        };

        MeshStats {
            vertex_count,
            face_count,
            edge_count,
            bbox_size,
            bbox_center,
            surface_area: total_area,
            volume,
            avg_face_area,
            min_face_area: if min_area == f64::MAX { 0.0 } else { min_area },
            max_face_area: if max_area == f64::MIN { 0.0 } else { max_area },
            isolated_parts,
            avg_edges_per_face,
            aspect_ratio,
        }
    }

    /// Вычислить bounding box
    fn compute_bbox(mesh: &Mesh) -> ([f64; 3], [f64; 3]) {
        let mut min = [f64::MAX, f64::MAX, f64::MAX];
        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for vertex in &mesh.vertices {
            let pos = vertex.position;
            min[0] = min[0].min(pos[0]);
            min[1] = min[1].min(pos[1]);
            min[2] = min[2].min(pos[2]);
            max[0] = max[0].max(pos[0]);
            max[1] = max[1].max(pos[1]);
            max[2] = max[2].max(pos[2]);
        }

        (min, max)
    }

    /// Вычислить площадь грани
    fn compute_face_area(mesh: &Mesh, face: &Face) -> f64 {
        let indices = &face.vertices;
        
        if indices.len() < 3 {
            return 0.0;
        }

        // Для треугольника - площадь через векторное произведение
        if indices.len() == 3 {
            let v0 = Vector3::new(
                mesh.vertices[indices[0]].position[0],
                mesh.vertices[indices[0]].position[1],
                mesh.vertices[indices[0]].position[2],
            );
            let v1 = Vector3::new(
                mesh.vertices[indices[1]].position[0],
                mesh.vertices[indices[1]].position[1],
                mesh.vertices[indices[1]].position[2],
            );
            let v2 = Vector3::new(
                mesh.vertices[indices[2]].position[0],
                mesh.vertices[indices[2]].position[1],
                mesh.vertices[indices[2]].position[2],
            );

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let cross = edge1.cross(&edge2);
            return cross.norm() / 2.0;
        }

        // Для многоугольника - триангуляция от первой вершины
        let mut total_area = 0.0;
        let v0 = Vector3::new(
            mesh.vertices[indices[0]].position[0],
            mesh.vertices[indices[0]].position[1],
            mesh.vertices[indices[0]].position[2],
        );

        for i in 1..indices.len() - 1 {
            let v1 = Vector3::new(
                mesh.vertices[indices[i]].position[0],
                mesh.vertices[indices[i]].position[1],
                mesh.vertices[indices[i]].position[2],
            );
            let v2 = Vector3::new(
                mesh.vertices[indices[i + 1]].position[0],
                mesh.vertices[indices[i + 1]].position[1],
                mesh.vertices[indices[i + 1]].position[2],
            );

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let cross = edge1.cross(&edge2);
            total_area += cross.norm() / 2.0;
        }

        total_area
    }

    /// Посчитать количество уникальных рёбер
    fn count_edges(mesh: &Mesh) -> u32 {
        use std::collections::HashSet;

        let mut edges: HashSet<(u32, u32)> = HashSet::new();

        for face in &mesh.faces {
            let indices = &face.vertices;
            for i in 0..indices.len() {
                let j = (i + 1) % indices.len();
                let a = indices[i] as u32;
                let b = indices[j] as u32;
                // Нормализуем ребро (меньший индекс первый)
                let edge = if a < b { (a, b) } else { (b, a) };
                edges.insert(edge);
            }
        }

        edges.len() as u32
    }

    /// Вычислить объём через дивергенцию (для замкнутых мешей)
    fn compute_volume(mesh: &Mesh) -> Option<f64> {
        let mut volume = 0.0;

        for face in &mesh.faces {
            let indices = &face.vertices;
            if indices.len() < 3 {
                continue;
            }

            // Триангуляция и вычисление объёма каждого тетраэдра
            for i in 1..indices.len() - 1 {
                let v0 = mesh.vertices.get(indices[0])?;
                let v1 = mesh.vertices.get(indices[i])?;
                let v2 = mesh.vertices.get(indices[i + 1])?;

                let p0 = Vector3::new(v0.position[0], v0.position[1], v0.position[2]);
                let p1 = Vector3::new(v1.position[0], v1.position[1], v1.position[2]);
                let p2 = Vector3::new(v2.position[0], v2.position[1], v2.position[2]);

                // Объём тетраэдра = (a · (b × c)) / 6
                volume += p0.dot(&p1.cross(&p2)) / 6.0;
            }
        }

        // Если объём отрицательный - нормали направлены внутрь
        Some(volume.abs())
    }

    /// Посчитать количество изолированных частей через BFS
    fn count_isolated_parts(mesh: &Mesh) -> u32 {
        if mesh.vertices.is_empty() {
            return 0;
        }

        use std::collections::{HashSet, VecDeque};

        let mut visited = HashSet::new();
        let mut parts = 0u32;

        // Строим граф смежности
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); mesh.vertices.len()];

        for face in &mesh.faces {
            let indices = &face.vertices;
            for i in 0..indices.len() {
                for j in (i + 1)..indices.len() {
                    let a = indices[i];
                    let b = indices[j];
                    adjacency[a].push(b);
                    adjacency[b].push(a);
                }
            }
        }

        // BFS для поиска компонент связности
        for start in 0..mesh.vertices.len() {
            if visited.contains(&start) {
                continue;
            }

            parts += 1;
            let mut queue = VecDeque::new();
            queue.push_back(start);
            visited.insert(start);

            while let Some(current) = queue.pop_front() {
                for &neighbor in &adjacency[current] {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        parts
    }

    /// Краткое резюме статистики
    pub fn summary(&self) -> String {
        format!(
            "Меш: {} вершин, {} граней, {} рёбер | {:.1}×{:.1}×{:.1} мм | S={:.1} мм²{}",
            self.vertex_count,
            self.face_count,
            self.edge_count,
            self.bbox_size[0],
            self.bbox_size[1],
            self.bbox_size[2],
            self.surface_area,
            self.volume
                .map(|v| format!(" | V={:.1} мм³", v))
                .unwrap_or_default()
        )
    }

    /// Развёрнутая статистика
    pub fn detailed(&self) -> String {
        format!(
            r#"Статистика 3D модели:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Вершин:           {}
Граней:           {}
Рёбер:            {}
Граней/вершина:   {:.2}
Рёбер/грань:      {:.2}

Размеры (мм):     {:.2} × {:.2} × {:.2}
Соотношение:      {:.2}:1
Центр:            ({:.2}, {:.2}, {:.2})

Площадь:          {:.2} мм²
Объём:            {}
Сред. грань:      {:.4} мм²
Мин. грань:       {:.6} мм²
Макс. грань:      {:.2} мм²

Изолированных:    {} частей
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"#,
            self.vertex_count,
            self.face_count,
            self.edge_count,
            if self.vertex_count > 0 {
                self.face_count as f64 / self.vertex_count as f64
            } else {
                0.0
            },
            self.avg_edges_per_face,
            self.bbox_size[0],
            self.bbox_size[1],
            self.bbox_size[2],
            self.aspect_ratio,
            self.bbox_center[0],
            self.bbox_center[1],
            self.bbox_center[2],
            self.surface_area,
            self.volume
                .map(|v| format!("{:.2} мм³", v))
                .unwrap_or("н/д (не замкнута)".to_string()),
            self.avg_face_area,
            self.min_face_area,
            self.max_face_area,
            self.isolated_parts
        )
    }

    #[cfg(feature = "llm")]
    /// Конвертировать в промпт для LLM
    pub fn to_prompt(&self) -> String {
        use crate::ai::prompts::MeshStatsPrompt;

        let prompt_stats = MeshStatsPrompt {
            vertex_count: self.vertex_count,
            face_count: self.face_count,
            edge_count: self.edge_count,
            bbox_size: self.bbox_size,
            surface_area: self.surface_area,
            volume: self.volume,
            avg_face_area: self.avg_face_area,
            min_face_area: self.min_face_area,
            max_face_area: self.max_face_area,
            isolated_parts: self.isolated_parts,
        };

        prompt_stats.to_text()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Vertex, Face};

    #[test]
    fn test_cube_stats() {
        // Создаём простой куб 100×100×100
        let mut mesh = Mesh::new("cube");

        // 8 вершин куба
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [100.0, 0.0, 0.0]),
            Vertex::new(2, [100.0, 100.0, 0.0]),
            Vertex::new(3, [0.0, 100.0, 0.0]),
            Vertex::new(4, [0.0, 0.0, 100.0]),
            Vertex::new(5, [100.0, 0.0, 100.0]),
            Vertex::new(6, [100.0, 100.0, 100.0]),
            Vertex::new(7, [0.0, 100.0, 100.0]),
        ];

        for v in vertices {
            mesh.add_vertex(v);
        }

        // 12 треугольных граней (2 на каждую сторону куба)
        let faces = vec![
            // Низ
            vec![0, 2, 1],
            vec![0, 3, 2],
            // Верх
            vec![4, 5, 6],
            vec![4, 6, 7],
            // Перед
            vec![0, 1, 5],
            vec![0, 5, 4],
            // Зад
            vec![2, 3, 7],
            vec![2, 7, 6],
            // Лево
            vec![0, 4, 7],
            vec![0, 7, 3],
            // Право
            vec![1, 2, 6],
            vec![1, 6, 5],
        ];

        for face_indices in faces {
            mesh.add_face(Face::new(face_indices[0], face_indices[1], face_indices[2]));
        }

        let stats = MeshStats::from_mesh(&mesh);

        assert_eq!(stats.vertex_count, 8);
        assert_eq!(stats.face_count, 12);
        assert_eq!(stats.edge_count, 18); // 12 рёбер куба, но каждое делится на 2 треугольника
        assert!((stats.bbox_size[0] - 100.0).abs() < 0.01);
        assert!((stats.bbox_size[1] - 100.0).abs() < 0.01);
        assert!((stats.bbox_size[2] - 100.0).abs() < 0.01);
        assert!((stats.surface_area - 60000.0).abs() < 100.0); // 6 сторон × 100×100
        assert!(stats.volume.is_some());
        assert!((stats.volume.unwrap() - 1000000.0).abs() < 10000.0); // 100³
        assert_eq!(stats.isolated_parts, 1);
    }

    #[test]
    fn test_summary() {
        let mut mesh = Mesh::new("test");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [10.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.0, 10.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));

        let stats = MeshStats::from_mesh(&mesh);
        let summary = stats.summary();

        assert!(summary.contains("3 вершин"));
        assert!(summary.contains("1 граней"));
        assert!(summary.contains("S="));
    }
}

