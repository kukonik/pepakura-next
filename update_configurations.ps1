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

function Update-FileContent {
    param(
        [string]$FilePath,
        [hashtable]$Replacements
    )
    
    if (-not (Test-Path $FilePath)) {
        Write-Log "⚠️ Файл не найден: $FilePath" -Level "Warning"
        return
    }
    
    $content = Get-Content -Path $FilePath -Raw
    
    foreach ($replacement in $Replacements.GetEnumerator()) {
        $oldValue = $replacement.Key
        $newValue = $replacement.Value
        
        if ($content -match [regex]::Escape($oldValue)) {
            if ($DryRun) {
                Write-Log "   [DRY RUN] В файле $($FilePath.Substring($ProjectPath.Length + 1)) будет заменено: '$oldValue' -> '$newValue'" -Level "Info"
            } else {
                $content = $content -replace [regex]::Escape($oldValue), $newValue
                Write-Log "🔄 Обновлено в файле $($FilePath.Substring($ProjectPath.Length + 1)): '$oldValue' -> '$newValue'" -Level "Info"
            }
        }
    }
    
    if (-not $DryRun -and $content -ne (Get-Content -Path $FilePath -Raw)) {
        Set-Content -Path $FilePath -Value $content -Force -Encoding UTF8
    }
}

Write-Log "🔧 ОБНОВЛЕНИЕ КОНФИГУРАЦИЙ И ПУТЕЙ" -Level "Important"

# Карта замен для файлов
$configFiles = @{
    # AI Gateway main.py
    "src\backend\ai-gateway\main.py" = @{
        "D:\Dev\pepakura-next\src\backend\ai-gateway" = "`$(pwd)"
        "D:\Dev\pepakura-next" = "`$(Get-Location)"
        "C:\Dev\PepakuraNext" = "`$(Get-Location)"
    }
    
    # Unfolding Core main.rs
    "src\backend\unfolding-core\src\main.rs" = @{
        "D:\Dev\pepakura-next\src\backend\unfolding-core" = "`$(pwd)"
        "D:\Dev\pepakura-next" = "`$(Get-Location)"
    }
    
    # Скрипты запуска
    "scripts\run_debug.ps1" = @{
        "..\src\backend\unfolding-core" = "..\..\..\src\backend\unfolding-core"
        "..\src\backend\ai-gateway" = "..\..\..\src\backend\ai-gateway"
        ".\venv\Scripts\Activate.ps1" = "..\venv\Scripts\Activate.ps1"
        "D:\Dev\pepakura-next" = "`$(Get-Location)"
    }
    
    # Тестовые скрипты
    "scripts\utils\full_workflow_test.ps1" = @{
        "D:\Dev\pepakura-next" = "`$(Get-Location)"
        "..\cube.gif" = "..\..\..\data\templates\cube.gif"
    }
    
    "scripts\utils\test_workflow.ps1" = @{
        "D:\Dev\pepakura-next" = "`$(Get-Location)"
        "..\cube.gif" = "..\..\..\data\templates\cube.gif"
    }
}

Write-Log "🔍 Анализ конфигурационных файлов..." -Level "Info"

# Обновление файлов
foreach ($config in $configFiles.GetEnumerator()) {
    $filePath = Join-Path -Path $ProjectPath -ChildPath $config.Key
    Update-FileContent -FilePath $filePath -Replacements $config.Value
}

Write-Log "✅ Обновление конфигураций завершено успешно" -Level "Success"