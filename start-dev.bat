@echo off
echo Starting Pepakura Next Development Server...
echo.

cd ui-desktop
echo Installing dependencies...
pnpm install
echo.

echo Starting development server...
pnpm tauri dev
cd ..