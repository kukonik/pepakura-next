# ============================================================================
# Fix: Restore dev script in ui-web/package.json
# ============================================================================

 $ProjectRoot = "D:\Dev\pepakura-next"
 $UiWebPkg = "$ProjectRoot\packages\ui-web\package.json"

Write-Host "=== Восстановление package.json ===" -ForegroundColor Cyan

# Полная, правильная структура package.json для ui-web
 $CorrectPackage = @"
{
  ""name"": ""ui-web"",
  ""private"": true,
  ""version"": ""0.1.0"",
  ""type"": ""module"",
  ""scripts"": {
    ""dev"": ""vite --port 5174"",
    ""build"": ""vite build"",
    ""preview"": ""vite preview""
  },
  ""dependencies"": {
    ""@pepakura-next/shared"": ""workspace:*"",
    ""@pepakura-next/backend-api"": ""workspace:*"",
    ""vue"": ""^3.4.0"",
    ""pinia"": ""^2.1.7"",
    ""vue-router"": ""^4.2.5""
  },
  ""devDependencies"": {
    ""@vitejs/plugin-vue"": ""^5.0.0"",
    ""typescript"": ""^5.3.0"",
    ""vite"": ""^5.0.0""
  }
}
"@ | ConvertFrom-Json

# Запись
Set-Content -Path $UiWebPkg -Value ($CorrectPackage | ConvertTo-Json -Depth 10) -Encoding UTF8

Write-Host "   [OK] package.json переписан." -ForegroundColor Green

Write-Host "`n============================================================" -ForegroundColor Cyan
Write-Host "1. Выполни: pnpm install" -ForegroundColor Yellow
Write-Host "2. Затем: pnpm --filter ui-web dev" -ForegroundColor Yellow
Write-Host "============================================================" -ForegroundColor Cyan