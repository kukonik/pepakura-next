#!/usr/bin/env pwsh
# init.ps1 - 100% локальная инициализация

if ($PSVersionTable.PSVersion.Major -lt 7) {
    Write-Host "❌ Требуется PowerShell 7+" -ForegroundColor Red
    exit 1
}

$projectName = "pepakura-next"
Write-Host "🚀 Создание $projectName..." -ForegroundColor Cyan

# Создание структуры
@(
    $projectName,
    "$projectName/scripts"
) | ForEach-Object {
    if (-not (Test-Path $_)) {
        New-Item -Path $_ -ItemType Directory -Force | Out-Null
    }
}

# Базовый utils.ps1
$utilsContent = @'
function Write-Colored { 
    param(
        [string]$m,
        [string]$l = "INFO"
    )

    $c = @{
        INFO  = "Cyan"
        ERROR = "Red"
        WARN  = "Yellow"
        DEBUG = "Gray"
    }[$l]

    if (-not $c) { $c = "White" }

    $ts = Get-Date -Format "HH:mm:ss"
    Write-Host "[$ts][$l] $m" -ForegroundColor $c
}
'@

$utilsPath = Join-Path $projectName "scripts/utils.ps1"
$utilsContent | Out-File -FilePath $utilsPath -Encoding UTF8 -Force

Write-Host "✅ Проект создан локально!" -ForegroundColor Green
Write-Host "cd $projectName" -ForegroundColor White
