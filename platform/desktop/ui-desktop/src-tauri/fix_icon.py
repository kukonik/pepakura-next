from PIL import Image
import os

# Создаём простое изображение 256x256 с синим квадратом
img = Image.new('RGBA', (256, 256), (0, 120, 215, 255))

# Сохраняем как ICO
ico_path = os.path.join('icons', 'icon.ico')
img.save(ico_path, format='ICO', sizes=[(256, 256), (128, 128), (64, 64), (32, 32), (16, 16)])

print(f"Generated {ico_path}")