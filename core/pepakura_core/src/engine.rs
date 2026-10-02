//! Main Pepakura engine
//!
//! Coordinates all stages of the unfolding process:
//! 1. Mesh loading and validation
//! 2. 3D unfolding
//! 3. 2D layout
//! 4. Export to various formats

use crate::model::{Mesh, LayoutSheet};
use crate::unfold::{Unwrapper3D, LayoutEngine};
use crate::export::{PngExporter, SvgExporter, ObjExporter};
use crate::errors::{PepakuraError, Result};
use crate::types::{UnfoldedTriangle};

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

/// Main Pepakura engine
pub struct PepakuraEngine {
    unwrapper: Unwrapper3D,
    layout_engine: LayoutEngine,
    flap_width: f64,
    sheet_width: f64,
    sheet_height: f64,
}

impl PepakuraEngine {
    /// Create new engine with default parameters
    pub fn new() -> Self {
        PepakuraEngine {
            unwrapper: Unwrapper3D::new(),
            layout_engine: LayoutEngine::new(),
            flap_width: 5.0, // mm
            sheet_width: 210.0, // A4 width
            sheet_height: 297.0, // A4 height
        }
    }

    /// Set flap width for gluing
    pub fn with_flap_width(mut self, width: f64) -> Self {
        self.flap_width = width;
        self.unwrapper = self.unwrapper.with_flap_width(width);
        self
    }

    /// Set sheet dimensions
    pub fn with_sheet_size(mut self, width: f64, height: f64) -> Self {
        self.sheet_width = width;
        self.sheet_height = height;
        self.layout_engine = self.layout_engine.with_sheet_size(width, height);
        self
    }

    /// Main unfold function
    /// Takes a 3D mesh and returns unfolded 2D layout
    pub fn unfold_mesh(&self, mesh: &Mesh) -> Result<UnfoldResult> {
        if mesh.vertices.is_empty() || mesh.triangles.is_empty() {
            return Err(PepakuraError::InvalidMesh(
                "Mesh has no vertices or triangles".to_string(),
            ));
        }

        // Step 1: Unfold triangles
        let unfolded = self.unwrapper.unfold_mesh(mesh)?;

        // Step 2: Layout on sheets
        let sheets = self.layout_engine.layout_sheets(&unfolded)?;

        // Step 3: Calculate efficiency
        let efficiency = self.calculate_efficiency(&sheets);

        Ok(UnfoldResult {
            sheets,
            total_triangles: mesh.triangles.len() as u32,
            unfolded_triangles: unfolded.len() as u32,
            efficiency,
        })
    }

    /// Export to SVG format
    pub fn export_svg(&self, result: &UnfoldResult) -> Result<String> {
        SvgExporter::export_sheets(&result.sheets)
    }

    /// Export to PNG format
    pub fn export_png(&self, result: &UnfoldResult) -> Result<Vec<u8>> {
        // For now, we'll export the first sheet as PNG
        if let Some(sheet) = result.sheets.first() {
            PngExporter::export_sheet(sheet)
        } else {
            Err(PepakuraError::ExportError("No sheets to export".to_string()))
        }
    }

    /// Export to OBJ format
    pub fn export_obj(&self, mesh: &Mesh) -> String {
        ObjExporter::export_mesh(mesh)
    }

    /// Calculate layout efficiency
    fn calculate_efficiency(&self, sheets: &[LayoutSheet]) -> f64 {
        if sheets.is_empty() {
            return 0.0;
        }

        let total_area = self.sheet_width * self.sheet_height * sheets.len() as f64;
        let used_area = sheets.iter().map(|sheet| {
            sheet.triangles.len() as f64 * 50.0 // Approximate area per triangle
        }).sum::<f64>();

        (used_area / total_area).min(1.0)
    }
}

impl Default for PepakuraEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Mesh, Triangle, Vec3};

    #[test]
    fn test_engine_creation() {
        let engine = PepakuraEngine::new();
        assert_eq!(engine.flap_width, 5.0);
        assert_eq!(engine.sheet_width, 210.0);
    }

    #[test]
    fn test_cube_unfolding() {
        let vertices = vec![
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
        ];

        let triangles = vec![
            Triangle::new(0, 1, 2),
            Triangle::new(0, 2, 3),
        ];

        let mesh = Mesh::new(vertices, triangles);
        let engine = PepakuraEngine::new();

        let result = engine.unfold_mesh(&mesh);
        assert!(result.is_ok());

        let unfold = result.unwrap();
        assert_eq!(unfold.total_triangles, 2);
    }
}