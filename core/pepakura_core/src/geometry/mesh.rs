//! Mesh data structures and operations.

use serde::{Deserialize, Serialize};
use crate::geometry::vertex::Vertex;
use crate::errors::{PepakuraError, Result};

/// A triangle face defined by three vertex indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Face {
    pub vertices: [usize; 3],
    pub material_id: Option<usize>,
}

impl Face {
    /// Creates a new face with given vertex indices.
    pub fn new(v0: usize, v1: usize, v2: usize) -> Self {
        Face {
            vertices: [v0, v1, v2],
            material_id: None,
        }
    }

    /// Creates a face with material.
    pub fn with_material(v0: usize, v1: usize, v2: usize, material_id: usize) -> Self {
        Face {
            vertices: [v0, v1, v2],
            material_id: Some(material_id),
        }
    }

    /// Checks if the face contains a vertex.
    pub fn contains_vertex(&self, vertex_id: usize) -> bool {
        self.vertices[0] == vertex_id
            || self.vertices[1] == vertex_id
            || self.vertices[2] == vertex_id
    }
}

/// Metadata for a mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshMetadata {
    pub author: Option<String>,
    pub created_at: Option<String>,
    pub units: String, // "mm", "cm", "inch"
    pub bounding_box: Option<BoundingBox>,
}

impl Default for MeshMetadata {
    fn default() -> Self {
        MeshMetadata {
            author: None,
            created_at: None,
            units: "mm".to_string(),
            bounding_box: None,
        }
    }
}

/// Axis-aligned bounding box.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BoundingBox {
    pub fn new(min: [f64; 3], max: [f64; 3]) -> Self {
        BoundingBox { min, max }
    }

    pub fn width(&self) -> f64 {
        self.max[0] - self.min[0]
    }

    pub fn height(&self) -> f64 {
        self.max[1] - self.min[1]
    }

    pub fn depth(&self) -> f64 {
        self.max[2] - self.min[2]
    }

    pub fn volume(&self) -> f64 {
        self.width() * self.height() * self.depth()
    }
}

/// A 3D mesh consisting of vertices and faces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub name: String,
    pub metadata: MeshMetadata,
}

impl Mesh {
    /// Creates a new mesh with given vertices and faces.
    pub fn new(vertices: Vec<Vertex>, faces: Vec<Face>) -> Result<Self> {
        // Validate indices
        for face in &faces {
            for &vid in &face.vertices {
                if vid >= vertices.len() {
                    return Err(PepakuraError::InvalidMesh(format!(
                        "Vertex index {} out of bounds (max {})",
                        vid,
                        vertices.len() - 1
                    )));
                }
            }
        }

        Ok(Mesh {
            vertices,
            faces,
            name: String::from("Unnamed Mesh"),
            metadata: MeshMetadata::default(),
        })
    }

    /// Creates a mesh with a name.
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// Returns the number of vertices.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of faces.
    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    /// Computes the bounding box of the mesh.
    pub fn bounding_box(&self) -> BoundingBox {
        if self.vertices.is_empty() {
            return BoundingBox::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }

        let mut min = [f64::INFINITY, f64::INFINITY, f64::INFINITY];
        let mut max = [f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY];

        for vertex in &self.vertices {
            let pos = vertex.position;
            for i in 0..3 {
                if pos[i] < min[i] {
                    min[i] = pos[i];
                }
                if pos[i] > max[i] {
                    max[i] = pos[i];
                }
            }
        }

        BoundingBox::new(min, max)
    }

    /// Computes the centroid (average position) of the mesh.
    pub fn centroid(&self) -> [f64; 3] {
        if self.vertices.is_empty() {
            return [0.0, 0.0, 0.0];
        }

        let mut sum = [0.0, 0.0, 0.0];
        for vertex in &self.vertices {
            let pos = vertex.position;
            for i in 0..3 {
                sum[i] += pos[i];
            }
        }

        let count = self.vertices.len() as f64;
        [sum[0] / count, sum[1] / count, sum[2] / count]
    }

    /// Scales the mesh uniformly by a factor.
    pub fn scale(&mut self, factor: f64) {
        for vertex in &mut self.vertices {
            vertex.position[0] *= factor;
            vertex.position[1] *= factor;
            vertex.position[2] *= factor;
        }
    }

    /// Translates the mesh by an offset.
    pub fn translate(&mut self, offset: [f64; 3]) {
        for vertex in &mut self.vertices {
            vertex.position[0] += offset[0];
            vertex.position[1] += offset[1];
            vertex.position[2] += offset[2];
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vertex::Vertex;

    #[test]
    fn test_mesh_new() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 1.0, 0.0]),
        ];
        let faces = vec![Face::new(0, 1, 2)];
        let mesh = Mesh::new(vertices, faces).unwrap();
        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.face_count(), 1);
        assert_eq!(mesh.name, "Unnamed Mesh");
    }

    #[test]
    fn test_mesh_invalid_index() {
        let vertices = vec![Vertex::new(0, [0.0, 0.0, 0.0])];
        let faces = vec![Face::new(0, 1, 2)]; // indices 1 and 2 out of bounds
        let result = Mesh::new(vertices, faces);
        assert!(result.is_err());
    }

    #[test]
    fn test_bounding_box() {
        let vertices = vec![
            Vertex::new(0, [-1.0, -2.0, -3.0]),
            Vertex::new(1, [2.0, 3.0, 4.0]),
        ];
        let faces = vec![Face::new(0, 1, 0)];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let bbox = mesh.bounding_box();
        assert_eq!(bbox.min, [-1.0, -2.0, -3.0]);
        assert_eq!(bbox.max, [2.0, 3.0, 4.0]);
        assert_eq!(bbox.width(), 3.0);
        assert_eq!(bbox.height(), 5.0);
        assert_eq!(bbox.depth(), 7.0);
    }

    #[test]
    fn test_centroid() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [2.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 2.0, 0.0]),
        ];
        let faces = vec![Face::new(0, 1, 2)];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let centroid = mesh.centroid();
        assert_eq!(centroid, [2.0 / 3.0, 2.0 / 3.0, 0.0]);
    }

    #[test]
    fn test_scale() {
        let vertices = vec![
            Vertex::new(0, [1.0, 2.0, 3.0]),
            Vertex::new(1, [4.0, 5.0, 6.0]),
        ];
        let faces = vec![Face::new(0, 1, 0)];
        let mut mesh = Mesh::new(vertices, faces).unwrap();
        mesh.scale(2.0);
        assert_eq!(mesh.vertices[0].position, [2.0, 4.0, 6.0]);
        assert_eq!(mesh.vertices[1].position, [8.0, 10.0, 12.0]);
    }

    #[test]
    fn test_translate() {
        let vertices = vec![
            Vertex::new(0, [1.0, 2.0, 3.0]),
            Vertex::new(1, [4.0, 5.0, 6.0]),
        ];
        let faces = vec![Face::new(0, 1, 0)];
        let mut mesh = Mesh::new(vertices, faces).unwrap();
        mesh.translate([1.0, -1.0, 0.0]);
        assert_eq!(mesh.vertices[0].position, [2.0, 1.0, 3.0]);
        assert_eq!(mesh.vertices[1].position, [5.0, 4.0, 6.0]);
    }
}