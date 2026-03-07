import trimesh
import numpy as np
import io
import xml.etree.ElementTree as ET
from typing import Optional


def unfold_mesh_to_svg(obj_text: str) -> str:
    """
    Naive unfold: проецируем каждую грань на XY и рисуем как многоугольники.
    Только для демонстрации.
    """
    try:
        # Читаем OBJ из строки
        mesh = trimesh.load(io.StringIO(obj_text), file_type='obj')

        # Пространство для SVG
        min_coord = np.min(mesh.vertices[:, :2], axis=0)
        max_coord = np.max(mesh.vertices[:, :2], axis=0)
        width_mm = (max_coord[0] - min_coord[0]) * 10
        height_mm = (max_coord[1] - min_coord[1]) * 10

        svg_ns = {
            'xmlns': "http://www.w3.org/2000/svg",
            'version': "1.1"
        }

        root = ET.Element('svg', attrib={
            'width': f'{width_mm}mm',
            'height': f'{height_mm}mm',
            'viewBox': f'0 0 {width_mm} {height_mm}',
            **svg_ns
        })

        group = ET.SubElement(root, 'g', attrib={'fill': 'none', 'stroke': '#000'})

        # Базовая проекция всех треугольников
        for triangle in mesh.triangles:
            projected_triangle = [(pt[0], pt[1]) for pt in triangle]
            points_str = ' '.join([f'{x},{y}' for x, y in projected_triangle])

            polygon = ET.SubElement(group, 'polygon', {
                'points': points_str,
                'stroke-width': '0.1'
            })

        return ET.tostring(root, encoding='unicode')
    except Exception as e:
        raise Exception(f'Ошибка развёртки: {str(e)}')
