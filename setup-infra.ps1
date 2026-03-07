param()

Write-Host "=== SETUP INFRA: START ===" -ForegroundColor Cyan

$root = "D:\Dev\pepakura-next"

if (-not (Test-Path $root)) {
  Write-Host "Корневая папка не найдена: $root" -ForegroundColor Red
  exit 1
}

Set-Location $root

# 1. Исправляем shared/package.json
Write-Host "`n[1/5] Настройка shared/package.json..." -ForegroundColor Cyan

$sharedDir = Join-Path $root "shared"
if (-not (Test-Path $sharedDir)) {
  New-Item -ItemType Directory -Path $sharedDir | Out-Null
}

$sharedPackageJsonPath = Join-Path $sharedDir "package.json"

$sharedPackageJson = @{
  name    = "@pepakura/shared"
  version = "0.1.0"
  private = $true
  type    = "module"
  main    = "src/index.ts"
  files   = @("src")
}

$sharedJsonString = $sharedPackageJson | ConvertTo-Json -Depth 10
Set-Content -Path $sharedPackageJsonPath -Value $sharedJsonString -Encoding UTF8

Write-Host "Создан/обновлён shared/package.json:" -ForegroundColor Green
Get-Content $sharedPackageJsonPath | Write-Host

# 2. Создаём конфиги eslint и vitest
Write-Host "`n[2/5] Создание конфигурационных файлов ESLint и Vitest..." -ForegroundColor Cyan

# .eslintrc.json
$eslintConfig = @{
  root  = $true
  env   = @{
    browser = $true
    es2021  = $true
  }
  extends = @(
    "eslint:recommended"
    "@typescript-eslint/recommended"
  )
  parser  = "@typescript-eslint/parser"
  plugins = @("@typescript-eslint")
  rules   = @{
    indent = @("error", 2)
    quotes = @("error", "single")
    semi   = @("error", "always")
  }
}

$eslintPath = Join-Path $root ".eslintrc.json"
$eslintJson = $eslintConfig | ConvertTo-Json -Depth 10
Set-Content -Path $eslintPath -Value $eslintJson -Encoding UTF8

# vitest.config.ts
$vitestConfigTs = @"
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    environment: 'jsdom',
    setupFiles: ['./tests/setup.ts'],
    globals: true,
  },
});
"@

$vitestConfigPath = Join-Path $root "vitest.config.ts"
Set-Content -Path $vitestConfigPath -Value $vitestConfigTs -Encoding UTF8

# tests/setup.ts
$testsDir = Join-Path $root "tests"
if (-not (Test-Path $testsDir)) {
  New-Item -ItemType Directory -Path $testsDir | Out-Null
}

$vitestSetup = @"
// Setup file for Vitest
import { JSDOM } from 'jsdom';

const dom = new JSDOM('<!DOCTYPE html><html><body></body></html>');
// @ts-ignore
global.window = dom.window;
// @ts-ignore
global.document = dom.window.document;
// @ts-ignore
global.navigator = dom.window.navigator;
"@

$vitestSetupPath = Join-Path $testsDir "setup.ts"
Set-Content -Path $vitestSetupPath -Value $vitestSetup -Encoding UTF8

# tests/sample.test.ts
$sampleTest = @"
import { describe, it, expect } from 'vitest';

describe('Sample Test', () => {
  it('should pass', () => {
    expect(1).toBe(1);
  });
});
"@

$sampleTestPath = Join-Path $testsDir "sample.test.ts"
Set-Content -Path $sampleTestPath -Value $sampleTest -Encoding UTF8

Write-Host "Созданы файлы:" -ForegroundColor Green
Write-Host "  - .eslintrc.json"
Write-Host "  - vitest.config.ts"
Write-Host "  - tests/setup.ts"
Write-Host "  - tests/sample.test.ts"

# 3. Обновляем scripts в корневом package.json, НЕ затирая существующие
Write-Host "`n[3/5] Обновление scripts в корневом package.json..." -ForegroundColor Cyan

$rootPackageJsonPath = Join-Path $root "package.json"
if (-not (Test-Path $rootPackageJsonPath)) {
  Write-Host "Файл package.json в корне не найден: $rootPackageJsonPath" -ForegroundColor Red
  exit 1
}

$rootPkgObj = Get-Content $rootPackageJsonPath -Raw | ConvertFrom-Json

if (-not $rootPkgObj.scripts) {
  $rootPkgObj | Add-Member -Name "scripts" -MemberType NoteProperty -Value (@{}) -Force
}

# Обновляем/добавляем конкретные скрипты
$rootPkgObj.scripts."dev:desktop" = "pnpm --filter ui-desktop tauri dev"
$rootPkgObj.scripts."dev:web"     = "pnpm --filter ui-web dev"
$rootPkgObj.scripts."dev:all"     = "concurrently `"pnpm dev:desktop`" `"pnpm dev:web`""
$rootPkgObj.scripts."lint"        = "eslint . --ext .ts,.vue"
$rootPkgObj.scripts."test"        = "vitest"
$rootPkgObj.scripts."test:ui"     = "vitest --ui"

$rootPkgJsonOut = $rootPkgObj | ConvertTo-Json -Depth 20
Set-Content -Path $rootPackageJsonPath -Value $rootPkgJsonOut -Encoding UTF8

Write-Host "Актуальные скрипты в корневом package.json:" -ForegroundColor Green
($rootPkgObj.scripts | Get-Member -MemberType NoteProperty).Name | ForEach-Object {
  Write-Host "  - $_"
}

# 4. Быстрая проверка: pnpm install, pnpm lint, pnpm test
Write-Host "`n[4/5] Базовая проверка команд..." -ForegroundColor Cyan

Write-Host "Проверка pnpm install..." -ForegroundColor Yellow
try {
    pnpm install
    Write-Host "pnpm install завершился с кодом $LASTEXITCODE" -ForegroundColor Green
} catch {
    Write-Host "Ошибка при pnpm install: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host "Проверка pnpm lint..." -ForegroundColor Yellow
try {
    pnpm lint
    Write-Host "pnpm lint завершился с кодом $LASTEXITCODE" -ForegroundColor Green
} catch {
    Write-Host "pnpm lint завершился с кодом $LASTEXITCODE (это нормально если есть ошибки в коде)" -ForegroundColor Yellow
}

Write-Host "Проверка pnpm test..." -ForegroundColor Yellow
try {
    pnpm test
    Write-Host "pnpm test завершился с кодом $LASTEXITCODE" -ForegroundColor Green
} catch {
    Write-Host "pnpm test завершился с кодом $LASTEXITCODE (это нормально для новых тестов)" -ForegroundColor Yellow
}

Write-Host "`n=== SETUP INFRA: DONE ===" -ForegroundColor Cyan
Write-Host "Инфраструктура подготовлена. Если есть ошибки в lint/test - пришли их отдельно."
