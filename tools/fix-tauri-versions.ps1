$ErrorActionPreference = "Stop"
$projectRoot = "D:\Dev\pepakura-next"

Write-Host "`n[INFO] Синхронизация версий Tauri 2.x для устранения конфликта tauri-utils..." -ForegroundColor Cyan

# 1. Резервная копия текущего Cargo.toml
$cargoTomlPath = "$projectRoot\src-tauri\Cargo.toml"
$backupPath = "$cargoTomlPath.backup-$(Get-Date -Format 'yyyyMMdd-HHmmss')"
Copy-Item -Path $cargoTomlPath -Destination $backupPath -Force
Write-Host "[BACKUP] Создана резервная копия: $backupPath" -ForegroundColor Yellow

# 2. Совместимые версии Tauri 2.x (все плагины на 2.0.x линейке)
$tauriVersion = "2.0.4"
$pluginVersion = "2.0.1"

$cargoTomlContent = @"
[package]
name = "pepakura-next"
version = "0.1.0"
description = "Pepakura Next Desktop — smart PDO viewer/editor"
authors = ["you"]
edition = "2021"
rust-version = "1.70"

[build-dependencies]
tauri-build = { version = "$tauriVersion", features = [] }

[dependencies]
tauri = { version = "$tauriVersion", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pepakura_core = { path = "../crates/pepakura_core" }

[target.'cfg(not(any(target_os = "android", target_os = "ios")))'.dependencies]
tauri-plugin-fs = { version = "$pluginVersion", features = [] }
tauri-plugin-dialog = { version = "$pluginVersion", features = [] }
tauri-plugin-shell = { version = "$pluginVersion", features = [] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
"@

Write-Host "[WRITE] src-tauri\Cargo.toml (синхронизированные версии Tauri 2.x)" -ForegroundColor Green
Set-Content -Path $cargoTomlPath -Value $cargoTomlContent -Encoding UTF8

# 3. Удаление Cargo.lock для чистого разрешения зависимостей
$lockPath = "$projectRoot\src-tauri\Cargo.lock"
if (Test-Path $lockPath) {
    Remove-Item -Path $lockPath -Force
    Write-Host "[CLEAN] Удалён Cargo.lock для пересборки дерева зависимостей" -ForegroundColor Yellow
}

# 4. Валидация
Write-Host "`n[CHECK] Запуск cargo metadata для проверки разрешения зависимостей..." -ForegroundColor Cyan
Set-Location "$projectRoot\src-tauri"

$metadataResult = cargo metadata --format-version=1 --no-deps 2>&1
$hasConflict = $false
$metadataResult | ForEach-Object {
    if ($_ -match "error: failed to select a version for `tauri-utils`") {
        $hasConflict = $true
        Write-Host $_ -ForegroundColor Red
    } elseif ($_ -match "error") {
        Write-Host $_ -ForegroundColor Red
    } else {
        Write-Host $_ -ForegroundColor DarkGray
    }
}

if (-not $hasConflict -and $LASTEXITCODE -eq 0) {
    Write-Host "`n✅ УСПЕХ: Зависимости Tauri синхронизированы!" -ForegroundColor Green
    Write-Host "   • Все плагины на версии $pluginVersion" -ForegroundColor Green
    Write-Host "   • tauri = $tauriVersion" -ForegroundColor Green
    Write-Host "   • Теперь можно запускать: cargo tauri dev" -ForegroundColor Cyan
    
    # Дополнительная проверка компиляции ядра
    Write-Host "`n[CHECK] Проверка сборки pepakura_core..." -ForegroundColor Cyan
    Set-Location "$projectRoot\crates\pepakura_core"
    cargo check --quiet 2>&1 | Where-Object { $_ -match "error" } | ForEach-Object { Write-Host $_ -ForegroundColor Red }
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ pepakura_core остаётся рабочим" -ForegroundColor Green
    }
} else {
    Write-Host "`n❌ ОШИБКА: Конфликт зависимостей не устранён" -ForegroundColor Red
    Write-Host "   Восстановлена резервная копия Cargo.toml" -ForegroundColor Yellow
    Copy-Item -Path $backupPath -Destination $cargoTomlPath -Force
}

Write-Host "`n[INFO] Нажмите любую клавишу для закрытия окна..." -ForegroundColor Magenta
$Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") | Out-Null
Set-Location $projectRoot