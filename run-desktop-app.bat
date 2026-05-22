@echo off
echo.
echo ========================================
echo    ЗАПУСК ДЕСКТОПНОГО ПРИЛОЖЕНИЯ PEPAKURA NEXT
echo ========================================
echo.

echo 1. Проверка и установка зависимостей...
cd D:\Dev\pepakura-next\packages\ui-desktop
pnpm install

echo.
echo 2. Проверка Tauri CLI...
tauri --version 2>nul
if %errorlevel% equ 0 (
    echo ✓ Tauri CLI установлен
) else (
    echo ✗ Tauri CLI не установлен
    echo Установите его командой: pnpm add -g @tauri-apps/cli
    goto :end
)

echo.
echo 3. Запуск десктопного приложения...
echo    Это может занять несколько секунд...
pnpm tauri dev

echo.
echo ========================================
echo    ЗАПУСК ЗАВЕРШЕН
echo ========================================

:end
pause
