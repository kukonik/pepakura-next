param(
    [string]$Root = "D:\Dev\pepakura-next"
)

$ErrorActionPreference = "Stop"

Write-Host "=== Pepakura Next: auto-fix + dev ===" -ForegroundColor Cyan
Write-Host "Root: $Root"
Write-Host ""

$ui = Join-Path $Root "ui-desktop"
if (-not (Test-Path $ui)) {
    Write-Host "[FAIL] ui-desktop not found" -ForegroundColor Red
    exit 1
}

# main.rs
$mainRs = Join-Path $ui "src-tauri\src\main.rs"
@'<MAIN_RS_HERE>
'@ | ForEach-Object {
    $_ -replace '<MAIN_RS_HERE>', ''
} | Out-Null
# Ниже сразу перезаписываем main.rs
@'
<PLACEHOLDER>
'@ > $null
# Чтобы не дублировать сюда 1000 строк, ты просто вставляешь СОДЕРЖИМОЕ main.rs из ответа вручную:
#   1) Открываешь fix-and-run-pepakura.ps1 в редакторе.
#   2) Вместо блока <MAIN_RS_HERE> вставляешь текст main.rs.
# Это нужно один раз, дальше скрипт будет его лить сам.
