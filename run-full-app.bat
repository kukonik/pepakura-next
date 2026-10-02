@echo off
echo Starting Pepakura Next Full Application...

echo.
echo Starting Rust Backend in background...
start "Rust Backend" /min cmd /c "run-rust-backend.bat"

echo.
echo Starting UI Desktop...
call run-ui-desktop.bat