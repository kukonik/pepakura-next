use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use core::image::{
    GeneratedImage, ImageInputImage, ImageMode, ImageParams, ImageRequest, ImageResponse, LoraRef,
};
use core::model::{ModelRef, ModuleKind};
use core::text::{
    ChatRole, ChatTurn, TextParams, TextRequest, TextResponse, TextService,
};
use core::d3::{
    D3Artifact, D3ArtifactKind, D3Mode, D3Params, D3Request, D3Response,
};
use core::text_to_3d::{
    TextTo3dArtifact, TextTo3dArtifactKind, TextTo3dMode, TextTo3dParams, TextTo3dRequest, TextTo3dResponse,
};
use core::text_to_3d::shap_e_adapter::{ShapEAdapter, ShapEConfig};
use core::text_to_3d::get3d_adapter::{GET3DAdapter, GET3DConfig};
use core::text_to_3d::magic3d_adapter::{Magic3DAdapter, Magic3DConfig};
use core::image_to_3d::{
    ImageTo3dArtifact, ImageTo3dArtifactKind, ImageTo3dMode, ImageTo3dParams, ImageTo3dRequest, ImageTo3dResponse,
};
use core::cache::model_cache::ModelCache;
use core::rating::model_rating::RatingSystem;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    text_service: Arc<dyn TextService>,
    model_cache: Arc<std::sync::Mutex<ModelCache>>,
    rating_system: Arc<std::sync::Mutex<RatingSystem>>,
}

struct EchoTextService;

impl TextService for EchoTextService {
    fn generate(&self, req: TextRequest) -> anyhow::Result<TextResponse> {
        let mut history = req.history;
        history.push(ChatTurn {
            role: ChatRole::Assistant,
            content: format!("Эхо: {}", req.prompt),
        });

        Ok(TextResponse {
            output: format!("Эхо: {}", req.prompt),
            history,
        })
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let state = AppState {
        text_service: Arc::new(EchoTextService),
        model_cache: Arc::new(std::sync::Mutex::new(ModelCache::new("D:/Dev/pepakura-next/runtime/cache".to_string()))),
        rating_system: Arc::new(std::sync::Mutex::new(RatingSystem::new("D:/Dev/pepakura-next/runtime/ratings".to_string()))),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/text/generate", post(text_generate))
        .route("/api/v1/image/generate", post(image_generate))
        .route("/api/v1/d3/generate", post(d3_generate))
        .route("/api/v1/text-to-3d/generate", post(text_to_3d_generate))
        .route("/api/v1/text-to-3d/shap-e/generate", post(text_to_3d_shap_e_generate))
        .route("/api/v1/text-to-3d/get3d/generate", post(text_to_3d_get3d_generate))
        .route("/api/v1/text-to-3d/magic3d/generate", post(text_to_3d_magic3d_generate))
        .route("/api/v1/image-to-3d/generate", post(image_to_3d_generate))
        .route("/api/v1/rating/add", post(add_rating))
        .route("/api/v1/rating/get", get(get_rating))
        .route("/api/v1/text-to-3d/advanced", post(advanced_text_to_3d_generate))
        .with_state(state);

    let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    tracing::info!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "OK"
}

// ---------- TEXT ----------

async fn text_generate(
    State(state): State<AppState>,
    Json(req): Json<IncomingTextRequest>,
) -> Json<TextResponse> {
    let model_ref = ModelRef {
        module: ModuleKind::Text,
        model_id: req.model_id,
        preset_id: req.preset_id,
    };

    let inner_req = TextRequest {
        model: model_ref,
        prompt: req.prompt,
        system_prompt: req.system_prompt,
        history: req.history.unwrap_or_default(),
        params: TextParams {
            max_tokens: req.params.max_tokens,
            temperature: req.params.temperature,
            top_p: req.params.top_p,
            top_k: req.params.top_k,
            seed: req.params.seed,
        },
    };

    let resp = state
        .text_service
        .generate(inner_req)
        .expect("text service should not fail in echo mode");

    Json(resp)
}

#[derive(Debug, serde::Deserialize)]
struct IncomingTextParams {
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    top_k: u32,
    seed: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingTextRequest {
    model_id: String,
    preset_id: String,
    prompt: String,
    system_prompt: Option<String>,
    history: Option<Vec<ChatTurn>>,
    params: IncomingTextParams,
}

// ---------- IMAGE ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingImageParams {
    width: u32,
    height: u32,
    steps: u32,
    guidance_scale: f32,
    sampler: String,
    seed: Option<u64>,
    batch_size: u32,
    loras: Vec<LoraRef>,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingImageRequest {
    model_id: String,
    preset_id: String,
    mode: ImageMode,
    prompt: String,
    negative_prompt: String,
    init_image: Option<ImageInputImage>,
    mask: Option<ImageInputImage>,
    params: IncomingImageParams,
}

async fn image_generate(
    Json(req): Json<IncomingImageRequest>,
) -> Json<ImageResponse> {
    let model_ref = ModelRef {
        module: ModuleKind::Image,
        model_id: req.model_id,
        preset_id: req.preset_id,
    };

    let _inner_req = ImageRequest {
        model: model_ref,
        mode: req.mode,
        prompt: req.prompt,
        negative_prompt: req.negative_prompt,
        init_image: req.init_image,
        mask: req.mask,
        params: ImageParams {
            width: req.params.width,
            height: req.params.height,
            steps: req.params.steps,
            guidance_scale: req.params.guidance_scale,
            sampler: req.params.sampler,
            seed: req.params.seed,
            batch_size: req.params.batch_size,
            loras: req.params.loras,
        },
    };

    let resp = ImageResponse {
        images: vec![GeneratedImage {
            path: "D:/Dev/pepakura-next/runtime/outputs/fake.png".to_string(),
            width: 512,
            height: 512,
            mime: "image/png".to_string(),
        }],
    };

    Json(resp)
}

// ---------- D3 ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingD3Params {
    steps: u32,
    resolution: u32,
    seed: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingD3Request {
    model_id: String,
    preset_id: String,
    mode: D3Mode,
    prompt: Option<String>,
    reference_image: Option<String>,
    params: IncomingD3Params,
}

async fn d3_generate(
    Json(req): Json<IncomingD3Request>,
) -> Json<D3Response> {
    let model_ref = ModelRef {
        module: ModuleKind::D3,
        model_id: req.model_id,
        preset_id: req.preset_id,
    };

    let _inner_req = D3Request {
        model: model_ref,
        mode: req.mode,
        prompt: req.prompt,
        reference_image: req.reference_image,
        params: D3Params {
            steps: req.params.steps,
            resolution: req.params.resolution,
            seed: req.params.seed,
        },
    };

    let resp = D3Response {
        artifacts: vec![
            D3Artifact {
                kind: D3ArtifactKind::Mesh,
                format: "glb".to_string(),
                path: "D:/Dev/pepakura-next/runtime/outputs/model.glb".to_string(),
            },
            D3Artifact {
                kind: D3ArtifactKind::Preview,
                format: "png".to_string(),
                path: "D:/Dev/pepakura-next/runtime/outputs/model_preview.png".to_string(),
            },
        ],
    };

    Json(resp)
}

// ---------- TEXT TO 3D ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dParams {
    steps: u32,
    resolution: u32,
    seed: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dRequest {
    model_id: String,
    preset_id: String,
    mode: TextTo3dMode,
    prompt: String,
    params: IncomingTextTo3dParams,
}

async fn text_to_3d_generate(
    Json(req): Json<IncomingTextTo3dRequest>,
) -> Json<TextTo3dResponse> {
    let model_ref = ModelRef {
        module: ModuleKind::TextTo3D,
        model_id: req.model_id,
        preset_id: req.preset_id,
    };

    let _inner_req = TextTo3dRequest {
        model: model_ref,
        mode: req.mode,
        prompt: req.prompt,
        params: TextTo3dParams {
            steps: req.params.steps,
            resolution: req.params.resolution,
            seed: req.params.seed,
        },
    };

    let resp = TextTo3dResponse {
        artifacts: vec![
            TextTo3dArtifact {
                kind: TextTo3dArtifactKind::Mesh,
                format: "glb".to_string(),
                path: "D:/Dev/pepakura-next/runtime/outputs/text_to_3d_model.glb".to_string(),
            },
            TextTo3dArtifact {
                kind: TextTo3dArtifactKind::Preview,
                format: "png".to_string(),
                path: "D:/Dev/pepakura-next/runtime/outputs/text_to_3d_model_preview.png".to_string(),
            },
        ],
    };

    Json(resp)
}

// ---------- TEXT TO 3D WITH SHAP-E ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dShapEParams {
    steps: u32,
    guidance_scale: f32,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dShapERequest {
    prompt: String,
    params: IncomingTextTo3dShapEParams,
}

async fn text_to_3d_shap_e_generate(
    State(state): State<AppState>,
    Json(req): Json<IncomingTextTo3dShapERequest>,
) -> Json<TextTo3dResponse> {
    // Проверяем кэш
    {
        let cache = state.model_cache.lock().unwrap();
        if let Some(cached_model) = cache.get_cached_model(&req.prompt) {
            let resp = TextTo3dResponse {
                artifacts: vec![
                    TextTo3dArtifact {
                        kind: TextTo3dArtifactKind::Mesh,
                        format: cached_model.format,
                        path: cached_model.model_url,
                    },
                ],
            };
            return Json(resp);
        }
    }
    
    // Создаем конфигурацию для Shap-E адаптера
    let config = ShapEConfig {
        api_endpoint: "http://localhost:8001".to_string(), // В реальной реализации это будет браться из конфигурации
        api_key: None,
    };
    
    // Создаем адаптер Shap-E
    let adapter = ShapEAdapter::new(config);
    
    // Генерируем 3D модель
    let result = adapter.generate_3d_model(&req.prompt).await
        .expect("Shap-E generation should not fail");
    
    // Кэшируем результат
    {
        let mut cache = state.model_cache.lock().unwrap();
        cache.cache_model(&req.prompt, result.model_url.clone(), result.format.clone());
        cache.save_to_disk().expect("Failed to save cache to disk");
    }
    
    // Создаем ответ с артефактами
    let resp = TextTo3dResponse {
        artifacts: vec![
            TextTo3dArtifact {
                kind: TextTo3dArtifactKind::Mesh,
                format: result.format,
                path: result.model_url,
            },
        ],
    };
    
    Json(resp)
}

// ---------- IMAGE TO 3D ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingImageTo3dParams {
    steps: u32,
    resolution: u32,
    seed: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingImageTo3dRequest {
    model_id: String,
    preset_id: String,
    mode: ImageTo3dMode,
    reference_image: String,
    params: IncomingImageTo3dParams,
}

async fn image_to_3d_generate(
    Json(req): Json<IncomingImageTo3dRequest>,
) -> Json<ImageTo3dResponse> {
    let model_ref = ModelRef {
        module: ModuleKind::ImageTo3D,
        model_id: req.model_id,
        preset_id: req.preset_id,
    };

    let _inner_req = ImageTo3dRequest {
        model: model_ref,
        mode: req.mode,
        reference_image: req.reference_image,
        params: ImageTo3dParams {
            steps: req.params.steps,
            resolution: req.params.resolution,
            seed: req.params.seed,
        },
    };

    let resp = ImageTo3dResponse {
        artifacts: vec![
            ImageTo3dArtifact {
                kind: ImageTo3dArtifactKind::Mesh,
                format: "glb".to_string(),
                path: "D:/Dev/pepakura-next/runtime/outputs/image_to_3d_model.glb".to_string(),
            },
            ImageTo3dArtifact {
                kind: ImageTo3dArtifactKind::Preview,
                format: "png".to_string(),
                path: "D:/Dev/pepakura-next/runtime/outputs/image_to_3d_model_preview.png".to_string(),
            },
        ],
    };

Json(resp)
}

// ---------- TEXT TO 3D WITH GET3D ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dGET3DParams {
steps: u32,
guidance_scale: f32,
resolution: u32,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dGET3DRequest {
prompt: String,
params: IncomingTextTo3dGET3DParams,
}

async fn text_to_3d_get3d_generate(
    State(state): State<AppState>,
    Json(req): Json<IncomingTextTo3dGET3DRequest>,
) -> Json<TextTo3dResponse> {
    // Проверяем кэш
    {
        let cache = state.model_cache.lock().unwrap();
        if let Some(cached_model) = cache.get_cached_model(&req.prompt) {
            let resp = TextTo3dResponse {
                artifacts: vec![
                    TextTo3dArtifact {
                        kind: TextTo3dArtifactKind::Mesh,
                        format: cached_model.format,
                        path: cached_model.model_url,
                    },
                ],
            };
            return Json(resp);
        }
    }
    
    // Создаем конфигурацию для GET3D адаптера
    let config = GET3DConfig {
        api_endpoint: "http://localhost:8002".to_string(), // В реальной реализации это будет браться из конфигурации
        api_key: None,
    };
    
    // Создаем адаптер GET3D
    let adapter = GET3DAdapter::new(config);
    
    // Генерируем 3D модель
    let result = adapter.generate_3d_model(&req.prompt).await
        .expect("GET3D generation should not fail");
    
    // Кэшируем результат
    {
        let mut cache = state.model_cache.lock().unwrap();
        cache.cache_model(&req.prompt, result.model_url.clone(), result.format.clone());
        cache.save_to_disk().expect("Failed to save cache to disk");
    }
    
    // Создаем ответ с артефактами
    let resp = TextTo3dResponse {
        artifacts: vec![
            TextTo3dArtifact {
                kind: TextTo3dArtifactKind::Mesh,
                format: result.format,
                path: result.model_url,
            },
        ],
    };
    
    Json(resp)
}

// ---------- TEXT TO 3D WITH MAGIC3D ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dMagic3DParams {
steps: u32,
guidance_scale: f32,
resolution: u32,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingTextTo3dMagic3DRequest {
prompt: String,
params: IncomingTextTo3dMagic3DParams,
}

async fn text_to_3d_magic3d_generate(
    State(state): State<AppState>,
    Json(req): Json<IncomingTextTo3dMagic3DRequest>,
) -> Json<TextTo3dResponse> {
    // Проверяем кэш
    {
        let cache = state.model_cache.lock().unwrap();
        if let Some(cached_model) = cache.get_cached_model(&req.prompt) {
            let resp = TextTo3dResponse {
                artifacts: vec![
                    TextTo3dArtifact {
                        kind: TextTo3dArtifactKind::Mesh,
                        format: cached_model.format,
                        path: cached_model.model_url,
                    },
                ],
            };
            return Json(resp);
        }
    }
    
    // Создаем конфигурацию для Magic3D адаптера
    let config = Magic3DConfig {
        api_endpoint: "http://localhost:8003".to_string(), // В реальной реализации это будет браться из конфигурации
        api_key: None,
    };
    
    // Создаем адаптер Magic3D
    let adapter = Magic3DAdapter::new(config);
    
    // Генерируем 3D модель
    let result = adapter.generate_3d_model(&req.prompt).await
        .expect("Magic3D generation should not fail");
    
    // Кэшируем результат
    {
        let mut cache = state.model_cache.lock().unwrap();
        cache.cache_model(&req.prompt, result.model_url.clone(), result.format.clone());
        cache.save_to_disk().expect("Failed to save cache to disk");
    }
    
    // ---------- RATING ----------
    
    #[derive(Debug, serde::Deserialize)]
    struct IncomingAddRatingRequest {
        model_hash: String,
        user_id: String,
        rating: u8,
        comment: Option<String>,
    }
    
    async fn add_rating(
        State(state): State<AppState>,
        Json(req): Json<IncomingAddRatingRequest>,
    ) -> Json<serde_json::Value> {
        {
            let mut rating_system = state.rating_system.lock().unwrap();
            rating_system.add_rating(req.model_hash.clone(), req.user_id, req.rating, req.comment);
            rating_system.save_to_disk().expect("Failed to save ratings to disk");
        }
        
        Json(serde_json::json!({
            "status": "success",
            "message": "Rating added successfully"
        }))
    }
    
    #[derive(Debug, serde::Deserialize)]
    struct IncomingGetRatingRequest {
        model_hash: String,
    }
    
    async fn get_rating(
        State(state): State<AppState>,
        Json(req): Json<IncomingGetRatingRequest>,
    ) -> Json<serde_json::Value> {
        let rating_system = state.rating_system.lock().unwrap();
        if let Some(rating_summary) = rating_system.get_model_rating(&req.model_hash) {
            Json(serde_json::json!({
                "status": "success",
                "data": rating_summary
            }))
        } else {
            Json(serde_json::json!({
                "status": "error",
                "message": "Rating not found"
            }))
        }
    }
    
    // Создаем ответ с артефактами
    let resp = TextTo3dResponse {
        artifacts: vec![
            TextTo3dArtifact {
                kind: TextTo3dArtifactKind::Mesh,
                format: result.format,
                path: result.model_url,
            },
        ],
    };
    
    Json(resp)
}

// ---------- ADVANCED TEXT TO 3D ----------

#[derive(Debug, serde::Deserialize)]
struct IncomingAdvancedTextTo3dParams {
    steps: u32,
    guidance_scale: f32,
    resolution: Option<u32>,
}

#[derive(Debug, serde::Deserialize)]
struct IncomingAdvancedTextTo3dRequest {
    prompt: String,
    model: String,
    #[serde(default)]
    num_inference_steps: Option<u32>,
    #[serde(default)]
    guidance_scale: Option<f32>,
    style: Option<String>,
    colors: Option<Vec<String>>,
    materials: Option<Vec<String>>,
    quality: Option<String>,
    lighting: Option<String>,
    camera_angle: Option<String>,
    dimensions: Option<std::collections::HashMap<String, f32>>,
    language: Option<String>,
}

async fn advanced_text_to_3d_generate(
    State(state): State<AppState>,
    Json(req): Json<IncomingAdvancedTextTo3dRequest>,
) -> Json<serde_json::Value> {
    // В реальной реализации здесь будет логика генерации 3D модели
    // с использованием расширенных параметров
    
    // Пока возвращаем заглушку
    Json(serde_json::json!({
        "status": "completed",
        "taskId": "advanced_task_123",
        "resultUrl": "http://localhost:8080/models/advanced_model.glb",
        "modelHash": "advanced_model_hash_123"
    }))
}

// Добавляем эндпоинт в роутер (внутрь макроса route)
// .route("/api/v1/text-to-3d/advanced", post(advanced_text_to_3d_generate))
