//! Клиент для работы с Ollama API

use super::{AiBackendConfig, AiResponse};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Запрос к Ollama API
#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: Option<GenerationOptions>,
}

/// Параметры генерации для Ollama
#[derive(Serialize)]
struct GenerationOptions {
    temperature: f32,
    top_p: f32,
    #[serde(rename = "num_predict")]
    max_tokens: u32,
}

/// Ответ от Ollama API
#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
    #[serde(rename = "total_duration")]
    total_duration: Option<u64>,
}

/// Отправляет сообщение в Ollama
pub fn send_message(
    message: &str,
    config: &AiBackendConfig,
) -> Result<AiResponse, String> {
    // Создаем параметры генерации
    let options = GenerationOptions {
        temperature: config.generation_params.temperature,
        top_p: config.generation_params.top_p,
        max_tokens: config.generation_params.max_tokens,
    };
    
    // Создаем запрос
    let request = OllamaRequest {
        model: config.model.clone(),
        prompt: message.to_string(),
        stream: false,
        options: Some(options),
    };
    
    // Сериализуем запрос
    let request_body = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;
    
    // Отправляем запрос
    let client = reqwest::blocking::Client::new();
    let start_time = Instant::now();
    
    let response = client
        .post(&config.endpoint)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .map_err(|e| format!("Failed to send request to Ollama: {}", e))?;
    
    // Проверяем статус ответа
    if !response.status().is_success() {
        return Err(format!("Ollama API returned error: {}", response.status()));
    }
    
    // Читаем тело ответа
    let response_body = response.text()
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    // Десериализуем ответ
    let ollama_response: OllamaResponse = serde_json::from_str(&response_body)
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;
    
    // Вычисляем время генерации
    let generation_time = start_time.elapsed().as_millis() as u64;
    
    Ok(AiResponse {
        content: ollama_response.response,
        generation_time,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ollama_request_serialization() {
        let config = AiBackendConfig {
            backend_type: super::super::AiBackendType::Ollama,
            endpoint: "http://localhost:11434/api/generate".to_string(),
            api_key: None,
            model: "llama2".to_string(),
            generation_params: super::super::GenerationParams::default(),
        };
        
        let options = GenerationOptions {
            temperature: config.generation_params.temperature,
            top_p: config.generation_params.top_p,
            max_tokens: config.generation_params.max_tokens,
        };
        
        let request = OllamaRequest {
            model: config.model.clone(),
            prompt: "Test message".to_string(),
            stream: false,
            options: Some(options),
        };
        
        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"model\":\"llama2\""));
        assert!(json.contains("\"prompt\":\"Test message\""));
    }
}