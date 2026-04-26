#!/usr/bin/env pwsh
# Pepakura Next - Dev Helper Script
# Автоматизация рутинных задач

param(
    [Parameter(Position=0)]
    [string]$Command = "help",
    
    [switch]$Help
)

$ErrorActionPreference = "Stop"
$RootPath = $PSScriptRoot

function Show-Help {
    Write-Host @"
Pepakura Next - Dev Helper

Использование: .\dev.ps1 <command>

Команды:
  setup       - Первичная настройка окружения
  dev         - Запуск разработки (frontend + Tauri)
  test        - Запуск всех тестов
  test:rust   - Rust тесты
  test:ts     - TypeScript тесты
  test:e2e    - E2E тесты
  build       - Сборка релиза
  clean       - Очистка артефактов
  lint        - Проверка кода
  format      - Форматирование кода
  coverage    - Запуск тестов с покрытием
  ai:check    - Проверка AI (Ollama)
  ai:start    - Запуск Ollama сервиса
  help        - Показать эту справку

Примеры:
  .\dev.ps1 setup
  .\dev.ps1 dev
  .\dev.ps1 test
"@
}

function Test-Command($Name) {
    return Get-Command $Name -ErrorAction SilentlyContinue -OutVariable null
}

function Invoke-Setup {
    Write-Host "=== Pepakura Next Setup ===" -ForegroundColor Cyan
    
    # Check Rust
    Write-Host "`n[1/5] Checking Rust..." -ForegroundColor Yellow
    if (Test-Command cargo) {
        Write-Host "  ✓ Rust: $(cargo --version)" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Rust not found. Install from https://rustup.rs/" -ForegroundColor Red
        return
    }
    
    # Check Node.js
    Write-Host "`n[2/5] Checking Node.js..." -ForegroundColor Yellow
    if (Test-Command node) {
        Write-Host "  ✓ Node.js: $(node --version)" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Node.js not found. Install from https://nodejs.org/" -ForegroundColor Red
        return
    }
    
    # Check pnpm
    Write-Host "`n[3/5] Checking pnpm..." -ForegroundColor Yellow
    if (Test-Command pnpm) {
        Write-Host "  ✓ pnpm: $(pnpm --version)" -ForegroundColor Green
    } else {
        Write-Host "  ✗ pnpm not found. Run: npm install -g pnpm" -ForegroundColor Red
        return
    }
    
    # Install dependencies
    Write-Host "`n[4/5] Installing dependencies..." -ForegroundColor Yellow
    Set-Location "$RootPath\ui-desktop"
    pnpm install --frozen-lockfile
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ✗ Failed to install dependencies" -ForegroundColor Red
        return
    }
    Write-Host "  ✓ Dependencies installed" -ForegroundColor Green
    
    # Build Rust core
    Write-Host "`n[5/5] Building Rust core..." -ForegroundColor Yellow
    Set-Location "$RootPath\crates\pepakura_core"
    cargo build
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ✗ Failed to build Rust core" -ForegroundColor Red
        return
    }
    Write-Host "  ✓ Rust core built" -ForegroundColor Green
    
    Write-Host "`n=== Setup Complete ===" -ForegroundColor Green
    Write-Host "Run '.\dev.ps1 dev' to start development"
}

function Invoke-Dev {
    Write-Host "=== Starting Development ===" -ForegroundColor Cyan
    
    # Start frontend in background
    Write-Host "`n[1/2] Starting frontend..." -ForegroundColor Yellow
    Start-Process powershell -ArgumentList "-NoExit", "-Command", "cd '$RootPath\ui-desktop'; pnpm dev"
    
    # Start Tauri
    Write-Host "[2/2] Starting Tauri..." -ForegroundColor Yellow
    Set-Location "$RootPath\src-tauri"
    cargo tauri dev
}

function Invoke-Test {
    Write-Host "=== Running All Tests ===" -ForegroundColor Cyan
    
    # Rust tests
    Write-Host "`n[1/3] Rust tests..." -ForegroundColor Yellow
    Set-Location "$RootPath\crates\pepakura_core"
    cargo test --lib
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ✗ Rust tests failed" -ForegroundColor Red
    } else {
        Write-Host "  ✓ Rust tests passed" -ForegroundColor Green
    }
    
    # TypeScript tests
    Write-Host "`n[2/3] TypeScript tests..." -ForegroundColor Yellow
    Set-Location "$RootPath\ui-desktop"
    pnpm test:unit
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ✗ TypeScript tests failed" -ForegroundColor Red
    } else {
        Write-Host "  ✓ TypeScript tests passed" -ForegroundColor Green
    }
    
    # E2E tests
    Write-Host "`n[3/3] E2E tests..." -ForegroundColor Yellow
    pnpm test:e2e
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ✗ E2E tests failed" -ForegroundColor Red
    } else {
        Write-Host "  ✓ E2E tests passed" -ForegroundColor Green
    }
    
    Write-Host "`n=== Tests Complete ===" -ForegroundColor Cyan
}

function Invoke-TestRust {
    Write-Host "=== Running Rust Tests ===" -ForegroundColor Cyan
    Set-Location "$RootPath\crates\pepakura_core"
    cargo test --lib
}

function Invoke-TestTs {
    Write-Host "=== Running TypeScript Tests ===" -ForegroundColor Cyan
    Set-Location "$RootPath\ui-desktop"
    pnpm test:unit
}

function Invoke-TestE2E {
    Write-Host "=== Running E2E Tests ===" -ForegroundColor Cyan
    Set-Location "$RootPath\ui-desktop"
    pnpm test:e2e
}

function Invoke-Build {
    Write-Host "=== Building Release ===" -ForegroundColor Cyan
    Set-Location "$RootPath\src-tauri"
    cargo tauri build
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n=== Build Complete ===" -ForegroundColor Green
        Write-Host "Installers location:"
        Write-Host "  - Windows: target\release\bundle\msi\"
        Write-Host "  - Linux: target\release\bundle\deb\"
        Write-Host "  - macOS: target\release\bundle\dmg\"
    } else {
        Write-Host "`n=== Build Failed ===" -ForegroundColor Red
    }
}

function Invoke-Clean {
    Write-Host "=== Cleaning ===" -ForegroundColor Cyan
    
    # Clean Rust
    Write-Host "`n[1/3] Cleaning Rust..." -ForegroundColor Yellow
    Set-Location "$RootPath\crates\pepakura_core"
    cargo clean
    
    # Clean Tauri
    Write-Host "[2/3] Cleaning Tauri..." -ForegroundColor Yellow
    Set-Location "$RootPath\src-tauri"
    cargo clean
    
    # Clean frontend
    Write-Host "[3/3] Cleaning frontend..." -ForegroundColor Yellow
    Set-Location "$RootPath\ui-desktop"
    Remove-Item -Recurse -Force node_modules -ErrorAction SilentlyContinue
    Remove-Item -Recurse -Force dist -ErrorAction SilentlyContinue
    
    Write-Host "`n=== Clean Complete ===" -ForegroundColor Green
}

function Invoke-Lint {
    Write-Host "=== Linting ===" -ForegroundColor Cyan
    
    # Rust lint
    Write-Host "`n[1/2] Rust lint..." -ForegroundColor Yellow
    Set-Location "$RootPath\crates\pepakura_core"
    cargo clippy -- -D warnings
    
    # TypeScript lint
    Write-Host "[2/2] TypeScript lint..." -ForegroundColor Yellow
    Set-Location "$RootPath\ui-desktop"
    pnpm lint
    
    Write-Host "`n=== Lint Complete ===" -ForegroundColor Cyan
}

function Invoke-Format {
    Write-Host "=== Formatting ===" -ForegroundColor Cyan
    
    # Rust format
    Write-Host "`n[1/2] Rust format..." -ForegroundColor Yellow
    Set-Location "$RootPath\crates\pepakura_core"
    cargo fmt
    
    # TypeScript format
    Write-Host "[2/2] TypeScript format..." -ForegroundColor Yellow
    Set-Location "$RootPath\ui-desktop"
    pnpm format
    
    Write-Host "`n=== Format Complete ===" -ForegroundColor Cyan
}

function Invoke-Coverage {
    Write-Host "=== Running Coverage ===" -ForegroundColor Cyan
    
    # Rust coverage
    Write-Host "`n[1/2] Rust coverage..." -ForegroundColor Yellow
    Set-Location "$RootPath\crates\pepakura_core"
    
    if (-not (Test-Command cargo-tarpaulin)) {
        Write-Host "  Installing cargo-tarpaulin..."
        cargo install cargo-tarpaulin
    }
    
    cargo tarpaulin --all-features --out Html
    Write-Host "  Report: target/tarpaulin-report.html" -ForegroundColor Green
    
    # TypeScript coverage
    Write-Host "`n[2/2] TypeScript coverage..." -ForegroundColor Yellow
    Set-Location "$RootPath\ui-desktop"
    pnpm test:unit --coverage
    Write-Host "  Report: coverage/index.html" -ForegroundColor Green
    
    Write-Host "`n=== Coverage Complete ===" -ForegroundColor Cyan
}

function Invoke-AiCheck {
    Write-Host "=== Checking AI (Ollama) ===" -ForegroundColor Cyan
    
    $response = Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -UseBasicParsing -ErrorAction SilentlyContinue
    
    if ($response.StatusCode -eq 200) {
        Write-Host "  ✓ Ollama is running" -ForegroundColor Green
        $models = $response.Content | ConvertFrom-Json
        Write-Host "  Models:"
        foreach ($model in $models.models) {
            Write-Host "    - $($model.name)" -ForegroundColor Green
        }
    } else {
        Write-Host "  ✗ Ollama is not running" -ForegroundColor Red
        Write-Host "  Run 'ollama serve' or '.\dev.ps1 ai:start'"
    }
}

function Invoke-AiStart {
    Write-Host "=== Starting Ollama ===" -ForegroundColor Cyan
    
    if (Test-Command ollama) {
        Start-Process ollama -ArgumentList "serve"
        Write-Host "  ✓ Ollama started" -ForegroundColor Green
    } else {
        Write-Host "  ✗ Ollama not found. Install from https://ollama.ai/" -ForegroundColor Red
    }
}

# Main command dispatcher
switch ($Command.ToLower()) {
    "setup" { Invoke-Setup }
    "dev" { Invoke-Dev }
    "test" { Invoke-Test }
    "test:rust" { Invoke-TestRust }
    "test:ts" { Invoke-TestTs }
    "test:e2e" { Invoke-TestE2E }
    "build" { Invoke-Build }
    "clean" { Invoke-Clean }
    "lint" { Invoke-Lint }
    "format" { Invoke-Format }
    "coverage" { Invoke-Coverage }
    "ai:check" { Invoke-AiCheck }
    "ai:start" { Invoke-AiStart }
    "help" { Show-Help }
    default { 
        if ($Help) {
            Show-Help
        } else {
            Write-Host "Unknown command: $Command" -ForegroundColor Red
            Write-Host "Run '.\dev.ps1 help' for usage" -ForegroundColor Yellow
        }
    }
}
