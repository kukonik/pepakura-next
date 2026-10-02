@echo off
echo Запуск Pepakura Next Desktop...
echo.

echo 1. Запуск веб-сервера на порту 3000...
cd D:\Dev\pepakura-next\packages\ui-desktop
pnpm dev

echo.
echo 2. Если нужно запустить Tauri приложение:
echo    cd D:\Dev\pepakura-next\packages\ui-desktop
echo    pnpm tauri dev
echo.
echo 3. Для запуска с конкретным портом:
echo    cd D:\Dev\pepakura-next\packages\ui-desktop
echo    vite --port 3000
