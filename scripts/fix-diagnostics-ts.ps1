#!/usr/bin/env pwsh
# scripts/fix-diagnostics-ts.ps1
# Добавляет Warning и diagnostics в packages/ui-desktop/src/types/pepakura.ts

$ErrorActionPreference = "Stop"

if ($PSScriptRoot) {
    $ProjectRoot = Split-Path $PSScriptRoot -Parent
} else {
    $ProjectRoot = Get-Location
}

$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
Write-Host "`n[INFO] fix-diagnostics-ts: проект $ProjectRoot`n" -ForegroundColor Cyan

function Backup-File {
    param([string]$Path)
    if (-not (Test-Path $Path)) { return }
    $backup = "$Path.$timestamp.bak"
    Copy-Item $Path $backup -Force
    Write-Host "  [BK] $backup" -ForegroundColor DarkGray
}

$tsPath = Join-Path $ProjectRoot "packages/ui-desktop/src/types/pepakura.ts"

if (-not (Test-Path $tsPath)) {
    Write-Host "[ERR] Файл не найден: $tsPath" -ForegroundColor Red
    exit 1
}

Write-Host "[TS] Патчим $tsPath" -ForegroundColor White
Backup-File $tsPath

$ts = Get-Content $tsPath -Raw -Encoding UTF8

# 1) Добавить interface Warning, если его нет
if ($ts -notmatch "export interface Warning") {
$tsWarning = @"
export interface Warning {
  code: string
  message: string
  partId?: number
  severity: 'info' | 'warning' | 'error'
}

"@
    $ts = $tsWarning + $ts
    Write-Host "  [+] Добавлен interface Warning" -ForegroundColor Green
} else {
    Write-Host "  [=] interface Warning уже существует" -ForegroundColor DarkGray
}

# 2) Добавить diagnostics в ParsePdoResult после stats
if ($ts -notmatch "diagnostics:\s*Warning\[]") {
    $pattern = "stats:[^\r\n]*"
    if ($ts -match $pattern) {
        $match = $Matches[0]
        $replacement = "$match`n  diagnostics: Warning[]"
        $ts = $ts -replace $pattern, $replacement
        Write-Host "  [+] Добавлено поле diagnostics в ParsePdoResult" -ForegroundColor Green
    } else {
        Write-Host "  [WARN] Не найдено поле stats в ParsePdoResult" -ForegroundColor Yellow
    }
} else {
    Write-Host "  [=] diagnostics уже есть в ParsePdoResult" -ForegroundColor DarkGray
}

Set-Content $tsPath -Value $ts -Encoding UTF8

Write-Host "`n[OK] Готово. Дальше:" -ForegroundColor Green
Write-Host "  pnpm lint (или vue-tsc) для проверки типов`n"
