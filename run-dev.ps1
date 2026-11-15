 $ErrorActionPreference = 'Continue'
 $root = $PSScriptRoot
if (-not $root) { $root = Split-Path -Parent $MyInvocation.MyCommand.Path }
Write-Host '=== Pepakura Next: dev launcher ==='
Stop-Process -Name 'pepakura-next' -Force -ErrorAction SilentlyContinue

 $psExe = 'powershell'
if (Get-Command pwsh -ErrorAction SilentlyContinue) { $psExe = 'pwsh' }

 $port = Get-NetTCPConnection -LocalPort 5173 -State Listen -ErrorAction SilentlyContinue
if ($port) {
  Write-Host '  [OK] Vite already listening on port 5173'
} else {
  Write-Host '  Starting Vite in separate window (A)...'
  Start-Process $psExe -ArgumentList '-NoExit','-ExecutionPolicy','Bypass','-Command',"Set-Location '$root\packages\ui-desktop'; pnpm dev"
  $ok = $false
  for ($k = 0; $k -lt 30; $k++) {
    Start-Sleep -Seconds 2
    if (Get-NetTCPConnection -LocalPort 5173 -State Listen -ErrorAction SilentlyContinue) { $ok = $true; break }
  }
  if ($ok) { Write-Host '  [OK] Vite is up on 5173' } else { Write-Host '  [FAIL] Vite did not start - check window A' }
}

Set-Location (Join-Path $root 'src-tauri')
Write-Host '  Starting application (cargo run, incremental build)...'
cargo run 2>&1 | ForEach-Object { "$_" } | Tee-Object -FilePath (Join-Path $root 'run-dev_app.log')
Write-Host '  Application exited. Log: run-dev_app.log'
