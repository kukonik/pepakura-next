param(
    [switch]$NoGPU,
    [switch]$RunAiEngine,
    [switch]$RunAll
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
        "Debug" = "Gray"
    }
    $color = if ($colorMap.ContainsKey($Level)) { $colorMap[$Level] } else { "White" }
    Write-Host " [$timestamp] $Message" -ForegroundColor $color
}

Write-Log "🚀 ЗАПУСК PEPAKURA NEXT (ИСПРАВЛЕННАЯ ВЕРСИЯ)" -Level "Important"
Write-Log "Текущая директория: (Get-Location)" -Level "Info"

# Активация Python окружения
$venvPath = Join-Path -Path (Get-Location) -ChildPath "venv"
$activateScript = Join-Path -Path $venvPath -ChildPath "Scripts\Activate.ps1"

if (Test-Path $activateScript) {
    Write-Log "🐍 Активация Python окружения..." -Level "Info"
    & $activateScript
} else {
    Write-Log "⚠️ Python окружение не найдено. Пропускаем..." -Level "Warning"
}

# --- Запуск AI Gateway ---
if ($RunAiEngine -or $RunAll) {
    $aiEngineDir = Join-Path -Path (Get-Location) -ChildPath "src\backend\ai-gateway"
    if (Test-Path $aiEngineDir) {
        Write-Log "🔧 Запуск AI Gateway из: $aiEngineDir" -Level "Info"
        Set-Location $aiEngineDir

        # Установка Python зависимостей
        Write-Log "📦 Установка Python зависимостей..." -Level "Info"
        pip install -r requirements.txt --no-cache-dir --upgrade

        Write-Log "🚀 Запуск AI Gateway сервера (порт 8000)..." -Level "Important"
        Write-Log "💡 Для остановки нажмите Ctrl+C в этом окне." -Level "Info"
        python main.py

        if ($LASTEXITCODE -ne 0) {
            Write-Log "❌ Ошибка при запуске AI Gateway (exit code: $LASTEXITCODE)" -Level "Error"
            # Не выходим, если запускаем все сервисы
            if (-not $RunAll) { exit $LASTEXITCODE }
        } else {
            Write-Log "✅ AI Gateway завершил работу" -Level "Success"
        }
    } else {
        Write-Log "❌ Директория AI Gateway не найдена: $aiEngineDir" -Level "Error"
        if (-not $RunAll) { exit 1 }
    }
    # Возврат в корень проекта, если запускаем только AI
    if (-not $RunAll) {
        Set-Location (Get-Location).Parent.Parent.Parent.Parent
        Write-Log "🏁 Запуск AI Engine завершен." -Level "Info"
        exit 0
    }
}

# --- Запуск Unfolding Core ---
if (-not $RunAiEngine) { # Запускаем Core, если не выбран только AI Engine
    $unfoldingCoreDir = Join-Path -Path (Get-Location) -ChildPath "src\backend\unfolding-core"

    if (Test-Path $unfoldingCoreDir) {
        Write-Log "🔧 Запуск Unfolding Core из: $unfoldingCoreDir" -Level "Info"
        Set-Location $unfoldingCoreDir

        # Сборка перед запуском
        Write-Log "📦 Сборка Unfolding Core..." -Level "Info"
        cargo build --release --features server --quiet

        # Запуск сервера
        Write-Log "🚀 Запуск сервера Unfolding Core (порт 3000)..." -Level "Important"
        Write-Log "💡 Для остановки нажмите Ctrl+C в этом окне." -Level "Info"
        cargo run --release --features server

        if ($LASTEXITCODE -ne 0) {
            Write-Log "❌ Ошибка при запуске Unfolding Core (exit code: $LASTEXITCODE)" -Level "Error"
            # Не выходим, если запускаем все сервисы
            if (-not $RunAll) { exit $LASTEXITCODE }
        } else {
            Write-Log "✅ Unfolding Core завершил работу" -Level "Success"
        }
    } else {
        Write-Log "❌ Директория Unfolding Core не найдена: $unfoldingCoreDir" -Level "Error"
        if (-not $RunAll) { exit 1 }
    }
    # Возврат в корень проекта, если запускаем только Core
    if (-not $RunAll) {
        Set-Location (Get-Location).Parent.Parent.Parent.Parent
        Write-Log "🏁 Запуск Unfolding Core завершен." -Level "Info"
        exit 0
    }
}

# Возврат в корень проекта, если запускаем все
if ($RunAll) {
    Set-Location (Get-Location).Parent.Parent.Parent.Parent
    Write-Log "🏁 Запуск всех сервисов завершен." -Level "Info"
}
