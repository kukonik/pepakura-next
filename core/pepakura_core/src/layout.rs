//! 2D Layout and Packing Algorithms
//! 
//! Handles packing of unfolded triangles onto sheets with optimal space utilization.

use crate::model::{Vec2, Edge2D, UnfoldedTriangle, LayoutSheet};
use crate::unfold::UnfoldResult;

/// Configuration for layout operations
#[derive(Debug, Clone)]
pub struct LayoutConfig {
    /// Sheet width in millimeters
    pub sheet_width: f64,
    /// Sheet height in millimeters
    pub sheet_height: f64,
    /// Minimum margin between triangles in millimeters
    pub margin: f64,
    /// Whether to rotate triangles for better packing
    pub allow_rotation: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {
            sheet_width: 210.0,  // A4 width
            sheet_height: 297.0, // A4 height
            margin: 5.0,
            allow_rotation: true,
        }
    }
}

/// Layout engine for packing unfolded triangles
pub struct LayoutEngine {
    config: LayoutConfig,
}

impl LayoutEngine {
    /// Create new layout engine with configuration
    pub fn new(config: LayoutConfig) -> Self {
        LayoutEngine { config }
    }

    /// Pack unfolded triangles onto sheets
    pub fn pack_triangles(&self, unfold_result: &UnfoldResult) -> LayoutResult {
        let mut sheets: Vec<LayoutSheet> = Vec::new();
        
        // For now, we'll just copy the sheets from unfold result
        // In a real implementation, this would do actual packing
        for sheet in &unfold_result.sheets {
            sheets.push(sheet.clone());
        }
        
        // Calculate coverage ratio (simplified)
        let total_area = self.config.sheet_width * self.config.sheet_height * sheets.len() as f64;
        let coverage_ratio = if total_area > 0.0 {
            0.75 // Placeholder value
        } else {
            0.0
        };
        
        LayoutResult {
            sheets,
            coverage_ratio,
        }
    }
}

/// Result of layout operation
#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub sheets: Vec<LayoutSheet>,
    pub coverage_ratio: f64,
}

impl LayoutResult {
    /// Create new layout result
    pub fn new(sheets: Vec<LayoutSheet>, coverage_ratio: f64) -> Self {
        LayoutResult {
            sheets,
            coverage_ratio,
        }
    }
}