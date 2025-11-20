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

Write-Log "🔄 ПЕРЕНОС СУЩЕСТВУЮЩИХ ФАЙЛОВ В НОВУЮ СТРУКТУРУ" -Level "Important"

# Карта переноса файлов (старый путь -> новый путь)
$fileMappings = @{
    # Unfolding Core (Rust)
    "src\backend\unfolding-core\Cargo.toml" = "src\backend\unfolding-core\Cargo.toml"
    "src\backend\unfolding-core\src\lib.rs" = "src\backend\unfolding-core\src\lib.rs"
    "src\backend\unfolding-core\src\main.rs" = "src\backend\unfolding-core\src\main.rs"
    
    # AI Gateway (Python)
    "src\backend\ai-gateway\main.py" = "src\backend\ai-gateway\main.py"
    "src\backend\ai-gateway\requirements.txt" = "src\backend\ai-gateway\requirements.txt"
    
    # Тестовые и служебные скрипты
    "run_debug.ps1" = "scripts\run_debug.ps1"
    "test_workflow.ps1" = "scripts\utils\test_workflow.ps1"
    "full_workflow_test.ps1" = "scripts\utils\full_workflow_test.ps1"
    "test_unfolding_core.ps1" = "scripts\utils\test_unfolding_core.ps1"
    "diagnose_unfolding_core.ps1" = "scripts\utils\diagnose_unfolding_core.ps1"
    "check_services.ps1" = "scripts\utils\check_services.ps1"
    
    # Данные и конфигурации
    "cube.gif" = "data\templates\cube.gif"
    
    # Логи
    "*.log" = "logs\"
}

Write-Log "🔍 Анализ существующих файлов..." -Level "Info"

# Перенос файлов
foreach ($mapping in $fileMappings.GetEnumerator()) {
    $sourcePattern = $mapping.Key
    $targetPath = $mapping.Value
    
    $sourceFullPath = Join-Path -Path $ProjectPath -ChildPath $sourcePattern
    $targetFullPath = Join-Path -Path $ProjectPath -ChildPath $targetPath
    
    # Получаем все файлы, соответствующие шаблону
    $files = Get-ChildItem -Path $sourceFullPath -ErrorAction SilentlyContinue
    
    if ($files) {
        foreach ($file in $files) {
            $relativePath = $file.FullName.Substring($ProjectPath.Length + 1)
            $newFilePath = Join-Path -Path $ProjectPath -ChildPath $targetPath
            
            # Если целевой путь - директория, сохраняем имя файла
            if ($targetPath.EndsWith("\")) {
                $newFilePath = Join-Path -Path $newFilePath -ChildPath $file.Name
            }
            
            $directory = Split-Path -Path $newFilePath -Parent
            
            if ($DryRun) {
                Write-Log "   [DRY RUN] $relativePath -> $newFilePath" -Level "Info"
            } else {
                # Создаем целевую директорию если не существует
                if (-not (Test-Path $directory)) {
                    New-Item -Path $directory -ItemType Directory -Force | Out-Null
                }
                
                # Переносим файл
                Move-Item -Path $file.FullName -Destination $newFilePath -Force
                
                # Если исходный файл был перемещен, удаляем пустую директорию
                $sourceDir = Split-Path -Path $file.FullName -Parent
                if ((Get-ChildItem -Path $sourceDir -ErrorAction SilentlyContinue | Measure-Object).Count -eq 0) {
                    Remove-Item -Path $sourceDir -Force -ErrorAction SilentlyContinue
                }
                
                Write-Log "✅ Перенесен: $relativePath -> $newFilePath" -Level "Success"
            }
        }
    } else {
        if ($DryRun) {
            Write-Log "   [DRY RUN] Файл не найден: $sourcePattern" -Level "Warning"
        } else {
            Write-Log "⚠️ Файл не найден для переноса: $sourcePattern" -Level "Warning"
        }
    }
}

Write-Log "✅ Перенос файлов завершен успешно" -Level "Success"