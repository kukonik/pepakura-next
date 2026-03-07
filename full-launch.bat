@echo off
echo.
echo ========================================
echo    ЗАПУСК ПОЛНОГО ПРОЕКТА PEPAKURA NEXT
echo ========================================
echo.

:: Запуск VITE (фронтенд) в новом окне
start "" cmd /k "cd /d D:\Dev\pepakura-next && pnpm install && pnpm dev"

:: Запуск FastAPI (сервер) в новом окне
start "" cmd /k "cd /d D:\Dev\pepakura-next\services\backend-python && python -m pip install -r requirements.txt && uvicorn app.main:app --reload"

timeout /t 3 >nul

echo.
echo ========================================
echo   Сервисы запускаются в двух окнах:
echo   1) Фронтенд: http://localhost:5173
echo   2) Бэкенд:  http://localhost:8000/docs
echo ========================================
