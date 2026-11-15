//! Минимальные AI-типы для Tauri-слоя (Этап A).
//!
//! Реальный AI-бэкенд (HTTP-клиент, стриминг, кэш LLM) подключается на
//! отдельном этапе. Файлы ai/assistant.rs и ai/prompts.rs пока НЕ
//! подключены (модуль не объявляет подмодулей), чтобы не тянуть их
//! зависимости.

use serde::{Deserialize, Serialize};

/// Конфигурация AI.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AiConfig {
    pub model: String,
    pub api_url: String,
}

/// Сообщение чата.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// AI-ассистент (заглушка).
pub struct PepakuraAssistant {
    pub config: AiConfig,
}

impl PepakuraAssistant {
    pub fn new(config: &AiConfig) -> Self {
        Self { config: config.clone() }
    }

    /// Временная заглушка: реальный бэкенд не подключён.
    pub fn send_message(&self, _msg: &str) -> Result<String, String> {
        Err("AI Assistant stub: backend not wired yet".to_string())
    }
}

// [56-E3] LLM-сабмодули (analysis::mesh_analyzer / mesh_stats)
pub mod prompts;
pub mod local_llm;
