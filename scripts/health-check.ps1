# Pepakura Next - Health Check Script
# Проверяет окружение для разработки

Write-Host "=== Pepakura Next Health Check ===" -ForegroundColor Cyan

$errors = @()
$warnings = @()

# Check Rust
Write-Host "`n[1/8] Checking Rust..." -ForegroundColor Yellow
if (Get-Command cargo -ErrorAction SilentlyContinue) {
    $rustVersion = cargo --version 2>&1
    Write-Host "  ✓ $rustVersion" -ForegroundColor Green
    
    # Check for required components
    $components = rustup component list 2>&1
    if ($components -notlike "*clippy (installed)*") {
        $warnings += "clippy not installed. Run: rustup component add clippy"
    }
    if ($components -notlike "*rustfmt (installed)*") {
        $warnings += "rustfmt not installed. Run: rustup component add rustfmt"
    }
} else {
    $errors += "Rust not found. Install from https://rustup.rs/"
}

# Check Node.js
Write-Host "`n[2/8] Checking Node.js..." -ForegroundColor Yellow
if (Get-Command node -ErrorAction SilentlyContinue) {
    $nodeVersion = node --version
    Write-Host "  ✓ Node.js: $nodeVersion" -ForegroundColor Green
    
    # Check version >= 20
    $majorVersion = [int]($nodeVersion -replace 'v(\d+)\..*', '$1')
    if ($majorVersion -lt 20) {
        $warnings += "Node.js version < 20. Recommended: 20+"
    }
} else {
    $errors += "Node.js not found. Install from https://nodejs.org/"
}

# Check pnpm
Write-Host "`n[3/8] Checking pnpm..." -ForegroundColor Yellow
if (Get-Command pnpm -ErrorAction SilentlyContinue) {
    $pnpmVersion = pnpm --version
    Write-Host "  ✓ pnpm: $pnpmVersion" -ForegroundColor Green
} else {
    $errors += "pnpm not found. Run: npm install -g pnpm"
}

# Check core crate
Write-Host "`n[4/8] Checking pepakura_core..." -ForegroundColor Yellow
Set-Location $PSScriptRoot\..\crates\pepakura_core
if (Test-Path "Cargo.toml") {
    Write-Host "  ✓ Cargo.toml found" -ForegroundColor Green
    
    $checkResult = cargo check 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "  ✓ cargo check passed" -ForegroundColor Green
    } else {
        $errors += "cargo check failed"
    }
} else {
    $errors += "pepakura_core/Cargo.toml not found"
}

# Check frontend
Write-Host "`n[5/8] Checking frontend..." -ForegroundColor Yellow
Set-Location $PSScriptRoot\..\ui-desktop
if (Test-Path "package.json") {
    Write-Host "  ✓ package.json found" -ForegroundColor Green
    
    if (Test-Path "node_modules") {
        Write-Host "  ✓ node_modules installed" -ForegroundColor Green
    } else {
        $warnings += "node_modules not found. Run: pnpm install"
    }
} else {
    $errors += "ui-desktop/package.json not found"
}

# Check Tauri
Write-Host "`n[6/8] Checking Tauri..." -ForegroundColor Yellow
Set-Location $PSScriptRoot\..\src-tauri
if (Test-Path "Cargo.toml") {
    Write-Host "  ✓ Tauri Cargo.toml found" -ForegroundColor Green
    
    if (Test-Path "tauri.conf.json") {
        Write-Host "  ✓ tauri.conf.json found" -ForegroundColor Green
    } else {
        $errors += "tauri.conf.json not found"
    }
} else {
    $errors += "src-tauri/Cargo.toml not found"
}

# Check documentation
Write-Host "`n[7/8] Checking documentation..." -ForegroundColor Yellow
Set-Location $PSScriptRoot\..
$docs = @("README.md", "CHANGELOG.md", "PROMPTS.md", "LICENSE")
foreach ($doc in $docs) {
    if (Test-Path $doc) {
        Write-Host "  ✓ $doc" -ForegroundColor Green
    } else {
        $warnings += "$doc not found"
    }
}

# Check GitHub Actions
Write-Host "`n[8/8] Checking GitHub Actions..." -ForegroundColor Yellow
if (Test-Path ".github\workflows") {
    $workflows = Get-ChildItem ".github\workflows" -Filter "*.yml"
    Write-Host "  ✓ $($workflows.Count) workflows found" -ForegroundColor Green
    foreach ($wf in $workflows) {
        Write-Host "    - $($wf.Name)" -ForegroundColor Gray
    }
} else {
    $warnings += ".github/workflows not found"
}

# Summary
Write-Host "`n=== Summary ===" -ForegroundColor Cyan

if ($errors.Count -eq 0 -and $warnings.Count -eq 0) {
    Write-Host "✓ All checks passed! Ready for development." -ForegroundColor Green
    exit 0
}

if ($errors.Count -gt 0) {
    Write-Host "`nERRORS:" -ForegroundColor Red
    foreach ($err in $errors) {
        Write-Host "  ✗ $err" -ForegroundColor Red
    }
}

if ($warnings.Count -gt 0) {
    Write-Host "`nWARNINGS:" -ForegroundColor Yellow
    foreach ($warn in $warnings) {
        Write-Host "  ⚠ $warn" -ForegroundColor Yellow
    }
}

if ($errors.Count -gt 0) {
    Write-Host "`nPlease fix errors before continuing." -ForegroundColor Red
    exit 1
} else {
    Write-Host "`nWarnings can be fixed, but are recommended." -ForegroundColor Yellow
    exit 0
}
