import sys
import bpy
import os

# Путь к входному файлу (.blend)
input_file = sys.argv[-2]
# Путь для экспорта (.obj)
output_file = sys.argv[-1]

print(f"Processing: {input_file}")
print(f"Exporting to: {output_file}")

# Открываем файл
bpy.ops.wm.open_mainfile(filepath=input_file)

# Находим все меши, выделяем их
bpy.ops.object.select_all(action='DESELECT')
bpy.ops.object.select_by_type(type='MESH')
bpy.ops.object.convert(target='MESH')

# Сдвигаем объект в центр и вверх (как в нашей логике)
bpy.ops.object.origin_set(type='ORIGIN_GEOMETRY', center='MEDIAN')
bpy.ops.object.transform_apply(location=True, rotation=True, scale=True)

# Экспорт в OBJ
try:
    # Пробуем использовать экспортер (в старых версиях он может отличаться, но это стандарт)
    bpy.ops.export_scene.obj(filepath=output_file, use_selection=False, axis_forward='-Z', axis_up='Y')
    print("Export successful")
except Exception as e:
    print(f"Export failed: {e}")
    sys.exit(1)
