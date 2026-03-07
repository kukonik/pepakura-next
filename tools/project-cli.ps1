<#
.SYNOPSIS
    Единая точка управления проектом Pepakura Next
.DESCRIPTION
    CLI для сканирования, чтения, редактирования и диагностики проекта
#>
param(
    [Parameter(Mandatory=$true)]
    [ValidateSet('scan', 'read', 'edit', 'deps', 'health', 'tree')]
    [string]$Command,
    
    [string]$Target,
    [string]$Search,
    [string]$Replace,
    [switch]$Full
)

$ErrorActionPreference = "Stop"
$ProjectRoot = "D:\Dev\pepakura-next"
$ToolsDir = "$ProjectRoot\tools"

Write-Host "`n🚀 Pepakura Next Project CLI" -ForegroundColor Magenta
Write-Host "Команда: $Command`n" -ForegroundColor Cyan

try {
    switch ($Command) {
        'scan' {
            & "$ToolsDir\project-scan.ps1" -Path $ProjectRoot -Depth 3 -Full:$Full
        }
        
        'tree' {
            & "$ToolsDir\project-scan.ps1" -Path $ProjectRoot -Depth 5
        }
        
        'read' {
            if (-not $Target) { throw "Укажите файл: -Target 'src-tauri/Cargo.toml'" }
            
            $FullPath = if ([System.IO.Path]::IsPathRooted($Target)) { $Target } else { Join-Path $ProjectRoot $Target }
            $FullPath = Resolve-Path $FullPath -ErrorAction Stop
            
            Write-Host "`n📄 Чтение файла: $($FullPath.Path.Replace($ProjectRoot, ''))`n" -ForegroundColor Cyan
            Get-Content $FullPath -Raw
        }
        
        'edit' {
            if (-not $Target -or -not $Search -or -not $Replace) {
                throw "Требуются параметры:`n  -Target 'путь/к/файлу'`n  -Search 'текст_для_поиска'`n  -Replace 'текст_для_замены'"
            }
            & "$ToolsDir\file-edit.ps1" -FilePath $Target -Search $Search -Replace $Replace
        }
        
        'deps' {
            Write-Host "`n📦 Зависимости проекта:`n" -ForegroundColor Magenta
            
            Write-Host "Frontend (pnpm):" -ForegroundColor Yellow
            $pnpmList = & pnpm list --depth=0 2>$null | Select-String "tauri|vue|vite|pinia" | Out-String
            if ($pnpmList) { Write-Host $pnpmList } else { Write-Host "  не найдены или ошибка pnpm" -ForegroundColor Red }
            
            Write-Host "`nBackend Rust (cargo):" -ForegroundColor Yellow
            $cargoToml = Join-Path $ProjectRoot "src-tauri\Cargo.toml"
            if (Test-Path $cargoToml) {
                $deps = Get-Content $cargoToml | Select-String "^\[dependencies\]" -Context 0,20 | Out-String
                Write-Host ($deps -split "`n" | Select-Object -First 15 | Where-Object { $_ -match "^\s*[a-z]" } | ForEach-Object { "  $_" })
            } else {
                Write-Host "  Cargo.toml не найден" -ForegroundColor Red
            }
            
            Write-Host "`nPython (venv):" -ForegroundColor Yellow
            $pip = Join-Path $ProjectRoot "env\Scripts\pip.exe"
            if (Test-Path $pip) {
                & $pip list 2>$null | Select-String "fastapi|uvicorn|pydantic" | ForEach-Object { "  $_" }
            } else {
                Write-Host "  виртуальное окружение не активировано" -ForegroundColor Red
            }
        }
        
        'health' {
            Write-Host "`n🏥 Диагностика сервисов:`n" -ForegroundColor Magenta
            
            # Проверка портов (ИСПРАВЛЕНО: ${port} вместо $port:)
            $ports = @(1420, 3000, 8000)
            foreach ($port in $ports) {
                $conn = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
                if ($conn) {
                    $proc = Get-Process -Id $conn.OwningProcess -ErrorAction SilentlyContinue
                    Write-Host "Порт ${port}: ЗАНЯТ" -ForegroundColor Green -NoNewline
                    Write-Host " ($($proc.ProcessName) PID:$($proc.Id))" -ForegroundColor DarkGray
                } else {
                    Write-Host "Порт ${port}: свободен" -ForegroundColor DarkGray
                }
            }
            
            # Проверка процессов
            Write-Host "`nПроцессы проекта:" -ForegroundColor Yellow
            Get-Process -Name "python*", "cargo", "node" -ErrorAction SilentlyContinue |
                Where-Object { $_.Path -like "*pepakura-next*" } |
                Select-Object ProcessName, Id, CPU, @{Name="Memory(MB)";Expression={[math]::Round($_.WS/1MB,1)}} |
                Format-Table -AutoSize
            
            # Проверка структуры
            Write-Host "`nСтруктура проекта:" -ForegroundColor Yellow
            $required = @(
                "src-tauri\tauri.conf.json",
                "src\main.ts",
                "package.json",
                "env\Scripts\python.exe"
            )
            foreach ($item in $required) {
                $path = Join-Path $ProjectRoot $item
                if (Test-Path $path) {
                    Write-Host "✅ $item" -ForegroundColor Green
                } else {
                    Write-Host "❌ $item (отсутствует)" -ForegroundColor Red
                }
            }
        }
    }
    
    Write-Host "`n✨ Операция завершена" -ForegroundColor Green
    
} catch {
    Write-Host "`n❌ Ошибка: $_" -ForegroundColor Red
    Write-Host "`n💡 Использование:`n  scan    [-Full]`n  tree`n  read    -Target 'путь/к/файлу'`n  edit    -Target файл -Search '...' -Replace '...'`n  deps`n  health`n" -ForegroundColor Yellow
    exit 1
}
