//! # Local LLM Router
//!
//! Маршрутизатор для локальных LLM-бэкендов (Ollama, llama.cpp).
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::ai::local_llm::LocalLlmRouter;
//!
//! let router = LocalLlmRouter::new("http://localhost:11434");
//! let response = router.generate("qwen2.5:7b", "Привет!", None).unwrap();
//! println!("{}", response.text);
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Конфигурация LLM-бэкенда
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmBackendConfig {
    /// URL бэкенда (например, "http://localhost:11434" для Ollama)
    pub url: String,
    /// Название модели (например, "qwen2.5:7b", "llama3:8b")
    pub model: String,
    /// Таймаут запроса в секундах
    pub timeout_secs: u64,
}

impl Default for LlmBackendConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:11434".to_string(),
            model: "qwen2.5:7b".to_string(),
            timeout_secs: 120,
        }
    }
}

/// Ответ от LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// Сгенерированный текст
    pub text: String,
    /// Время генерации в миллисекундах
    pub generation_time_ms: u64,
    /// Количество токенов в ответе
    pub tokens_generated: u32,
}

/// Статус LLM-бэкенда
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmStatus {
    /// Доступен ли бэкенд
    pub available: bool,
    /// Название бэкенда (Ollama, llama.cpp, etc.)
    pub backend_name: String,
    /// Версия бэкенда
    pub version: Option<String>,
    /// Список доступных моделей
    pub models: Vec<String>,
    /// Ошибка, если бэкенд недоступен
    pub error: Option<String>,
}

/// Маршрутизатор для локальных LLM
pub struct LocalLlmRouter {
    config: LlmBackendConfig,
    client: ureq::Agent,
}

impl LocalLlmRouter {
    /// Создать новый маршрутизатор с конфигурацией по умолчанию
    pub fn new(url: &str) -> Self {
        Self::with_config(LlmBackendConfig {
            url: url.to_string(),
            ..Default::default()
        })
    }

    /// Создать новый маршрутизатор с кастомной конфигурацией
    pub fn with_config(config: LlmBackendConfig) -> Self {
        let client = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build();

        Self { config, client }
    }

    /// Получить URL бэкенда
    pub fn url(&self) -> &str {
        &self.config.url
    }

    /// Получить название модели
    pub fn model(&self) -> &str {
        &self.config.model
    }

    /// Проверить доступность бэкенда
    pub fn check_status(&self) -> LlmStatus {
        // Попытка получить информацию о бэкенде через /api/tags (Ollama)
        match self.client.get(&format!("{}/api/tags", self.config.url)).call() {
            Ok(response) => {
                if let Ok(json) = response.into_json::<serde_json::Value>() {
                    if let Some(models) = json.get("models").and_then(|m: &serde_json::Value| m.as_array()) {
                        let model_names: Vec<String> = models
                            .iter()
                            .filter_map(|m: &serde_json::Value| m.get("name").and_then(|n: &serde_json::Value| n.as_str()).map(String::from))
                            .collect();

                        // Попытка получить версию через /api/version
                        let version = self.client
                            .get(&format!("{}/api/version", self.config.url))
                            .call()
                            .ok()
                            .and_then(|r: ureq::Response| r.into_json::<serde_json::Value>().ok())
                            .and_then(|v: serde_json::Value| v.get("version").and_then(|s: &serde_json::Value| s.as_str()).map(String::from));

                        return LlmStatus {
                            available: true,
                            backend_name: "Ollama".to_string(),
                            version,
                            models: model_names,
                            error: None,
                        };
                    }
                }

                LlmStatus {
                    available: true,
                    backend_name: "Unknown".to_string(),
                    version: None,
                    models: vec![],
                    error: None,
                }
            }
            Err(e) => LlmStatus {
                available: false,
                backend_name: "Unknown".to_string(),
                version: None,
                models: vec![],
                error: Some(e.to_string()),
            },
        }
    }

    /// Сгенерировать ответ на промпт
    ///
    /// # Аргументы
    /// * `model` - название модели (например, "qwen2.5:7b")
    /// * `prompt` - текстовый промпт
    /// * `system_prompt` - опциональный системный промпт
    ///
    /// # Возвращает
    /// * `Ok(LlmResponse)` - успешный ответ
    /// * `Err(String)` - ошибка
    pub fn generate(&self, model: &str, prompt: &str, system_prompt: Option<&str>) -> Result<LlmResponse, String> {
        let start = std::time::Instant::now();

        // Формируем запрос к Ollama API
        let mut request_body = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
            "format": "json"
        });

        // Добавляем системный промпт, если указан
        if let Some(sys) = system_prompt {
            request_body["system"] = serde_json::json!(sys);
        }

        // Отправляем запрос
        let response = self
            .client
            .post(&format!("{}/api/generate", self.config.url))
            .send_json(ureq::json!(request_body))
            .map_err(|e| format!("Ошибка запроса к LLM: {}", e))?;

        let json: serde_json::Value = response
            .into_json()
            .map_err(|e| format!("Ошибка парсинга JSON ответа: {}", e))?;

        let text = json
            .get("response")
            .and_then(|r: &serde_json::Value| r.as_str())
            .unwrap_or("")
            .to_string();

        let tokens = json
            .get("eval_count")
            .and_then(|c: &serde_json::Value| c.as_u64())
            .unwrap_or(0) as u32;

        let generation_time = start.elapsed().as_millis() as u64;

        Ok(LlmResponse {
            text,
            generation_time_ms: generation_time,
            tokens_generated: tokens,
        })
    }

    /// Сгенерировать ответ с JSON-форматом
    ///
    /// # Аргументы
    /// * `model` - название модели
    /// * `prompt` - текстовый промпт
    /// * `system_prompt` - опциональный системный промпт
    ///
    /// # Возвращает
    /// * `Ok(serde_json::Value)` - распарсенный JSON ответ
    /// * `Err(String)` - ошибка
    pub fn generate_json(&self, model: &str, prompt: &str, system_prompt: Option<&str>) -> Result<serde_json::Value, String> {
        let response = self.generate(model, prompt, system_prompt)?;

        // Пытаемся распарсить JSON из ответа
        let json_text = response.text.trim();

        // Ollama может возвращать JSON в markdown-блоке
        let json_text = if json_text.starts_with("```json") {
            json_text
                .trim_start_matches("```json")
                .trim_end_matches("```")
                .trim()
        } else if json_text.starts_with("```") {
            json_text
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim()
        } else {
            json_text
        };

        serde_json::from_str(json_text)
            .map_err(|e| format!("Ошибка парсинга JSON: {}. Ответ: {}", e, json_text))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router_creation() {
        let router = LocalLlmRouter::new("http://localhost:11434");
        assert_eq!(router.url(), "http://localhost:11434");
        assert_eq!(router.model(), "qwen2.5:7b");
    }

    #[test]
    fn test_router_with_config() {
        let config = LlmBackendConfig {
            url: "http://localhost:11435".to_string(),
            model: "llama3:8b".to_string(),
            timeout_secs: 60,
        };
        let router = LocalLlmRouter::with_config(config);
        assert_eq!(router.url(), "http://localhost:11435");
        assert_eq!(router.model(), "llama3:8b");
    }
}


