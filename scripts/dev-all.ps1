# Параллельный запуск desktop + ui-web
# Использование: .\scripts\dev-all.ps1

Set-Location -Path "$PSScriptRoot\.."
Write-Host "Параллельный запуск desktop + ui-web..." -ForegroundColor Green

# Запуск обоих режимов параллельно
Start-Process -FilePath "powershell" -ArgumentList "-Command", "Set-Location '$(Get-Location)\packages\ui-desktop'; pnpm tauri dev" -Verb RunAs
Start-Process -FilePath "powershell" -ArgumentList "-Command", "Set-Location '$(Get-Location)\packages\ui-web'; pnpm dev" -Verb RunAs

Write-Host "Оба процесса запущены. Нажмите любую клавишу для завершения..." -ForegroundColor Yellow
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")