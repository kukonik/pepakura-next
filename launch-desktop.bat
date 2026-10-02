@echo off
echo.
echo ========================================
echo    ЗАПУСК PEPAKURA NEXT DESKTOP
echo ========================================
echo.

echo 1. Установка зависимостей (если нужно)...
cd D:\Dev\pepakura-next\packages\ui-desktop
pnpm install

echo.
echo 2. Запуск веб-сервера на порту 3000...
echo    (или используйте 'pnpm dev' для запуска с Tauri)
pnpm dev

echo.
echo ========================================
echo    ДОСТУПНЫЕ КОМАНДЫ:
echo    pnpm dev     - запуск веб-сервера
echo    pnpm tauri dev - запуск десктопного приложения
echo    pnpm install   - установка зависимостей
echo ========================================
