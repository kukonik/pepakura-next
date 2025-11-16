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

Write-Log "🚀 ЗАПУСК ВСЕХ СЕРВИСОВ PEPAKURA NEXT (ИСПРАВЛЕННАЯ ВЕРСИЯ)" -Level "Important"
Write-Log "Текущая директория: $(Get-Location)" -Level "Info"

# Проверка и освобождение портов
function Free-Port {
    param([int]$Port)
    
    try {
        $connection = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
        if ($connection) {
            $process = Get-Process -Id $connection.OwningProcess -ErrorAction SilentlyContinue
            if ($process) {
                Write-Log "🛑 Остановка процесса $($process.ProcessName) (PID: $($process.Id)), занимающего порт $Port" -Level "Warning"
                Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
                Start-Sleep -Seconds 2
                return $true
            }
        }
        return $false
    } catch {
        Write-Log "⚠️ Не удалось освободить порт $Port: $($_.Exception.Message)" -Level "Warning"
        return $false
    }
}

# Активация виртуального окружения
$venvPath = Join-Path -Path (Get-Location) -ChildPath "venv"
$activateScript = Join-Path -Path $venvPath -ChildPath "Scripts\Activate.ps1"

if (Test-Path $activateScript) {
    Write-Log "🐍 Активация Python окружения..." -Level "Info"
    & $activateScript
} else {
    Write-Log "❌ Python окружение не найдено. Пропускаем..." -Level "Error"
}

# Стоп всех существующих сервисов
Write-Log "🔄 Остановка всех существующих сервисов..." -Level "Info"
Get-Process | Where-Object { 
    $_.ProcessName -match "python" -or 
    $_.ProcessName -match "pepakura" -or 
    $_.ProcessName -match "cargo"
} | Stop-Process -Force -ErrorAction SilentlyContinue

# Запуск Unfolding Core
Write-Log "🔧 Запуск Unfolding Core..." -Level "Info"
$unfoldingJob = Start-Job -ScriptBlock {
    param($path)
    Set-Location "$path\src\backend\unfolding-core"
    cargo run --release --features server
} -ArgumentList (Get-Location)

# Запуск AI Gateway
Write-Log "🔧 Запуск AI Gateway..." -Level "Info"
$aiJob = Start-Job -ScriptBlock {
    param($path)
    Set-Location "$path\src\backend\ai-gateway"
    python main.py
} -ArgumentList (Get-Location)

Write-Log "⏳ Ожидание запуска сервисов (30 секунд)..." -Level "Info"
Start-Sleep -Seconds 30

# Проверка состояния сервисов
$services = @(
    @{Job = $aiJob; Name = "AI Gateway"; Port = 8000; Url = "http://localhost:8000/health"},
    @{Job = $unfoldingJob; Name = "Unfolding Core"; Port = 3000; Url = "http://localhost:3000/health"}
)

foreach ($service in $services) {
    $job = $service.Job
    $status = $job.State
    
    if ($status -eq "Running") {
        try {
            # Проверка состояния порта
            $portCheck = Test-NetConnection -ComputerName "localhost" -Port $service.Port -WarningAction SilentlyContinue -TimeoutSeconds 5
            if (-not $portCheck.TcpTestSucceeded) {
                Write-Log "⚠️ Порт $($service.Port) не отвечает для $($service.Name), но процесс работает" -Level "Warning"
            }
            
            # Проверка эндпоинта
            $response = Invoke-WebRequest -Uri $service.Url -TimeoutSec 10 -ErrorAction Stop
            if ($response.StatusCode -eq 200) {
                Write-Log "✅ $($service.Name) работает корректно (порт $($service.Port))" -Level "Success"
            } else {
                Write-Log "❌ $($service.Name) вернул статус $($response.StatusCode)" -Level "Error"
            }
        } catch {
            $errorMessage = $_.Exception.Message
            Write-Log "❌ $($service.Name) недоступен: $errorMessage" -Level "Error"
            
            # Дополнительная диагностика
            try {
                $jobOutput = Receive-Job $job -Keep -ErrorAction SilentlyContinue
                if ($jobOutput) {
                    Write-Log "📄 Вывод процесса $($service.Name):" -Level "Warning"
                    $jobOutput | ForEach-Object { Write-Log $_ -Level "Warning" }
                }
            } catch {
                # Продолжить без вывода
            }
        }
    } else {
        Write-Log "❌ $($service.Name) не запустился. Состояние: $status" -Level "Error"
        
        try {
            $jobOutput = Receive-Job $job -Keep -ErrorAction SilentlyContinue
            if ($jobOutput) {
                Write-Log "📄 Вывод процесса $($service.Name):" -Level "Warning"
                $jobOutput | ForEach-Object { Write-Log $_ -Level "Warning" }
            }
        } catch {
            # Продолжить без вывода
        }
    }
}

Write-Log "💡 Для остановки всех сервисов нажмите Ctrl+C" -Level "Important"

try {
    while ($true) {
        Start-Sleep -Seconds 60
    }
} finally {
    Write-Log "🛑 Остановка всех сервисов..." -Level "Warning"
    Get-Job | Stop-Job -ErrorAction SilentlyContinue
    Get-Job | Remove-Job -Force -ErrorAction SilentlyContinue
    Write-Log "✅ Все сервисы остановлены" -Level "Success"
}