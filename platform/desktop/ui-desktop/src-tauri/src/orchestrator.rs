//! AI Orchestrator Server (Axum + CORS)
//! Локальный HTTP-сервер для интеграции с Ollama

use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Deserialize, Debug)]
pub struct AnalyzeRequest {
    pub mesh_stats: String,
    pub user_prompt: String,
}

#[derive(Serialize, Debug)]
pub struct AnalyzeResponse {
    pub ai_advice: String,
    pub difficulty: String,
}

async fn analyze_mesh(Json(payload): Json<AnalyzeRequest>) -> Json<AnalyzeResponse> {
    let client = reqwest::Client::new();
    
    let system_prompt = "You are an AI assistant for creating papercraft unfoldings. Analyze the 3D model and give advice on unfolding, gluing, and simplification. Answer in Russian.";
    let prompt = format!("Model stats:\n{}\n\nUser question: {}", payload.mesh_stats, payload.user_prompt);

    let ollama_res = client.post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "qwen2.5-coder:7b",
            "prompt": prompt,
            "system": system_prompt,
            "stream": false
        }))
        .send().await;

    match ollama_res {
        Ok(res) => {
            let body: serde_json::Value = res.json().await.unwrap_or_default();
            let advice = body["response"].as_str().unwrap_or("AI returned empty response").to_string();
            Json(AnalyzeResponse { 
                ai_advice: advice, 
                difficulty: "Medium".into() 
            })
        },
        Err(e) => {
            eprintln!("Error connecting to Ollama: {}", e);
            Json(AnalyzeResponse { 
                ai_advice: "Error: Ollama is not running or model qwen2.5-coder is not loaded.".into(), 
                difficulty: "Unknown".into() 
            })
        }
    }
}

pub async fn run_ai_server() {
    let cors = CorsLayer::permissive();

    let app = Router::new()
        .route("/api/analyze", post(analyze_mesh))
        .layer(cors);

    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to start AI Orchestrator on port 3000: {}", e);
            return;
        }
    };

    println!("🤖 AI Orchestrator successfully started on http://localhost:3000 (CORS enabled)");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
