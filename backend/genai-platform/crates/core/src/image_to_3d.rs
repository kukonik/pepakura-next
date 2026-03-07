use serde::{Deserialize, Serialize};

use crate::model::ModelRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageTo3dMode {
    ImageToShape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTo3dParams {
    pub steps: u32,
    pub resolution: u32,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTo3dRequest {
    pub model: ModelRef,
    pub mode: ImageTo3dMode,
    pub reference_image: String,
    pub params: ImageTo3dParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageTo3dArtifactKind {
    Mesh,
    Preview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTo3dArtifact {
    pub kind: ImageTo3dArtifactKind,
    pub format: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageTo3dResponse {
    pub artifacts: Vec<ImageTo3dArtifact>,
}

pub trait ImageTo3dService: Send + Sync {
    fn generate(&self, req: ImageTo3dRequest) -> anyhow::Result<ImageTo3dResponse>;
}