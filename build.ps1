#!/usr/bin/env pwsh
# Build script for Pepakura Next - Hybrid v4.0

param(
    [ValidateSet("all", "core", "wasm", "desktop", "web", "addons")]
    [string]$Target = "all",
    
    [switch]$Release,
    
    [switch]$Help
)

$ErrorActionPreference = "Stop"
$ProjectRoot = $PSScriptRoot
$Config = if ($Release) { "release" } else { "debug" }

function Write-Header {
    param([string]$Text)
    Write-Host ""
    Write-Host "═" * 60 -ForegroundColor Cyan
    Write-Host "  $Text" -ForegroundColor Cyan
    Write-Host "═" * 60 -ForegroundColor Cyan
    Write-Host ""
}

function Build-Core {
    Write-Header "Building Pepakura Core"
    Set-Location "$ProjectRoot\crates\pepakura_core"
    cargo check --verbose
    
    if ($LASTEXITCODE -ne 0) {
        throw "Core build failed"
    }
    
    Write-Host "✓ Core build successful" -ForegroundColor Green
}

function Build-Wasm {
    Write-Header "Building WASM Module"
    
    # Check if wasm-pack is installed
    if (-not (Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
        Write-Host "Installing wasm-pack..." -ForegroundColor Yellow
        cargo install wasm-pack
    }
    
    Set-Location "$ProjectRoot\crates\pepakura_wasm"
    
    $Args = @("build", "--target", "web")
    if ($Release) {
        $Args += "--release"
    }
    
    wasm-pack @Args
    
    if ($LASTEXITCODE -ne 0) {
        throw "WASM build failed"
    }
    
    # Copy to web platform
    $WasmOutput = "$ProjectRoot\crates\pepakura_wasm\pkg"
    $WebWasm = "$ProjectRoot\platform\web\public\wasm"
    
    if (Test-Path $WebWasm) {
        Remove-Item $WebWasm -Recurse -Force
    }
    
    New-Item -ItemType Directory -Path $WebWasm | Out-Null
    Copy-Item "$WasmOutput\*" -Destination $WebWasm -Recurse
    
    Write-Host "✓ WASM build successful" -ForegroundColor Green
    Write-Host "  Output: $WebWasm" -ForegroundColor Gray
}

function Build-Desktop {
    Write-Header "Building Desktop Application (Tauri)"
    
    Set-Location "$ProjectRoot\platform\desktop\ui-desktop"
    
    # Install dependencies
    pnpm install
    
    # Build frontend
    pnpm build
    
    # Build Tauri
    $TauriArgs = @("tauri", "build")
    if ($Release) {
        $TauriArgs += "--release"
    }
    
    pnpm @TauriArgs
    
    if ($LASTEXITCODE -ne 0) {
        throw "Desktop build failed"
    }
    
    Write-Host "✓ Desktop build successful" -ForegroundColor Green
}

function Build-Web {
    Write-Header "Building Web Application"
    
    Set-Location "$ProjectRoot\platform\web"
    
    # Install dependencies
    pnpm install
    
    # Build frontend
    $BuildArgs = @("build")
    if ($Release) {
        $BuildArgs += "--mode", "production"
    }
    
    pnpm @BuildArgs
    
    if ($LASTEXITCODE -ne 0) {
        throw "Web build failed"
    }
    
    Write-Host "✓ Web build successful" -ForegroundColor Green
}

function Build-Addons {
    Write-Header "Building Addons Framework"
    
    Set-Location "$ProjectRoot\crates\pepakura_addons"
    cargo check --verbose
    
    if ($LASTEXITCODE -ne 0) {
        throw "Addons build failed"
    }
    
    # Build example addon
    if (Test-Path "$ProjectRoot\addons\example-rust-addon") {
        Set-Location "$ProjectRoot\addons\example-rust-addon"
        cargo check --verbose
    }
    
    Write-Host "✓ Addons build successful" -ForegroundColor Green
}

function Show-Help {
    Write-Host @"
Pepakura Next Build Script
==========================

Usage:
  .\build.ps1 [-Target <all|core|wasm|desktop|web|addons>] [-Release]

Examples:
  .\build.ps1                    # Build everything (debug)
  .\build.ps1 -Target core       # Build only core
  .\build.ps1 -Release           # Build everything (release)
  .\build.ps1 -Target wasm -Release  # Build WASM (release)

Targets:
  all      - Build all components (default)
  core     - Build Rust core library
  wasm     - Build WASM module
  desktop  - Build Tauri desktop application
  web      - Build web application
  addons   - Build addons framework

"@ -ForegroundColor Cyan
}

# Main
if ($Help) {
    Show-Help
    exit 0
}

Write-Host ""
Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║          Pepakura Next - Build Script v4.0              ║" -ForegroundColor Cyan
Write-Host "║          Hybrid Architecture: Web-First + Bridge        ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""
Write-Host "Configuration: $Config" -ForegroundColor Yellow
Write-Host "Target: $Target" -ForegroundColor Yellow
Write-Host ""

try {
    switch ($Target) {
        "all" {
            Build-Core
            Build-Wasm
            Build-Addons
            Build-Desktop
            Build-Web
        }
        "core" {
            Build-Core
        }
        "wasm" {
            Build-Wasm
        }
        "desktop" {
            Build-Desktop
        }
        "web" {
            Build-Web
        }
        "addons" {
            Build-Addons
        }
    }
    
    Write-Host ""
    Write-Header "BUILD COMPLETED SUCCESSFULLY"
    Write-Host ""
}
catch {
    Write-Host ""
    Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Red
    Write-Host "║                    BUILD FAILED                         ║" -ForegroundColor Red
    Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Red
    Write-Host ""
    Write-Host "Error: $_" -ForegroundColor Red
    Write-Host ""
    exit 1
}
finally {
    Set-Location $ProjectRoot
}
