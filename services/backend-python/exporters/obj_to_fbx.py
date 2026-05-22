import tempfile
import os


def convert_obj_string_to_fbx(obj_str: str) -> bytes:
    """Преобразует строку .obj в .fbx (временно сохраняет, потом удаляет)."""
    
    try:
        import trimesh
        import numpy as np

        # Сохраняем временный файл
        temp_dir = tempfile.mkdtemp()
        input_path = os.path.join(temp_dir, "temp.obj")
        output_path = os.path.join(temp_dir, "output.fbx")

        with open(input_path, "w") as f:
            f.write(obj_str)

        mesh = trimesh.load(input_path)
        mesh.export(output_path, file_type='fbx')

        with open(output_path, "rb") as f:
            fbx_bytes = f.read()

        # Удаление временных файлов
        os.remove(input_path)
        os.remove(output_path)
        os.rmdir(temp_dir)

        return fbx_bytes
    except ImportError:
        raise Exception("Библиотеки trimesh и numpy не установлены.")
    except Exception as e:
        raise Exception(f"Ошибка экспорта: {str(e)}")