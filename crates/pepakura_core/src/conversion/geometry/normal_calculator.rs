//! Калькулятор нормалей для меша

use crate::geometry::{Vertex, Face};
use crate::compat::*;

/// Калькулятор нормалей для меша
pub struct NormalCalculator;

impl NormalCalculator {
    /// Вычисляет нормали для всех вершин
    pub fn compute_normals(vertices: &[Vertex], faces: &[Face]) -> Vec<[f64; 3]> {
        let mut normals = vec![[0.0, 0.0, 0.0]; vertices.len()];

        // Accumulate face normals at each vertex
        for face in faces {
            let v0 = &vertices[face.vertices[0]].position;
            let v1 = &vertices[face.vertices[1]].position;
            let v2 = &vertices[face.vertices[2]].position;

            // Векторы сторон
            let edge1 = [
                v1[0] - v0[0],
                v1[1] - v0[1],
                v1[2] - v0[2],
            ];
            let edge2 = [
                v2[0] - v0[0],
                v2[1] - v0[1],
                v2[2] - v0[2],
            ];

            // Векторное произведение
            let normal = [
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ];

            // Добавляем нормаль к каждой вершине грани
            for &vertex_idx in &face.vertices {
                normals[vertex_idx][0] += normal[0];
                normals[vertex_idx][1] += normal[1];
                normals[vertex_idx][2] += normal[2];
            }
        }

        // Нормализация (параллельно)
        normals.par_iter_mut().for_each(|normal| {
            let len =
                (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
            if len > 1e-10 {
                normal[0] /= len;
                normal[1] /= len;
                normal[2] /= len;
            }
        });

        normals
    }

    /// Вычисляет нормали для одной грани
    pub fn compute_face_normal(
        v0: &[f64; 3],
        v1: &[f64; 3],
        v2: &[f64; 3],
    ) -> [f64; 3] {
        let edge1 = [
            v1[0] - v0[0],
            v1[1] - v0[1],
            v1[2] - v0[2],
        ];
        let edge2 = [
            v2[0] - v0[0],
            v2[1] - v0[1],
            v2[2] - v0[2],
        ];

        let mut normal = [
            edge1[1] * edge2[2] - edge1[2] * edge2[1],
            edge1[2] * edge2[0] - edge1[0] * edge2[2],
            edge1[0] * edge2[1] - edge1[1] * edge2[0],
        ];

        // Нормализация
        let len = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
        if len > 1e-10 {
            normal[0] /= len;
            normal[1] /= len;
            normal[2] /= len;
        }

        normal
    }

    /// Вычисляет нормали с проверкой на вырожденные грани
    pub fn compute_normals_safe(
        vertices: &[Vertex],
        faces: &[Face],
    ) -> (Vec<[f64; 3]>, Vec<usize>) {
        let mut normals = vec![[0.0, 0.0, 0.0]; vertices.len()];
        let mut degenerate_faces = Vec::new();

        for (face_idx, face) in faces.iter().enumerate() {
            let v0 = &vertices[face.vertices[0]].position;
            let v1 = &vertices[face.vertices[1]].position;
            let v2 = &vertices[face.vertices[2]].position;

            let edge1 = [
                v1[0] - v0[0],
                v1[1] - v0[1],
                v1[2] - v0[2],
            ];
            let edge2 = [
                v2[0] - v0[0],
                v2[1] - v0[1],
                v2[2] - v0[2],
            ];

            let normal = [
                edge1[1] * edge2[2] - edge1[2] * edge2[1],
                edge1[2] * edge2[0] - edge1[0] * edge2[2],
                edge1[0] * edge2[1] - edge1[1] * edge2[0],
            ];

            let len = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();

            if len < 1e-10 {
                // Вырожденная грань
                degenerate_faces.push(face_idx);
                continue;
            }

            // Добавляем нормаль к каждой вершине грани
            for &vertex_idx in &face.vertices {
                normals[vertex_idx][0] += normal[0];
                normals[vertex_idx][1] += normal[1];
                normals[vertex_idx][2] += normal[2];
            }
        }

        // Нормализация
        normals.par_iter_mut().for_each(|normal| {
            let len =
                (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
            if len > 1e-10 {
                normal[0] /= len;
                normal[1] /= len;
                normal[2] /= len;
            }
        });

        (normals, degenerate_faces)
    }

    /// Инвертирует направление нормалей
    pub fn flip_normals(normals: &mut [[f64; 3]]) {
        normals.par_iter_mut().for_each(|normal| {
            normal[0] = -normal[0];
            normal[1] = -normal[1];
            normal[2] = -normal[2];
        });
    }

    /// Проверяет ориентацию нормалей (должны смотреть наружу)
    pub fn check_normal_orientation(
        vertices: &[Vertex],
        normals: &[[f64; 3]],
        faces: &[Face],
    ) -> f64 {
        let centroid = Self::compute_centroid(vertices);

        let mut total_dot = 0.0;
        let mut count = 0;

        for face in faces {
            for &vertex_idx in &face.vertices {
                let vertex = &vertices[vertex_idx];
                let normal = normals[vertex_idx];

                // Вектор от центроида к вершине
                let to_vertex = [
                    vertex.position[0] - centroid[0],
                    vertex.position[1] - centroid[1],
                    vertex.position[2] - centroid[2],
                ];

                // Скалярное произведение
                let dot = normal[0] * to_vertex[0]
                    + normal[1] * to_vertex[1]
                    + normal[2] * to_vertex[2];

                total_dot += dot;
                count += 1;
            }
        }

        if count == 0 {
            0.0
        } else {
            total_dot / count as f64
        }
    }

    /// Вычисляет центроид меша
    fn compute_centroid(vertices: &[Vertex]) -> [f64; 3] {
        if vertices.is_empty() {
            return [0.0, 0.0, 0.0];
        }

        let mut sum = [0.0, 0.0, 0.0];
        for vertex in vertices {
            sum[0] += vertex.position[0];
            sum[1] += vertex.position[1];
            sum[2] += vertex.position[2];
        }

        let count = vertices.len() as f64;
        [sum[0] / count, sum[1] / count, sum[2] / count]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_normals_cube_corner() {
        // Простой тест: один треугольник в XY плоскости
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 1.0, 0.0]),
        ];

        let faces = vec![Face::new(0, 1, 2)];

        let normals = NormalCalculator::compute_normals(&vertices, &faces);

        // Нормаль должна смотреть вдоль Z
        assert!(normals[0][2].abs() - 1.0 < 0.0001);
        assert!(normals[1][2].abs() - 1.0 < 0.0001);
        assert!(normals[2][2].abs() - 1.0 < 0.0001);
    }

    #[test]
    fn test_compute_face_normal() {
        let v0 = [0.0, 0.0, 0.0];
        let v1 = [1.0, 0.0, 0.0];
        let v2 = [0.0, 1.0, 0.0];

        let normal = NormalCalculator::compute_face_normal(&v0, &v1, &v2);

        // Нормаль должна быть [0, 0, 1] или [0, 0, -1]
        assert!(normal[0].abs() < 0.0001);
        assert!(normal[1].abs() < 0.0001);
        assert!((normal[2].abs() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_flip_normals() {
        let mut normals = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

        NormalCalculator::flip_normals(&mut normals);

        assert_eq!(normals[0], [-1.0, 0.0, 0.0]);
        assert_eq!(normals[1], [0.0, -1.0, 0.0]);
        assert_eq!(normals[2], [0.0, 0.0, -1.0]);
    }

    #[test]
    fn test_degenerate_face_detection() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [2.0, 0.0, 0.0]), // Коллинеарные точки
        ];

        let faces = vec![Face::new(0, 1, 2)];

        let (_, degenerate) = NormalCalculator::compute_normals_safe(&vertices, &faces);

        assert_eq!(degenerate.len(), 1);
        assert_eq!(degenerate[0], 0);
    }
}
