from __future__ import annotations
import math
from typing import List, Tuple, Dict

from fastapi import FastAPI, HTTPException
# Импортируем CORS middleware
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
import numpy as np

app = FastAPI(title="Pepakura Next AI Seams API", version="0.1.0")

# === НАСТРОЙКА CORS ===
# Разрешаем запросы от нашего Vite-сервера (5173 и 5174)
app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "http://localhost:5173",
        "http://localhost:5174",
        "http://127.0.0.1:5173",
        "http://127.0.0.1:5174"
    ],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# ==================== DTO (Data Transfer Objects) ====================

class Vector3(BaseModel):
    x: float
    y: float
    z: float

class MeshIn(BaseModel):
    vertices: List[Vector3] = Field(..., description="Массив вершин")
    indices: List[int] = Field(..., description="Индексы треугольников")

class SeamEdge(BaseModel):
    v1: int = Field(..., description="Индекс первой вершины")
    v2: int = Field(..., description="Индекс второй вершины")
    confidence: float = Field(default=1.0, ge=0.0, le=1.0, description="Уверенность алгоритма")

class SeamsOut(BaseModel):
    edges: List[SeamEdge]
    score: float = Field(..., description="Оценка сложности разрезки")

# ==================== Логика анализа (Mock AI) ====================

def _normalize(v: np.ndarray) -> np.ndarray:
    norm = np.linalg.norm(v)
    if norm == 0:
        return v
    return v / norm

def _compute_face_normals(vertices: np.ndarray, indices: np.ndarray) -> np.ndarray:
    v0 = vertices[indices[:, 0]]
    v1 = vertices[indices[:, 1]]
    v2 = vertices[indices[:, 2]]
    edge1 = v1 - v0
    edge2 = v2 - v0
    normals = np.cross(edge1, edge2)
    return _normalize(normals)

def _build_edge_adjacency(indices: np.ndarray) -> Dict[Tuple[int, int], List[int]]:
    edge_map: Dict[Tuple[int, int], List[int]] = {}
    for face_idx, tri in enumerate(indices):
        a, b, c = tri.tolist()
        for v1, v2 in [(a, b), (b, c), (c, a)]:
            key = (min(v1, v2), max(v1, v2))
            edge_map.setdefault(key, []).append(face_idx)
    return edge_map

def predict_seams_logic(vertices: np.ndarray, indices: np.ndarray, curvature_deg: float = 30.0) -> SeamsOut:
    face_normals = _compute_face_normals(vertices, indices)
    edge_adjacency = _build_edge_adjacency(indices)
    threshold_rad = math.radians(curvature_deg)
    
    seams: List[SeamEdge] = []
    total_score = 0.0

    for (v1, v2), face_indices in edge_adjacency.items():
        is_boundary = len(face_indices) == 1
        is_complex_topology = len(face_indices) > 2

        if is_boundary:
            seams.append(SeamEdge(v1=v1, v2=v2, confidence=1.0))
            total_score += 1.0
            continue
        
        if is_complex_topology:
            seams.append(SeamEdge(v1=v1, v2=v2, confidence=0.8))
            total_score += 0.8
            continue

        # Анализ угла между двумя соседними гранями
        f1, f2 = face_indices[0], face_indices[1]
        n1 = face_normals[f1]
        n2 = face_normals[f2]
        
        dot = float(np.dot(n1, n2))
        dot = max(-1.0, min(1.0, dot))
        angle = math.acos(dot)

        if angle >= threshold_rad:
            # Нормализация уверенности от 0 до 1
            conf = min(1.0, angle / (math.pi / 2))
            seams.append(SeamEdge(v1=v1, v2=v2, confidence=conf))
            total_score += conf

    final_score = total_score / len(seams) if seams else 0.0
    return SeamsOut(edges=seams, score=final_score)

# ==================== Endpoints ====================

@app.post("/api/seams/predict", response_model=SeamsOut)
async def predict_seams(mesh: MeshIn, curvature_deg: float = 30.0):
    if len(mesh.indices) % 3 != 0:
        raise HTTPException(status_code=400, detail="Indices count must be multiple of 3")

    verts_np = np.array([[v.x, v.y, v.z] for v in mesh.vertices], dtype=np.float32)
    inds_np = np.array(mesh.indices, dtype=np.uint32).reshape(-1, 3)

    result = predict_seams_logic(verts_np, inds_np, curvature_deg)
    return result

@app.get("/health")
def health_check():
    return {"status": "ok", "cors": "enabled"}
