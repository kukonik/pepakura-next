//! Модуль для интеграции с AI бэкендами

pub mod commands;
pub mod ollama_client;

use serde::{Deserialize, Serialize};

/// Конфигурация AI бэкенда
#[derive(Serialize, Deserialize, Clone)]
pub struct AiBackendConfig {
    /// Тип бэкенда
    pub backend_type: AiBackendType,
    /// URL эндпоинта
    pub endpoint: String,
    /// API ключ (если требуется)
    pub api_key: Option<String>,
    /// Имя модели
    pub model: String,
    /// Параметры генерации
    pub generation_params: GenerationParams,
}

/// Тип AI бэкенда
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AiBackendType {
    /// Ollama локальный бэкенд
    Ollama,
    /// OpenAI API
    OpenAI,
    /// Пользовательский HTTP-совместимый бэкенд
    Custom,
}

/// Параметры генерации текста
#[derive(Serialize, Deserialize, Clone)]
pub struct GenerationParams {
    /// Температура генерации (0.0 - 1.0)
    pub temperature: f32,
    /// Максимальное количество токенов
    pub max_tokens: u32,
    /// Верхнее значение для отбора токенов (0.0 - 1.0)
    pub top_p: f32,
}

impl Default for GenerationParams {
    fn default() -> Self {
        Self {
            temperature: 0.7,
            max_tokens: 1000,
            top_p: 0.9,
        }
    }
}

/// Сообщение в чате
#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    /// Роль отправителя
    pub role: String,
    /// Содержание сообщения
    pub content: String,
}

/// Ответ AI
#[derive(Serialize, Deserialize, Clone)]
pub struct AiResponse {
    /// Сгенерированный текст
    pub content: String,
    /// Время генерации в миллисекундах
    pub generation_time: u64,
}