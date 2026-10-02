# Проверка наличия ключевых папок и файлов
# Использование: .\scripts\check-structure.ps1

Set-Location -Path "$PSScriptRoot\.."
Write-Host "Проверка структуры проекта..." -ForegroundColor Green

# Список обязательных файлов и папок
$requiredPaths = @(
    "core",
    "core/Cargo.toml",
    "core/src",
    "packages",
    "packages/ui-desktop",
    "packages/ui-desktop/package.json",
    "packages/ui-web",
    "packages/ui-web/package.json",
    "shared",
    "shared/package.json",
    "src",
    "src-tauri",
    "scripts"
)

$allExist = $true

foreach ($path in $requiredPaths) {
    if (Test-Path $path) {
        Write-Host "  [OK] $path" -ForegroundColor Green
    } else {
        Write-Host "  [MISSING] $path" -ForegroundColor Red
        $allExist = $false
    }
}

if ($allExist) {
    Write-Host "Все обязательные файлы и папки присутствуют!" -ForegroundColor Green
} else {
    Write-Host "Некоторые обязательные файлы или папки отсутствуют!" -ForegroundColor Red
}

# Показать структуру папок второго уровня
Write-Host "`nСтруктура папок второго уровня:" -ForegroundColor Yellow
Get-ChildItem -Depth 2 | Select-Object FullName