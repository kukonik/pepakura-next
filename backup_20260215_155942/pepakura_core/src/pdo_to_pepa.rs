use serde::{Deserialize, Serialize};
use crate::pdo_parser::PdoModel;

#[derive(Serialize, Deserialize, Debug)]
pub struct PepaScene {
    pub meshes: Vec<PepaMesh>,
    pub materials: Vec<PepaMaterial>,
    pub bounding_box: BoundingBox,
    pub extensions: Option<PepaExtensions>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PepaMesh {
    pub id: String,
    pub name: String,
    pub topology: String,
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Option<Vec<f32>>,
    pub uvs: Option<Vec<f32>>,
    pub material_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PepaMaterial {
    pub id: String,
    pub name: String,
    pub diffuse_color: [f32; 3],
    pub opacity: f32,
    pub texture_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PepaExtensions {
    pub pdo: Option<PepaPDO>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PepaPDO {
    pub version: u32,
    pub unfolded_parts: Vec<PepaUnfoldedPart>,
    pub fold_lines: Vec<PepaFoldLine>,
    pub glue_tabs: Vec<PepaGlueTab>,
    pub labels: Vec<PepaLabel>,
}

// Заглушки для структур
#[derive(Serialize, Deserialize, Debug)]
pub struct PepaUnfoldedPart {}
#[derive(Serialize, Deserialize, Debug)]
pub struct PepaFoldLine {}
#[derive(Serialize, Deserialize, Debug)]
pub struct PepaGlueTab {}
#[derive(Serialize, Deserialize, Debug)]
pub struct PepaLabel {}

// Реализация конверсии (заглушка)
impl From<PdoModel> for PepaScene {
    fn from(_model: PdoModel) -> Self {
        PepaScene {
            meshes: vec![],
            materials: vec![],
            bounding_box: BoundingBox { min: [0.0; 3], max: [0.0; 3] },
            extensions: None,
        }
    }
}
