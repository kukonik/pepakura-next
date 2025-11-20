<#
.SYNOPSIS
    Автоматическая диагностика Unfolding Core
.DESCRIPTION
    Скрипт выполняет комплексную проверку состояния Unfolding Core
    и предлагает решения для обнаруженных проблем.
#>
param(
    [string]$ProjectPath = "D:\Dev\pepakura-next"
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

function Write-Section {
    param([string]$Title)
    Write-Host "`n$('-' * 60)" -ForegroundColor Cyan
    Write-Host "$Title" -ForegroundColor Magenta
    Write-Host "$('-' * 60)" -ForegroundColor Cyan
}

function Write-Status {
    param(
        [string]$Message,
        [ValidateSet("OK", "WARNING", "ERROR", "INFO")]
        [string]$Status = "INFO"
    )
    
    $color = @{
        "OK" = "Green"
        "WARNING" = "Yellow"
        "ERROR" = "Red"
        "INFO" = "Cyan"
    }[$Status]
    
    $prefix = @{
        "OK" = "✅"
        "WARNING" = "⚠️"
        "ERROR" = "❌"
        "INFO" = "ℹ️"
    }[$Status]
    
    Write-Host "$prefix $Message" -ForegroundColor $color
}

# 1. Проверка директории проекта
Write-Section "ПРОВЕРКА ДИРЕКТОРИИ ПРОЕКТА"
if (-not (Test-Path $ProjectPath)) {
    Write-Status "Директория проекта не существует: $ProjectPath" "ERROR"
    exit 1
}
Write-Status "Директория проекта найдена: $ProjectPath" "OK"

# 2. Проверка Rust
Write-Section "ПРОВЕРКА RUST"
if (-not (Get-Command "cargo" -ErrorAction SilentlyContinue)) {
    Write-Status "Rust не установлен!" "ERROR"
    Write-Status "👉 Установите Rust: https://rustup.rs/" "INFO"
    exit 1
}
$rustVersion = rustc --version 2>&1
Write-Status "Rust версия: $rustVersion" "OK"

# 3. Проверка Unfolding Core
Write-Section "ПРОВЕРКА UNFOLDING CORE"
$unfoldingCorePath = Join-Path $ProjectPath "src\backend\unfolding-core"
if (-not (Test-Path $unfoldingCorePath)) {
    Write-Status "Директория Unfolding Core не найдена: $unfoldingCorePath" "ERROR"
    exit 1
}
Write-Status "Unfolding Core директория найдена" "OK"

Set-Location $unfoldingCorePath
Write-Status "Текущая директория: $(Get-Location)" "INFO"

# 4. Проверка Cargo.toml
Write-Section "ПРОВЕРКА ЗАВИСИМОСТЕЙ"
if (-not (Test-Path "Cargo.toml")) {
    Write-Status "Файл Cargo.toml отсутствует!" "ERROR"
    # Автоматическое создание базового Cargo.toml
    Write-Status "Создание базового Cargo.toml..." "INFO"
    @"
[package]
name = "pepakura-unfolding-core"
version = "0.1.0"
edition = "2021"

[features]
default = ["server"]
server = ["axum", "tokio", "tracing-subscriber"]

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
"@ | Set-Content -Path "Cargo.toml" -Encoding UTF8
    Write-Status "✅ Базовый Cargo.toml создан" "OK"
}

# 5. Проверка порта
Write-Section "ПРОВЕРКА ПОРТА"
$port = 8080
$portStatus = Test-NetConnection -ComputerName "localhost" -Port $port -WarningAction SilentlyContinue -ErrorAction SilentlyContinue
if ($portStatus.TcpTestSucceeded) {
    Write-Status "Порт $port доступен для подключения" "OK"
} else {
    Write-Status "Порт $port недоступен" "WARNING"
    $existingProcess = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
    if ($existingProcess) {
        $process = Get-Process -Id $existingProcess.OwningProcess -ErrorAction SilentlyContinue
        Write-Status "Порт $port занят процессом: $($process.ProcessName) (PID: $($process.Id))" "WARNING"
    } else {
        Write-Status "Порт $port свободен, но сервис не отвечает" "WARNING"
    }
}

# 6. Попытка запуска
Write-Section "ПОПЫТКА ЗАПУСКА"
Write-Status "Запуск Unfolding Core..." "INFO"
try {
    cargo build --release --features server --quiet
    Write-Status "✅ Сборка выполнена успешно" "OK"
    
    # Запуск в фоновом режиме для проверки
    Start-Process -FilePath "cargo" -ArgumentList "run --release --features server" -NoNewWindow -PassThru
    Start-Sleep -Seconds 2
    
    # Проверка порта после запуска
    $portCheck = Test-NetConnection -ComputerName "localhost" -Port $port -WarningAction SilentlyContinue -ErrorAction SilentlyContinue
    if ($portCheck.TcpTestSucceeded) {
        Write-Status "✅ Unfolding Core успешно запущен на порту $port" "OK"
        
        # Проверка health endpoint
        try {
            $healthCheck = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method Get -TimeoutSec 5
            Write-Status "✅ Health endpoint доступен: $($healthCheck | ConvertTo-Json -Depth 1)" "OK"
        } catch {
            Write-Status "⚠️ Health endpoint недоступен: $($_.Exception.Message)" "WARNING"
        }
    } else {
        Write-Status "❌ Unfolding Core не запустился или не слушает порт $port" "ERROR"
        Write-Status "💡 Перезапустите скрипт для детальной диагностики" "INFO"
    }
} catch {
    Write-Status "❌ Ошибка при сборке/запуске: $($_.Exception.Message)" "ERROR"
    Write-Status "💡 Предложение по решению:" "INFO"
    Write-Status "   1. Очистите кэш: cargo clean" "INFO"
    Write-Status "   2. Обновите зависимости: cargo update" "INFO"
    Write-Status "   3. Попробуйте запустить в режиме отладки: cargo run --features server" "INFO"
}

Write-Host "`n$('-' * 60)" -ForegroundColor Cyan
Write-Host "ДИАГНОСТИКА ЗАВЕРШЕНА" -ForegroundColor Magenta
Write-Host "$('-' * 60)`n" -ForegroundColor Cyan