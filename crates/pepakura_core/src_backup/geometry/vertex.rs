//! Модуль для работы с вершинами меша.

use serde::{Deserialize, Serialize};

/// Вершина 3D-меша.
/// 
/// Содержит позицию в 3D пространстве, опциональную нормаль и UV-координаты.
/// 
/// # Примеры
/// 
/// ```
/// use pepakura_core::geometry::Vertex;
/// 
/// let vertex = Vertex::new(0, [1.0, 2.0, 3.0]);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vertex {
    /// Уникальный идентификатор вершины
    pub id: usize,
    /// Позиция в 3D пространстве [x, y, z]
    pub position: [f64; 3],
    /// Нормаль вершины (опционально)
    pub normal: Option<[f64; 3]>,
    /// UV-координаты для текстур (опционально)
    pub uv: Option<[f64; 2]>,
}

impl Vertex {
    /// Создаёт новую вершину с заданным id и позицией.
    /// 
    /// # Аргументы
    /// * `id` - уникальный идентификатор вершины
    /// * `position` - позиция [x, y, z]
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::Vertex;
    /// 
    /// let vertex = Vertex::new(0, [1.0, 2.0, 3.0]);
    /// assert_eq!(vertex.id, 0);
    /// assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
    /// ```
    pub fn new(id: usize, position: [f64; 3]) -> Self {
        Self {
            id,
            position,
            normal: None,
            uv: None,
        }
    }

    /// Создаёт вершину с нормалью.
    /// 
    /// # Аргументы
    /// * `id` - уникальный идентификатор
    /// * `position` - позиция [x, y, z]
    /// * `normal` - нормаль [nx, ny, nz]
    pub fn with_normal(id: usize, position: [f64; 3], normal: [f64; 3]) -> Self {
        Self {
            id,
            position,
            normal: Some(normal),
            uv: None,
        }
    }

    /// Создаёт вершину с нормалью и UV-координатами.
    pub fn with_normal_and_uv(
        id: usize,
        position: [f64; 3],
        normal: [f64; 3],
        uv: [f64; 2],
    ) -> Self {
        Self {
            id,
            position,
            normal: Some(normal),
            uv: Some(uv),
        }
    }

    /// Вычисляет расстояние до другой вершины.
    /// 
    /// # Примеры
    /// 
    /// ```
    /// use pepakura_core::geometry::Vertex;
    /// 
    /// let v1 = Vertex::new(0, [0.0, 0.0, 0.0]);
    /// let v2 = Vertex::new(1, [1.0, 0.0, 0.0]);
    /// assert_eq!(v1.distance_to(&v2), 1.0);
    /// ```
    pub fn distance_to(&self, other: &Vertex) -> f64 {
        let dx = self.position[0] - other.position[0];
        let dy = self.position[1] - other.position[1];
        let dz = self.position[2] - other.position[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Возвращает середину между двумя вершинами.
    pub fn midpoint(&self, other: &Vertex) -> [f64; 3] {
        [
            (self.position[0] + other.position[0]) / 2.0,
            (self.position[1] + other.position[1]) / 2.0,
            (self.position[2] + other.position[2]) / 2.0,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_new() {
        let vertex = Vertex::new(0, [1.0, 2.0, 3.0]);
        assert_eq!(vertex.id, 0);
        assert_eq!(vertex.position, [1.0, 2.0, 3.0]);
        assert!(vertex.normal.is_none());
        assert!(vertex.uv.is_none());
    }

    #[test]
    fn test_vertex_with_normal() {
        let vertex = Vertex::with_normal(0, [1.0, 2.0, 3.0], [0.0, 1.0, 0.0]);
        assert_eq!(vertex.normal, Some([0.0, 1.0, 0.0]));
    }

    #[test]
    fn test_vertex_distance() {
        let v1 = Vertex::new(0, [0.0, 0.0, 0.0]);
        let v2 = Vertex::new(1, [3.0, 4.0, 0.0]);
        assert_eq!(v1.distance_to(&v2), 5.0);
    }

    #[test]
    fn test_vertex_midpoint() {
        let v1 = Vertex::new(0, [0.0, 0.0, 0.0]);
        let v2 = Vertex::new(1, [2.0, 2.0, 2.0]);
        assert_eq!(v1.midpoint(&v2), [1.0, 1.0, 1.0]);
    }
}
