use crate::text_to_3d::{TextTo3DAdapter, TextTo3DResult};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Magic3DConfig {
    pub api_endpoint: String,
    pub api_key: Option<String>,
}

pub struct Magic3DAdapter {
    config: Magic3DConfig,
    client: reqwest::Client,
}

impl Magic3DAdapter {
    pub fn new(config: Magic3DConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }
}

#[derive(Debug, Serialize)]
struct Magic3DRequest {
    prompt: String,
    num_inference_steps: u32,
    guidance_scale: f32,
    resolution: u32,
}

#[derive(Debug, Deserialize)]
struct Magic3DResponse {
    model_url: String,
    task_id: String,
}

#[derive(Debug, Deserialize)]
struct TaskStatusResponse {
    status: String,
    result_url: Option<String>,
}

#[async_trait]
impl TextTo3DAdapter for Magic3DAdapter {
    async fn generate_3d_model(&self, prompt: &str) -> Result<TextTo3DResult, Box<dyn Error + Send + Sync>> {
        // Отправляем запрос на генерацию
        let request = Magic3DRequest {
            prompt: prompt.to_string(),
            num_inference_steps: 200,
            guidance_scale: 7.0,
            resolution: 512,
        };

        let response = self.client
            .post(&format!("{}/generate", self.config.api_endpoint))
            .json(&request)
            .send()
            .await?;

        let magic3d_response: Magic3DResponse = response.json().await?;

        // Проверяем статус задачи
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            
            let status_response = self.client
                .get(&format!("{}/task/{}", self.config.api_endpoint, magic3d_response.task_id))
                .send()
                .await?
                .json::<TaskStatusResponse>()
                .await?;

            match status_response.status.as_str() {
                "completed" => {
                    if let Some(result_url) = status_response.result_url {
                        return Ok(TextTo3DResult {
                            model_url: result_url,
                            format: "fbx".to_string(),
                        });
                    } else {
                        return Err("Task completed but no result URL provided".into());
                    }
                }
                "failed" => return Err("Magic3D task failed".into()),
                _ => continue, // Продолжаем проверку
            }
        }
    }
}