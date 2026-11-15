pub mod lscm;
pub mod mds_optimized;
pub mod seam_cut;

use serde::{Serialize, Deserialize};
use crate::geometry::Mesh;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnfoldAlgorithm { Mst, Lscm, Mds }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldConfig {
    pub algorithm: UnfoldAlgorithm,
}

impl Default for UnfoldConfig {
    fn default() -> Self { Self { algorithm: UnfoldAlgorithm::Mst } }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedMeshMeta {
    pub is_degenerate: bool,
    pub area_2d: f64,
}

#[derive(Debug, Clone)]
pub struct UnfoldedMesh {
    pub vertices_2d: Vec<[f64; 2]>,
    pub faces: Vec<crate::geometry::Face>,
    pub metadata: UnfoldedMeshMeta,
}

#[derive(Debug)]
pub enum UnfoldError { Generic(String) }

pub fn unfold_mst(mesh: &Mesh, _config: &UnfoldConfig) -> Result<UnfoldedMesh, String> {
    let v2d: Vec<[f64; 2]> = mesh.vertices.iter().map(|v| [v.x as f64, v.y as f64]).collect();
    Ok(UnfoldedMesh { vertices_2d: v2d, faces: mesh.faces.clone(), metadata: UnfoldedMeshMeta { is_degenerate: false, area_2d: 0.0 } })
}
