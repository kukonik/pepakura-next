@echo off
TITLE Pepakura Full Dev Stack Launcher
cls
echo.
echo =============================================
echo         🚀 STARTING FULL DEVELOPMENT STACK
echo =============================================
echo.

REM Логи
set LOG_DIR=logs
if not exist %LOG_DIR% mkdir %LOG_DIR%

REM Запуск фронтенда
start "Frontend Vite" cmd /k "cd /d D:\Dev\pepakura-next && pnpm dev > logs/frontend.log 2>&1"

REM Запуск бэкенда
start "Backend FastAPI" cmd /k "cd /d D:\Dev\pepakura-next\services\backend-python && .\venv\Scripts\activate && uvicorn app.main:app --reload > ..\..\logs\backend.log 2>&1"

echo 🌐 Frontend: http://localhost:5173
echo 🧠 Backend Docs: http://localhost:8000/docs
echo 📝 Логи: logs\
echo.
echo ✅ Все сервисы запущены!
timeout /t 3 >nul
