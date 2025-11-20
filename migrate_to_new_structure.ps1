param(
    [string]$ProjectPath = "D:\Dev\pepakura-next",
    [switch]$DryRun
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

function Write-Log {
    param(
        [string]$Message,
        [string]$Level = "Info"
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $colorMap = @{
        "Success" = "Green"
        "Info" = "Cyan"
        "Warning" = "Yellow"
        "Error" = "Red"
        "Important" = "Magenta"
    }
    $color = if ($colorMap.ContainsKey($Level)) { $colorMap[$Level] } else { "White" }
    Write-Host "[$timestamp] $Message" -ForegroundColor $color
}

Write-Log "🔍 НАЧАЛО МИГРАЦИИ НА НОВУЮ СТРУКТУРУ" -Level "Important"
Write-Log "📁 Текущий путь проекта: $ProjectPath" -Level "Info"
Write-Log "🧪 Режим dry run: $($DryRun ? 'Включен' : 'Отключен')" -Level "Info"

# Новая структура директорий
$newStructure = @(
    "src",
    "src\backend",
    "src\backend\unfolding-core",
    "src\backend\unfolding-core\src",
    "src\backend\unfolding-core\benches",
    "src\backend\unfolding-core\examples",
    "src\backend\unfolding-core\target",
    "src\backend\unfolding-core\.cargo",
    "src\backend\ai-gateway",
    "src\backend\ai-gateway\providers",
    "src\backend\ai-gateway\utils",
    "src\backend\ai-gateway\models",
    "src\backend\ai-gateway\pipelines",
    "src\backend\gateway",
    "src\backend\gateway\src",
    "src\frontend",
    "src\frontend\web",
    "src\frontend\web\src",
    "src\frontend\web\public",
    "src\frontend\web\src\components",
    "src\frontend\web\src\pages",
    "src\frontend\web\src\services",
    "src\frontend\web\src\utils",
    "src\frontend\web\src\assets",
    "src\frontend\desktop",
    "src\frontend\desktop\src-tauri",
    "src\shared",
    "src\shared\models",
    "src\shared\utils",
    "data",
    "data\models",
    "data\templates",
    "data\cache",
    "data\temp",
    "logs",
    "logs\ai-engine",
    "logs\storage",
    "models",
    "models\cpu-optimized",
    "models\gpu-optimized",
    "templates",
    "templates\basic",
    "templates\standard",
    "templates\premium",
    "scripts",
    "scripts\install",
    "scripts\utils",
    "scripts\ci",
    "tests",
    "tests\unit",
    "tests\integration",
    "tests\e2e",
    "docs",
    "docs\api",
    "docs\user-guides",
    "docs\architecture",
    "docker",
    "deploy",
    "deploy\terraform",
    "deploy\kubernetes",
    "venv",
    "venv\Scripts"
)

Write-Log "📁 Создание новой структуры директорий..." -Level "Info"
foreach ($dir in $newStructure) {
    $fullPath = Join-Path -Path $ProjectPath -ChildPath $dir
    
    if ($DryRun) {
        Write-Log "   [DRY RUN] Будет создана директория: $fullPath" -Level "Info"
    } else {
        if (-not (Test-Path $fullPath)) {
            New-Item -Path $fullPath -ItemType Directory -Force | Out-Null
            Write-Log "✅ Создана директория: $dir" -Level "Success"
        } else {
            Write-Log "⚠️ Директория уже существует: $dir" -Level "Warning"
        }
    }
}
Write-Log "✅ Структура директорий создана успешно" -Level "Success"