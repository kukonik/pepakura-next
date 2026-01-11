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
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    text_service: Arc<dyn TextService>,
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
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/v1/text/generate", post(text_generate))
        .route("/api/v1/image/generate", post(image_generate))
        .route("/api/v1/d3/generate", post(d3_generate))
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
