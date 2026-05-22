param(
    [string]$Root = "D:\Dev\pepakura-next"
)

$ErrorActionPreference = "Stop"

Write-Host "=== Pepakura Next UI/Tauri audit ===" -ForegroundColor Cyan
Write-Host "Root: $Root"
Write-Host ""

function Check-Path($relative, [switch]$Required) {
    $full = Join-Path $Root $relative
    if (Test-Path $full) {
        Write-Host "[OK ] $relative" -ForegroundColor Green
    } else {
        $msg = "[MISS] $relative"
        if ($Required) {
            Write-Host $msg -ForegroundColor Red
        } else {
            Write-Host $msg -ForegroundColor Yellow
        }
    }
}

Write-Host "== Structure ==" -ForegroundColor Cyan
Check-Path "ui-desktop\package.json" -Required
Check-Path "ui-desktop\pnpm-lock.yaml"
Check-Path "src-tauri\Cargo.toml" -Required
Check-Path "tauri.conf.json"
Write-Host ""

Write-Host "== Frontend files ==" -ForegroundColor Cyan
Check-Path "ui-desktop\src\components\layout\ModelViewer.vue" -Required
Check-Path "ui-desktop\src\components\layout\Scene3D.vue" -Required
Check-Path "ui-desktop\src\components\layout\ViewerSection.vue" -Required
Check-Path "ui-desktop\src\composables\useImport3DModel.ts" -Required
Write-Host ""

Write-Host "== Toolchain versions ==" -ForegroundColor Cyan
try { node -v } catch { Write-Host "node not found" -ForegroundColor Red }
try { pnpm -v } catch { Write-Host "pnpm not found" -ForegroundColor Red }
try { cargo tauri -V } catch { Write-Host "cargo tauri not found" -ForegroundColor Yellow }
Write-Host ""

$pkgPath = Join-Path $Root "ui-desktop\package.json"
if (Test-Path $pkgPath) {
    Write-Host "== package.json check ==" -ForegroundColor Cyan
    $pkgJson = Get-Content $pkgPath -Raw | ConvertFrom-Json
    $deps = @{}
    if ($pkgJson.dependencies) { $deps = $pkgJson.dependencies }
    if ($deps."@tauri-apps/api") {
        Write-Host "[OK ] @tauri-apps/api in dependencies: $($deps.'@tauri-apps/api')" -ForegroundColor Green
    } else {
        Write-Host "[MISS] @tauri-apps/api not in dependencies" -ForegroundColor Red
    }
    Write-Host ""
}

Write-Host "== node_modules check ==" -ForegroundColor Cyan
$nmPath = Join-Path $Root "ui-desktop\node_modules"
if (Test-Path $nmPath) {
    Write-Host "[OK ] node_modules exists" -ForegroundColor Green
    $tauriApiPath = Join-Path $nmPath "@tauri-apps\api\package.json"
    if (Test-Path $tauriApiPath) {
        $apiJson = Get-Content $tauriApiPath -Raw | ConvertFrom-Json
        Write-Host "[OK ] @tauri-apps/api installed: $($apiJson.version)" -ForegroundColor Green
    } else {
        Write-Host "[MISS] @tauri-apps/api not installed in node_modules" -ForegroundColor Red
    }
} else {
    Write-Host "[MISS] ui-desktop\node_modules (run pnpm install in ui-desktop)" -ForegroundColor Red
}
Write-Host ""

Write-Host "== Tauri version (Rust) ==" -ForegroundColor Cyan
$cargoPath = Join-Path $Root "src-tauri\Cargo.toml"
if (Test-Path $cargoPath) {
    $cargo = Get-Content $cargoPath
    $tauriLine = $cargo | Where-Object { $_ -match 'tauri.*version' }
    if ($tauriLine) {
        Write-Host $tauriLine -ForegroundColor Green
        if ($tauriLine -match '\"1\.') {
            Write-Host "Detected Tauri 1.x -> use: import { invoke } from '@tauri-apps/api/tauri'" -ForegroundColor Yellow
        } elseif ($tauriLine -match '\"2\.') {
            Write-Host "Detected Tauri 2.x -> use: import { core } from '@tauri-apps/api'; await core.invoke(...)" -ForegroundColor Yellow
        }
    } else {
        Write-Host "No tauri dependency line found in Cargo.toml" -ForegroundColor Yellow
    }
}
Write-Host ""

Write-Host "== Suggested commands ==" -ForegroundColor Cyan
Write-Host "cd `"$Root\ui-desktop`""
Write-Host "pnpm install"
Write-Host "pnpm tauri dev"
Write-Host ""
Write-Host "Audit finished." -ForegroundColor Cyan
