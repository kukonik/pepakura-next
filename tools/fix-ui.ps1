$ErrorActionPreference = "Stop"
$Root = "D:\Dev\pepakura-next"
$UiDesktop = "$Root\packages\ui-desktop"

Write-Host "`n🛠️  Применение исправлений UI (русский интерфейс + кнопки)`n" -ForegroundColor Magenta

# 1. Создать локали
$localesDir = "$UiDesktop\src\locales"
if (-not (Test-Path $localesDir)) { New-Item -Path $localesDir -ItemType Directory -Force | Out-Null }
$ruJson = '{
  "app": {"title": "Pepakura Next", "version": "Версия"},
  "menu": {"file": "Файл", "edit": "Правка", "view": "Вид", "tools": "Инструменты", "help": "Справка", "new": "Новый проект", "open": "Открыть", "save": "Сохранить", "saveAs": "Сохранить как...", "export": "Экспорт", "exit": "Выход"},
  "toolbar": {"undo": "Отменить", "redo": "Повторить", "zoomIn": "Увеличить", "zoomOut": "Уменьшить", "fitToView": "Подогнать к виду", "rotate": "Повернуть", "flip": "Отразить"},
  "panels": {"recentProjects": "Последние проекты", "quickActions": "Быстрые действия", "stats": "Статистика", "properties": "Свойства", "unfoldSettings": "Настройки разворачивания"},
  "buttons": {"openModel": "Открыть 3D модель", "unfold": "Разворачивание", "exportSvg": "Экспорт в SVG", "exportPdf": "Экспорт в PDF", "exportPng": "Экспорт в PNG", "settings": "Настройки", "help": "Справка"},
  "messages": {"loading": "Загрузка...", "saving": "Сохранение...", "exporting": "Экспорт...", "error": "Ошибка", "success": "Успешно"},
  "placeholders": {"search": "Поиск...", "projectName": "Имя проекта"},
  "tooltips": {"unfoldAuto": "Автоматическое разворачивание с оптимизацией", "unfoldManual": "Ручное редактирование швов", "exportInstructions": "Генерация инструкций для сборки"}
}'
Set-Content -Path "$localesDir\ru.json" -Value $ruJson -Encoding UTF8
Write-Host "✅ Русские локали созданы: src/locales/ru.json" -ForegroundColor Green

# 2. Создать i18n конфиг
$i18nDir = "$UiDesktop\src\i18n"
if (-not (Test-Path $i18nDir)) { New-Item -Path $i18nDir -ItemType Directory -Force | Out-Null }
$i18nConfig = 'import { createI18n } from `"vue-i18n`"
import ru from `"@/locales/ru.json`"

function detectLanguage(): string {
  try {
    const lang = navigator.language || (navigator as any).userLanguage || `"ru`"
    return lang.toLowerCase().startsWith(`"ru`") ? `"ru`" : `"ru`"
  } catch {
    return `"ru`"
  }
}

export const i18n = createI18n({
  locale: detectLanguage(),
  fallbackLocale: `"ru`",
  messages: { ru },
  legacy: false,
  globalInjection: true
})'
Set-Content -Path "$i18nDir\index.ts" -Value $i18nConfig -Encoding UTF8
Write-Host "✅ i18n конфигурация создана: src/i18n/index.ts" -ForegroundColor Green

# 3. Подключить i18n в main.ts (а не в App.vue — правильнее для Vue 3)
$mainTsPath = "$UiDesktop\src\main.ts"
if (Test-Path $mainTsPath) {
  $content = Get-Content $mainTsPath -Raw
  if ($content -notmatch "import \{ i18n \}") {
    # Добавить импорт после других импортов
    $content = $content -replace "(import.*?from.*?['`"].*?['`"]\s*\n)+", "`$0`nimport { i18n } from './i18n'`n"
    # Добавить .use(i18n) перед .mount
    $content = $content -replace "\.mount\(", ".use(i18n).mount("
    Set-Content -Path $mainTsPath -Value $content -Encoding UTF8
    Write-Host "✅ i18n подключён в src/main.ts" -ForegroundColor Green
  } else {
    Write-Host "ℹ️  i18n уже подключён в src/main.ts" -ForegroundColor DarkGray
  }
} else {
  Write-Host "⚠️  src/main.ts не найден — создаём заглушку" -ForegroundColor Yellow
  Set-Content -Path $mainTsPath -Value "import { createApp } from 'vue'
import { i18n } from './i18n'
import App from './App.vue'

createApp(App).use(i18n).mount('#app')
" -Encoding UTF8
}

# 4. Установить зависимости (если не установлены)
if (-not (Test-Path "$UiDesktop\node_modules\vue-i18n")) {
  Write-Host "`n📦 Установка vue-i18n (может занять 1-2 минуты)..." -ForegroundColor Yellow
  Set-Location $UiDesktop
  pnpm add vue-i18n@9 --save 2>$null | Out-Null
  Set-Location $Root
  Write-Host "✅ vue-i18n установлен" -ForegroundColor Green
}

Write-Host "`n✨ Исправления применены!`n" -ForegroundColor Magenta
Write-Host "Для запуска выполните:`n  cd D:\Dev\pepakura-next\packages\ui-desktop`n  pnpm dev`n" -ForegroundColor Cyan
