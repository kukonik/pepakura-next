import io
import zipfile
from typing import Dict, Any
from .obj_to_glb import convert_obj_string_to_glb
from ..unfolders.svg_unfolder import unfold_mesh_to_svg
from .instruction_pdf_generator import generate_build_instructions_pdf


def generate_project_package(prompt_data: Dict[str, Any], model_obj: str) -> bytes:
    """
    Генерирует ZIP архив для экспорта проекта: .obj, .mtl, .png (текстура), .svg развертка.
    """
    zip_buffer = io.BytesIO()

    # Генерируем SVG-развертку
    svg_data = unfold_mesh_to_svg(model_obj)

    with zipfile.ZipFile(zip_buffer, "w") as zf:
        # 1. OBJ модель
        zf.writestr("model.obj", model_obj)

        # 2. MTL (базовое)
        mtl_content = """
newmtl Material
Ka 1.0 1.0 1.0
Kd 1.0 1.0 1.0
Ks 0.0 0.0 0.0
Ns 100
illum 2
map_Kd texture.png
"""
        zf.writestr("model.mtl", mtl_content.strip())

        # 3. Текстура Placeholder (заменить на реальную, если есть)
        zf.writestr("texture.png", b"")

        # 4. SVG развертка
        zf.writestr("unfold.svg", svg_data)

        # 5. PDF Instructions
        pdf_bytes = generate_build_instructions_pdf(
            model_title=prompt_data.get("title", "Paper Model"),
            model_description=prompt_data["prompt"],
            svg_preview=svg_data[:5000]  # ограничим длину для скорости
        )
        zf.writestr("build_guide.pdf", pdf_bytes)

        # 6. Plain Text Instructions
        instructions = f"""
# Проект: AI Paper Craft Model

Запрос был:
"{prompt_data['prompt']}"

Содержит:
- model.obj - геометрия
- model.mtl - материал
- texture.png - текстура (пустая заглушка)
- unfold.svg - развёртка для печати
- build_guide.pdf - инструкция сборки в формате PDF
"""
        zf.writestr("instructions.txt", instructions.strip())

    zip_buffer.seek(0)
    return zip_buffer.getvalue()
