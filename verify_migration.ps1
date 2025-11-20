param(
    [string]$ProjectPath = "D:\Dev\pepakura-next"
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

Write-Log "✅ ПРОВЕРКА РАБОТОСПОСОБНОСТИ ПОСЛЕ МИГРАЦИИ" -Level "Important"

# 1. Проверка существования ключевых файлов
$keyFiles = @(
    "src\backend\unfolding-core\Cargo.toml",
    "src\backend\unfolding-core\src\main.rs",
    "src\backend\ai-gateway\main.py",
    "src\backend\ai-gateway\requirements.txt",
    "scripts\run_debug.ps1",
    "data\templates\cube.gif",
    "scripts\utils\full_workflow_test.ps1"
)

Write-Log "🔍 Проверка ключевых файлов..." -Level "Info"
$allFilesExist = $true

foreach ($file in $keyFiles) {
    $fullPath = Join-Path -Path $ProjectPath -ChildPath $file
    if (Test-Path $fullPath) {
        Write-Log "✅ Найден файл: $file" -Level "Success"
    } else {
        Write-Log "❌ Файл не найден: $file" -Level "Error"
        $allFilesExist = $false
    }
}

# 2. Проверка запуска сервисов
if ($allFilesExist) {
    Write-Log "🔧 Проверка запуска сервисов..." -Level "Info"
    
    # Активация виртуального окружения
    $venvPath = Join-Path -Path $ProjectPath -ChildPath "venv\Scripts\Activate.ps1"
    if (Test-Path $venvPath) {
        & $venvPath
        Write-Log "✅ Виртуальное окружение активировано" -Level "Success"
    } else {
        Write-Log "⚠️ Виртуальное окружение не найдено. Продолжаю без активации." -Level "Warning"
    }
    
    # Запуск AI Gateway
    $aiGatewayPath = Join-Path -Path $ProjectPath -ChildPath "src\backend\ai-gateway"
    if (Test-Path $aiGatewayPath) {
        Set-Location $aiGatewayPath
        try {
            python main.py --port 8000 &
            Start-Sleep -Seconds 5
            $healthCheck = Invoke-RestMethod -Uri "http://localhost:8000/health" -Method Get
            Write-Log "✅ AI Gateway запущен и отвечает на health check" -Level "Success"
        } catch {
            Write-Log "❌ Ошибка при запуске AI Gateway: $_" -Level "Error"
        }
        Set-Location $ProjectPath
    } else {
        Write-Log "❌ Директория AI Gateway не найдена: $aiGatewayPath" -Level "Error"
    }
    
    # Запуск Unfolding Core
    $unfoldingCorePath = Join-Path -Path $ProjectPath -ChildPath "src\backend\unfolding-core"
    if (Test-Path $unfoldingCorePath) {
        Set-Location $unfoldingCorePath
        try {
            cargo run --release --features server -- --port 8080 &
            Start-Sleep -Seconds 5
            $healthCheck = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method Get
            Write-Log "✅ Unfolding Core запущен и отвечает на health check" -Level "Success"
        } catch {
            Write-Log "❌ Ошибка при запуске Unfolding Core: $_" -Level "Error"
        }
        Set-Location $ProjectPath
    } else {
        Write-Log "❌ Директория Unfolding Core не найдена: $unfoldingCorePath" -Level "Error"
    }
} else {
    Write-Log "❌ Не все ключевые файлы найдены. Проверка сервисов пропущена." -Level "Error"
}

# 3. Финальный отчет
Write-Log "📋 ФИНАЛЬНЫЙ ОТЧЕТ МИГРАЦИИ" -Level "Important"

if ($allFilesExist) {
    Write-Log "✅ Все ключевые файлы успешно перенесены в новую структуру" -Level "Success"
} else {
    Write-Log "❌ Некоторые ключевые файлы не найдены. Требуется ручная проверка." -Level "Error"
}

Write-Log "💡 СЛЕДУЮЩИЕ ШАГИ:" -Level "Info"
Write-Log "  1. Запустите полный workflow тест: .\scripts\utils\full_workflow_test.ps1" -Level "Info"
Write-Log "  2. Проверьте работу всех сервисов в продакшен режиме" -Level "Info"
Write-Log "  3. Обновите документацию в соответствии с новой структурой" -Level "Info"
Write-Log "  4. Удалите резервную копию после окончательной проверки" -Level "Info"