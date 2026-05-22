Write-Host "=== ЗАПУСК PEPAKURA NEXT ==="

# Запускаем веб-сервер на порту 3004 в фоне
Start-Job -ScriptBlock {
    cd D:\Dev\pepakura-next\packages\ui-desktop
    pnpm run dev -- --port 3004
} | Out-Null

Write-Host "Веб-сервер запускается на порту 3004..."
Start-Sleep -Seconds 3

# Запускаем Tauri
Write-Host "Запуск десктоп-приложения Tauri..."
cd D:\Dev\pepakura-next\packages\ui-desktop
pnpm tauri dev
