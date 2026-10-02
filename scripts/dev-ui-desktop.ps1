# Запуск Tauri dev для десктоп-приложения
# Использование: .\scripts\dev-ui-desktop.ps1

Set-Location -Path "$PSScriptRoot\.."
Write-Host "Запуск Tauri dev для десктоп-приложения..." -ForegroundColor Green

# Переход в директорию ui-desktop
Set-Location -Path "packages/ui-desktop"

# Запуск Tauri dev
pnpm tauri dev