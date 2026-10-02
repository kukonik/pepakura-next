//! Vertex definition for 3D meshes.

use serde::{Deserialize, Serialize};

/// A vertex in 3D space with optional normal and UV coordinates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub id: usize,
    pub position: [f64; 3],
    pub normal: Option<[f64; 3]>,
    pub uv: Option<[f64; 2]>,
}

impl Vertex {
    /// Creates a new vertex with position only.
    pub fn new(id: usize, position: [f64; 3]) -> Self {
        Vertex {
            id,
            position,
            normal: None,
            uv: None,
        }
    }

    /// Creates a vertex with position and normal.
    pub fn with_normal(id: usize, position: [f64; 3], normal: [f64; 3]) -> Self {
        Vertex {
            id,
            position,
            normal: Some(normal),
            uv: None,
        }
    }

    /// Creates a vertex with position, normal, and UV.
    pub fn with_all(id: usize, position: [f64; 3], normal: [f64; 3], uv: [f64; 2]) -> Self {
        Vertex {
            id,
            position,
            normal: Some(normal),
            uv: Some(uv),
        }
    }

    /// Returns the distance to another vertex.
    pub fn distance_to(&self, other: &Vertex) -> f64 {
        let dx = self.position[0] - other.position[0];
        let dy = self.position[1] - other.position[1];
        let dz = self.position[2] - other.position[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_new() {
        let v = Vertex::new(0, [1.0, 2.0, 3.0]);
        assert_eq!(v.id, 0);
        assert_eq!(v.position, [1.0, 2.0, 3.0]);
        assert!(v.normal.is_none());
        assert!(v.uv.is_none());
    }

    #[test]
    fn test_vertex_with_normal() {
        let v = Vertex::with_normal(1, [0.0, 0.0, 0.0], [0.0, 0.0, 1.0]);
        assert_eq!(v.id, 1);
        assert_eq!(v.normal, Some([0.0, 0.0, 1.0]));
        assert!(v.uv.is_none());
    }

    #[test]
    fn test_vertex_with_all() {
        let v = Vertex::with_all(2, [1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.5, 0.5]);
        assert_eq!(v.id, 2);
        assert_eq!(v.normal, Some([0.0, 1.0, 0.0]));
        assert_eq!(v.uv, Some([0.5, 0.5]));
    }

    #[test]
    fn test_distance_to() {
        let v1 = Vertex::new(0, [0.0, 0.0, 0.0]);
        let v2 = Vertex::new(1, [3.0, 4.0, 0.0]);
        assert_eq!(v1.distance_to(&v2), 5.0);
    }
}