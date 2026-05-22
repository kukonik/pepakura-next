import sys
import json
import os
import trimesh
import numpy as np
from typing import Dict, Any


def unfold_mesh_to_svg(input_obj_path: str, output_svg_path: str) -> Dict[str, Any]:
    """
    Развертывает 3D модель в 2D SVG с использованием простой проекции.
    
    Алгоритм:
    1. Загружает 3D модель из OBJ файла
    2. Применяет простую проекцию на плоскость XY
    3. Создает SVG с полигонами деталей
    
    Args:
        input_obj_path: Путь к входному OBJ файлу
        output_svg_path: Путь к выходному SVG файлу
         
    Returns:
        Dict с результатом операции
    """
    try:
        print("Generating SVG...")  # Добавляем логирование
        print(f"Input path: {input_obj_path}")  # Добавляем логирование
        print(f"Output path: {output_svg_path}")  # Добавляем логирование
        
        # Загружаем 3D модель
        mesh = trimesh.load(input_obj_path)
        
        if not hasattr(mesh, 'faces'):
            error_msg = "Не удалось загрузить модель или модель не содержит граней"
            print(f"Error: {error_msg}")  # Добавляем логирование
            return {
                "status": "error",
                "message": error_msg
            }
        
        # Пространство для SVG
        vertices_2d = mesh.vertices[:, :2]  # Простая проекция на XY
        min_coord = np.min(vertices_2d, axis=0)
        max_coord = np.max(vertices_2d, axis=0)
        
        # Добавляем отступы
        padding = 10
        width = (max_coord[0] - min_coord[0]) + 2 * padding
        height = (max_coord[1] - min_coord[1]) + 2 * padding
         
        # Нормализуем координаты
        normalized_vertices = vertices_2d - min_coord + padding
         
        # Создаем SVG
        svg_content = [
            f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}mm" height="{height}mm" '
            f'viewBox="0 0 {width} {height}" version="1.1">',
            '<g fill="none" stroke="#000" stroke-width="0.1">'
        ]
         
        # Добавляем полигоны для каждой грани
        for face in mesh.faces:
            points = []
            for vertex_index in face:
                x, y = normalized_vertices[vertex_index]
                points.append(f"{x:.2f},{y:.2f}")
             
            points_str = " ".join(points)
            svg_content.append(f'<polygon points="{points_str}" />')
         
        svg_content.append('</g>')
        svg_content.append('</svg>')
         
        # Записываем SVG файл
        with open(output_svg_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(svg_content))
         
        print(f"SVG file created successfully at: {output_svg_path}")  # Добавляем логирование
         
        return {
            "status": "success",
            "message": "Развёртка успешно выполнена",
            "svg_path": output_svg_path
        }
         
    except Exception as e:
        error_msg = f"Ошибка при выполнении развёртки: {str(e)}"
        print(f"Exception: {error_msg}")  # Добавляем логирование
        return {
            "status": "error",
            "message": error_msg
        }


def main():
    """
    Основная точка входа для вызова из Rust.
    """
    if len(sys.argv) != 3:
        print(json.dumps({
            "status": "error",
            "message": "Неверное количество аргументов. Ожидается: input_obj_path output_svg_path"
        }))
        sys.exit(1)
    
    input_obj_path = sys.argv[1]
    output_svg_path = sys.argv[2]
    
    # Проверяем существование входного файла
    if not os.path.exists(input_obj_path):
        print(json.dumps({
            "status": "error",
            "message": f"Входной файл не найден: {input_obj_path}"
        }))
        sys.exit(1)
    
    # Выполняем развёртку
    result = unfold_mesh_to_svg(input_obj_path, output_svg_path)
    
    # Выводим результат в формате JSON
    print(json.dumps(result, ensure_ascii=False))


if __name__ == "__main__":
    main()