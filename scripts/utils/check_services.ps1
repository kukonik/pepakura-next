param(
    [switch]$Verbose
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

Write-Log "🔍 ПРОВЕРКА СОСТОЯНИЯ СЕРВИСОВ PEPAKURA NEXT" -Level "Important"

$services = @(
    @{Name = "AI Gateway"; Url = "http://localhost:8000/health"; Port = 8000},
    @{Name = "Unfolding Core Health"; Url = "http://localhost:3000/health"; Port = 3000},
    @{Name = "Unfolding Core Test Cube"; Url = "http://localhost:3000/test-cube"; Port = 3000}
)

foreach ($service in $services) {
    try {
        $response = Invoke-RestMethod -Uri $service.Url -Method Get -TimeoutSec 10 -ErrorAction Stop
        Write-Log "✅ $($service.Name) работает корректно (порт $($service.Port))" -Level "Success"
        if ($Verbose) {
            Write-Log "📊 Ответ: $($response | ConvertTo-Json -Depth 4)" -Level "Info"
        }
    } catch {
        $errorMessage = $_.Exception.Message
        Write-Log "❌ $($service.Name) недоступен: $errorMessage" -Level "Error"
        
        # Дополнительная диагностика
        try {
            $portStatus = Test-NetConnection -ComputerName "localhost" -Port $service.Port -WarningAction SilentlyContinue
            if ($portStatus.TcpTestSucceeded) {
                Write-Log "⚠️ Порт $($service.Port) открыт, но сервис не отвечает" -Level "Warning"
            } else {
                Write-Log "⚠️ Порт $($service.Port) закрыт или занят другим процессом" -Level "Warning"
            }
        } catch {
            Write-Log "⚠️ Не удалось проверить порт $($service.Port): $($_.Exception.Message)" -Level "Warning"
        }
    }
}

Write-Log "📋 СВОДКА:" -Level "Info"
if ($global:allServicesOk -eq $true) {
    Write-Log "🎉 Все сервисы работают корректно!" -Level "Success"
} else {
    Write-Log "⚠️ Некоторые сервисы не работают. См. детали выше." -Level "Warning"
    Write-Log "💡 Рекомендуемые действия:" -Level "Info"
    Write-Log "   1. Перезапустите проблемные сервисы" -Level "Info"
    Write-Log "   2. Проверьте логи сервисов на предмет ошибок" -Level "Info"
    Write-Log "   3. Убедитесь, что порты не заняты другими процессами" -Level "Info"
}