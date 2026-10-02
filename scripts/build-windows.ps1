# Pepakura Next - Build Script for Windows
# Requires: Rust, Node.js 20+, pnpm 8+

Write-Host "=== Pepakura Next Build ===" -ForegroundColor Cyan

# Check prerequisites
Write-Host "`n[1/5] Checking prerequisites..." -ForegroundColor Yellow

# Check Rust
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Rust not found. Install from https://rustup.rs/" -ForegroundColor Red
    exit 1
}
Write-Host "  ✓ Rust: $(cargo --version)" -ForegroundColor Green

# Check Node.js
if (!(Get-Command node -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: Node.js not found. Install from https://nodejs.org/" -ForegroundColor Red
    exit 1
}
Write-Host "  ✓ Node.js: $(node --version)" -ForegroundColor Green

# Check pnpm
if (!(Get-Command pnpm -ErrorAction SilentlyContinue)) {
    Write-Host "ERROR: pnpm not found. Run: npm install -g pnpm" -ForegroundColor Red
    exit 1
}
Write-Host "  ✓ pnpm: $(pnpm --version)" -ForegroundColor Green

# Install frontend dependencies
Write-Host "`n[2/5] Installing frontend dependencies..." -ForegroundColor Yellow
Set-Location $PSScriptRoot\ui-desktop
pnpm install --frozen-lockfile
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to install frontend dependencies" -ForegroundColor Red
    exit 1
}

# Build frontend
Write-Host "`n[3/5] Building frontend..." -ForegroundColor Yellow
pnpm build
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to build frontend" -ForegroundColor Red
    exit 1
}

# Build Rust core
Write-Host "`n[4/5] Building Rust core..." -ForegroundColor Yellow
Set-Location $PSScriptRoot\crates\pepakura_core
cargo build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to build Rust core" -ForegroundColor Red
    exit 1
}

# Build Tauri app
Write-Host "`n[5/5] Building Tauri application..." -ForegroundColor Yellow
Set-Location $PSScriptRoot\src-tauri
cargo tauri build --release
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: Failed to build Tauri app" -ForegroundColor Red
    exit 1
}

Write-Host "`n=== Build Complete ===" -ForegroundColor Green
Write-Host "Installers location:" -ForegroundColor Cyan
Write-Host "  - MSI: src-tauri\target\release\bundle\msi\" -ForegroundColor White
Write-Host "  - NSIS: src-tauri\target\release\bundle\nsis\" -ForegroundColor White
