import json
import sys
import traceback
import os
import xml.etree.ElementTree as ET


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


def create_simple_svg_from_obj(path: str) -> str:
    """Создает простой SVG из OBJ файла как заглушку."""
    # Читаем OBJ файл и извлекаем вершины
    vertices = []
    faces = []
    
    try:
        with open(path, "r", encoding="utf-8", errors="ignore") as f:
            for line in f:
                if line.startswith("v "):
                    # Извлекаем координаты вершины
                    coords = line.strip().split()[1:]
                    if len(coords) >= 2:
                        x = float(coords[0])
                        y = float(coords[1])
                        vertices.append((x, y))
                elif line.startswith("f "):
                    # Извлекаем индексы вершин грани
                    indices = line.strip().split()[1:]
                    face_indices = []
                    for idx in indices:
                        # Обрабатываем различные форматы индексов (вершина/текстура/нормаль)
                        vertex_idx = idx.split("/")[0]
                        try:
                            face_indices.append(int(vertex_idx) - 1)  # OBJ индексы начинаются с 1
                        except ValueError:
                            pass
                    if len(face_indices) >= 2:
                        faces.append(face_indices)
    except Exception as e:
        raise Exception(f"Failed to parse OBJ file: {e}")
    
    # Создаем SVG
    if not vertices:
        # Если вершин нет, создаем простой SVG с сообщением
        svg_content = '''<svg xmlns="http://www.w3.org/2000/svg" width="400" height="300" viewBox="0 0 400 300">
  <rect width="100%" height="100%" fill="#f0f0f0"/>
  <text x="50%" y="50%" font-family="Arial" font-size="16" fill="#333" text-anchor="middle" dominant-baseline="middle">
    No vertices found in OBJ file
  </text>
</svg>'''
        return svg_content
    
    # Определяем границы модели для масштабирования
    if vertices:
        min_x = min(v[0] for v in vertices)
        max_x = max(v[0] for v in vertices)
        min_y = min(v[1] for v in vertices)
        max_y = max(v[1] for v in vertices)
        
        # Добавляем отступ
        padding = 10
        width = max_x - min_x + 2 * padding
        height = max_y - min_y + 2 * padding
        
        # Масштабируем, чтобы заполнить SVG
        scale = min(380 / width if width > 0 else 1, 280 / height if height > 0 else 1)
        
        # Создаем SVG с полигонами
        svg_content = f'<svg xmlns="http://www.w3.org/2000/svg" width="400" height="300" viewBox="0 0 400 300">\n'
        svg_content += '  <rect width="100%" height="100%" fill="#ffffff"/>\n'
        
        # Рисуем каждую грань как полигон
        for face_indices in faces:
            points = []
            for idx in face_indices:
                if 0 <= idx < len(vertices):
                    x = (vertices[idx][0] - min_x + padding) * scale
                    y = (vertices[idx][1] - min_y + padding) * scale
                    points.append(f"{x:.2f},{y:.2f}")
            
            if len(points) >= 2:
                points_str = " ".join(points)
                svg_content += f'  <polygon points="{points_str}" fill="none" stroke="#333333" stroke-width="1"/>\n'
        
        svg_content += '</svg>'
    else:
        # Если нет вершин, создаем простой SVG с сообщением
        svg_content = '''<svg xmlns="http://www.w3.org/2000/svg" width="400" height="300" viewBox="0 0 400 300">
  <rect width="100%" height="100%" fill="#f0f0f0"/>
  <text x="50%" y="50%" font-family="Arial" font-size="16" fill="#333" text-anchor="middle" dominant-baseline="middle">
    No vertices found in OBJ file
  </text>
</svg>'''
    
    return svg_content


def handle_unfold_model(payload: dict) -> dict:
    """Обработчик для развертки модели в SVG."""
    path = payload.get("path")
    
    if not path:
        return {
            "ok": False,
            "error": "unfold_model: payload.path is required",
        }
    
    # Проверяем существование файла
    if not os.path.exists(path):
        return {
            "ok": False,
            "error": f"unfold_model: file not found: {path}",
        }
    
    try:
        # Создаем SVG из OBJ файла
        svg_content = create_simple_svg_from_obj(path)
        
        return {
            "ok": True,
            "result": {
                "svg": svg_content,
                "message": "Model unfolded successfully"
            }
        }
    except Exception as e:
        return {
            "ok": False,
            "error": f"unfold_model failed: {e}",
        }


def handle_request(req: dict) -> dict:
    tool = req.get("tool")
    op = req.get("op")
    payload = req.get("payload", {})

    # Новый инструмент: анализ модели
    if tool == "model_tools" and op == "analyze_model":
        return handle_model_analyze(payload)
    
    # Новый инструмент: развертка модели
    if tool == "model_tools" and op == "unfold_model":
        return handle_unfold_model(payload)

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