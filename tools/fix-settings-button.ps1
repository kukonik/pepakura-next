$ErrorActionPreference = "Stop"
$Root = "D:\Dev\pepakura-next\packages\ui-desktop"

Write-Host "`n🔧 Исправление кнопки 'Настройки'...`n" -ForegroundColor Magenta

# Найти все .vue файлы
$vueFiles = Get-ChildItem "$Root\src" -Recurse -Filter "*.vue" -Depth 4

$fixedCount = 0

foreach ($file in $vueFiles) {
  $content = Get-Content $file.FullName -Raw
  $originalContent = $content
  
  # Проверить, есть ли кнопка с текстом "Настройки" или $t('buttons.settings')
  if ($content -match "Настройки|`\$t\('buttons\.settings'\)") {
    
    # Если есть @click но нет метода
    if ($content -match "@click=`"([^`"]+)`"" -or $content -match '@click=''([^\']+)''') {
      $methodName = $matches[1]
      
      # Проверить, определён ли метод
      if ($content -notmatch "const $methodName\s*=" -and $content -notmatch "const $methodName\s*=\s*\(") {
        
        # Добавить метод навигации
        if ($content -match "<script setup") {
          # Composition API
          if ($content -notmatch "const router = useRouter") {
            $content = $content -replace "(<script setup[^>]*>)", "`$1`nimport { useRouter } from 'vue-router'`nconst router = useRouter()`n"
          }
          
          # Добавить метод goToSettings если его нет
          if ($content -notmatch "const goToSettings") {
            $content = $content -replace "(const router = useRouter\(\))", "`$1`nconst goToSettings = () => { router.push('/settings') }`n"
          }
          
          # Заменить @click на правильный метод
          $content = $content -replace "@click=`"$methodName`"", '@click="goToSettings"'
          $content = $content -replace "@click='$methodName'", "@click='goToSettings'"
        }
      }
    }
  }
  
  if ($content -ne $originalContent) {
    Set-Content -Path $file.FullName -Value $content -Encoding UTF8
    $fixedCount++
    Write-Host "✅ Исправлен: $($file.Name)" -ForegroundColor Green
  }
}

if ($fixedCount -eq 0) {
  Write-Host "⚠️  Кнопка 'Настройки' не найдена или уже исправлена" -ForegroundColor Yellow
} else {
  Write-Host "`n✨ Исправление завершено!" -ForegroundColor Magenta
  Write-Host "  Исправлено компонентов: $fixedCount" -ForegroundColor Green
  Write-Host "`n💡 Перезапустите сервер: pnpm dev`n" -ForegroundColor Cyan
}
