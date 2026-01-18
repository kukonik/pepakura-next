param(
    [string]$DownloadUrl = "https://ollama.com/download/OllamaSetup.exe",
    [string]$InstallerPath = "$env:TEMP\OllamaSetup.exe"
)

Write-Host "Downloading Ollama installer..." -ForegroundColor Cyan
Invoke-WebRequest -Uri $DownloadUrl -OutFile $InstallerPath

if (-not (Test-Path $InstallerPath)) {
    Write-Error "Failed to download Ollama installer."
    exit 1
}

Write-Host "Running Ollama installer..." -ForegroundColor Cyan
Start-Process -FilePath $InstallerPath -ArgumentList "/S" -Wait

Write-Host "Ollama installation finished. You may need to log out and log in again." -ForegroundColor Green
