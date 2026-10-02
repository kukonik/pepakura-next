# Запуск dev-сервера для веб-приложения
# Использование: .\scripts\dev-ui-web.ps1

Set-Location -Path "$PSScriptRoot\.."
Write-Host "Запуск dev-сервера для веб-приложения..." -ForegroundColor Green

# Переход в директорию ui-web
Set-Location -Path "packages/ui-web"

# Запуск dev-сервера
pnpm dev