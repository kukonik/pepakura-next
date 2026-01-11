use serde::{Deserialize, Serialize};

use crate::model::ModelRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum D3Mode {
    Txt2Shape,
    Img2Shape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Params {
    pub steps: u32,
    pub resolution: u32,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Request {
    pub model: ModelRef,
    pub mode: D3Mode,
    pub prompt: Option<String>,
    pub reference_image: Option<String>,
    pub params: D3Params,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum D3ArtifactKind {
    Mesh,
    Preview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Artifact {
    pub kind: D3ArtifactKind,
    pub format: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Response {
    pub artifacts: Vec<D3Artifact>,
}

pub trait D3Service: Send + Sync {
    fn generate(&self, req: D3Request) -> anyhow::Result<D3Response>;
}
