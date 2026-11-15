//! Модуль для работы с вершинами меша.

use serde::{Deserialize, Serialize};

/// Вершина 3D-меша.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vertex {
    pub id: usize,
    pub position: [f64; 3],
    pub normal: Option<[f64; 3]>,
    pub uv: Option<[f64; 2]>,
}

impl Vertex {
    pub fn new(id: usize, position: [f64; 3]) -> Self {
        Self { id, position, normal: None, uv: None }
    }

    pub fn with_normal(id: usize, position: [f64; 3], normal: [f64; 3]) -> Self {
        Self { id, position, normal: Some(normal), uv: None }
    }

    pub fn with_normal_and_uv(id: usize, position: [f64; 3], normal: [f64; 3], uv: [f64; 2]) -> Self {
        Self { id, position, normal: Some(normal), uv: Some(uv) }
    }

    pub fn distance_to(&self, other: &Vertex) -> f64 {
        let dx = self.position[0] - other.position[0];
        let dy = self.position[1] - other.position[1];
        let dz = self.position[2] - other.position[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

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
    fn test_vertex_distance() {
        let v1 = Vertex::new(0, [0.0, 0.0, 0.0]);
        let v2 = Vertex::new(1, [3.0, 4.0, 0.0]);
        assert_eq!(v1.distance_to(&v2), 5.0);
    }
}