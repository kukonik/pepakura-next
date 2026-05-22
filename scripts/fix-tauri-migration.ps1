# PowerShell скрипт для автоматического применения всех исправлений
# Запуск из корня проекта: .\scripts\fix-tauri-migration.ps1

Write-Host "Начинаем исправление миграции Tauri..." -ForegroundColor Green

# Шаг 1: Изолировать ядро от Tauri
Write-Host "Шаг 1: Изолируем ядро от Tauri..." -ForegroundColor Yellow
Set-Content -Path "crates/pepakura_core/Cargo.toml" -Value @'
[package]
name = "pepakura_core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
byteorder = "1.4"
flate2 = "1.0"
thiserror = "1.0"
'@

# Шаг 2: Синхронизировать версии Tauri 2.x в основном приложении
Write-Host "Шаг 2: Синхронизируем версии Tauri 2.x..." -ForegroundColor Yellow
Set-Content -Path "src-tauri/Cargo.toml" -Value @'
[package]
name = "pepakura-next"
version = "0.1.0"
description = "Pepakura Next Desktop — smart PDO viewer/editor"
authors = ["you"]
edition = "2021"
rust-version = "1.70"

[build-dependencies]
tauri-build = { version = "2.0.1", features = [] }

[dependencies]
tauri = { version = "2.0.1", features = [] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pepakura_core = { path = "../crates/pepakura_core" }

[target.'cfg(not(any(target_os = "android", target_os = "ios")))'.dependencies]
tauri-plugin-fs = { version = "2.0.1", features = [] }
tauri-plugin-dialog = { version = "2.0.1", features = [] }
tauri-plugin-shell = { version = "2.0.1", features = [] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
'@

# Шаг 3: Обновить фронтенд до @tauri-apps/api v2
Write-Host "Шаг 3: Обновляем фронтенд до @tauri-apps/api v2..." -ForegroundColor Yellow
Set-Content -Path "packages/ui-desktop/package.json" -Value @'
{
  "private": true,
  "devDependencies": {
    "@tauri-apps/cli": "^1.6.1",
    "@types/node": "^20.19.30",
    "@vitejs/plugin-vue": "^5.2.4",
    "typescript": "^5.7.2",
    "vite": "^6.4.1"
  },
  "dependencies": {
    "@tauri-apps/api": "^2.0.3",
    "@tauri-apps/plugin-shell": "^2.3.4",
    "@types/three": "^0.182.0",
    "pinia": "^2.2.4",
    "three": "^0.160.1",
    "vue": "^3.4.4",
    "vue-router": "^4.6.4"
  },
  "version": "0.1.0",
  "type": "module",
  "name": "@pepakura/ui-desktop",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  }
}
'@

# Шаг 4: Корректный конфиг Tauri v2
Write-Host "Шаг 4: Обновляем конфиг Tauri v2..." -ForegroundColor Yellow
Set-Content -Path "src-tauri/tauri.conf.json" -Value @'
{
  "identifier": "com.pepakura.next",
  "productName": "Pepakura Next",
  "version": "0.1.0",
  "build": {
    "beforeDevCommand": "cd packages/ui-desktop && pnpm dev",
    "beforeBuildCommand": "cd packages/ui-desktop && pnpm build",
    "frontendDist": "packages/ui-desktop/dist",
    "devUrl": "http://localhost:5173"
  },
  "app": {
    "windows": [{
      "label": "main",
      "title": "Pepakura Next",
      "width": 1280,
      "height": 720
    }],
    "security": {
      "csp": null
    }
  }
}
'@

# Шаг 5: Валидация конвейера данных
Write-Host "Шаг 5: Обновляем команды Tauri..." -ForegroundColor Yellow
Set-Content -Path "src-tauri/src/commands.rs" -Value @'
use tauri::command;
use pepakura_core::{parse_pdo_to_pepa_core, ParsePdoResult};

#[command]
pub fn parse_pdo_to_pepa(data: Vec<u8>) -> ParsePdoResult {
    parse_pdo_to_pepa_core(&data)
}
'@

Write-Host "Все исправления применены успешно!" -ForegroundColor Green
Write-Host "Теперь можно запускать: cargo tauri dev" -ForegroundColor Cyan