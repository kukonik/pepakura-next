//! Конфигурация AI-провайдеров.

use serde::{Deserialize, Serialize};

/// Конфигурация AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// AI-провайдер
    pub provider: AiProvider,
    /// URL для Ollama
    pub ollama_url: String,
    /// Модель для генерации
    pub model: String,
    /// Температура (0.0 - 1.0)
    pub temperature: f32,
    /// Максимальное количество токенов
    pub max_tokens: usize,
    /// Таймаут запроса (сек)
    pub timeout_sec: u64,
    /// Кэширование включено
    pub cache_enabled: bool,
}

/// AI-провайдер.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq)]
pub enum AiProvider {
    /// Ollama (локальная LLM)
    #[default]
    Ollama,
    /// OpenAI (облачная LLM)
    OpenAI,
}

impl Default for AiConfig {
    fn default() -> Self {
        Self {
            provider: AiProvider::Ollama,
            ollama_url: "http://localhost:11434".to_string(),
            model: "llama3.2".to_string(),
            temperature: 0.7,
            max_tokens: 2048,
            timeout_sec: 60,
            cache_enabled: true,
        }
    }
}

impl AiConfig {
    /// Создаёт новую конфигурацию для Ollama.
    pub fn ollama() -> Self {
        Self::default()
    }
    
    /// Создаёт конфигурацию для Ollama с кастомным URL.
    pub fn ollama_with_url(url: &str) -> Self {
        Self {
            ollama_url: url.to_string(),
            ..Self::default()
        }
    }
    
    /// Создаёт конфигурацию для OpenAI.
    pub fn openai(_api_key: &str) -> Self {
        Self {
            provider: AiProvider::OpenAI,
            model: "gpt-4".to_string(),
            ..Self::default()
        }
    }
    
    /// Устанавливает модель.
    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }
    
    /// Устанавливает температуру.
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = temperature.clamp(0.0, 1.0);
        self
    }
    
    /// Устанавливает таймаут.
    pub fn with_timeout(mut self, sec: u64) -> Self {
        self.timeout_sec = sec;
        self
    }
}

/// Статус AI-сервиса.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStatus {
    /// Доступен ли сервис
    pub available: bool,
    /// Список доступных моделей
    pub models: Vec<String>,
    /// Версия сервиса
    pub version: Option<String>,
}

impl Default for AiStatus {
    fn default() -> Self {
        Self {
            available: false,
            models: vec![],
            version: None,
        }
    }
}
