@echo off
cls
echo ================================
echo  AUTOINSTALL PYTHON & DEPENDENCIES FOR PEPAKURA-NEXT PROJECT
echo ================================

REM Проверим, установлен ли Python
python --version >nul 2>&1
if %errorlevel% equ 0 (
    echo ✅ Python уже установлен.
) else (
    echo ⚠️ Python не найден. Будет выполнен автоматический инсталл.
    powershell -Command "& {[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-WebRequest 'https://www.python.org/ftp/python/3.11.9/python-3.11.9-amd64.exe' -OutFile '%TEMP%\python-installer.exe'}"
    echo Installing Python silently...
    "%TEMP%\python-installer.exe" /quiet InstallAllUsers=1 PrependPath=1 Include_test=0
    echo Done!
)

timeout /t 5 >nul

REM Переход к папке backend
cd ../services/backend-python

REM Создание и активация virtualenv
echo 🛠 Создаем Virtual Environment...
python -m venv venv
call venv\Scripts\activate.bat

REM Обновляем pip
python -m pip install --upgrade pip

REM Установка зависимостей
echo 💾 Установка зависимостей из requirements.txt...
pip install -r requirements.txt

echo 💾 Установка дополнительных библиотек для GUI...
pip install requests pyqt5

echo.
echo ✅ Готово! Теперь можно запускать сервер:
echo.
echo 🔧 Запуск сервера (введите в другом окне):
echo        cd services\backend-python
echo        .\venv\Scripts\activate
echo        uvicorn app.main:app --reload
echo.
echo 🌐 Сервер будет доступен здесь: http://localhost:8000/docs
echo.
echo 🖥 GUI Monitor: tools/gui_status_monitor.py
pause
