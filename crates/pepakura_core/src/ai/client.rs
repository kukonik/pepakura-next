//! AI-клиент для взаимодействия с LLM.

use crate::ai::{AiConfig, AiStatus};
use crate::PepakuraError;
use serde::{Deserialize, Serialize};

/// AI-клиент для Ollama.
pub struct OllamaClient {
    pub config: AiConfig,
    client: reqwest::Client,
}

/// Сообщение для чата.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Роль сообщения
    pub role: String,
    /// Содержимое сообщения
    pub content: String,
}

impl ChatMessage {
    /// Создаёт сообщение пользователя.
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }
    
    /// Создаёт сообщение ассистента.
    pub fn assistant(content: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

/// Ответ от Ollama API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaResponse {
    /// Сгенерированный текст
    pub response: String,
    /// Название модели
    pub model: String,
    /// Время генерации (нс)
    pub total_duration: Option<u64>,
    /// Время загрузки модели (нс)
    pub load_duration: Option<u64>,
    /// Количество токенов в промпте
    pub prompt_eval_count: Option<u32>,
    /// Количество сгенерированных токенов
    pub eval_count: Option<u32>,
}

/// Список моделей Ollama.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModelsResponse {
    /// Список моделей
    pub models: Vec<OllamaModelInfo>,
}

/// Информация о модели.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaModelInfo {
    /// Название модели
    pub name: String,
    /// Размер модели (байты)
    pub size: Option<u64>,
    /// Формат модели
    pub format: Option<String>,
    /// Семейство модели
    pub family: Option<String>,
}

impl OllamaClient {
    /// Создаёт новый клиент с конфигурацией.
    pub fn new(config: &AiConfig) -> Result<Self, PepakuraError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_sec))
            .build()
            .map_err(|e| PepakuraError::AiError(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            config: config.clone(),
            client,
        })
    }
    
    /// Проверяет доступность Ollama.
    pub async fn check_status(&self) -> Result<AiStatus, PepakuraError> {
        let url = format!("{}/api/tags", self.config.ollama_url);
        
        match self.client.get(&url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    let models_response: OllamaModelsResponse = response
                        .json()
                        .await
                        .map_err(|e| PepakuraError::AiError(format!("Failed to parse response: {}", e)))?;
                    
                    let models: Vec<String> = models_response
                        .models
                        .iter()
                        .map(|m| m.name.clone())
                        .collect();
                    
                    Ok(AiStatus {
                        available: true,
                        models,
                        version: None,
                    })
                } else {
                    Ok(AiStatus::default())
                }
            }
            Err(_) => Ok(AiStatus::default()),
        }
    }
    
    /// Отправляет запрос к LLM.
    pub async fn generate(&self, prompt: &str) -> Result<String, PepakuraError> {
        let url = format!("{}/api/generate", self.config.ollama_url);
        
        let body = serde_json::json!({
            "model": self.config.model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": self.config.temperature,
                "num_predict": self.config.max_tokens
            }
        });
        
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| PepakuraError::AiError(format!("Request failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(PepakuraError::AiError(format!(
                "API returned error: {}",
                response.status()
            )));
        }
        
        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| PepakuraError::AiError(format!("Failed to parse response: {}", e)))?;
        
        Ok(ollama_response.response)
    }
    
    /// Отправляет запрос в режиме чата.
    pub async fn chat(&self, messages: &[ChatMessage]) -> Result<String, PepakuraError> {
        let url = format!("{}/api/chat", self.config.ollama_url);
        
        let body = serde_json::json!({
            "model": self.config.model,
            "messages": messages,
            "stream": false,
            "options": {
                "temperature": self.config.temperature,
                "num_predict": self.config.max_tokens
            }
        });
        
        let response = self.client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| PepakuraError::AiError(format!("Request failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(PepakuraError::AiError(format!(
                "API returned error: {}",
                response.status()
            )));
        }
        
        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| PepakuraError::AiError(format!("Failed to parse response: {}", e)))?;
        
        Ok(ollama_response.response)
    }
    
    /// Получает список доступных моделей.
    pub async fn list_models(&self) -> Result<Vec<String>, PepakuraError> {
        let status = self.check_status().await?;
        Ok(status.models)
    }
    
    /// Проверяет, существует ли модель.
    pub async fn has_model(&self, model_name: &str) -> Result<bool, PepakuraError> {
        let models = self.list_models().await?;
        Ok(models.iter().any(|m| m == model_name || m.starts_with(model_name)))
    }
}

/// AI-клиент для OpenAI.
pub struct OpenAiClient {
    config: AiConfig,
    _api_key: String,
    client: reqwest::Client,
}

impl OpenAiClient {
    /// Создаёт новый клиент OpenAI.
    pub fn new(config: &AiConfig, api_key: &str) -> Result<Self, PepakuraError> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_sec))
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    reqwest::header::HeaderValue::from_str(&format!("Bearer {}", api_key))
                        .map_err(|e| PepakuraError::AiError(format!("Invalid API key: {}", e)))?,
                );
                headers
            })
            .build()
            .map_err(|e| PepakuraError::AiError(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            config: config.clone(),
            _api_key: api_key.to_string(),
            client,
        })
    }
    
    /// Отправляет запрос к GPT.
    pub async fn chat(&self, messages: &[ChatMessage]) -> Result<String, PepakuraError> {
        let url = "https://api.openai.com/v1/chat/completions";
        
        let body = serde_json::json!({
            "model": self.config.model,
            "messages": messages,
            "max_tokens": self.config.max_tokens,
            "temperature": self.config.temperature
        });
        
        let response = self.client
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| PepakuraError::AiError(format!("Request failed: {}", e)))?;
        
        if !response.status().is_success() {
            return Err(PepakuraError::AiError(format!(
                "API returned error: {}",
                response.status()
            )));
        }
        
        let openai_response: serde_json::Value = response
            .json()
            .await
            .map_err(|e| PepakuraError::AiError(format!("Failed to parse response: {}", e)))?;
        
        let content = openai_response["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ai_config_default() {
        let config = AiConfig::default();
        assert_eq!(config.provider, AiProvider::Ollama);
        assert_eq!(config.ollama_url, "http://localhost:11434");
        assert_eq!(config.model, "llama3.2");
        assert_eq!(config.temperature, 0.7);
    }
    
    #[test]
    fn test_ai_config_ollama_with_url() {
        let config = AiConfig::ollama_with_url("http://192.168.1.100:11434");
        assert_eq!(config.ollama_url, "http://192.168.1.100:11434");
    }
    
    #[test]
    fn test_ai_config_with_model() {
        let config = AiConfig::default().with_model("mistral");
        assert_eq!(config.model, "mistral");
    }
    
    #[test]
    fn test_ai_config_with_temperature() {
        let config = AiConfig::default().with_temperature(0.9);
        assert_eq!(config.temperature, 0.9);
        
        // Clamp test
        let config = AiConfig::default().with_temperature(1.5);
        assert_eq!(config.temperature, 1.0);
        
        let config = AiConfig::default().with_temperature(-0.5);
        assert_eq!(config.temperature, 0.0);
    }
    
    #[test]
    fn test_ai_config_with_timeout() {
        let config = AiConfig::default().with_timeout(120);
        assert_eq!(config.timeout_sec, 120);
    }
    
    #[test]
    fn test_ai_config_openai() {
        let config = AiConfig::openai("sk-test-key");
        assert_eq!(config.provider, AiProvider::OpenAI);
        assert_eq!(config.model, "gpt-4");
    }
    
    #[test]
    fn test_chat_message() {
        let msg = ChatMessage::user("Hello");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Hello");
        
        let msg = ChatMessage::assistant("Hi there!");
        assert_eq!(msg.role, "assistant");
        assert_eq!(msg.content, "Hi there!");
    }
    
    #[test]
    fn test_ai_status_default() {
        let status = AiStatus::default();
        assert!(!status.available);
        assert!(status.models.is_empty());
        assert!(status.version.is_none());
    }
    
    #[test]
    fn test_ai_status_available() {
        let status = AiStatus {
            available: true,
            models: vec!["llama3.2".to_string(), "mistral".to_string()],
            version: Some("0.1.0".to_string()),
        };
        
        assert!(status.available);
        assert_eq!(status.models.len(), 2);
        assert_eq!(status.version, Some("0.1.0".to_string()));
    }
}
