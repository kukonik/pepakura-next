# ShowProjectStructure.ps1
# Предполагается запуск из корня проекта: D:\Dev\pepakura-next\

Write-Host "=== Структура проекта (ui-desktop) ===" -ForegroundColor Green
Set-Location -Path ".\ui-desktop"
try {
    tree /f
} catch {
    Write-Host "Команда 'tree' не найдена, используем Get-ChildItem" -ForegroundColor Yellow
    Get-ChildItem -Path . -Recurse -Depth 2 | Format-Tree
}
Set-Location -Path ".."

Write-Host "`n=== Структура проекта (core) ===" -ForegroundColor Green
Set-Location -Path ".\core"
try {
    tree /f
} catch {
    Write-Host "Команда 'tree' не найдена, используем Get-ChildItem" -ForegroundColor Yellow
    Get-ChildItem -Path . -Recurse -Depth 2 | Format-Tree
}
Set-Location -Path ".."

Write-Host "`n=== Содержимое файлов Tauri/Rust (ui-desktop/src-tauri/) ===" -ForegroundColor Green

$tauriFiles = @(
    ".\ui-desktop\src-tauri\Cargo.toml",
    ".\ui-desktop\src-tauri\tauri.conf.json",
    ".\ui-desktop\src-tauri\src\main.rs",
    ".\ui-desktop\src-tauri\src\lib.rs"
)

foreach ($file in $tauriFiles) {
    if (Test-Path $file) {
        Write-Host "`n--- $file ---" -ForegroundColor Cyan
        Get-Content $file
    } else {
        Write-Host "`n--- $file ---" -ForegroundColor Red
        Write-Host "Файл не найден." -ForegroundColor Red
    }
}

Write-Host "`n=== Содержимое файлов Core (core/) ===" -ForegroundColor Green

$coreFiles = @(
    ".\core\Cargo.toml",
    ".\core\src\lib.rs"
)

foreach ($file in $coreFiles) {
    if (Test-Path $file) {
        Write-Host "`n--- $file ---" -ForegroundColor Cyan
        Get-Content $file
    } else {
        Write-Host "`n--- $file ---" -ForegroundColor Red
        Write-Host "Файл не найден." -ForegroundColor Red
    }
}

Write-Host "`n=== Содержимое файлов Frontend (ui-desktop/src/) ===" -ForegroundColor Green

$frontendFiles = @(
    ".\ui-desktop\src\index.html",
    ".\ui-desktop\src\main.js",
    ".\ui-desktop\vite.config.js"
)

foreach ($file in $frontendFiles) {
    if (Test-Path $file) {
        Write-Host "`n--- $file ---" -ForegroundColor Cyan
        Get-Content $file
    } else {
        Write-Host "`n--- $file ---" -ForegroundColor Red
        Write-Host "Файл не найден." -ForegroundColor Red
    }
}