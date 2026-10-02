$ErrorActionPreference = "Stop"
$Root = "D:\Dev\pepakura-next\packages\ui-desktop"
$vueFiles = Get-ChildItem "$Root\src" -Recurse -Filter "*.vue" -Depth 3

Write-Host "`n🔧 Исправление локализации в компонентах...`n" -ForegroundColor Magenta

$fixedCount = 0

foreach ($file in $vueFiles) {
  $content = Get-Content $file.FullName -Raw
  $originalContent = $content
  
  # Если есть <script setup> и нет useI18n
  if ($content -match "<script setup" -and $content -notmatch "useI18n") {
    # Добавить импорт и использование useI18n
    $content = $content -replace "(<script setup[^>]*>)", "`$1`nimport { useI18n } from 'vue-i18n'`nconst { t } = useI18n()`n"
  }
  
  # Если есть <script> (options API) и нет i18n
  if ($content -match "<script[^>]*>" -and $content -notmatch "setup\s*\(" -and $content -notmatch "i18n") {
    # Добавить i18n в export default
    $content = $content -replace "(export default \{)", "`$1`n  i18n,`n"
  }
  
  if ($content -ne $originalContent) {
    Set-Content -Path $file.FullName -Value $content -Encoding UTF8
    $fixedCount++
    Write-Host "✅ Исправлен: $($file.Name)" -ForegroundColor Green
  }
}

Write-Host "`n✨ Исправление завершено!" -ForegroundColor Magenta
Write-Host "  Исправлено компонентов: $fixedCount" -ForegroundColor Green
Write-Host "`n💡 Перезапустите сервер: pnpm dev`n" -ForegroundColor Cyan
