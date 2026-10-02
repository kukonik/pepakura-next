Write-Host "=== ПОЛНОЕ ИСПРАВЛЕНИЕ RUST ПРОЕКТА ==="

cd D:\Dev\pepakura-next\core\src

# 1. Очищаем старые файлы
Get-ChildItem -Recurse -Include *.rs | Remove-Item -Force 2>

# 2. Создаём структуру директорий
New-Item -ItemType Directory -Force -Path ".\model" | Out-Null
New-Item -ItemType Directory -Force -Path ".\unfold" | Out-Null
New-Item -ItemType Directory -Force -Path ".\export" | Out-Null
New-Item -ItemType Directory -Force -Path ".\util" | Out-Null

# 3. Создаём чистые файлы (код из предыдущих шагов)
# [Вставьте сюда код создания файлов из шагов 1-4 выше]

Write-Host "✅ Rust проект полностью пересоздан"
Write-Host "Теперь можно запускать Tauri..."
