# Запуск проекта Pepakura Next
param()

Write-Host "=== ЗАПУСК PEPAKURA NEXT ===" -ForegroundColor Cyan

# Проверка директории
$projectRoot = "D:\Dev\pepakura-next"
$uiDesktopPath = Join-Path $projectRoot "packages\ui-desktop"

if (-not (Test-Path $uiDesktopPath)) {
    Write-Host "Ошибка: директория проекта не найдена!" -ForegroundColor Red
    exit 1
}

Set-Location $uiDesktopPath

Write-Host "1. Проверка зависимостей..." -ForegroundColor Yellow
try {
    # Проверим, установлены ли зависимости
    if (-not (Test-Path "node_modules")) {
        Write-Host "   Установка зависимостей..." -ForegroundColor Yellow
        pnpm install
    } else {
        Write-Host "   Зависимости уже установлены" -ForegroundColor Green
    }
} catch {
    Write-Host "   Ошибка установки зависимостей: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "`n2. Доступные команды:" -ForegroundColor Cyan
Write-Host "   pnpm dev      - запуск веб-сервера" -ForegroundColor Green
Write-Host "   pnpm tauri dev - запуск десктопного приложения" -ForegroundColor Green
Write-Host "   pnpm install  - установка зависимостей" -ForegroundColor Green

Write-Host "`n3. Запуск веб-сервера на порту 3000..." -ForegroundColor Yellow
Write-Host "   Если возникают ошибки с портами, закройте все запущенные процессы" -ForegroundColor Yellow

# Запуск веб-сервера
pnpm dev
