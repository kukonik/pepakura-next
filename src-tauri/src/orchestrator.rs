//! AI Orchestrator Server
//! Локальный HTTP-сервер (Axum) для интеграции с локальными моделями (Ollama/Qwen)
//! Запускается на порту 3000 параллельно с Tauri.

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct AnalyzeRequest {
    /// JSON-строка со статистикой меша
    pub mesh_stats: String,
    /// Вопрос или команда от пользователя из UI
    pub user_prompt: String,
}

#[derive(Serialize, Debug)]
pub struct AnalyzeResponse {
    /// Рекомендация от ИИ
    pub ai_advice: String,
    /// Оценка сложности сборки
    pub difficulty: String,
}

/// Запускает HTTP-сервер оркестратора
pub async fn run_ai_server() {
    let app = Router::new().route("/api/analyze", post(analyze_mesh));

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => l,
        Err(e) => {
            log::error!("❌ Не удалось запустить AI Orchestrator на порту 3000: {}", e);
            return;
        }
    };

    log::info!("🤖 AI Orchestrator listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

/// Хэндлер для POST /api/analyze
async fn analyze_mesh(Json(payload): Json<AnalyzeRequest>) -> Json<AnalyzeResponse> {
    let client = Client::new();
    
    // Формируем промпт для локального Qwen/Ollama
    let system_prompt = "Ты — ИИ-ассистент для создания паперкрафт выкроек. Анализируй 3D модель и давай советы по развёртке, склейке и упрощению.";
    let prompt = format!(
        "Статистика модели:\n{}\n\nВопрос пользователя: {}", 
        payload.mesh_stats, 
        payload.user_prompt
    );

    // Запрос к локальной Ollama
    let ollama_res = client.post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "qwen2.5-coder:7b", // Используем настроенную модель
            "prompt": prompt,
            "system": system_prompt,
            "stream": false
        }))
        .send().await;

    match ollama_res {
        Ok(res) => {
            let body: serde_json::Value = res.json().await.unwrap_or_default();
            let advice = body["response"].as_str().unwrap_or("ИИ не вернул текстовый ответ").to_string();
            Json(AnalyzeResponse { 
                ai_advice: advice, 
                difficulty: "Medium".into() // В будущем можно парсить из ответа
            })
        },
        Err(e) => {
            log::error!("Ошибка запроса к Ollama: {}", e);
            Json(AnalyzeResponse { 
                ai_advice: "Ошибка: Ollama не запущена или модель не загружена.".into(), 
                difficulty: "Unknown".into() 
            })
        }
    }
}
