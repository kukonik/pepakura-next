// [56-E2] Локальный LLM-роутер (std-only стаб; HTTP к Ollama - отдельный этап)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmBackendStatus {
    pub available: bool,
    pub backend_name: String,
    pub version: Option<String>,
    pub models: Vec<String>,
    pub error: Option<String>,
}

pub struct LocalLlmRouter {
    url: String,
}

impl LocalLlmRouter {
    pub fn new(url: &str) -> Self {
        LocalLlmRouter { url: url.to_string() }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn check_status(&self) -> LlmBackendStatus {
        LlmBackendStatus {
            available: false,
            backend_name: "disabled".to_string(),
            version: None,
            models: Vec::new(),
            error: Some("LLM backend not wired in std-only build".to_string()),
        }
    }

    pub fn generate_json(&self, _model: &str, _prompt: &str, _system: Option<&str>) -> Result<serde_json::Value, String> {
        Err("LLM backend not wired in std-only build".to_string())
    }
}