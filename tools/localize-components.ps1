$ErrorActionPreference = "Stop"
$Root = "D:\Dev\pepakura-next\packages\ui-desktop"
$vueFiles = Get-ChildItem "$Root\src" -Recurse -Filter "*.vue" -Depth 4

Write-Host "`n🌍 Локализация компонентов...`n" -ForegroundColor Magenta

# Словарь замен: английский текст → ключ локализации
$translations = @{
  "Pepakura Next" = 'app.title'
  "Advanced 3D Unfolding Tool" = 'app.description'
  "Quick Actions" = 'panels.quickActions'
  "Open Project" = 'buttons.openProject'
  "New Project" = 'buttons.newProject'
  "Import Model" = 'buttons.importModel'
  "Settings" = 'buttons.settings'
  "Recent Projects" = 'panels.recentProjects'
  "AI Settings" = 'panels.aiSettings'
  "Provider Selection" = 'ai.providerSelection'
  "Ollama" = 'ai.providers.ollama'
  "OpenAI" = 'ai.providers.openai'
  "Custom" = 'ai.providers.custom'
  "Endpoint:" = 'ai.endpoint'
  "Model:" = 'ai.model'
  "Temperature:" = 'ai.temperature'
  "Maximum number of tokens:" = 'ai.maxTokens'
  "Save" = 'buttons.save'
  "Cancel" = 'buttons.cancel'
  "Close" = 'buttons.close'
  "Export" = 'buttons.export'
  "Help" = 'buttons.help'
  "Undo" = 'toolbar.undo'
  "Redo" = 'toolbar.redo'
  "Zoom In" = 'toolbar.zoomIn'
  "Zoom Out" = 'toolbar.zoomOut'
  "Fit to View" = 'toolbar.fitToView'
  "Rotate" = 'toolbar.rotate'
  "Flip" = 'toolbar.flip'
  "Loading..." = 'messages.loading'
  "Error" = 'messages.error'
  "Success" = 'messages.success'
  "Search..." = 'placeholders.search'
}

$fixedCount = 0
$filesProcessed = 0

foreach ($file in $vueFiles) {
  $content = Get-Content $file.FullName -Raw
  $originalContent = $content
  $filesProcessed++
  
  # Заменить тексты на $t('key')
  foreach ($pair in $translations.GetEnumerator()) {
    $english = [regex]::Escape($pair.Key)
    $key = $pair.Value
    
    # Заменить в шаблоне: >Текст< → >{{ $t('key') }}<
    $content = $content -replace ">$([regex]::Escape($pair.Key))<", ">{{ `$t('$key') }}<"
    
    # Заменить в атрибутах placeholder
    $content = $content -replace "placeholder=`"$([regex]::Escape($pair.Key))`"", ":placeholder=`"`$t('$key')`""
    $content = $content -replace 'placeholder=''$([regex]::Escape($pair.Key))''', ":placeholder='`$t(''$key'')'"
  }
  
  # Добавить использование i18n в <script setup> если отсутствует
  if ($content -match "<script setup" -and $content -notmatch "const \{ t \} = useI18n") {
    $content = $content -replace "(<script setup[^>]*>)", "`$1`nimport { useI18n } from 'vue-i18n'`nconst { t } = useI18n()`n"
  }
  
  if ($content -ne $originalContent) {
    Set-Content -Path $file.FullName -Value $content -Encoding UTF8
    $fixedCount++
    Write-Host "✅ Локализован: $($file.Name)" -ForegroundColor Green
  }
}

Write-Host "`n✨ Локализация завершена!" -ForegroundColor Magenta
Write-Host "  Обработано файлов: $filesProcessed" -ForegroundColor Gray
Write-Host "  Локализовано: $fixedCount" -ForegroundColor Green
Write-Host "`n💡 Перезапустите сервер (Ctrl+C, затем pnpm dev) для применения изменений`n" -ForegroundColor Cyan
