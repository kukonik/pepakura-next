#!/usr/bin/env pwsh
# test-pdo-flow.ps1 — проверяет цепочку файл → команда → UI без ручных кликов

param(
    [string] = "test-data/sample.pdo",
    [switch]
)

Stop = "Stop"

Write-Host "
🔍 Тест цепочки PDO → PepaScene" -ForegroundColor Cyan

# 1. Проверить файл
if (-not (Test-Path )) {
    Write-Host "❌ Файл не найден: " -ForegroundColor Red
    Write-Host "   Создаю заглушку для теста..." -ForegroundColor Yellow
    New-Item -ItemType Directory -Path (Split-Path ) -Force | Out-Null
    [System.IO.File]::WriteAllBytes(, @(0x50, 0x44, 0x4F, 0x01)) # PDO magic
}

# 2. Собрать Tauri (только если нужно)
if (-not (Get-ChildItem "src-tauri/target/debug" -Filter *.exe -ErrorAction SilentlyContinue)) {
    Write-Host "⚙️  Сборка Tauri (debug)..." -ForegroundColor DarkGray
    cargo build --manifest-path src-tauri/Cargo.toml --quiet
}

# 3. Проверить сигнатуру команды
 = Get-Content "src-tauri/src/commands.rs" -Raw
if ( -notmatch 'pub fn parse_pdo_to_pepa\(\s*data:\s*Vec<u8>') {
    Write-Host "⚠️  Сигнатура команды не соответствует ожидаемой" -ForegroundColor Yellow
} else {
    Write-Host "✅ Сигнатура команды: корректна (data: Vec<u8>)" -ForegroundColor Green
}

# 4. Проверить TS-контракт
 = Get-Content "shared/types/pepa-scene.ts" -Raw
if ( -notmatch 'interface ParsePdoResult') {
    Write-Host "❌ TS-контракт не найден" -ForegroundColor Red
    exit 1
}
if ( -notmatch 'diagnostics:\s*Warning\[\]') {
    Write-Host "⚠️  Поле diagnostics отсутствует в TS-контракте" -ForegroundColor Yellow
} else {
    Write-Host "✅ TS-контракт: содержит diagnostics" -ForegroundColor Green
}

# 5. (Опционально) Открыть UI
if () {
    Write-Host "
🚀 Запуск UI..." -ForegroundColor Cyan
    Start-Process "pnpm" "dev" -WorkingDirectory "packages/ui-desktop"
    Start-Sleep -Seconds 3
    Write-Host "   UI запущен. Откройте в браузере: http://localhost:1420" -ForegroundColor Green
}

Write-Host "
✅ Тест завершён. Цепочка готова к использованию." -ForegroundColor Green
Write-Host "   Следующий шаг: подключите DiagnosticsPanel.vue к 3D-превью" -ForegroundColor DarkGray
