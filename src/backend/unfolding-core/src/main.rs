use axum::{
    extract::State,
    http::StatusCode,
    response::{Json, IntoResponse},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::Arc,
    time::Instant,
};
use tracing::{info, instrument};

// Используем текущий крейт вместо unfolding_core
use pepakura_unfolding_core::{
    UnfoldingCore, UnfoldingRequest, UnfoldingConfig, QualityLevel,
};

#[derive(Debug, Clone)]
struct AppState {
    version: String,
    unfolding_core: UnfoldingCore,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    service: String,
}

#[derive(Serialize)]
struct UnfoldResponse {
    sheets: Vec<Vec<[f64; 2]>>,
    success: bool,
    processing_time_ms: u128,
    metadata: UnfoldingMetadata,
}

#[derive(Serialize)]
struct UnfoldingMetadata {
    sheet_count: usize,
    total_area: f64,
    bounds: [f64; 4],
}

#[derive(Deserialize)]
struct UnfoldRequest {
    vertices: Vec<f64>,
    faces: Vec<Vec<usize>>,
    quality: Option<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    code: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let status = match self.code.as_str() {
            "INVALID_MESH" => StatusCode::BAD_REQUEST,
            "PROCESSING_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        
        (status, Json(self)).into_response()
    }
}

#[instrument]
async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    info!("Health check requested");
    
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: state.version.clone(),
        service: "Pepakura Unfolding API".to_string(),
    })
}

#[instrument]
async fn test_cube() -> Json<UnfoldResponse> {
    info!("Test cube requested");
    
    let sheet = vec![
        [0.0, 0.0],
        [100.0, 0.0], 
        [100.0, 100.0],
        [0.0, 100.0],
    ];

    Json(UnfoldResponse {
        sheets: vec![sheet],
        success: true,
        processing_time_ms: 0,
        metadata: UnfoldingMetadata {
            sheet_count: 1,
            total_area: 10000.0,
            bounds: [0.0, 0.0, 100.0, 100.0],
        },
    })
}

#[instrument(skip_all)]
async fn unfold_mesh(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UnfoldRequest>,
) -> Result<Json<UnfoldResponse>, ErrorResponse> {
    info!("Unfolding mesh with {} vertices and {} faces", payload.vertices.len(), payload.faces.len());

    let start_time = Instant::now();

    // Определяем уровень качества
    let quality_level = match payload.quality.as_deref() {
        Some("draft") => QualityLevel::Draft,
        Some("high") => QualityLevel::High,
        Some("production") => QualityLevel::Production,
        _ => QualityLevel::Standard,
    };

    // Создаем конфигурацию
    let config = UnfoldingConfig {
        quality_level,
        ..Default::default()
    };

    // Создаем запрос на развертку
    let request = match UnfoldingRequest::from_flat_data(payload.vertices, payload.faces, config) {
        Ok(req) => req,
        Err(e) => {
            return Err(ErrorResponse {
                error: e.to_string(),
                code: "INVALID_MESH".to_string(),
            });
        }
    };

    // Выполняем развертку
    let result = match state.unfolding_core.unfold_mesh(&request) {
        Ok(res) => res,
        Err(e) => {
            return Err(ErrorResponse {
                error: e.to_string(),
                code: "PROCESSING_ERROR".to_string(),
            });
        }
    };

    let processing_time = start_time.elapsed();

    // Конвертируем результат в формат API
    let sheets: Vec<Vec<[f64; 2]>> = result.sheets
        .into_iter()
        .map(|sheet| {
            sheet.into_iter()
                .map(|v| [v.x, v.y])
                .collect()
        })
        .collect();

    let response = UnfoldResponse {
        sheets,
        success: true,
        processing_time_ms: processing_time.as_millis(),
        metadata: UnfoldingMetadata {
            sheet_count: result.metadata.sheet_count,
            total_area: result.metadata.total_area,
            bounds: result.metadata.bounds,
        },
    };

    info!("Unfolding completed in {}ms", response.processing_time_ms);
    Ok(Json(response))
}

#[instrument]
async fn server_info(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    info!("Server info requested");

    Json(serde_json::json!({
        "service": "Pepakura Unfolding API",
        "version": state.version,
        "endpoints": {
            "health": "/health",
            "test_cube": "/test-cube", 
            "unfold": "/unfold",
            "info": "/info"
        },
        "features": {
            "max_vertices": 100000,
            "supported_formats": ["obj", "stl", "ply"],
            "quality_levels": ["draft", "standard", "high", "production"]
        }
    }))
}

async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Инициализируем логгирование
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Pepakura Unfolding Server...");
    
    // Создаем состояние приложения
    let state = Arc::new(AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        unfolding_core: UnfoldingCore::with_default_config(),
    });
    
    // Создаем маршруты
    let app = Router::new()
        .route("/health", get(health))
        .route("/test-cube", get(test_cube))
        .route("/unfold", post(unfold_mesh))
        .route("/info", get(server_info))
        .with_state(state);
    
    // Настраиваем адрес
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Server listening on {}", addr);
    info!("Available endpoints:");
    info!("  GET  /health     - Health check");
    info!("  GET  /test-cube  - Test cube unfolding");
    info!("  POST /unfold     - Unfold custom mesh");
    info!("  GET  /info       - Server information");
    
    // Запускаем сервер
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_server().await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}