param(
    [switch]$NoGPU
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

Write-Log "🚀 ЗАПУСК ВСЕХ СЕРВИСОВ PEPAKURA NEXT" -Level "Important"
Write-Log "Текущая директория: $(Get-Location)" -Level "Info"

# Активация виртуального окружения
$venvPath = Join-Path -Path (Get-Location) -ChildPath "venv"
$activateScript = Join-Path -Path $venvPath -ChildPath "Scripts\Activate.ps1"

if (Test-Path $activateScript) {
    Write-Log "🐍 Активация Python окружения..." -Level "Info"
    & $activateScript
} else {
    Write-Log "❌ Python окружение не найдено. Пропускаем..." -Level "Error"
    exit 1
}

# Запуск AI Gateway
Write-Log "🔄 Запуск AI Gateway..." -Level "Info"
$aiJob = Start-Job -ScriptBlock {
    param($path)
    Set-Location "$path\src\backend\ai-gateway"
    python main.py
} -ArgumentList (Get-Location)

# Запуск Unfolding Core  
Write-Log "🔄 Запуск Unfolding Core..." -Level "Info"
$unfoldingJob = Start-Job -ScriptBlock {
    param($path)
    Set-Location "$path\src\backend\unfolding-core"
    cargo run --release --features server
} -ArgumentList (Get-Location)

Write-Log "⏳ Ожидание запуска сервисов (30 секунд)..." -Level "Info"
Start-Sleep -Seconds 30

# Проверка состояния
$services = @(
    @{Job = $aiJob; Name = "AI Gateway"; Port = 8000; Url = "http://localhost:8000/health"},
    @{Job = $unfoldingJob; Name = "Unfolding Core"; Port = 3000; Url = "http://localhost:3000/health"}
)

foreach ($service in $services) {
    $status = $service.Job.State
    if ($status -eq "Running") {
        try {
            $response = Invoke-WebRequest -Uri $service.Url -TimeoutSec 5 -ErrorAction Stop
            if ($response.StatusCode -eq 200) {
                Write-Log "✅ $($service.Name) работает корректно (порт $($service.Port))" -Level "Success"
            } else {
                Write-Log "❌ $($service.Name) вернул статус $($response.StatusCode)" -Level "Error"
            }
        } catch {
            Write-Log "❌ $($service.Name) недоступен: $($_.Exception.Message)" -Level "Error"
        }
    } else {
        Write-Log "❌ $($service.Name) не запустился. Состояние: $status" -Level "Error"
    }
}

Write-Log "💡 Для остановки всех сервисов нажмите Ctrl+C" -Level "Important"