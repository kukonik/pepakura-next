#!/usr/bin/env python3
"""
Тестовый скрипт для проверки работы развёртки
"""

import sys
import os
import tempfile
import numpy as np
import trimesh

# Добавляем путь к addon_server.py
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from addon_server import unfold_mesh_to_svg


def create_test_cube_obj(filepath):
    """Создаёт тестовый OBJ файл с кубом"""
    cube_obj = """# Cube
v 0 0 0
v 1 0 0
v 1 1 0
v 0 1 0
v 0 0 1
v 1 0 1
v 1 1 1
v 0 1 1
f 1 2 3 4
f 5 6 7 8
f 1 2 6 5
f 2 3 7 6
f 3 4 8 7
f 4 1 5 8
"""
    
    with open(filepath, 'w') as f:
        f.write(cube_obj)


def main():
    """Основная функция для тестирования"""
    print("Тестирование развёртки 3D модели...")
    
    # Создаём временные файлы
    with tempfile.TemporaryDirectory() as temp_dir:
        input_obj = os.path.join(temp_dir, "test_cube.obj")
        output_svg = os.path.join(temp_dir, "unfolded_cube.svg")
        
        # Создаём тестовую модель
        create_test_cube_obj(input_obj)
        print(f"Создана тестовая модель: {input_obj}")
        
        # Выполняем развёртку
        result = unfold_mesh_to_svg(input_obj, output_svg)
        print(f"Результат: {result}")
        
        # Проверяем результат
        if result["status"] == "success":
            print(f"SVG файл создан: {output_svg}")
            
            # Читаем и выводим содержимое SVG
            with open(output_svg, 'r', encoding='utf-8') as f:
                svg_content = f.read()
                print("Содержимое SVG файла:")
                print(svg_content[:500] + "..." if len(svg_content) > 500 else svg_content)
        else:
            print(f"Ошибка: {result['message']}")


if __name__ == "__main__":
    main()