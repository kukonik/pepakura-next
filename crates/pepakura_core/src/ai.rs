use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)] pub struct AiConfig { pub model: String, pub api_url: String }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct ChatMessage { pub role: String, pub content: String }

pub struct PepakuraAssistant { pub config: AiConfig }
impl PepakuraAssistant {
    pub fn new(config: AiConfig) -> Self { Self { config } }
    // Временная заглушка: вместо потока возвращаем готовую строку
    pub fn send_message(&self, _msg: &str) -> Result<String, String> {
        Err("AI Assistant stub: real streaming requires async runtime in core".to_string())
    }
}
