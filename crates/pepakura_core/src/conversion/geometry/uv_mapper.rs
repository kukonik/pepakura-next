//! UV маппинг для текстур

use crate::pdo_parser::PdoModel;
use crate::geometry::{Vertex, Face};

/// UV маппер
pub struct UvMapper;

impl UvMapper {
    /// Вычисляет UV координаты для вершин
    ///
    /// Если PDO содержит UV координаты, использует их.
    /// Иначе применяет проекционную развертку.
    pub fn compute_uvs(
        __pdo_model: &PdoModel,
        vertices: &[Vertex],
        _faces: &[Face],
    ) -> Vec<[f64; 2]> {
        // Пока используем простую проекционную развертку
        // В будущем можно добавить поддержку UV из PDO
        Self::project_uvs(vertices)
    }

    /// Проекционная развертка UV (planar projection)
    pub fn project_uvs(vertices: &[Vertex]) -> Vec<[f64; 2]> {
        if vertices.is_empty() {
            return vec![];
        }

        // Вычисляем bounding box
        let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for vertex in vertices {
            for i in 0..3 {
                min[i] = min[i].min(vertex.position[i]);
                max[i] = max[i].max(vertex.position[i]);
            }
        }

        // Используем XY проекцию (можно улучшить, выбирая наибольшую грань)
        let width = (max[0] - min[0]).max(1e-10);
        let height = (max[1] - min[1]).max(1e-10);

        vertices
            .iter()
            .map(|vertex| {
                [
                    (vertex.position[0] - min[0]) / width,
                    (vertex.position[1] - min[1]) / height,
                ]
            })
            .collect()
    }

    /// Сферическая развертка UV
    pub fn spherical_uvs(vertices: &[Vertex]) -> Vec<[f64; 2]> {
        if vertices.is_empty() {
            return vec![];
        }

        // Вычисляем центроид
        let mut centroid = [0.0, 0.0, 0.0];
        for vertex in vertices {
            centroid[0] += vertex.position[0];
            centroid[1] += vertex.position[1];
            centroid[2] += vertex.position[2];
        }
        let count = vertices.len() as f64;
        centroid[0] /= count;
        centroid[1] /= count;
        centroid[2] /= count;

        vertices
            .iter()
            .map(|vertex| {
                // Вектор от центроида к вершине
                let dx = vertex.position[0] - centroid[0];
                let dy = vertex.position[1] - centroid[1];
                let dz = vertex.position[2] - centroid[2];

                // Сферические координаты
                let r = (dx * dx + dy * dy + dz * dz).sqrt();
                let theta = (dz / r).acos(); // Полярный угол
                let phi = (dy / (dx * dx + dy * dy).sqrt()).atan2(dx); // Азимутальный угол

                // Нормализуем к [0, 1]
                let u = (phi + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
                let v = theta / std::f64::consts::PI;

                [u, v]
            })
            .collect()
    }

    /// Цилиндрическая развертка UV
    pub fn cylindrical_uvs(vertices: &[Vertex], axis: usize) -> Vec<[f64; 2]> {
        if vertices.is_empty() {
            return vec![];
        }

        // Вычисляем центроид
        let mut centroid = [0.0, 0.0, 0.0];
        for vertex in vertices {
            centroid[0] += vertex.position[0];
            centroid[1] += vertex.position[1];
            centroid[2] += vertex.position[2];
        }
        let count = vertices.len() as f64;
        centroid[0] /= count;
        centroid[1] /= count;
        centroid[2] /= count;

        vertices
            .iter()
            .map(|vertex| {
                let dx = vertex.position[0] - centroid[0];
                let dy = vertex.position[1] - centroid[1];
                let dz = vertex.position[2] - centroid[2];

                let (u, v) = match axis {
                    0 => {
                        // Проекция на YZ: угол в плоскости YZ, atan2(dz, dy)
                        let angle = dy.atan2(dz);
                        let u = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
                        let v = dx;
                        (u, v)
                    }
                    1 => {
                        // Проекция на XZ: угол в плоскости XZ, atan2(dz, dx)
                        let angle = dx.atan2(dz);
                        let u = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
                        let v = dy;
                        (u, v)
                    }
                    _ => {
                        // Проекция на XY (default Z axis): угол в плоскости XY, atan2(dy, dx)
                        let angle = dx.atan2(dy);
                        let u = (angle + std::f64::consts::PI) / (2.0 * std::f64::consts::PI);
                        let v = dz;
                        (u, v)
                    }
                };

                [u, v]
            })
            .collect()
    }

    /// Box развертка UV (для кубических объектов)
    pub fn box_uvs(vertices: &[Vertex], faces: &[Face]) -> Vec<[f64; 2]> {
        if vertices.is_empty() {
            return vec![];
        }

        // Вычисляем нормали для каждой грани и определяем ось проекции
        let mut uvs = vec![[0.0, 0.0]; vertices.len()];

        // Вычисляем bounding box
        let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for vertex in vertices {
            for i in 0..3 {
                min[i] = min[i].min(vertex.position[i]);
                max[i] = max[i].max(vertex.position[i]);
            }
        }

        let size = [
            (max[0] - min[0]).max(1e-10),
            (max[1] - min[1]).max(1e-10),
            (max[2] - min[2]).max(1e-10),
        ];

        for face in faces {
            // Вычисляем нормаль грани
            let v0 = &vertices[face.vertices[0]].position;
            let v1 = &vertices[face.vertices[1]].position;
            let v2 = &vertices[face.vertices[2]].position;

            let edge1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
            let edge2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

            let normal = [
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ];

            // Выбираем ось проекции по наибольшей компоненте нормали
            let abs_normal = [normal[0].abs(), normal[1].abs(), normal[2].abs()];
            let (u_axis, v_axis) = if abs_normal[0] >= abs_normal[1] && abs_normal[0] >= abs_normal[2] {
                // X axis - проекция на YZ
                (1, 2)
            } else if abs_normal[1] >= abs_normal[2] {
                // Y axis - проекция на XZ
                (0, 2)
            } else {
                // Z axis - проекция на XY
                (0, 1)
            };

            // Вычисляем UV для каждой вершины грани
            for &vertex_idx in &face.vertices {
                let vertex = &vertices[vertex_idx];
                let u = (vertex.position[u_axis] - min[u_axis]) / size[u_axis];
                let v = (vertex.position[v_axis] - min[v_axis]) / size[v_axis];
                uvs[vertex_idx] = [u, v];
            }
        }

        uvs
    }

    /// Масштабирует UV координаты
    pub fn scale_uvs(uvs: &mut [[f64; 2]], scale: [f64; 2]) {
        for uv in uvs.iter_mut() {
            uv[0] *= scale[0];
            uv[1] *= scale[1];
        }
    }

    /// Сдвигает UV координаты
    pub fn translate_uvs(uvs: &mut [[f64; 2]], offset: [f64; 2]) {
        for uv in uvs.iter_mut() {
            uv[0] += offset[0];
            uv[1] += offset[1];
        }
    }

    /// Поворачивает UV координаты на 90 градусов
    pub fn rotate_uvs_90(uvs: &mut [[f64; 2]], clockwise: bool) {
        for uv in uvs.iter_mut() {
            if clockwise {
                let temp = uv[0];
                uv[0] = 1.0 - uv[1];
                uv[1] = temp;
            } else {
                let temp = uv[1];
                uv[1] = 1.0 - uv[0];
                uv[0] = temp;
            }
        }
    }

    /// Оптимизирует UV packing (простая версия)
    pub fn pack_uvs(uvs: &mut [[f64; 2]], padding: f64) {
        // Находим минимальные UV
        let mut min_u = f64::INFINITY;
        let mut min_v = f64::INFINITY;

        for uv in uvs.iter() {
            min_u = min_u.min(uv[0]);
            min_v = min_v.min(uv[1]);
        }

        // Сдвигаем к началу координат
        for uv in uvs.iter_mut() {
            uv[0] -= min_u - padding;
            uv[1] -= min_v - padding;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_uvs_unit_square() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 1.0, 0.0]),
            Vertex::new(3, [1.0, 1.0, 0.0]),
        ];

        let uvs = UvMapper::project_uvs(&vertices);

        assert_eq!(uvs.len(), 4);
        assert_eq!(uvs[0], [0.0, 0.0]);
        assert_eq!(uvs[1], [1.0, 0.0]);
        assert_eq!(uvs[2], [0.0, 1.0]);
        assert_eq!(uvs[3], [1.0, 1.0]);
    }

    #[test]
    fn test_project_uvs_empty() {
        let vertices: Vec<Vertex> = vec![];
        let uvs = UvMapper::project_uvs(&vertices);
        assert!(uvs.is_empty());
    }

    #[test]
    fn test_spherical_uvs() {
        let vertices = vec![
            Vertex::new(0, [1.0, 0.0, 0.0]),
            Vertex::new(1, [0.0, 1.0, 0.0]),
            Vertex::new(2, [0.0, 0.0, 1.0]),
        ];

        let uvs = UvMapper::spherical_uvs(&vertices);

        assert_eq!(uvs.len(), 3);
        // Проверяем, что UV в диапазоне [0, 1]
        for uv in &uvs {
            assert!(uv[0] >= 0.0 && uv[0] <= 1.0);
            assert!(uv[1] >= 0.0 && uv[1] <= 1.0);
        }
    }

    #[test]
    fn test_scale_uvs() {
        let mut uvs = vec![[0.5, 0.5], [1.0, 1.0]];
        UvMapper::scale_uvs(&mut uvs, [2.0, 0.5]);
        assert_eq!(uvs[0], [1.0, 0.25]);
        assert_eq!(uvs[1], [2.0, 0.5]);
    }

    #[test]
    fn test_translate_uvs() {
        let mut uvs = vec![[0.0, 0.0], [1.0, 1.0]];
        UvMapper::translate_uvs(&mut uvs, [0.1, 0.2]);
        assert_eq!(uvs[0], [0.1, 0.2]);
        assert_eq!(uvs[1], [1.1, 1.2]);
    }

    #[test]
    fn test_rotate_uvs_90_clockwise() {
        let mut uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
        UvMapper::rotate_uvs_90(&mut uvs, true);
        
        // Проверяем, что значения в диапазоне [0, 1]
        for uv in &uvs {
            assert!(uv[0] >= 0.0 && uv[0] <= 1.0);
            assert!(uv[1] >= 0.0 && uv[1] <= 1.0);
        }
    }
}
