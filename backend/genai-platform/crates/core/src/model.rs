use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleKind {
    Text,
    Image,
    D3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRef {
    pub module: ModuleKind,
    pub model_id: String,
    pub preset_id: String,
}
