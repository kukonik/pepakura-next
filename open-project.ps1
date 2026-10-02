# Скрипт для открытия проекта Pepakura Next в VS Code

# Проверяем, что мы в правильной директории
Write-Host "Текущая директория: $(Get-Location)"

# Открываем проект в VS Code
Write-Host "Открываем проект в VS Code..."
code .

# Запускаем Tauri dev сервер в новом терминале
Write-Host "Запускаем Tauri dev сервер..."
Start-Process -NoNewWindow -FilePath "npm" -ArgumentList "run", "tauri", "dev"

Write-Host "Проект открыт в VS Code. Tauri dev сервер запущен в отдельном терминале."