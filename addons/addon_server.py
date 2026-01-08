import json
import sys
import traceback
import os


def analyze_obj(path: str) -> dict:
    """Примитивный анализатор OBJ: считает вершины и грани по строкам v / f."""
    vertices = 0
    faces = 0

    try:
        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            for line in f:
                # вершины
                if line.startswith("v "):
                    vertices += 1
                # грани
                elif line.startswith("f "):
                    faces += 1
    except Exception as e:
        return {
            "ok": False,
            "error": f"analyze_obj failed: {e}",
        }

    # parts как заглушка = 1 (цельная модель)
    return {
        "ok": True,
        "result": {
            "format": "obj",
            "faces": faces,
            "vertices": vertices,
            "parts": 1,
        },
    }


def analyze_stl(path: str) -> dict:
    """Очень грубая заглушка для STL: считает количество строк 'facet normal' как число граней."""
    faces = 0

    try:
        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            for line in f:
                if "facet normal" in line:
                    faces += 1
    except Exception as e:
        return {
            "ok": False,
            "error": f"analyze_stl failed: {e}",
        }

    return {
        "ok": True,
        "result": {
            "format": "stl",
            "faces": faces,
            "vertices": None,
            "parts": 1,
        },
    }


def analyze_gltf_like(path: str) -> dict:
    """
    Заглушка для GLTF/GLB: пока только возвращает формат и размер файла.
    При желании можно подключить pygltflib / open3d.
    """
    try:
        size_bytes = os.path.getsize(path)
    except Exception as e:
        return {
            "ok": False,
            "error": f"analyze_gltf_like failed: {e}",
        }

    return {
        "ok": True,
        "result": {
            "format": "gltf_glb",
            "faces": None,
            "vertices": None,
            "parts": 1,
            "file_size": size_bytes,
        },
    }


def handle_model_analyze(payload: dict) -> dict:
    """Роутер для model_tools/analyze_model."""
    path = payload.get("path")
    fmt = payload.get("format", "auto")

    if not path:
        return {
            "ok": False,
            "error": "analyze_model: payload.path is required",
        }

    if not os.path.exists(path):
        return {
            "ok": False,
            "error": f"analyze_model: file not found: {path}",
        }

    # автоопределение формата по расширению
    ext = os.path.splitext(path)[1].lower().lstrip(".")
    if fmt == "auto":
        fmt = ext

    if fmt in ("obj",):
        return analyze_obj(path)

    if fmt in ("stl",):
        return analyze_stl(path)

    if fmt in ("gltf", "glb"):
        return analyze_gltf_like(path)

    return {
        "ok": False,
        "error": f"analyze_model: unsupported format '{fmt}' for file {path}",
    }


def handle_request(req: dict) -> dict:
    tool = req.get("tool")
    op = req.get("op")
    payload = req.get("payload", {})

    # Новый инструмент: анализ модели
    if tool == "model_tools" and op == "analyze_model":
        return handle_model_analyze(payload)

    if tool == "blender" and op == "unfold":
        return {
            "ok": True,
            "result": {
                "message": "Blender-unfold заглушка, всё прошло ок",
                "input": payload,
            },
        }

    if tool == "mock" and op == "ping":
        return {
            "ok": True,
            "result": {
                "message": "pong from addon_server",
            },
        }

    return {
        "ok": False,
        "error": f"Unknown tool/op combination: {tool}/{op}",
    }


def main():
    try:
        raw = sys.stdin.read()
        if not raw:
            print(json.dumps({"ok": False, "error": "empty stdin"}))
            return

        try:
            req = json.loads(raw)
        except json.JSONDecodeError as e:
            print(json.dumps({"ok": False, "error": f"json decode error: {e}"}))
            return

        resp = handle_request(req)
        print(json.dumps(resp))
    except Exception as e:
        print(
            json.dumps(
                {
                    "ok": False,
                    "error": f"exception: {e}",
                    "traceback": traceback.format_exc(),
                }
            )
        )


if __name__ == "__main__":
    main()
