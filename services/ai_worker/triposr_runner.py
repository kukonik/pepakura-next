#!/usr/bin/env python3
import argparse
import json
import os
import sys
from pathlib import Path
from typing import Literal
import shutil

CURRENT_DIR = Path(__file__).parent
TRIPOSR_DIR = CURRENT_DIR / "TripoSR"
if TRIPOSR_DIR.exists():
    sys.path.append(str(TRIPOSR_DIR))
else:
    print(json.dumps({"success": False, "error": f"TripoSR directory not found at {TRIPOSR_DIR}"}))
    sys.exit(1)

import torch
from PIL import Image
import numpy as np
import trimesh

try:
    from tsr.system import TSR
except ImportError as e:
    print(json.dumps({"success": False, "error": f"Failed to import TripoSR: {e}. Check dependencies."}))
    sys.exit(1)

try:
    from cache_manager import get_cached_mesh_path, save_to_cache
except ImportError:
    def get_cached_mesh_path(*args, **kwargs): return None
    def save_to_cache(*args, **kwargs): pass

Quality = Literal["fast", "balanced", "high"]

def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--input", required=True)
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--quality", default="balanced", choices=["fast", "balanced", "high"])
    return parser.parse_args()

def select_device():
    return "cuda" if torch.cuda.is_available() else "cpu"

def map_quality(quality):
    if quality == "fast": return {"image_size": 256, "chunk_size": 16384}
    if quality == "high": return {"image_size": 512, "chunk_size": 4096}
    return {"image_size": 384, "chunk_size": 8192}

def load_image(path, size):
    img = Image.open(path).convert("RGB").resize((size, size), Image.BICUBIC)
    return np.array(img)

def init_model(device, params):
    model = TSR.from_pretrained("stabilityai/TripoSR")
    model.renderer.set_chunk_size(params["chunk_size"])
    model = model.to(device)
    model.eval()
    return model

def main():
    args = parse_args()
    input_path = Path(args.input)
    output_dir = Path(args.output_dir)
    quality = args.quality

    result = {"success": False, "mesh_path": None, "error": None, "cached": False, "device": None}

    try:
        cached = get_cached_mesh_path(input_path, quality)
        if cached:
            output_path = output_dir / "triposr_mesh.obj"
            shutil.copy2(cached, output_path)
            result.update({"success": True, "mesh_path": str(output_path.resolve()), "cached": True, "device": "cache"})
            print(json.dumps(result))
            return

        device = select_device()
        params = map_quality(quality)
        model = init_model(device, params)
        image_np = load_image(input_path, params["image_size"])

        scene_codes = model.from_images([image_np], device=device)
        meshes = model.extract_mesh(scene_codes, has_vertex_color=False)

        if not meshes:
            raise RuntimeError("No meshes generated")
        mesh = meshes[0]

        output_dir.mkdir(parents=True, exist_ok=True)
        mesh_path = output_dir / "triposr_mesh.obj"
        mesh.export(str(mesh_path))

        save_to_cache(input_path, quality, mesh_path, {"vertex_count": len(mesh.vertices)})
        result.update({"success": True, "mesh_path": str(mesh_path.resolve()), "cached": False, "device": device})
    except Exception as e:
        result["error"] = str(e)

    print(json.dumps(result, ensure_ascii=False))

if __name__ == "__main__":
    main()
