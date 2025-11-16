<#
.SYNOPSIS
    Полная автоматическая установка Pepakura Next
.DESCRIPTION
    Этот скрипт создает полную структуру проекта Pepakura Next в указанной директории,
    устанавливает необходимые зависимости и готовит проект к запуску в режиме отладки.
.PARAMETER InstallPath
    Путь для установки проекта (по умолчанию: текущая директория).
.PARAMETER NoGPU
    Установить без поддержки GPU (для слабых ПК).
.PARAMETER ForceReinstall
    Принудительно переустановить все зависимости.
.EXAMPLE
    .\install.ps1 -InstallPath "D:\Dev\pepakura-next" -NoGPU -ForceReinstall
#>
param(
    [string]$InstallPath = $(Get-Location).Path,
    [switch]$NoGPU,
    [switch]$ForceReinstall
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

function Write-Log {
    param(
        [string]$Message,
        [string]$Level = "Info"
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $colorMap = @{
        "Success" = "Green"
        "Info" = "Cyan"
        "Warning" = "Yellow"
        "Error" = "Red"
        "Important" = "Magenta"
        "Debug" = "Gray"
    }
    $color = if ($colorMap.ContainsKey($Level)) { $colorMap[$Level] } else { "White" }
    Write-Host "[$timestamp] $Message" -ForegroundColor $color
}

function Test-AdminRights {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

function Main {
    if (-not (Test-AdminRights)) {
        Write-Log "❌ Скрипт требует прав администратора. Пожалуйста, запустите PowerShell от имени администратора." -Level "Error"
        exit 1
    }

    Write-Log "🚀 НАЧАЛО УСТАНОВКИ PEPAKURA NEXT (ПОЛНОСТЬЮ ИСПРАВЛЕННАЯ ВЕРСИЯ)" -Level "Important"
    Write-Log "📁 Папка установки: $InstallPath" -Level "Info"
    Write-Log "🖥️  Режим GPU: $(if($NoGPU) {'Отключен'} else {'Включен'})" -Level "Info"
    Write-Log "🔄 Принудительная переустановка: $(if($ForceReinstall) {'Включена'} else {'Отключена'})" -Level "Info"

    # 1. Создание структуры папок
    Write-Log "📁 Создание структуры папок..." -Level "Info"
    $folders = @(
        "src\backend\unfolding-core\src",
        "src\backend\ai-gateway",
        "src\frontend\web\public",
        "src\frontend\web\src\components",
        "src\frontend\web\src\pages",
        "src\frontend\web\src\services",
        "src\frontend\web\src\utils",
        "src\frontend\web\src\assets",
        "data\models",
        "data\templates",
        "data\cache",
        "data\temp",
        "logs",
        "scripts",
        "models\cpu-optimized",
        "models\gpu-optimized",
        "templates\basic",
        "templates\standard",
        "templates\premium",
        "docs",
        "venv\Scripts"
    )

    foreach ($folder in $folders) {
        $fullPath = Join-Path -Path $InstallPath -ChildPath $folder
        if (-not (Test-Path $fullPath)) {
            New-Item -Path $fullPath -ItemType Directory -Force | Out-Null
            Write-Log "✅ Создана папка: $folder" -Level "Debug"
        } else {
            Write-Log "✅ Папка уже существует: $folder" -Level "Debug"
        }
    }
    Write-Log "✅ Структура папок создана." -Level "Success"

    # 2. Создание файлов Unfolding Core
    Write-Log "🔧 Создание файлов Rust Unfolding Core..." -Level "Info"
    $unfoldingCoreDir = Join-Path -Path $InstallPath -ChildPath "src\backend\unfolding-core"
    $srcDir = Join-Path -Path $unfoldingCoreDir -ChildPath "src"

    # Cargo.toml
    $cargoTomlContent = @"
[package]
name = "pepakura-unfolding-core"
version = "0.1.0"
edition = "2021"

[features]
default = ["server"]
server = ["axum", "tokio", "tracing-subscriber"]

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
thiserror = "1.0"
anyhow = "1.0"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
"@
    $cargoTomlPath = Join-Path -Path $unfoldingCoreDir -ChildPath "Cargo.toml"
    Set-Content -Path $cargoTomlPath -Value $cargoTomlContent -Force -Encoding UTF8
    Write-Log "✅ Cargo.toml создан: $cargoTomlPath" -Level "Success"

    # src/lib.rs
    $libRsContent = @"
#![deny(clippy::all)]
#![deny(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{debug, info};

#[derive(Debug, thiserror::Error)]
pub enum UnfoldingError {
    #[error("Invalid mesh: {0}")]
    InvalidMesh(String),
    #[error("Processing failed: {0}")]
    ProcessingFailed(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfoldingResult {
    pub sheets: Vec<Vec<f64>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UnfoldingConfig {
    pub quality_level: String,
    pub sheet_size: [f64; 2],
    pub optimize_folding_lines: bool,
    pub add_tabs: bool,
}

impl Default for UnfoldingConfig {
    fn default() -> Self {
        Self {
            quality_level: "standard".to_string(),
            sheet_size: [210.0, 297.0],
            optimize_folding_lines: true,
            add_tabs: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnfoldingRequest {
    pub vertices: Vec<f64>,
    pub faces: Vec<Vec<usize>>,
    pub config: UnfoldingConfig,
}

pub struct UnfoldingCore {
    config: UnfoldingConfig,
}

impl UnfoldingCore {
    pub fn new() -> Self {
        Self {
            config: UnfoldingConfig::default(),
        }
    }

    pub fn unfold_mesh(&self, request: &UnfoldingRequest) -> Result<UnfoldingResult, UnfoldingError> {
        info!("Starting unfolding process");
        let start_time = Instant::now();
        
        self.validate_mesh(&request.vertices, &request.faces)?;
        debug!("Mesh validation passed");
        
        let sheets = self.calculate_mock_unfolding(&request.vertices, &request.faces, &request.config)?;
        debug!("Unfolding calculation completed");
        
        let elapsed = start_time.elapsed();
        info!("Unfolding completed in {:.3?}", elapsed);
        
        Ok(UnfoldingResult { sheets })
    }

    fn validate_mesh(&self, vertices: &[f64], faces: &[Vec<usize>]) -> Result<(), UnfoldingError> {
        if vertices.is_empty() {
            return Err(UnfoldingError::InvalidMesh("Mesh has no vertices".to_string()));
        }
        if faces.is_empty() {
            return Err(UnfoldingError::InvalidMesh("Mesh has no faces".to_string()));
        }
        
        if vertices.len() % 3 != 0 {
            return Err(UnfoldingError::InvalidMesh(format!(
                "Vertex count must be multiple of 3, got {}",
                vertices.len()
            )));
        }

        for (i, face) in faces.iter().enumerate() {
            if face.len() < 3 {
                return Err(UnfoldingError::InvalidMesh(format!(
                    "Face {} has less than 3 vertices ({} vertices)",
                    i, face.len()
                )));
            }
            
            for &vertex_index in face {
                let max_index = vertices.len() / 3 - 1;
                if vertex_index > max_index {
                    return Err(UnfoldingError::InvalidMesh(format!(
                        "Vertex index {} out of bounds (max: {})",
                        vertex_index, max_index
                    )));
                }
            }
        }
        Ok(())
    }

    fn calculate_mock_unfolding(
        &self,
        _vertices: &[f64],
        _faces: &[Vec<usize>],
        _config: &UnfoldingConfig,
    ) -> Result<Vec<Vec<f64>>, UnfoldingError> {
        info!("Calculating mock unfolding...");
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        let sheet = vec![0.0, 0.0, 100.0, 0.0, 100.0, 100.0, 0.0, 100.0];
        Ok(vec![sheet])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_mesh_ok() {
        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
        let faces = vec![vec![0, 1, 2]];
        let core = UnfoldingCore::new();
        assert!(core.validate_mesh(&vertices, &faces).is_ok());
    }

    #[test]
    fn test_validate_mesh_invalid_index() {
        let vertices = vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0];
        let faces = vec![vec![0, 1, 5]];
        let core = UnfoldingCore::new();
        assert!(core.validate_mesh(&vertices, &faces).is_err());
    }

    #[test]
    fn test_unfold_mesh_ok() {
        let vertices = vec![
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0
        ];
        let faces = vec![
            vec![0, 1, 2, 3],
            vec![4, 7, 6, 5],
            vec![0, 4, 5, 1],
            vec![2, 6, 7, 3],
            vec![0, 3, 7, 4],
            vec![1, 5, 6, 2]
        ];
        let request = UnfoldingRequest {
            vertices,
            faces,
            config: UnfoldingConfig::default(),
        };
        let core = UnfoldingCore::new();
        let result = core.unfold_mesh(&request);
        assert!(result.is_ok());
        
        if let Ok(unfold_result) = result {
            assert!(!unfold_result.sheets.is_empty());
            assert_eq!(unfold_result.sheets.len(), 1);
            assert_eq!(unfold_result.sheets[0].len(), 8);
        }
    }
}
"@
    $libRsPath = Join-Path -Path $srcDir -ChildPath "lib.rs"
    Set-Content -Path $libRsPath -Value $libRsContent -Force -Encoding UTF8
    Write-Log "✅ lib.rs создан: $libRsPath" -Level "Success"

    # src/main.rs
    $mainRsContent = @"
#[cfg(feature = "server")]
mod server_logic {
    use axum::{
        extract::Json,
        http::StatusCode,
        response::IntoResponse,
        routing::{get, post},
        Router,
    };
    use serde::Serialize;
    use std::net::SocketAddr;
    use tracing::info;
    use pepakura_unfolding_core::{UnfoldingCore, UnfoldingConfig, UnfoldingRequest, UnfoldingResult, UnfoldingError};
    
    #[derive(Serialize)]
    struct ApiResponse {
        status: String,
        version: String,
    }

    #[derive(Serialize)]
    struct UnfoldResponse {
        sheets: Vec<Vec<f64>>,
        success: bool,
    }

    async fn health() -> Json<ApiResponse> {
        info!("Health check requested");
        Json(ApiResponse {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    async fn test_cube() -> impl IntoResponse {
        let cube_request = UnfoldingRequest {
            vertices: vec![
                0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0
            ],
            faces: vec![
                vec![0, 1, 2, 3],
                vec![4, 7, 6, 5],
                vec![0, 4, 5, 1],
                vec![2, 6, 7, 3],
                vec![0, 3, 7, 4],
                vec![1, 5, 6, 2]
            ],
            config: UnfoldingConfig::default(),
        };
        
        let core = UnfoldingCore::new();
        let start_time = std::time::Instant::now();
        
        match core.unfold_mesh(&cube_request) {
            Ok(result) => {
                let elapsed = start_time.elapsed().as_secs_f64();
                info!("Test cube processed in {:.3}s", elapsed);
                (StatusCode::OK, Json(UnfoldResponse { sheets: result.sheets, success: true }))
            }
            Err(e) => {
                info!("Test cube processing failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(UnfoldResponse { sheets: vec![], success: false }))
            }
        }
    }

    pub async fn run_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .init();
        
        info!("Starting Pepakura Next Unfolding Core server...");
        
        let app = Router::new()
            .route("/health", get(health))
            .route("/test-cube", get(test_cube));
        
        let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
        info!("Server listening on {}", addr);
        
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
        
        Ok(())
    }
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    if let Err(e) = server_logic::run_server().await {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(not(feature = "server"))]
fn main() {
    eprintln!("This binary requires the 'server' feature to be enabled. Run with --features server.");
}
"@
    $mainRsPath = Join-Path -Path $srcDir -ChildPath "main.rs"
    Set-Content -Path $mainRsPath -Value $mainRsContent -Force -Encoding UTF8
    Write-Log "✅ main.rs создан: $mainRsPath" -Level "Success"

    Write-Log "✅ Файлы Unfolding Core созданы успешно." -Level "Success"

    # 3. Создание Python AI Gateway
    Write-Log "🧠 Создание файлов Python AI Gateway..." -Level "Info"
    $aiGatewayDir = Join-Path -Path $InstallPath -ChildPath "src\backend\ai-gateway"
    
    # requirements.txt
    $requirementsContent = @"
fastapi==0.110.0
uvicorn==0.29.0
pydantic==2.7.1
python-multipart==0.0.9
numpy==1.26.4
Pillow==10.3.0
imageio==2.34.0
imageio-ffmpeg==0.4.9
structlog==24.1.0
"@
    $requirementsPath = Join-Path -Path $aiGatewayDir -ChildPath "requirements.txt"
    Set-Content -Path $requirementsPath -Value $requirementsContent -Force -Encoding UTF8
    Write-Log "✅ requirements.txt создан: $requirementsPath" -Level "Success"
    
    # main.py
    $aiGatewayContent = @"
import asyncio
import base64
import io
import os
import sys
import time
from datetime import datetime
from typing import Dict, List, Optional, Union

import numpy as np
from PIL import Image
import imageio.v2 as imageio
from fastapi import FastAPI, UploadFile, File, Form, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import structlog
import logging

# Настройка логирования
structlog.configure(
    processors=[
        structlog.stdlib.filter_by_level,
        structlog.stdlib.add_logger_name,
        structlog.stdlib.add_log_level,
        structlog.stdlib.PositionalArgumentsFormatter(),
        structlog.processors.TimeStamper(fmt="iso"),
        structlog.processors.StackInfoRenderer(),
        structlog.processors.format_exc_info,
        structlog.processors.UnicodeDecoder(),
        structlog.processors.JSONRenderer(),
    ],
    context_class=dict,
    logger_factory=structlog.stdlib.LoggerFactory(),
    wrapper_class=structlog.stdlib.BoundLogger,
    cache_logger_on_first_use=True,
)
logger = structlog.get_logger()

class ProcessingResult(BaseModel):
    model_url: str
    preview_image: str  # base64 PNG
    vertex_count: int
    face_count: int
    confidence_score: float
    processing_time: float
    bounding_box: dict
    metadata: Dict[str, Union[str, int, float, bool]] = {}

class GifTo3DResponse(BaseModel):
    status: str
    data: ProcessingResult

class HealthCheckResponse(BaseModel):
    status: str
    service: str
    version: str
    timestamp: str
    python_version: str
    environment: str

app = FastAPI(
    title="Pepakura Next AI Gateway",
    version="0.1.0",
    description="AI-powered 3D model generation and unfolding service.",
    docs_url="/docs", 
    redoc_url="/redoc"
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"]
)

@app.get("/health", response_model=HealthCheckResponse)
async def health_check():
    logger.info("Health check requested")
    return HealthCheckResponse(
        status="healthy",
        service="ai-gateway",
        version="0.1.0",
        timestamp=datetime.utcnow().isoformat(),
        python_version=f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}",
        environment="development"
    )

@app.post("/gif2mesh", response_model=GifTo3DResponse)
async def gif_to_3d(
    file: UploadFile = File(...),
    description: str = Form("")
):
    start_time = time.time()
    request_id = int(start_time * 1e6)
    log = logger.bind(request_id=request_id)
    log.info("GIF processing request received", filename=file.filename, description=description, content_type=file.content_type)
    
    # Валидация файла
    if not file.content_type or not file.content_type.startswith("image/gif"):
        log.error("Invalid file type", content_type=file.content_type)
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail="File must be a GIF")

    file_content = await file.read()
    file_size = len(file_content)
    log.info("File read successfully", size=file_size)
    
    if file_size > 10 * 1024 * 1024:  # 10MB limit
        log.warning("File too large", size=file_size)
        raise HTTPException(status_code=status.HTTP_413_REQUEST_ENTITY_TOO_LARGE, detail="File size exceeds 10MB limit")

    # Извлечение кадров из GIF
    frames = []
    try:
        gif_reader = imageio.get_reader(io.BytesIO(file_content), format="GIF")
        frame_count = 0
        for frame in gif_reader:
            if frame_count >= 10:  # Ограничиваем до 10 кадров
                break
            # Обработка изображения
            if frame.ndim == 3 and frame.shape[-1] == 4:  # RGBA to RGB
                frame = frame[..., :3]
            # Нормализация данных
            frame = (frame * 255).astype(np.uint8) if frame.dtype != np.uint8 else frame
            pil_image = Image.fromarray(frame)
            frames.append(pil_image)
            frame_count += 1
        gif_reader.close()
        log.info("Frames extracted", frame_count=frame_count)
    except Exception as e:
        log.error("Failed to process GIF", error=str(e))
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail=f"Failed to process GIF: {str(e)}")

    # Генерация preview изображения
    preview_buffer = io.BytesIO()
    if frames:
        preview_image = frames[0].convert("RGB").resize((300, 300))
        preview_image.save(preview_buffer, format="PNG")
        preview_base64 = base64.b64encode(preview_buffer.getvalue()).decode("utf-8")
    else:
        # Пустое изображение если кадры не извлечены
        empty_img = Image.new("RGB", (300, 300), color="lightgray")
        empty_img.save(preview_buffer, format="PNG")
        preview_base64 = base64.b64encode(preview_buffer.getvalue()).decode("utf-8")

    # Моковые данные 3D модели
    processing_time = time.time() - start_time
    vertices = [
        0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0
    ]
    faces = [
        [0, 1, 2, 3], [4, 7, 6, 5], [0, 4, 5, 1], [2, 6, 7, 3],
        [0, 3, 7, 4], [1, 5, 6, 2]
    ]

    result = ProcessingResult(
        model_url=f"/models/{request_id}.obj",
        preview_image=preview_base64,
        vertex_count=len(vertices) // 3,
        face_count=len(faces),
        confidence_score=0.85 if frame_count > 0 else 0.3,
        processing_time=processing_time,
        bounding_box={"min": [0.0, 0.0, 0.0], "max": [1.0, 1.0, 1.0]},
        metadata={
            "source_frames": frame_count,
            "description": description,
            "processing_date": datetime.utcnow().isoformat(),
            "ai_model": "mock-3d-generator-v1",
            "frame_dimensions": (frames[0].width, frames[0].height) if frames else (0, 0)
        }
    )
    
    log.info("Processing completed successfully", 
             processing_time=processing_time,
             vertex_count=result.vertex_count,
             face_count=result.face_count)
    
    return GifTo3DResponse(status="success", data=result)

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    logger.info("Starting AI Gateway server")
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000, log_level="info", access_log=True)
"@
    $aiGatewayMainPath = Join-Path -Path $aiGatewayDir -ChildPath "main.py"
    Set-Content -Path $aiGatewayMainPath -Value $aiGatewayContent -Force -Encoding UTF8
    Write-Log "✅ main.py создан: $aiGatewayMainPath" -Level "Success"

    Write-Log "✅ Файлы AI Gateway созданы успешно." -Level "Success"

    # 4. Создание виртуального окружения Python
    Write-Log "🐍 Создание Python виртуального окружения..." -Level "Info"
    $venvPath = Join-Path -Path $InstallPath -ChildPath "venv"
    if (-not (Test-Path $venvPath) -or $ForceReinstall) {
        if (Get-Command "python" -ErrorAction SilentlyContinue) {
            python -m venv $venvPath
            Write-Log "✅ Виртуальное окружение создано: $venvPath" -Level "Success"
        } else {
            Write-Log "⚠️ Python не найден. Пропускаю создание виртуального окружения." -Level "Warning"
        }
    } else {
        Write-Log "✅ Виртуальное окружение уже существует: $venvPath" -Level "Success"
    }

    # 5. Создание скрипта запуска отладки
    Write-Log "🚀 Создание скрипта запуска отладки (run_debug.ps1)..." -Level "Info"
    $runDebugContent = @"
param(
    [switch]`$NoGPU,
    [switch]`$RunAiEngine,
    [switch]`$RunAll
)

`$ErrorActionPreference = "Stop"
`$ProgressPreference = "SilentlyContinue"

function Write-Log {
    param(
        [string]`$Message,
        [string]`$Level = "Info"
    )
    `$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    `$colorMap = @{
        "Success" = "Green"
        "Info" = "Cyan"
        "Warning" = "Yellow"
        "Error" = "Red"
        "Important" = "Magenta"
        "Debug" = "Gray"
    }
    `$color = if (`$colorMap.ContainsKey(`$Level)) { `$colorMap[`$Level] } else { "White" }
    Write-Host " [`$timestamp] `$Message" -ForegroundColor `$color
}

Write-Log "🚀 ЗАПУСК PEPAKURA NEXT (ИСПРАВЛЕННАЯ ВЕРСИЯ)" -Level "Important"
Write-Log "Текущая директория: `(Get-Location`)" -Level "Info"

# Активация Python окружения
`$venvPath = Join-Path -Path (Get-Location) -ChildPath "venv"
`$activateScript = Join-Path -Path `$venvPath -ChildPath "Scripts\Activate.ps1"

if (Test-Path `$activateScript) {
    Write-Log "🐍 Активация Python окружения..." -Level "Info"
    & `$activateScript
} else {
    Write-Log "⚠️ Python окружение не найдено. Пропускаем..." -Level "Warning"
}

# --- Запуск AI Gateway ---
if (`$RunAiEngine -or `$RunAll) {
    `$aiEngineDir = Join-Path -Path (Get-Location) -ChildPath "src\backend\ai-gateway"
    if (Test-Path `$aiEngineDir) {
        Write-Log "🔧 Запуск AI Gateway из: `$aiEngineDir" -Level "Info"
        Set-Location `$aiEngineDir

        # Установка Python зависимостей
        Write-Log "📦 Установка Python зависимостей..." -Level "Info"
        pip install -r requirements.txt --no-cache-dir --upgrade

        Write-Log "🚀 Запуск AI Gateway сервера (порт 8000)..." -Level "Important"
        Write-Log "💡 Для остановки нажмите Ctrl+C в этом окне." -Level "Info"
        python main.py

        if (`$LASTEXITCODE -ne 0) {
            Write-Log "❌ Ошибка при запуске AI Gateway (exit code: `$LASTEXITCODE)" -Level "Error"
            # Не выходим, если запускаем все сервисы
            if (-not `$RunAll) { exit `$LASTEXITCODE }
        } else {
            Write-Log "✅ AI Gateway завершил работу" -Level "Success"
        }
    } else {
        Write-Log "❌ Директория AI Gateway не найдена: `$aiEngineDir" -Level "Error"
        if (-not `$RunAll) { exit 1 }
    }
    # Возврат в корень проекта, если запускаем только AI
    if (-not `$RunAll) {
        Set-Location (Get-Location).Parent.Parent.Parent.Parent
        Write-Log "🏁 Запуск AI Engine завершен." -Level "Info"
        exit 0
    }
}

# --- Запуск Unfolding Core ---
if (-not `$RunAiEngine) { # Запускаем Core, если не выбран только AI Engine
    `$unfoldingCoreDir = Join-Path -Path (Get-Location) -ChildPath "src\backend\unfolding-core"

    if (Test-Path `$unfoldingCoreDir) {
        Write-Log "🔧 Запуск Unfolding Core из: `$unfoldingCoreDir" -Level "Info"
        Set-Location `$unfoldingCoreDir

        # Сборка перед запуском
        Write-Log "📦 Сборка Unfolding Core..." -Level "Info"
        cargo build --release --features server --quiet

        # Запуск сервера
        Write-Log "🚀 Запуск сервера Unfolding Core (порт 3000)..." -Level "Important"
        Write-Log "💡 Для остановки нажмите Ctrl+C в этом окне." -Level "Info"
        cargo run --release --features server

        if (`$LASTEXITCODE -ne 0) {
            Write-Log "❌ Ошибка при запуске Unfolding Core (exit code: `$LASTEXITCODE)" -Level "Error"
            # Не выходим, если запускаем все сервисы
            if (-not `$RunAll) { exit `$LASTEXITCODE }
        } else {
            Write-Log "✅ Unfolding Core завершил работу" -Level "Success"
        }
    } else {
        Write-Log "❌ Директория Unfolding Core не найдена: `$unfoldingCoreDir" -Level "Error"
        if (-not `$RunAll) { exit 1 }
    }
    # Возврат в корень проекта, если запускаем только Core
    if (-not `$RunAll) {
        Set-Location (Get-Location).Parent.Parent.Parent.Parent
        Write-Log "🏁 Запуск Unfolding Core завершен." -Level "Info"
        exit 0
    }
}

# Возврат в корень проекта, если запускаем все
if (`$RunAll) {
    Set-Location (Get-Location).Parent.Parent.Parent.Parent
    Write-Log "🏁 Запуск всех сервисов завершен." -Level "Info"
}
"@
    $runDebugPath = Join-Path -Path $InstallPath -ChildPath "run_debug.ps1"
    Set-Content -Path $runDebugPath -Value $runDebugContent -Force -Encoding UTF8
    Write-Log "✅ run_debug.ps1 создан: $runDebugPath" -Level "Success"

    Write-Log "✅ ВСЕ ФАЙЛЫ СОЗДАНЫ УСПЕШНО!" -Level "Success"
    Write-Log "📋 ИНСТРУКЦИИ ПО ЗАПУСКУ:" -Level "Important"
    Write-Log " 1. Перейдите в папку проекта: cd `"$InstallPath`"" -Level "Info"
    Write-Log " 2. Запустите отладку: .\run_debug.ps1 -NoGPU -RunAiEngine" -Level "Info"
    Write-Log " 3. Откройте браузер: http://localhost:8000/health" -Level "Info"
    Write-Log " 4. Для остановки нажмите Ctrl+C в консоли" -Level "Info"
}

Main