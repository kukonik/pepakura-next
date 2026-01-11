use serde::{Deserialize, Serialize};

use crate::model::ModelRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChatRole {
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTurn {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextParams {
    pub max_tokens: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub seed: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRequest {
    pub model: ModelRef,
    pub prompt: String,
    pub system_prompt: Option<String>,
    pub history: Vec<ChatTurn>,
    pub params: TextParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextResponse {
    pub output: String,
    pub history: Vec<ChatTurn>,
}

pub trait TextService: Send + Sync {
    fn generate(&self, req: TextRequest) -> anyhow::Result<TextResponse>;
}
