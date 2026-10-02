import tempfile
import os


def convert_obj_string_to_stl(obj_str: str) -> bytes:
    """Преобразует строку .obj в .stl (временно сохраняет, потом удаляет)."""
    
    try:
        import trimesh
        import numpy as np

        # Сохраняем временный файл
        temp_dir = tempfile.mkdtemp()
        input_path = os.path.join(temp_dir, "temp.obj")
        output_path = os.path.join(temp_dir, "output.stl")

        with open(input_path, "w") as f:
            f.write(obj_str)

        mesh = trimesh.load(input_path)
        mesh.export(output_path, file_type='stl')

        with open(output_path, "rb") as f:
            stl_bytes = f.read()

        # Удаление временных файлов
        os.remove(input_path)
        os.remove(output_path)
        os.rmdir(temp_dir)

        return stl_bytes
    except ImportError:
        raise Exception("Библиотеки trimesh и numpy не установлены.")
    except Exception as e:
        raise Exception(f"Ошибка экспорта: {str(e)}")