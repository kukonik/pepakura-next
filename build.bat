@echo off
REM Pepakura Next - Quick Build Script
REM Usage: build.bat [core|wasm|desktop|web|addons|all]

setlocal enabledelayedexpansion

set PROJECT_ROOT=%~dp0
set TARGET=%1
if "%TARGET%"=="" set TARGET=all

echo.
echo ╔══════════════════════════════════════════════════════════╗
echo ║          Pepakura Next - Quick Build v4.0               ║
echo ╚══════════════════════════════════════════════════════════╝
echo.
echo Target: %TARGET%
echo.

if "%TARGET%"=="all" goto :build_all
if "%TARGET%"=="core" goto :build_core
if "%TARGET%"=="wasm" goto :build_wasm
if "%TARGET%"=="desktop" goto :build_desktop
if "%TARGET%"=="web" goto :build_web
if "%TARGET%"=="addons" goto :build_addons

echo Unknown target: %TARGET%
echo Valid targets: all, core, wasm, desktop, web, addons
exit /b 1

:build_all
    call :build_core
    call :build_wasm
    call :build_addons
    call :build_desktop
    call :build_web
    goto :success

:build_core
    echo [1/5] Building Pepakura Core...
    cd "%PROJECT_ROOT%crates\pepakura_core"
    cargo check --verbose
    if errorlevel 1 exit /b 1
    echo ✓ Core build successful
    cd "%PROJECT_ROOT%"
    goto :eof

:build_wasm
    echo [2/5] Building WASM Module...
    cd "%PROJECT_ROOT%crates\pepakura_wasm"
    wasm-pack build --target web --out-dir ../platform/web/public/wasm
    if errorlevel 1 exit /b 1
    echo ✓ WASM build successful
    cd "%PROJECT_ROOT%"
    goto :eof

:build_addons
    echo [3/5] Building Addons Framework...
    cd "%PROJECT_ROOT%crates\pepakura_addons"
    cargo check --verbose
    if errorlevel 1 exit /b 1
    echo ✓ Addons build successful
    cd "%PROJECT_ROOT%"
    goto :eof

:build_desktop
    echo [4/5] Building Desktop Application...
    cd "%PROJECT_ROOT%platform\desktop\ui-desktop"
    call pnpm install
    call pnpm tauri build
    if errorlevel 1 exit /b 1
    echo ✓ Desktop build successful
    cd "%PROJECT_ROOT%"
    goto :eof

:build_web
    echo [5/5] Building Web Application...
    cd "%PROJECT_ROOT%platform\web"
    call pnpm install
    call pnpm build
    if errorlevel 1 exit /b 1
    echo ✓ Web build successful
    cd "%PROJECT_ROOT%"
    goto :eof

:success
    echo.
    echo ╔══════════════════════════════════════════════════════════╗
    echo ║              BUILD COMPLETED SUCCESSFULLY                ║
    echo ╚══════════════════════════════════════════════════════════╝
    echo.
    exit /b 0
