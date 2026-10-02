@echo off
echo Building Pepakura Next Application...
echo.

cd ui-desktop
echo Installing dependencies...
pnpm install
echo.

echo Building application...
pnpm tauri build
echo.

echo Build completed! Check the src-tauri/target directory for the built application.
pause