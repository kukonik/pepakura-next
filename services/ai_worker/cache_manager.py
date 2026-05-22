import hashlib
import json
import os
from pathlib import Path
from typing import Optional, Dict, Any, Union

CACHE_DIR = Path.home() / ".pepakura-next" / "cache"

def get_cache_dir() -> Path:
    CACHE_DIR.mkdir(parents=True, exist_ok=True)
    return CACHE_DIR

def compute_hash(file_path: Union[Path, str], params: Dict) -> str:
    hasher = hashlib.sha256()
    # Если передан хэш (строка), используем его напрямую
    if isinstance(file_path, str) and (len(file_path) == 32 or len(file_path) == 64):
        # Это уже хэш (MD5 или SHA256)
        hasher.update(file_path.encode("utf-8"))
        hasher.update(json.dumps(params, sort_keys=True).encode("utf-8"))
        return hasher.hexdigest()
    # Иначе читаем файл
    file_path = Path(file_path)
    if not file_path.exists():
        raise FileNotFoundError(f"File not found: {file_path}")
    with open(file_path, "rb") as f:
        while chunk := f.read(8192):
            hasher.update(chunk)
    hasher.update(json.dumps(params, sort_keys=True).encode("utf-8"))
    return hasher.hexdigest()

def get_cached_mesh_path(image_path: Union[Path, str], quality: str) -> Optional[Path]:
    cache_key = compute_hash(image_path, {"quality": quality})
    cached_obj = get_cache_dir() / f"{cache_key}.obj"
    return cached_obj if cached_obj.exists() else None

def save_to_cache(image_path: Union[Path, str], quality: str, mesh_path: Path, metadata: Dict):
    cache_key = compute_hash(image_path, {"quality": quality})
    target_obj = get_cache_dir() / f"{cache_key}.obj"
    target_meta = get_cache_dir() / f"{cache_key}.meta.json"

    import shutil
    shutil.copy2(mesh_path, target_obj)

    meta = metadata.copy()
    meta.update({"source": str(image_path), "params": {"quality": quality}})
    with open(target_meta, "w") as f:
        json.dump(meta, f)
