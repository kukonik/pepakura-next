#!/usr/bin/env pwsh
# Pepakura Next - CI/CD Pre-check Script
# Проверяет проект перед отправкой в CI/CD

param(
    [switch]$Fix,
    [switch]$Help
)

$ErrorActionPreference = "Stop"
$RootPath = $PSScriptRoot
$ExitCode = 0

function Show-Help {
    Write-Host @"
Pepakura Next - CI/CD Pre-check

Использование: .\ci-check.ps1 [--Fix]

Опции:
  --Fix   - Автоматически исправлять ошибки
  --Help  - Показать эту справку

Проверки:
  ✓ Rust компиляция
  ✓ Rust тесты
  ✓ Rust lint (clippy)
  ✓ Rust формат
  ✓ TypeScript компиляция
  ✓ TypeScript тесты
  ✓ TypeScript lint
  ✓ E2E тесты (опционально)
"@
}

function Test-Step($Name, $ScriptBlock) {
    Write-Host "`n[CHECK] $Name" -ForegroundColor Cyan
    try {
        & $ScriptBlock
        Write-Host "  ✓ PASSED" -ForegroundColor Green
        return $true
    } catch {
        Write-Host "  ✗ FAILED: $_" -ForegroundColor Red
        return $false
    }
}

# Rust компиляция
function Invoke-RustBuild {
    Set-Location "$RootPath\crates\pepakura_core"
    cargo check 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        throw "Rust компиляция не удалась"
    }
}

# Rust тесты
function Invoke-RustTest {
    Set-Location "$RootPath\crates\pepakura_core"
    cargo test --lib 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        throw "Rust тесты не прошли"
    }
}

# Rust lint
function Invoke-RustLint {
    Set-Location "$RootPath\crates\pepakura_core"
    cargo clippy -- -D warnings 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        throw "Rust lint не прошёл"
    }
}

# Rust формат
function Invoke-RustFormat {
    Set-Location "$RootPath\crates\pepakura_core"
    cargo fmt --check 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        if ($Fix) {
            Write-Host "  Исправление формата..." -ForegroundColor Yellow
            cargo fmt
        } else {
            throw "Rust формат не совпадает. Запустите с --Fix"
        }
    }
}

# TypeScript компиляция
function Invoke-TsBuild {
    Set-Location "$RootPath\ui-desktop"
    pnpm run typecheck 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        throw "TypeScript компиляция не удалась"
    }
}

# TypeScript тесты
function Invoke-TsTest {
    Set-Location "$RootPath\ui-desktop"
    pnpm run test:unit 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        throw "TypeScript тесты не прошли"
    }
}

# TypeScript lint
function Invoke-TsLint {
    Set-Location "$RootPath\ui-desktop"
    pnpm run lint 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        if ($Fix) {
            Write-Host "  Исправление lint..." -ForegroundColor Yellow
            pnpm run lint:fix
        } else {
            throw "TypeScript lint не прошёл. Запустите с --Fix"
        }
    }
}

# E2E тесты (опционально)
function Invoke-E2ETest {
    Set-Location "$RootPath\ui-desktop"
    pnpm run test:e2e 2>&1 | Out-String
    if ($LASTEXITCODE -ne 0) {
        throw "E2E тесты не прошли"
    }
}

# Главная проверка
Write-Host "=== Pepakura Next CI/CD Pre-check ===" -ForegroundColor Cyan
Write-Host "Root: $RootPath" -ForegroundColor Gray
if ($Fix) {
    Write-Host "Mode: Auto-fix enabled" -ForegroundColor Yellow
}

# Rust проверки
Write-Host "`n=== Rust Checks ===" -ForegroundColor Cyan

if (!(Test-Step "Rust компиляция" { Invoke-RustBuild })) { $ExitCode++ }
if (!(Test-Step "Rust тесты" { Invoke-RustTest })) { $ExitCode++ }
if (!(Test-Step "Rust lint" { Invoke-RustLint })) { $ExitCode++ }
if (!(Test-Step "Rust формат" { Invoke-RustFormat })) { $ExitCode++ }

# TypeScript проверки
Write-Host "`n=== TypeScript Checks ===" -ForegroundColor Cyan

if (!(Test-Step "TypeScript компиляция" { Invoke-TsBuild })) { $ExitCode++ }
if (!(Test-Step "TypeScript тесты" { Invoke-TsTest })) { $ExitCode++ }
if (!(Test-Step "TypeScript lint" { Invoke-TsLint })) { $ExitCode++ }

# Итоги
Write-Host "`n=== Summary ===" -ForegroundColor Cyan

if ($ExitCode -eq 0) {
    Write-Host "✓ Все проверки пройдены!" -ForegroundColor Green
    Write-Host "Готово к CI/CD 🚀"
} else {
    Write-Host "✗ $ExitCode проверок не пройдено" -ForegroundColor Red
    Write-Host "Исправьте ошибки перед отправкой в CI/CD"
}

Set-Location $RootPath
exit $ExitCode
