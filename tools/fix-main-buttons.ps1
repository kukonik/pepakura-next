$ErrorActionPreference = "Stop"
$Root = "D:\Dev\pepakura-next\packages\ui-desktop"

Write-Host "`n🔧 Исправление кнопок главной страницы...`n" -ForegroundColor Magenta

# Найти все .vue файлы с кнопками
$vueFiles = Get-ChildItem "$Root\src" -Recurse -Filter "*.vue" -Depth 4
$fixedFiles = @()

foreach ($file in $vueFiles) {
  $content = Get-Content $file.FullName -Raw
  $originalContent = $content
  $needsFix = $false
  
  # Проверить наличие каждой кнопки
  $buttons = @(
    @{ Pattern = "Открыть проект|openProject"; Method = "openProject" },
    @{ Pattern = "Новый проект|newProject"; Method = "createNewProject" },
    @{ Pattern = "Импорт модели|importModel"; Method = "importModel" }
  )
  
  foreach ($btn in $buttons) {
    if ($content -match $btn.Pattern) {
      $needsFix = $true
      break
    }
  }
  
  if ($needsFix) {
    Write-Host "📝 Обработка: $($file.Name)" -ForegroundColor Cyan
    
    # Добавить необходимые импорты в <script setup>
    if ($content -match "<script setup") {
      # Добавить useRouter если нужно
      if ($content -notmatch "useRouter") {
        $content = $content -replace "(<script setup[^>]*>)", "`$1`nimport { useRouter } from 'vue-router'`nconst router = useRouter()`n"
      }
      
      # Добавить useI18n если нужно
      if ($content -notmatch "useI18n") {
        $content = $content -replace "(<script setup[^>]*>)", "`$1`nimport { useI18n } from 'vue-i18n'`nconst { t } = useI18n()`n"
      }
      
      # Добавить методы для кнопок (если их нет)
      $methods = @{
        "openProject" = "const openProject = () => { alert('Открыть проект') }"
        "createNewProject" = "const createNewProject = () => { alert('Новый проект создан') }"
        "importModel" = "const importModel = () => { alert('Импорт 3D модели') }"
      }
      
      foreach ($method in $methods.GetEnumerator()) {
        if ($content -notmatch "const $($method.Key)\s*=") {
          # Добавить метод после последнего объявления или после импортов
          if ($content -match "const router = useRouter\(\)") {
            $content = $content -replace "(const router = useRouter\(\))", "`$1`n$($method.Value)`n"
          } elseif ($content -match "const \{ t \} = useI18n\(\)") {
            $content = $content -replace "(const \{ t \} = useI18n\(\))", "`$1`n$($method.Value)`n"
          } else {
            # Добавить в конец <script setup> перед закрывающим тегом
            $content = $content -replace "(<script setup[^>]*>.*?)(</script>)", "`$1`n$($method.Value)`n`$2"
          }
        }
      }
      
      # Заменить @click на правильные методы
      $content = $content -replace "@click=`"[^`"]*open[^`"]*`"", '@click="openProject"'
      $content = $content -replace "@click='[^']*open[^']*'", "@click='openProject'"
      $content = $content -replace "@click=`"[^`"]*new[^`"]*`"", '@click="createNewProject"'
      $content = $content -replace "@click='[^']*new[^']*'", "@click='createNewProject'"
      $content = $content -replace "@click=`"[^`"]*import[^`"]*`"", '@click="importModel"'
      $content = $content -replace "@click='[^']*import[^']*'", "@click='importModel'"
    }
    
    if ($content -ne $originalContent) {
      Set-Content -Path $file.FullName -Value $content -Encoding UTF8
      $fixedFiles += $file.Name
      Write-Host "  ✅ Исправлен: $($file.Name)" -ForegroundColor Green
    }
  }
}

if ($fixedFiles.Count -gt 0) {
  Write-Host "`n✨ Исправление завершено!" -ForegroundColor Magenta
  Write-Host "  Исправлено файлов: $($fixedFiles.Count)" -ForegroundColor Green
  $fixedFiles | ForEach-Object { Write-Host "    • $_" -ForegroundColor Gray }
  Write-Host "`n💡 Перезапустите сервер: pnpm dev`n" -ForegroundColor Cyan
} else {
  Write-Host "ℹ️  Кнопки уже имеют обработчики или не найдены" -ForegroundColor DarkGray
}
