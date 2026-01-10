//! Core data types for the Pepakura unfolding engine
//! 
//! Defines fundamental structures for 3D meshes, 2D layouts, and geometric operations.

use serde::{Deserialize, Serialize};
use std::fmt;

/// 2D vector with floating-point coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 1e-10 {
            Vec2 {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn distance(&self, other: &Vec2) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// 3D vector with floating-point coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 1e-10 {
            Vec3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            *self
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn distance(&self, other: &Vec3) -> f64 {
        ((self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2))
        .sqrt()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// Triangle face defined by three vertex indices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Triangle {
    pub v0: u32,
    pub v1: u32,
    pub v2: u32,
}

impl Triangle {
    pub fn new(v0: u32, v1: u32, v2: u32) -> Self {
        Triangle { v0, v1, v2 }
    }

    pub fn contains_vertex(&self, vertex_id: u32) -> bool {
        self.v0 == vertex_id || self.v1 == vertex_id || self.v2 == vertex_id
    }

    pub fn shares_edge(&self, other: &Triangle) -> bool {
        let edges_self = [self.v0, self.v1, self.v1, self.v2, self.v2, self.v0];
        let edges_other = [other.v0, other.v1, other.v1, other.v2, other.v2, other.v0];

        for i in (0..edges_self.len()).step_by(2) {
            for j in (0..edges_other.len()).step_by(2) {
                if (edges_self[i] == edges_other[j] && edges_self[i + 1] == edges_other[j + 1])
                    || (edges_self[i] == edges_other[j + 1] && edges_self[i + 1] == edges_other[j])
                {
                    return true;
                }
            }
        }
        false
    }
}

/// 3D mesh containing vertices and faces
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub name: String,
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, triangles: Vec<Triangle>) -> Self {
        Mesh {
            vertices,
            triangles,
            name: String::from("Mesh"),
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertices.len() as u32
    }

    pub fn triangle_count(&self) -> u32 {
        self.triangles.len() as u32
    }

    pub fn get_triangle_vertices(&self, tri: &Triangle) -> Option<[Vec3; 3]> {
        if tri.v0 < self.vertices.len() as u32
            && tri.v1 < self.vertices.len() as u32
            && tri.v2 < self.vertices.len() as u32
        {
            Some([
                self.vertices[tri.v0 as usize],
                self.vertices[tri.v1 as usize],
                self.vertices[tri.v2 as usize],
            ])
        } else {
            None
        }
    }
}

/// Edge in 2D unfolded pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Edge2D {
    pub p0: u32,
    pub p1: u32,
}

impl Edge2D {
    pub fn new(p0: u32, p1: u32) -> Self {
        Edge2D { p0, p1 }
    }
}

/// Unfolded pattern for a triangle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedTriangle {
    pub id: u32,
    pub vertices_2d: [Vec2; 3],
    pub original_triangle: Triangle,
    pub flap_indices: Vec<u32>, // Indices of flaps for gluing
}

/// Layout for organizing unfolded triangles on sheets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutSheet {
    pub id: u32,
    pub width: f64,
    pub height: f64,
    pub triangles: Vec<UnfoldedTriangle>,
    pub margin: f64,
}

impl LayoutSheet {
    pub fn new(id: u32, width: f64, height: f64) -> Self {
        LayoutSheet {
            id,
            width,
            height,
            triangles: Vec::new(),
            margin: 10.0, // Default margin in mm
        }
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec2({:.2}, {:.2})", self.x, self.y)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({:.2}, {:.2}, {:.2})", self.x, self.y, self.z)
    }
}