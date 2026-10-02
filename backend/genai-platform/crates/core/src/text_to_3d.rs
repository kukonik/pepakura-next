use serde::{Deserialize, Serialize};

use crate::model::ModelRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextTo3dMode {
    TextToShape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTo3dParams {
    pub steps: u32,
    pub resolution: u32,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTo3dRequest {
    pub model: ModelRef,
    pub mode: TextTo3dMode,
    pub prompt: String,
    pub params: TextTo3dParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextTo3dArtifactKind {
    Mesh,
    Preview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTo3dArtifact {
    pub kind: TextTo3dArtifactKind,
    pub format: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextTo3dResponse {
    pub artifacts: Vec<TextTo3dArtifact>,
}

pub trait TextTo3dService: Send + Sync {
    fn generate(&self, req: TextTo3dRequest) -> anyhow::Result<TextTo3dResponse>;
}