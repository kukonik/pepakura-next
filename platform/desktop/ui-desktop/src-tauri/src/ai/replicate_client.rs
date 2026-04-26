//! Replicate API client for AI generation (Text-to-3D)
//! 
//! Этот модуль предоставляет клиент для взаимодействия с Replicate API,
//! специализирующийся на генерации 3D моделей из текстовых описаний.
//! 
//! Основные возможности:
//! - Асинхронная генерация через модель TripoSR
//! - Polling статуса с таймаутом
//! - Загрузка результата (GLB/OBJ файлы)
//! - Безопасное хранение API ключа

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::{interval, sleep};
use thiserror::Error;
use log::{info, warn, error};

const REPLICATE_API_URL: &str = "https://api.replicate.com/v1/predictions";
const POLLING_INTERVAL_MS: u64 = 2000; // 2 секунды
const MAX_POLLING_ATTEMPTS: u32 = 60; // 120 секунд при интервале 2с
const TIMEOUT_SECONDS: u64 = 120;

/// Ошибки Replicate клиента
#[derive(Debug, Error)]
pub enum ReplicateError {
    #[error("Сетевая ошибка: {0}")]
    Network(String),
    #[error("Ошибка API Replicate: {0}")]
    Api(String),
    #[error("Таймаут генерации (превышено {0} секунд)")]
    Timeout(u64),
    #[error("Генерация отменена или завершилась с ошибкой: {0}")]
    GenerationFailed(String),
    #[error("Невалидный ответ API: {0}")]
    InvalidResponse(String),
    #[error("Отсутствует API ключ")]
    MissingApiKey,
}

/// Запрос на создание предикции
#[derive(Debug, Serialize)]
struct CreatePredictionRequest {
    version: String,
    input: PredictionInput,
}

/// Входные данные для предикции
#[derive(Debug, Serialize)]
struct PredictionInput {
    prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance_scale: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_inference_steps: Option<u32>,
}

/// Ответ на создание предикции
#[derive(Debug, Deserialize)]
struct PredictionResponse {
    id: String,
    status: String,
    output: Option<Vec<String>>,
    error: Option<String>,
    urls: Option<PredictionUrls>,
}

/// URL для получения статуса и отмены
#[derive(Debug, Deserialize)]
struct PredictionUrls {
    get: String,
    cancel: String,
}

/// Детали предикции (для polling)
#[derive(Debug, Deserialize)]
struct PredictionDetails {
    id: String,
    status: String,
    output: Option<Vec<String>>,
    error: Option<String>,
    logs: Option<String>,
}

/// Клиент для работы с Replicate API
pub struct ReplicateClient {
    api_token: String,
    client: Client,
}

impl ReplicateClient {
    /// Создает новый клиент Replicate
    /// 
    /// # Аргументы
    /// * `api_token` - API ключ Replicate
    /// 
    /// # Возвращает
    /// * `ReplicateClient` - новый клиент
    pub fn new(api_token: String) -> Self {
        Self {
            api_token,
            client: Client::builder()
                .timeout(Duration::from_secs(TIMEOUT_SECONDS))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    /// Генерирует 3D модель из текстового описания
    /// 
    /// # Аргументы
    /// * `prompt` - текстовое описание модели
    /// * `model_version` - версия модели (по умолчанию TripoSR)
    /// 
    /// # Возвращает
    /// * `Result<Vec<u8>, ReplicateError>` - байты сгенерированной модели (GLB/OBJ)
    pub async fn generate(&self, prompt: &str, model_version: Option<&str>) -> Result<Vec<u8>, ReplicateError> {
        info!("Начинаем генерацию модели для промпта: '{}'", prompt);
        
        let version = model_version.unwrap_or("c871bb9b046607b680449ecbae55fd8c6d945e0a9949202a9fe3d4e7dd89f846").to_string();
        
        // 1. Создаем предикцию
        let prediction_id = self.create_prediction(prompt, &version).await?;
        info!("Создана предикция с ID: {}", prediction_id);
        
        // 2. Ожидаем завершения генерации
        let output_urls = self.wait_for_completion(&prediction_id).await?;
        
        // 3. Загружаем результат (первый файл)
        if let Some(url) = output_urls.first() {
            info!("Загружаем результат из: {}", url);
            let model_bytes = self.download_file(url).await?;
            info!("Загружено {} байт модели", model_bytes.len());
            Ok(model_bytes)
        } else {
            Err(ReplicateError::InvalidResponse("Нет выходных файлов в ответе".to_string()))
        }
    }

    /// Создает новую предикцию
    async fn create_prediction(&self, prompt: &str, version: &str) -> Result<String, ReplicateError> {
        let request = CreatePredictionRequest {
            version: version.to_string(),
            input: PredictionInput {
                prompt: prompt.to_string(),
                negative_prompt: None,
                guidance_scale: None,
                num_inference_steps: None,
            },
        };

        let response = self.client
            .post(REPLICATE_API_URL)
            .header("Authorization", format!("Token {}", self.api_token))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| ReplicateError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ReplicateError::Api(format!("HTTP {}: {}", status, error_text)));
        }

        let prediction: PredictionResponse = response.json()
            .await
            .map_err(|e| ReplicateError::InvalidResponse(e.to_string()))?;

        if let Some(err) = prediction.error {
            return Err(ReplicateError::GenerationFailed(err));
        }

        Ok(prediction.id)
    }

    /// Ожидает завершения генерации через polling
    async fn wait_for_completion(&self, prediction_id: &str) -> Result<Vec<String>, ReplicateError> {
        let mut attempts = 0;
        let mut interval = interval(Duration::from_millis(POLLING_INTERVAL_MS));
        
        info!("Начинаем polling для предикции {}", prediction_id);
        
        loop {
            attempts += 1;
            
            if attempts > MAX_POLLING_ATTEMPTS {
                return Err(ReplicateError::Timeout(TIMEOUT_SECONDS));
            }
            
            // Ждем перед следующим запросом (кроме первой итерации)
            if attempts > 1 {
                interval.tick().await;
            }
            
            let details = self.get_prediction_details(prediction_id).await?;
            
            match details.status.as_str() {
                "succeeded" => {
                    info!("Генерация успешно завершена");
                    if let Some(output) = details.output {
                        return Ok(output);
                    } else {
                        return Err(ReplicateError::InvalidResponse("Нет выходных данных".to_string()));
                    }
                }
                "failed" | "canceled" => {
                    let error_msg = details.error.unwrap_or_else(|| "Неизвестная ошибка".to_string());
                    return Err(ReplicateError::GenerationFailed(error_msg));
                }
                "processing" | "starting" => {
                    // Продолжаем ожидание
                    if let Some(logs) = details.logs {
                        info!("Логи генерации: {}", logs);
                    }
                    continue;
                }
                _ => {
                    warn!("Неизвестный статус: {}", details.status);
                    continue;
                }
            }
        }
    }

    /// Получает детали предикции
    async fn get_prediction_details(&self, prediction_id: &str) -> Result<PredictionDetails, ReplicateError> {
        let url = format!("{}/{}", REPLICATE_API_URL, prediction_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Token {}", self.api_token))
            .send()
            .await
            .map_err(|e| ReplicateError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ReplicateError::Api(format!("HTTP {}: {}", status, error_text)));
        }

        let details: PredictionDetails = response.json()
            .await
            .map_err(|e| ReplicateError::InvalidResponse(e.to_string()))?;

        Ok(details)
    }

    /// Загружает файл по URL
    async fn download_file(&self, url: &str) -> Result<Vec<u8>, ReplicateError> {
        info!("Загрузка файла: {}", url);
        
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| ReplicateError::Network(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(ReplicateError::Api(format!("HTTP {}: {}", status, error_text)));
        }

        let bytes = response.bytes()
            .await
            .map_err(|e| ReplicateError::Network(e.to_string()))?
            .to_vec();

        Ok(bytes)
    }

    /// Проверяет валидность API ключа
    pub async fn validate_api_key(&self) -> Result<bool, ReplicateError> {
        let response = self.client
            .get("https://api.replicate.com/v1/models")
            .header("Authorization", format!("Token {}", self.api_token))
            .send()
            .await
            .map_err(|e| ReplicateError::Network(e.to_string()))?;

        Ok(response.status().is_success())
    }
}

/// Упрощенный клиент для использования в Tauri командах
pub struct AiGenerator {
    client: Option<ReplicateClient>,
}

impl AiGenerator {
    /// Создает новый генератор
    pub fn new() -> Self {
        Self { client: None }
    }
    
    /// Устанавливает API ключ
    pub fn set_api_key(&mut self, api_key: String) {
        self.client = Some(ReplicateClient::new(api_key));
    }
    
    /// Проверяет, установлен ли API ключ
    pub fn has_api_key(&self) -> bool {
        self.client.is_some()
    }
    
    /// Генерирует модель (обертка для Tauri команд)
    pub async fn generate_model(&self, prompt: &str) -> Result<Vec<u8>, ReplicateError> {
        match &self.client {
            Some(client) => client.generate(prompt, None).await,
            None => Err(ReplicateError::MissingApiKey),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_replicate_client_creation() {
        let client = ReplicateClient::new("test_token".to_string());
        // Проверяем, что клиент создан
        assert!(!client.api_token.is_empty());
    }
    
    #[tokio::test]
    async fn test_validate_api_key_invalid() {
        let client = ReplicateClient::new("invalid_token".to_string());
        let result = client.validate_api_key().await;
        // С невалидным ключом должен вернуть false или ошибку
        // Но так как это сетевой запрос, пропускаем в CI
        // assert!(result.is_err() || !result.unwrap());
    }
}