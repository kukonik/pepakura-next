# ============================================
# PEAPKURA-NEXT MONOREPO SETUP SCRIPT
# ============================================

# 1. Проверка и подготовка среды
Write-Host "`n🔍 Проверяю среду..." -ForegroundColor Cyan

# Проверяем наличие pnpm
if (!(Get-Command pnpm -ErrorAction SilentlyContinue)) {
    Write-Host "❌ pnpm не установлен!" -ForegroundColor Red
    Write-Host "Установите: npm install -g pnpm" -ForegroundColor Yellow
    exit 1
}

# 2. Создаем бэкап важных файлов
Write-Host "`n💾 Создаю резервную копию..." -ForegroundColor Cyan
$backupDir = "D:\Dev\pepakura-next\_backup_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
New-Item -ItemType Directory -Path $backupDir -Force | Out-Null

# Копируем Rust код и исходники
$itemsToBackup = @(
    "ui-desktop\src-tauri",
    "ui-desktop\src",
    "ui-desktop\package.json",
    "ui-desktop\vite.config.js"
)

foreach ($item in $itemsToBackup) {
    $source = "D:\Dev\pepakura-next\$item"
    if (Test-Path $source) {
        Copy-Item -Path $source -Destination "$backupDir\$item" -Recurse -Force
        Write-Host "  ✅ $item скопирован" -ForegroundColor Green
    }
}

# 3. Удаляем старые зависимости
Write-Host "`n🧹 Очищаю старые зависимости..." -ForegroundColor Cyan
Get-ChildItem "D:\Dev\pepakura-next" -Recurse -Directory -Name "node_modules" | ForEach-Object {
    $path = "D:\Dev\pepakura-next\$_"
    Remove-Item -Path $path -Recurse -Force -ErrorAction SilentlyContinue
    Write-Host "  ✅ Удалено: $_" -ForegroundColor Green
}

# Удаляем lock-файлы
@("package-lock.json", "yarn.lock", "pnpm-lock.yaml") | ForEach-Object {
    if (Test-Path "D:\Dev\pepakura-next\$_") {
        Remove-Item "D:\Dev\pepakura-next\$_" -Force
    }
}

# 4. Создаем структуру монорепозитория
Write-Host "`n📁 Создаю структуру монорепозитория..." -ForegroundColor Cyan

# Создаем директории
$directories = @(
    "apps\web\src",
    "apps\web\public",
    "apps\desktop\src",
    "packages\shared\src\components",
    "packages\shared\src\composables",
    "packages\shared\src\stores",
    "packages\shared\src\i18n\locales",
    "packages\shared\src\types",
    "packages\shared\src\utils"
)

foreach ($dir in $directories) {
    New-Item -ItemType Directory -Path "D:\Dev\pepakura-next\$dir" -Force | Out-Null
}

# 5. Копируем существующий код на новые места
Write-Host "`n🔄 Переношу существующий код..." -ForegroundColor Cyan

# A) Копируем Rust код Tauri
if (Test-Path "D:\Dev\pepakura-next\ui-desktop\src-tauri") {
    Copy-Item -Path "D:\Dev\pepakura-next\ui-desktop\src-tauri" -Destination "D:\Dev\pepakura-next\apps\desktop\" -Recurse -Force
    Write-Host "  ✅ Rust код Tauri перенесен" -ForegroundColor Green
}

# B) Копируем исходный код Vue (будет основой для shared и web)
if (Test-Path "D:\Dev\pepakura-next\ui-desktop\src") {
    # Копируем для web приложения
    Copy-Item -Path "D:\Dev\pepakura-next\ui-desktop\src\*" -Destination "D:\Dev\pepakura-next\apps\web\src\" -Recurse -Force -Exclude "node_modules"
    
    # Находим Vue компоненты для shared
    $vueFiles = Get-ChildItem "D:\Dev\pepakura-next\ui-desktop\src" -Recurse -Filter "*.vue"
    foreach ($file in $vueFiles) {
        # Пример: переносим компоненты, но не App.vue
        if ($file.Name -ne "App.vue") {
            $relativePath = $file.FullName.Substring("D:\Dev\pepakura-next\ui-desktop\src".Length)
            $destDir = "D:\Dev\pepakura-next\packages\shared\src\components" + (Split-Path $relativePath)
            New-Item -ItemType Directory -Path $destDir -Force | Out-Null
            Copy-Item $file.FullName -Destination $destDir -Force
        }
    }
    Write-Host "  ✅ Vue код перенесен" -ForegroundColor Green
}

# 6. Создаем корневой package.json
Write-Host "`n📦 Создаю корневой package.json..." -ForegroundColor Cyan
$rootPackageJson = @{
    name = "pepakura-next-monorepo"
    private = $true
    version = "0.1.0"
    packageManager = "pnpm@9.0.0"
    scripts = @{
        "dev:web" = "pnpm --filter pepakura-web dev"
        "build:web" = "pnpm --filter pepakura-web build"
        "dev:desktop" = "pnpm --filter pepakura-desktop tauri dev"
        "build:desktop" = "pnpm --filter pepakura-desktop tauri build"
    }
    workspaces = @(
        "packages/shared"
        "apps/web"
        "apps/desktop"
    )
} | ConvertTo-Json -Depth 10

Set-Content -Path "D:\Dev\pepakura-next\package.json" -Value $rootPackageJson -Encoding UTF8
Write-Host "  ✅ Корневой package.json создан" -ForegroundColor Green

# 7. Создаем package.json для shared
Write-Host "`n📦 Создаю shared package.json..." -ForegroundColor Cyan
$sharedPackageJson = @{
    name = "pepakura-shared"
    version = "0.1.0"
    type = "module"
    main = "./src/index.ts"
    types = "./src/index.ts"
    exports = @{
        "." = @{
            import = "./src/index.ts"
        }
        "./components/*" = "./src/components/*"
        "./composables/*" = "./src/composables/*"
        "./i18n" = "./src/i18n/index.ts"
    }
    peerDependencies = @{
        vue = "^3.3.0"
        three = "^0.168.0"
    }
} | ConvertTo-Json -Depth 10

Set-Content -Path "D:\Dev\pepakura-next\packages\shared\package.json" -Value $sharedPackageJson -Encoding UTF8

# Создаем файл index.ts для shared
$sharedIndex = @"
// Основной экспорт shared пакета
export { default as ModelViewer } from './components/ModelViewer.vue'
export { default as PepakuraLayout } from './components/PepakuraLayout.vue'
export { useThreeJsScene } from './composables/useThreeJsScene'
export { i18n } from './i18n'
export type { MeshData } from './types/model'
"@

Set-Content -Path "D:\Dev\pepakura-next\packages\shared\src\index.ts" -Value $sharedIndex -Encoding UTF8
Write-Host "  ✅ Shared пакет настроен" -ForegroundColor Green

# 8. Создаем package.json для web приложения
Write-Host "`n📦 Создаю web package.json..." -ForegroundColor Cyan
$webPackageJson = @{
    name = "pepakura-web"
    private = $true
    type = "module"
    scripts = @{
        dev = "vite"
        build = "vite build"
        preview = "vite preview"
    }
    dependencies = @{
        vue = "^3.3.0"
        "pepakura-shared" = "workspace:*"
        "vue-i18n" = "^9.9.0"
        three = "^0.168.0"
    }
    devDependencies = @{
        "@vitejs/plugin-vue" = "^5.0.0"
        vite = "^5.0.0"
        "@types/three" = "^0.168.0"
    }
} | ConvertTo-Json -Depth 10

Set-Content -Path "D:\Dev\pepakura-next\apps\web\package.json" -Value $webPackageJson -Encoding UTF8

# Создаем vite.config.js для web
$webViteConfig = @"
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { fileURLToPath, URL } from 'url'
import path from 'path'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@shared': path.resolve(__dirname, '../../packages/shared/src')
    }
  },
  server: {
    port: 5173,
    host: true
  }
})
"@

Set-Content -Path "D:\Dev\pepakura-next\apps\web\vite.config.js" -Value $webViteConfig -Encoding UTF8

# Создаем index.html для web
$webIndexHtml = @"
<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Pepakura Next | Веб-версия</title>
</head>
<body>
    <div id="app"></div>
    <script type="module" src="/src/main.js"></script>
</body>
</html>
"@

Set-Content -Path "D:\Dev\pepakura-next\apps\web\index.html" -Value $webIndexHtml -Encoding UTF8

# Создаем main.js для web
$webMainJs = @"
import { createApp } from 'vue'
import App from './App.vue'
import { i18n } from 'pepakura-shared'

const app = createApp(App)
app.use(i18n)
app.mount('#app')
"@

Set-Content -Path "D:\Dev\pepakura-next\apps\web\src\main.js" -Value $webMainJs -Encoding UTF8
Write-Host "  ✅ Web приложение настроено" -ForegroundColor Green

# 9. Создаем package.json для desktop приложения
Write-Host "`n📦 Создаю desktop package.json..." -ForegroundColor Cyan
$desktopPackageJson = @{
    name = "pepakura-desktop"
    private = $true
    scripts = @{
        tauri = "tauri"
    }
    dependencies = @{
        "@tauri-apps/api" = "latest"
        "@tauri-apps/plugin-dialog" = "latest"
        "pepakura-shared" = "workspace:*"
        vue = "^3.3.0"
    }
    devDependencies = @{
        "@tauri-apps/cli" = "latest"
    }
} | ConvertTo-Json -Depth 10

Set-Content -Path "D:\Dev\pepakura-next\apps\desktop\package.json" -Value $desktopPackageJson -Encoding UTF8

# 10. Настраиваем tauri.conf.json
Write-Host "`n⚙️  Настраиваю Tauri конфигурацию..." -ForegroundColor Cyan
if (Test-Path "D:\Dev\pepakura-next\apps\desktop\src-tauri\tauri.conf.json") {
    $tauriConfig = Get-Content "D:\Dev\pepakura-next\apps\desktop\src-tauri\tauri.conf.json" -Raw | ConvertFrom-Json
    
    # Обновляем пути
    $tauriConfig.build.beforeDevCommand = "pnpm --filter pepakura-web dev"
    $tauriConfig.build.devPath = "http://localhost:5173"
    $tauriConfig.build.beforeBuildCommand = "pnpm --filter pepakura-web build"
    $tauriConfig.build.distDir = "../../apps/web/dist"
    
    $tauriConfig | ConvertTo-Json -Depth 10 | Set-Content "D:\Dev\pepakura-next\apps\desktop\src-tauri\tauri.conf.json" -Encoding UTF8
    Write-Host "  ✅ Tauri конфиг обновлен" -ForegroundColor Green
} else {
    Write-Host "  ⚠️  Tauri конфиг не найден, создаю новый..." -ForegroundColor Yellow
    
    $newTauriConfig = @{
        build = @{
            beforeDevCommand = "pnpm --filter pepakura-web dev"
            devPath = "http://localhost:5173"
            beforeBuildCommand = "pnpm --filter pepakura-web build"
            distDir = "../../apps/web/dist"
        }
        app = @{
            withGlobalTauri = $false
        }
        tauri = @{
            allowlist = @{
                all = $false
                dialog = @{
                    all = $true
                    open = $true
                }
                shell = @{
                    all = $false
                    open = $true
                }
            }
            bundle = @{
                icon = @()
            }
            windows = @(
                @{
                    title = "Pepakura Next"
                    width = 1400
                    height = 900
                }
            )
        }
    } | ConvertTo-Json -Depth 10
    
    Set-Content -Path "D:\Dev\pepakura-next\apps\desktop\src-tauri\tauri.conf.json" -Value $newTauriConfig -Encoding UTF8
    Write-Host "  ✅ Новый Tauri конфиг создан" -ForegroundColor Green
}

# 11. Настраиваем i18n
Write-Host "`n🌍 Настраиваю i18n..." -ForegroundColor Cyan

# Создаем базовые файлы переводов
$ruTranslations = @{
    app = @{
        title = "Pepakura Next"
        loading = "Загрузка..."
    }
    buttons = @{
        import = "Импорт 3D"
        export = "Экспорт"
        save = "Сохранить"
    }
} | ConvertTo-Json -Depth 10

$enTranslations = @{
    app = @{
        title = "Pepakura Next"
        loading = "Loading..."
    }
    buttons = @{
        import = "Import 3D"
        export = "Export"
        save = "Save"
    }
} | ConvertTo-Json -Depth 10

Set-Content -Path "D:\Dev\pepakura-next\packages\shared\src\i18n\locales\ru.json" -Value $ruTranslations -Encoding UTF8
Set-Content -Path "D:\Dev\pepakura-next\packages\shared\src\i18n\locales\en.json" -Value $enTranslations -Encoding UTF8

# Создаем i18n конфигурацию
$i18nConfig = @"
import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import ru from './locales/ru.json'

export const i18n = createI18n({
  legacy: false,
  locale: 'ru',
  fallbackLocale: 'en',
  messages: { en, ru }
})

export async function loadLocaleMessages(locale: string) {
  try {
    const messages = await import(\`./locales/\${locale}.json\`)
    i18n.global.setLocaleMessage(locale, messages.default)
    return messages.default
  } catch (error) {
    console.warn(\`Locale \${locale} not found, using fallback\`)
    return null
  }
}
"@

Set-Content -Path "D:\Dev\pepakura-next\packages\shared\src\i18n\index.ts" -Value $i18nConfig -Encoding UTF8
Write-Host "  ✅ i18n настроен (ru/en)" -ForegroundColor Green

# 12. Создаем базовый TypeScript тип для модели
Write-Host "`n📐 Создаю типы TypeScript..." -ForegroundColor Cyan
$modelType = @"
export interface MeshData {
  name: string
  vertices: number[]
  normals?: number[]
  triangles: Array<{ vertices: [number, number, number] }>
  materials?: Array<{
    diffuse: { r: number; g: number; b: number }
    specular: { r: number; g: number; b: number }
    shininess: number
  }>
}

export interface ModelStats {
  vertices: number
  triangles: number
  parts: number
}
"@

Set-Content -Path "D:\Dev\pepakura-next\packages\shared\src\types\model.ts" -Value $modelType -Encoding UTF8
Write-Host "  ✅ Типы созданы" -ForegroundColor Green

# 13. Устанавливаем зависимости
Write-Host "`n📥 Устанавливаю зависимости pnpm..." -ForegroundColor Cyan
Set-Location "D:\Dev\pepakura-next"
pnpm install
if ($LASTEXITCODE -eq 0) {
    Write-Host "  ✅ Зависимости установлены успешно" -ForegroundColor Green
} else {
    Write-Host "  ⚠️  Были ошибки при установке зависимостей" -ForegroundColor Yellow
}

# 14. Создаем App.vue для web (базовый)
Write-Host "`n🎨 Создаю базовый App.vue..." -ForegroundColor Cyan
$appVueContent = @"
<template>
  <div id="app">
    <h1>{{ \$t('app.title') }}</h1>
    <p>{{ \$t('app.loading') }}</p>
    <button @click="test">{{ \$t('buttons.import') }}</button>
    
    <!-- TODO: Вставить ваш интерфейс из paste.txt -->
    <div class="placeholder">
      Интерфейс будет здесь. Перенесите HTML/CSS из вашего файла.
    </div>
  </div>
</template>

<script setup>
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

const test = () => {
  console.log('Тест i18n:', t('buttons.import'))
  alert('Работает! Теперь перенесите ваш интерфейс в этот компонент.')
}
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Inter', sans-serif;
  background: linear-gradient(155deg, #0b1120 0%, #1a202c 100%);
  color: #e2e8f0;
  min-height: 100vh;
}

#app {
  padding: 2rem;
}

.placeholder {
  margin-top: 2rem;
  padding: 2rem;
  background: rgba(255, 255, 255, 0.05);
  border-radius: 10px;
  border: 1px dashed rgba(255, 255, 255, 0.1);
}
</style>
"@

Set-Content -Path "D:\Dev\pepakura-next\apps\web\src\App.vue" -Value $appVueContent -Encoding UTF8
Write-Host "  ✅ App.vue создан" -ForegroundColor Green

# 15. Финальный отчет
Write-Host "`n" + "="*50 -ForegroundColor Cyan
Write-Host "🏁 МОНОРЕПОЗИТОРИЙ УСПЕШНО СОЗДАН!" -ForegroundColor Green
Write-Host "="*50 -ForegroundColor Cyan

Write-Host "`n📁 СТРУКТУРА ПРОЕКТА:" -ForegroundColor Yellow
Write-Host "D:\Dev\pepakura-next\" -ForegroundColor White
Write-Host "├── apps/" -ForegroundColor White
Write-Host "│   ├── web/           # Веб-приложение (Vite + Vue)" -ForegroundColor White
Write-Host "│   └── desktop/       # Десктопное приложение (Tauri)" -ForegroundColor White
Write-Host "├── packages/" -ForegroundColor White
Write-Host "│   └── shared/        # Общий код (компоненты, i18n, типы)" -ForegroundColor White
Write-Host "├── _backup_.../       # Резервная копия вашего кода" -ForegroundColor White
Write-Host "└── package.json       # Корневой конфиг pnpm workspaces" -ForegroundColor White

Write-Host "`n🚀 КОМАНДЫ ДЛЯ ЗАПУСКА:" -ForegroundColor Yellow
Write-Host "1. Запуск веб-приложения:" -ForegroundColor White
Write-Host "   pnpm dev:web" -ForegroundColor Green
Write-Host "   → Откроется на http://localhost:5173" -ForegroundColor Gray

Write-Host "`n2. Запуск десктопного приложения (в ОТДЕЛЬНОМ терминале):" -ForegroundColor White
Write-Host "   pnpm dev:desktop" -ForegroundColor Green
Write-Host "   → Tauri запустит окно, подключившись к веб-серверу" -ForegroundColor Gray

Write-Host "`n3. Сборка для production:" -ForegroundColor White
Write-Host "   pnpm build:web      # Сборка веб-версии" -ForegroundColor Green
Write-Host "   pnpm build:desktop  # Сборка десктопной версии" -ForegroundColor Green

Write-Host "`n📝 СЛЕДУЮЩИЕ ШАГИ:" -ForegroundColor Yellow
Write-Host "1. Откройте файл: apps\web\src\App.vue" -ForegroundColor White
Write-Host "2. Замените его содержимое на ваш интерфейс из paste.txt" -ForegroundColor White
Write-Host "3. Выделите общие компоненты в packages\shared\src\components\" -ForegroundColor White
Write-Host "4. Настройте импорт Three.js в shared/composables/" -ForegroundColor White

Write-Host "`n⚠️  РЕЗЕРВНАЯ КОПИЯ:" -ForegroundColor Yellow
Write-Host "Ваш исходный код сохранен в: $backupDir" -ForegroundColor White

Write-Host "`n✅ Готово! Проект организован как профессиональный монорепозиторий." -ForegroundColor Green