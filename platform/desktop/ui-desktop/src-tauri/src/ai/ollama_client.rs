//! Ollama AI client for the desktop application

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Ollama client for interacting with the Ollama API
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

/// Request structure for Ollama chat API
#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
    options: Option<HashMap<String, serde_json::Value>>,
}

/// Message structure for Ollama chat API
#[derive(Serialize, Deserialize)]
pub struct Message {
    role: String,
    content: String,
}

/// Response structure from Ollama chat API
#[derive(Deserialize)]
struct ChatResponse {
    model: String,
    created_at: String,
    message: Message,
    done: bool,
    total_duration: Option<u64>,
    load_duration: Option<u64>,
    prompt_eval_count: Option<u32>,
    prompt_eval_duration: Option<u64>,
    eval_count: Option<u32>,
    eval_duration: Option<u64>,
}

impl OllamaClient {
    /// Create a new Ollama client
    /// 
    /// # Arguments
    /// * `base_url` - The base URL for the Ollama API (e.g., "http://localhost:11434")
    /// 
    /// # Returns
    /// * `OllamaClient` - The new Ollama client
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Send a chat message to Ollama
    /// 
    /// # Arguments
    /// * `model` - The model to use
    /// * `messages` - The messages to send
    /// * `options` - Additional options for the request
    /// 
    /// # Returns
    /// * `Result<String, Box<dyn std::error::Error>>` - The response content or an error
    pub async fn chat(
        &self,
        model: &str,
        messages: Vec<Message>,
        options: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream: false,
            options,
        };

        let url = format!("{}/api/chat", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;
        
        if response.status().is_success() {
            let chat_response: ChatResponse = response.json().await?;
            Ok(chat_response.message.content)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(format!("Ollama API error {}: {}", status, error_text).into())
        }
    }

    /// Generate text using Ollama
    /// 
    /// # Arguments
    /// * `model` - The model to use
    /// * `prompt` - The prompt to generate from
    /// * `options` - Additional options for the request
    /// 
    /// # Returns
    /// * `Result<String, Box<dyn std::error::Error>>` - The generated text or an error
    pub async fn generate(
        &self,
        model: &str,
        prompt: &str,
        options: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        #[derive(Serialize)]
        struct GenerateRequest {
            model: String,
            prompt: String,
            stream: bool,
            options: Option<HashMap<String, serde_json::Value>>,
        }

        let request = GenerateRequest {
            model: model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
            options,
        };

        let url = format!("{}/api/generate", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;
        
        if response.status().is_success() {
            #[derive(Deserialize)]
            struct GenerateResponse {
                response: String,
            }
            
            let generate_response: GenerateResponse = response.json().await?;
            Ok(generate_response.response)
        } else {
            let status = response.status();
            let error_text = response.text().await?;
            Err(format!("Ollama API error {}: {}", status, error_text).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_client_creation() {
        let client = OllamaClient::new("http://localhost:11434".to_string());
        assert_eq!(client.base_url, "http://localhost:11434");
    }

    #[tokio::test]
    async fn test_generate_request() {
        // This test requires a running Ollama instance
        // It's commented out to prevent failures in CI environments
        /*
        let client = OllamaClient::new("http://localhost:11434".to_string());
        let result = client.generate("llama2", "Hello, world!", None).await;
        assert!(result.is_ok());
        */
    }
}