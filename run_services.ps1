param(
    [int]$StartupDelay = 25,
    [int]$HealthCheckTimeout = 30,
    [switch]$ForceKillPorts,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"
$VerboseLog = $Verbose.IsPresent

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $colorMap = @{
        "Success" = "Green"
        "Info" = "Cyan"
        "Warning" = "Yellow"
        "Error" = "Red"
        "Important" = "Magenta"
        "Debug" = "Gray"
    }
    $color = $colorMap[$Level]
    Write-Host "[$timestamp] [$Level] $Message" -ForegroundColor ($color ? $color : "White")
}

function Get-PythonExecutable {
    $venvPython = "D:\Dev\pepakura-next\venv\Scripts\python.exe"
    if (Test-Path $venvPython) {
        Write-Log "🐍 Python in venv: $venvPython" "Success"
        return $venvPython
    }
    $globalPython = Get-Command "python" -ErrorAction SilentlyContinue
    if ($globalPython) {
        Write-Log "⚠️ Python venv not found, using global: $($globalPython.Path)" "Warning"
        return $globalPython.Path
    }
    Write-Log "❌ Python not found!" "Error"
    throw "Python not found"
}

function Stop-ProcessOnPort {
    param([int]$Port, [string]$ProcessNameFilter = "*")
    $connections = Get-NetTCPConnection -LocalPort $Port -ErrorAction SilentlyContinue
    foreach ($connection in $connections) {
        $process = Get-Process -Id $connection.OwningProcess -ErrorAction SilentlyContinue
        if ($process -and $process.ProcessName -like $ProcessNameFilter) {
            Write-Log "🛑 Killing $($process.ProcessName) (PID: $($process.Id)) on port $Port" "Warning"
            Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
        }
    }
}

function Test-ServiceHealth {
    param([string]$Url, [int]$TimeoutSec = 15)
    try {
        $response = Invoke-RestMethod -Uri $Url -Method Get -TimeoutSec $TimeoutSec -ErrorAction Stop
        return @{ Success = $true; Response = $response }
    } catch {
        return @{ Success = $false; Error = $_.Exception.Message }
    }
}

$pythonExe = Get-PythonExecutable

$Services = @{
    "AI Gateway" = @{
        "Path" = "D:\Dev\pepakura-next\src\backend\ai-gateway";
        "Cmd" = "$pythonExe main.py --port 8000";
        "Port" = 8000;
        "Health" = "http://localhost:8000/health";
        "ProcessName" = "python*"
    };
    "Unfolding Core" = @{
        "Path" = "D:\Dev\pepakura-next\src\backend\unfolding-core";
        "Cmd" = "cargo run --release --features server -- --port 8080";
        "Port" = 8080;
        "Health" = "http://localhost:8080/health";
        "ProcessName" = "cargo*"
    }
}

Write-Log "🚀 PEPAKURA NEXT ORCHESTRATOR" "Important"
Write-Log "📍 Directory: D:\Dev\pepakura-next" "Info"

foreach ($svc in $Services.Keys) {
    if (-not (Test-Path $Services[$svc]["Path"])) {
        Write-Log "❌ '$svc' folder missing: $($Services[$svc]['Path'])" "Error"
        exit 1
    }
}

if ($ForceKillPorts) {
    foreach ($svc in $Services.Keys) {
        Stop-ProcessOnPort -Port $Services[$svc]["Port"] -ProcessNameFilter $Services[$svc]["ProcessName"]
    }
    Start-Sleep -Seconds 2
}

$jobs = @{}
foreach ($svc in $Services.Keys) {
    $execDir = $Services[$svc]["Path"]
    $execCmd = $Services[$svc]["Cmd"]
    Write-Log ("🔧 {0} start in {1}: {2}" -f $svc, $execDir, $execCmd) "Info"
    $jobs[$svc] = Start-Job -ScriptBlock {
        param($execDir, $execCmd, $verbose)
        Set-Location $execDir
        if ($verbose) { Write-Host "[DEBUG] $execCmd" -ForegroundColor Gray }
        Invoke-Expression $execCmd
    } -ArgumentList $execDir, $execCmd, $VerboseLog
    Write-Log ("🔄 '{0}' Started (Job ID: {1})" -f $svc, $jobs[$svc].Id) "Info"
}

Write-Log "⏳ Waiting $StartupDelay seconds..." "Info"
Start-Sleep -Seconds $StartupDelay

foreach ($svc in $Services.Keys) {
    $health = Test-ServiceHealth -Url $Services[$svc]["Health"] -TimeoutSec $HealthCheckTimeout
    if ($health.Success) {
        Write-Log "✅ '$svc' running OK" "Success"
    } else {
        Write-Log "❌ '$svc' health FAILED: $($health.Error)" "Error"
        Write-Log "📄 Output:" "Warning"
        Receive-Job $jobs[$svc] -Keep -ErrorAction SilentlyContinue | ForEach-Object { Write-Log $_ "Warning" }
    }
}

Write-Log "💡 Для остановки сервисов нажмите Ctrl+C" "Info"

try {
    while ($true) { Start-Sleep -Seconds 1 }
} finally {
    Write-Log "🛑 Остановка всех сервисов..." "Warning"
    foreach ($svc in $jobs.Keys) {
        Stop-Job $jobs[$svc] -ErrorAction SilentlyContinue
        Remove-Job $jobs[$svc] -ErrorAction SilentlyContinue
        Write-Log "✅ $svc остановлен" "Success"
    }
    Write-Log "✅ Скрипт завершён." "Success"
}
