@echo off
setlocal
set PSH=powershell
where pwsh >nul 2>nul
if %errorlevel%==0 set PSH=pwsh
%PSH% -NoExit -ExecutionPolicy Bypass -File "%~dp0run-dev.ps1"
