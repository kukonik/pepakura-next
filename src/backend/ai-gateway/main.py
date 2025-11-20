import asyncio
import base64
import io
import os
import sys
import time
from datetime import datetime
from typing import Dict, Union

import numpy as np
from PIL import Image
import imageio.v2 as imageio
from fastapi import FastAPI, UploadFile, File, Form, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
import structlog
import logging

# Логирование
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

class ImageTo3DResponse(BaseModel):
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
    version="0.2.0",
    description="AI-powered image to 3D model service (GIF, PNG, JPG).",
    docs_url="/docs", 
    redoc_url="/redoc"
)

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
        version="0.2.0",
        timestamp=datetime.utcnow().isoformat(),
        python_version=f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}",
        environment="development"
    )

@app.post("/image2mesh", response_model=ImageTo3DResponse)
async def image_to_3d(
    file: UploadFile = File(...),
    description: str = Form("")
):
    start_time = time.time()
    request_id = int(start_time * 1e6)
    log = logger.bind(request_id=request_id)
    log.info("Image2Mesh request received", filename=file.filename, description=description, content_type=file.content_type)
    
    # Разрешённые типы файлов
    allowed_types = ["image/gif", "image/png", "image/jpeg", "image/jpg"]
    if not file.content_type or file.content_type.lower() not in allowed_types:
        log.error("Invalid file type", content_type=file.content_type)
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail="File must be GIF, PNG или JPEG")

    file_content = await file.read()
    file_size = len(file_content)
    log.info("File read successfully", size=file_size)
    
    if file_size > 10 * 1024 * 1024:  # 10MB ограничение
        log.warning("File too large", size=file_size)
        raise HTTPException(status_code=status.HTTP_413_REQUEST_ENTITY_TOO_LARGE, detail="File size exceeds 10MB limit")

    frames = []
    frame_count = 0
    ext = file.filename.lower().split('.')[-1]
    try:
        if file.content_type.lower() == "image/gif":
            gif_reader = imageio.get_reader(io.BytesIO(file_content), format="GIF")
            for frame in gif_reader:
                if frame_count >= 10: break
                if frame.ndim == 3 and frame.shape[-1] == 4:
                    frame = frame[..., :3]
                frame = (frame * 255).astype(np.uint8) if frame.dtype != np.uint8 else frame
                pil_image = Image.fromarray(frame)
                frames.append(pil_image)
                frame_count += 1
            gif_reader.close()
            log.info("GIF frames extracted", frame_count=frame_count)
        else:
            # Для PNG/JPG одной картинкой
            image = Image.open(io.BytesIO(file_content))
            frames.append(image.convert("RGB"))
            frame_count = 1
            log.info("Static image loaded", format=ext)
    except Exception as e:
        log.error("Failed to process image", error=str(e))
        raise HTTPException(status_code=status.HTTP_400_BAD_REQUEST, detail=f"Failed to process image: {str(e)}")

    preview_buffer = io.BytesIO()
    if frames:
        preview_image_obj = frames[0].resize((300, 300))
        preview_image_obj.save(preview_buffer, format="PNG")
        preview_base64 = base64.b64encode(preview_buffer.getvalue()).decode("utf-8")
        frame_dimensions_str = f"{frames[0].width}x{frames[0].height}"
    else:
        empty_img = Image.new("RGB", (300, 300), color="lightgray")
        empty_img.save(preview_buffer, format="PNG")
        preview_base64 = base64.b64encode(preview_buffer.getvalue()).decode("utf-8")
        frame_dimensions_str = "0x0"

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
        confidence_score=0.9 if frame_count > 0 else 0.3,
        processing_time=processing_time,
        bounding_box={"min": [0.0, 0.0, 0.0], "max": [1.0, 1.0, 1.0]},
        metadata={
            "source_frames": frame_count,
            "description": description,
            "processing_date": datetime.utcnow().isoformat(),
            "ai_model": "mock-3d-generator-v1",
            "frame_dimensions": frame_dimensions_str, 
            "input_type": file.content_type
        }
    )
    
    log.info("Processing completed successfully", 
             processing_time=processing_time,
             vertex_count=result.vertex_count,
             face_count=result.face_count)
    
    return ImageTo3DResponse(status="success", data=result)

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    logger.info("Starting AI Gateway server")
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000, log_level="info", access_log=True)
