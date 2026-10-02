//! Core data types for the Pepakura unfolding engine
//! 
//! Defines fundamental structures for 3D meshes, 2D layouts, and geometric operations.

// Re-export types from model module for backward compatibility
pub use crate::model::{Vec2, Vec3, Triangle, Mesh, Edge2D, UnfoldedTriangle, LayoutSheet};

/// Main unfolding result structure
#[derive(Debug, Clone)]
pub struct UnfoldResult {
    pub sheets: Vec<LayoutSheet>,
    pub total_triangles: u32,
    pub unfolded_triangles: u32,
    pub efficiency: f64, // Percentage of sheet utilization
}

/// Layout result after packing
#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub sheets: Vec<LayoutSheet>,
    pub coverage_ratio: f64,
}

impl UnfoldResult {
    /// Create new unfold result
    pub fn new(sheets: Vec<LayoutSheet>, total_triangles: u32, unfolded_triangles: u32) -> Self {
        let efficiency = if total_triangles > 0 {
            unfolded_triangles as f64 / total_triangles as f64
        } else {
            0.0
        };
        
        UnfoldResult {
            sheets,
            total_triangles,
            unfolded_triangles,
            efficiency,
        }
    }
}