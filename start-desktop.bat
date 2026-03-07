@echo off
echo.
echo ========================================
echo    ЗАПУСК ДЕСКТОПНОГО ПРИЛОЖЕНИЯ PEPAKURA NEXT
echo ========================================
echo.

echo 1. Проверка зависимостей...
cd D:\Dev\pepakura-next\packages\ui-desktop
pnpm install

echo.
echo 2. Проверка Tauri CLI...
tauri --version

echo.
echo 3. Запуск десктопного приложения...
echo    Это может занять несколько минут при первом запуске...
tauri dev

echo.
echo ========================================
echo    ЗАПУСК ЗАВЕРШЕН
echo ========================================
