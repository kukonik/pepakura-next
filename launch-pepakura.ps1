# Script for launching Pepakura Next project
# This script will:
# 1. Check if required tools are installed
# 2. Start the Tauri development server
# 3. Open the project in VS Code

# Проверка наличия необходимых инструментов
function Test-CommandExists {
    param ([string]$command)
    $exists = $null -ne (Get-Command $command -ErrorAction SilentlyContinue)
    return $exists
}

# Проверка наличия Node.js
if (-not (Test-CommandExists "node")) {
    Write-Host "Node.js не найден. Пожалуйста, установите Node.js с https://nodejs.org/" -ForegroundColor Red
    exit 1
}

# Проверка наличия npm
if (-not (Test-CommandExists "npm")) {
    Write-Host "npm не найден. Пожалуйста, установите Node.js с https://nodejs.org/" -ForegroundColor Red
    exit 1
}

# Проверка наличия Rust
if (-not (Test-CommandExists "rustc")) {
    Write-Host "Rust не найден. Пожалуйста, установите Rust с https://www.rust-lang.org/" -ForegroundColor Red
    exit 1
}

# Проверка наличия Tauri CLI
if (-not (Test-CommandExists "tauri")) {
    Write-Host "Tauri CLI не найден. Устанавливаем..." -ForegroundColor Yellow
    npm install -g @tauri-apps/cli
}

# Проверка наличия VS Code
$codeExists = Test-CommandExists "code"
if (-not $codeExists) {
    Write-Host "VS Code не найден. Пожалуйста, установите VS Code с https://code.visualstudio.com/" -ForegroundColor Red
    $codeExists = $false
}

# Установка зависимостей
Write-Host "Установка зависимостей..." -ForegroundColor Green
npm install

# Открытие проекта в VS Code (если доступно)
if ($codeExists) {
    Write-Host "Открытие проекта в VS Code..." -ForegroundColor Green
    code .
}

# Запуск Tauri dev сервера
Write-Host "Запуск Tauri dev сервера..." -ForegroundColor Green
Write-Host "Приложение будет доступно по адресу: http://localhost:1420" -ForegroundColor Cyan
npm run tauri dev