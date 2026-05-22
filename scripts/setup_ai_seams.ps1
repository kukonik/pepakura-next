$ErrorActionPreference = "Stop"
$ProjectRoot = "D:\Dev\pepakura-next"
Write-Host "=== НАЧАЛО УСТАНОВКИ AI SEAMS ===" -ForegroundColor Magenta

if (-not (Get-Command python -ErrorAction SilentlyContinue)) { Write-Error "Python не найден!"; exit 1 }

Write-Host "[1/4] Установка зависимостей Python..." -ForegroundColor Cyan
python -m pip install fastapi uvicorn numpy pydantic

Write-Host "[2/4] Проверка структуры папок..." -ForegroundColor Cyan
New-Item -ItemType Directory -Force -Path "$ProjectRoot\backend\ai_seams" | Out-Null

Write-Host "[3/4] Компиляция TypeScript..." -ForegroundColor Cyan
$TscPath = "$ProjectRoot\node_modules\typescript\bin\tsc"
& $TscPath "$ProjectRoot\src\modules\ai\seam-ai-client.ts" --target ES2019 --module commonjs --outDir "$ProjectRoot\src\modules\ai" --esModuleInterop true --strict true
& $TscPath "$ProjectRoot\src\modules\renderer-3d\sceneRuntime.ts" --target ES2019 --module commonjs --outDir "$ProjectRoot\src\modules\renderer-3d" --esModuleInterop true --strict true

Write-Host "[4/4] Запуск FastAPI сервера..." -ForegroundColor Cyan
$BackendDir = "$ProjectRoot\backend\ai_seams"
$ServerCmd = "cd '$BackendDir'; python -m uvicorn app:app --reload --host 127.0.0.1 --port 8000"
Start-Process pwsh -ArgumentList "-NoExit", "-Command", $ServerCmd

Start-Sleep -Seconds 5
Write-Host "Запуск теста..." -ForegroundColor Cyan
node "$ProjectRoot\tools\test_ai_seams.js"
Write-Host "=== ГОТОВО ===" -ForegroundColor Green
