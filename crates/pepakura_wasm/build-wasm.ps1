#!/usr/bin/env pwsh
# Build script for Pepakura WASM

param(
    [string]$Target = "web",
    [switch]$Release
)

$ErrorActionPreference = "Stop"

Write-Host "Building Pepakura WASM for target: $Target" -ForegroundColor Cyan

if (-not (Get-Command "wasm-pack" -ErrorAction SilentlyContinue)) {
    Write-Error "wasm-pack is not installed. Install with: cargo install wasm-pack"
    exit 1
}

$Args = @("build")
if ($Release) {
    $Args += "--release"
}
$Args += "--target", $Target

Write-Host "Running: wasm-pack $Args" -ForegroundColor Yellow
& wasm-pack @Args

if ($LASTEXITCODE -ne 0) {
    Write-Error "Build failed"
    exit $LASTEXITCODE
}

Write-Host "Build successful!" -ForegroundColor Green
Write-Host "Output directory: pkg/" -ForegroundColor Green
Write-Host "To test the example, run a local HTTP server in this directory and open examples/index.html" -ForegroundColor Cyan