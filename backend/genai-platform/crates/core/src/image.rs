use serde::{Deserialize, Serialize};

use crate::model::ModelRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageMode {
    Txt2Img,
    Img2Img,
    Inpaint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageInputImage {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoraRef {
    pub id: String,
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageParams {
    pub width: u32,
    pub height: u32,
    pub steps: u32,
    pub guidance_scale: f32,
    pub sampler: String,
    pub seed: Option<u64>,
    pub batch_size: u32,
    pub loras: Vec<LoraRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageRequest {
    pub model: ModelRef,
    pub mode: ImageMode,
    pub prompt: String,
    pub negative_prompt: String,
    pub init_image: Option<ImageInputImage>,
    pub mask: Option<ImageInputImage>,
    pub params: ImageParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    pub path: String,
    pub width: u32,
    pub height: u32,
    pub mime: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResponse {
    pub images: Vec<GeneratedImage>,
}

pub trait ImageService: Send + Sync {
    fn generate(&self, req: ImageRequest) -> anyhow::Result<ImageResponse>;
}
