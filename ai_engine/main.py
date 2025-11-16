# -*- coding: utf-8 -*-
import os
from fastapi import FastAPI, UploadFile, File, HTTPException
from fastapi.responses import JSONResponse
from fastapi.middleware.cors import CORSMiddleware
import uvicorn
from datetime import datetime
import json

app = FastAPI(title="Pepakura Next AI Engine", version="1.0.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# State
engine_state = {
    "status": "ready",
    "version": "1.0.0",
    "models_loaded": 0,
}

@app.get("/health")
def health_check():
    return JSONResponse(content={"status": "ok", "data": engine_state})

@app.get("/api/status")
def get_status():
    return JSONResponse(content={
        "engine": "Pepakura Next AI",
        "version": "1.0.0",
        "timestamp": datetime.now().isoformat(),
    })

@app.post("/api/gif2svg")
async def gif2svg(file: UploadFile = File(...)):
    filename = file.filename
    _, ext = os.path.splitext(filename)
    ext = ext.lower()
    if ext not in [".gif", ".png", ".jpg", ".jpeg"]:
        raise HTTPException(status_code=400, detail="Allowed: .gif .png .jpg .jpeg")
    
    ts = datetime.now().strftime("%Y%m%d_%H%M%S")
    input_path = f"input_{ts}{ext}"
    with open(input_path, "wb") as buffer:
        buffer.write(await file.read())
    
    svg_path = f"output_{ts}.svg"
    svg_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
  <rect x="10" y="10" width="180" height="180" fill="#e0e0e0" stroke="#333" stroke-width="2"/>
  <text x="100" y="100" font-size="14" text-anchor="middle" dominant-baseline="middle" font-family="Arial">{filename}</text>
  <text x="100" y="130" font-size="12" text-anchor="middle" fill="#666">Processed: {ts}</text>
</svg>"""
    
    with open(svg_path, "w", encoding="utf-8") as f:
        f.write(svg_content)
    
    return JSONResponse(content={
        "input_file": input_path,
        "svg_file": svg_path,
        "status": "completed",
        "timestamp": ts,
    })

@app.post("/api/model3d")
async def process_3d_model(file: UploadFile = File(...)):
    filename = file.filename
    _, ext = os.path.splitext(filename)
    ext = ext.lower()
    if ext not in [".obj", ".stl", ".ply", ".glb", ".gltf"]:
        raise HTTPException(status_code=400, detail="Allowed: .obj .stl .ply .glb .gltf")
    
    ts = datetime.now().strftime("%Y%m%d_%H%M%S")
    input_path = f"model_{ts}{ext}"
    with open(input_path, "wb") as buffer:
        buffer.write(await file.read())
    
    return JSONResponse(content={
        "model_file": input_path,
        "status": "processing",
        "timestamp": ts,
    })

if __name__ == "__main__":
    uvicorn.run(app, host="0.0.0.0", port=9000, reload=False)
