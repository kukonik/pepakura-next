use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleKind {
    Text,
    Image,
    D3,
    TextTo3D,
    ImageTo3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRef {
    pub module: ModuleKind,
    pub model_id: String,
    pub preset_id: String,
}
