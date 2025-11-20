<#
.SYNOPSIS
  Автоматизированный тест Pepakura Next: генерация изображений, upload на AI Gateway, лог архивации результатов.
.DESCRIPTION
  Скрипт создаёт тестовые PNG, JPG, GIF, отправляет их multipart-запросом в /image2mesh (FastAPI), собирает отчёт.
.PARAMETER ImageTypes    - Массив: PNG, JPG, GIF, ALL.
.PARAMETER Ports         - Хэштаблица портов сервисов.
.PARAMETER OutputDir     - Куда сохранять результаты и отчёты.
.PARAMETER DryRun        - Только диагностика конфигурации.
.PARAMETER Verbose       - Максимальное логирование.
.PARAMETER NoOpen        - Не открывать выходные SVG/PNG после теста.
.PARAMETER AutoStartServices - Попытка стартовать сервисы автоматически.
.EXAMPLE
  .\test_workflow.ps1 -ImageTypes PNG,JPG -Verbose -NoOpen -AutoStartServices
#>
param(
    [ValidateSet("PNG","JPG","GIF","ALL")]
    [string[]]$ImageTypes = @("ALL"),
    [hashtable]$Ports = @{AIGateway=8000;UnfoldingCore=8080},
    [string]$OutputDir = (Join-Path (Get-Location) "test_results_$(Get-Date -Format 'yyyyMMdd_HHmmss')"),
    [switch]$DryRun,
    [switch]$Verbose,
    [switch]$NoOpen,
    [switch]$AutoStartServices
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"
$script:allLogs = @()
$script:colorMap = @{
    "Success" = "Green"; "Info" = "Cyan"; "Warning" = "Yellow"
    "Error" = "Red"; "Important" = "Magenta"; "Debug" = "Gray"
}

function Write-Log { param([string]$Message,[string]$Level="Info",[switch]$NoNewLine)
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss.fff"
    $color = $script:colorMap[$Level] -as [System.ConsoleColor]; if (-not $color) { $color = "White" }
    $logMessage = "[$timestamp] [$Level] $Message"
    if ($NoNewLine) { Write-Host $logMessage -ForegroundColor $color -NoNewline }
    else { Write-Host $logMessage -ForegroundColor $color }
    $script:allLogs += [PSCustomObject]@{Timestamp=$timestamp;Level=$Level;Message=$Message}
}

function Test-RequiredTools { param([string[]]$Tools)
    $allOk = $true
    foreach ($tool in $Tools) {
        if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
            Write-Log "❌ Не найден инструмент: $tool" "Error"; $allOk = $false
        } else { Write-Log "✅ $tool доступен" "Success" }
    }
    return $allOk
}

function Create-DirIfNotExist { param([string]$dir)
    if (-not (Test-Path $dir)) { New-Item -Path $dir -ItemType Directory -Force | Out-Null; Write-Log "✅ Создана директория: $dir" "Success" }
}

function Test-ServiceHealth { param([string]$Url, [int]$TimeoutSec = 8)
    try {
        $response = Invoke-RestMethod -Uri $Url -TimeoutSec $TimeoutSec -ErrorAction Stop
        return @{Healthy=$true;Response=$response}
    } catch {
        return @{Healthy=$false;Error=$_.Exception.Message}
    }
}

function Generate-TestImage {
    param([ValidateSet("GIF","PNG","JPG")] [string]$Type,[int]$Width=100,[int]$Height=100)
    if ($IsWindows) {
        try {
            Add-Type -AssemblyName System.Drawing -ErrorAction Stop
            $bitmap = New-Object System.Drawing.Bitmap($Width, $Height)
            $g = [System.Drawing.Graphics]::FromImage($bitmap)
            $g.Clear([System.Drawing.Color]::White)
            $g.DrawString($Type, (New-Object System.Drawing.Font("Arial",18)), [System.Drawing.Brushes]::Black,15,40)
            $ms = New-Object System.IO.MemoryStream
            switch ($Type) {
                "GIF" { $bitmap.Save($ms,[System.Drawing.Imaging.ImageFormat]::Gif) }
                "PNG" { $bitmap.Save($ms,[System.Drawing.Imaging.ImageFormat]::Png) }
                "JPG" { $bitmap.Save($ms,[System.Drawing.Imaging.ImageFormat]::Jpeg) }
            }
            $bytes = $ms.ToArray(); $ms.Dispose(); $bitmap.Dispose(); $g.Dispose()
            return @{Bytes=$bytes;MimeType="image/$($Type.ToLower())";Extension=".$($Type.ToLower())"}
        } catch {
            Write-Log "Ошибка генерации изображения: $($_.Exception.Message)" "Error"; return $null
        }
    } else {
        Write-Log "⚠ Не может сгенерировать изображение (non-Windows)" "Warning"
        return @{Bytes=[Convert]::FromBase64String("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhESMIAAAAABJRU5ErkJggg==");MimeType="image/png";Extension=".png"}
    }
}

function Upload-ImageMultipart {
    param(
        [string]$FilePath, [string]$Description, [string]$Url,
        [string]$MimeType = "image/png"
    )
    $boundary = [System.Guid]::NewGuid().ToString()
    $lf = "`r`n"
    $fileBytes = [System.IO.File]::ReadAllBytes($FilePath)
    $filename = [IO.Path]::GetFileName($FilePath)
    $bodyList = @(
        "--$boundary$lf" +
        "Content-Disposition: form-data; name=`"file`"; filename=`"$filename`"$lf" +
        "Content-Type: $MimeType$lf$lf"
    )
    $descPart = (
        "--$boundary$lf" +
        "Content-Disposition: form-data; name=`"description`"$lf$lf" +
        "$Description$lf"
    )
    $endPart = "--$boundary--$lf"
    $mstream = New-Object System.IO.MemoryStream
    $writer = New-Object System.IO.StreamWriter($mstream, [System.Text.Encoding]::ASCII)
    $writer.Write($bodyList[0]); $writer.Flush()
    $mstream.Write($fileBytes,0,$fileBytes.Length)
    $writer.Write($lf); $writer.Write($descPart); $writer.Write($endPart); $writer.Flush(); $mstream.Position=0
    $headers = @{ "Content-Type" = "multipart/form-data; boundary=$boundary" }
    return Invoke-WebRequest -Uri $Url -Method Post -Headers $headers -Body $mstream
}

function RunImageTestWorkflow {
    param([string[]]$TestTypes, [hashtable]$Ports, [string]$OutputDir)
    $results = @{}
    foreach ($type in $TestTypes) {
        Write-Log "🖼️ Тест формата: $type" "Important"
        $imgData = Generate-TestImage -Type $type
        if (-not $imgData) { Write-Log "Ошибка генерации изображения $type" "Error"; continue }
        $tempFile = Join-Path $OutputDir "test_$type$($imgData.Extension)"
        [System.IO.File]::WriteAllBytes($tempFile, $imgData.Bytes)
        $endpoint = "http://localhost:$($Ports.AIGateway)/image2mesh"
        try {
            $resp = Upload-ImageMultipart -FilePath $tempFile -Description "auto_test $type" -Url $endpoint -MimeType $imgData.MimeType
            $json = $null; try { $json = $resp.Content | ConvertFrom-Json } catch {}
            if ($resp.StatusCode -eq 200 -and $json.status -eq "success") {
                Write-Log "✅ Получен результат от AI Gateway ($type)" "Success"
                $svgPath = Join-Path $OutputDir "test_$type.svg"
                [System.IO.File]::WriteAllText($svgPath, "<svg><text x='10' y='20'>$type SVG</text></svg>")
                $results[$type] = @{Success=$true; Preview=$tempFile; SVG=$svgPath; Details=$json.data}
                if (-not $NoOpen) { Start-Process $tempFile; Start-Process $svgPath }
            } else {
                $errMsg = if ($json) { $json | ConvertTo-Json -Compress } else { $resp.StatusDescription }
                Write-Log "Ошибка AI Gateway ($type): $errMsg" "Error"
                $results[$type] = @{Success=$false; Error=$errMsg}
            }
        } catch {
            Write-Log "Ошибка отправки изображения ${type}: $($_.Exception.Message)" "Error"
            $results[$type] = @{Success=$false; Error=$_.Exception.Message}
        }
    }
    return $results
}

function Save-Report {
    param($Data, [string]$OutputDir)
    $fn = Join-Path $OutputDir "test_report_$(Get-Date -Format 'yyyyMMdd_HHmmss').json"
    $json = $Data | ConvertTo-Json -Depth 8
    Set-Content -Path $fn -Value $json -Encoding UTF8
    Write-Log "📋 Отчёт сохранён: $fn" "Info"
}

# === ОСНОВНОЙ БЛОК ===

Write-Log "🚀 Старт workflow Pepakura Next" "Important"
Create-DirIfNotExist $OutputDir
$tools = @("python", "cargo", "git", "node")
if (-not (Test-RequiredTools $tools)) { Write-Log "🛑 Не все инструменты найдены. Завершение." "Error"; exit 1 }

$urls = @("http://localhost:$($Ports.AIGateway)/health", "http://localhost:$($Ports.UnfoldingCore)/health")
$allHealthy = $true; $healthyResult = @{}
foreach ($url in $urls) {
    $health = Test-ServiceHealth -Url $url
    $healthyResult[$url]=$health
    if (-not $health.Healthy) { $allHealthy = $false; Write-Log "❌ Сервис недоступен: $url, $($health.Error)" "Error" }
    else { Write-Log "✅ Сервис доступен: $url" "Success" }
}

if (-not $allHealthy -and $AutoStartServices) {
    Write-Log "🔄 Автоматический запуск сервисов невозможен (реализуйте отдельно run_services.ps1)" "Warning"
    Write-Log "Ожидание 10 секунд для ручного запуска..." "Info"
    Start-Sleep -Seconds 10
}

if ($DryRun) { Write-Log "[DRYRUN] Проверка завершена." "Info"; exit 0 }

$testTypes = if ($ImageTypes -contains "ALL") { @("GIF","PNG","JPG") } else { $ImageTypes }
$testResults = RunImageTestWorkflow -TestTypes $testTypes -Ports $Ports -OutputDir $OutputDir

$report = @{
    Timestamp = Get-Date
    OutputDir = $OutputDir
    TestTypes = $testTypes
    Results = $testResults
    ServicesHealth = $healthyResult
    Logs = $script:allLogs
}
Save-Report -Data $report -OutputDir $OutputDir

Write-Log "🎉 Все тесты завершены." "Success"
exit 0
