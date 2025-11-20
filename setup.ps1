<#
.SYNOPSIS
    Автоматическая подготовка среды Pepakura Next под Electron+Vue, backend, мультиязык.
.DESCRIPTION
    Проверяет/устанавливает Node.js, npm, Python, Rust, Electron, Vue, собирает frontend, активирует backend.
    Выводит ярлык на рабочий стол для запуска GUI.
    Не требует копирования данных с Github.
.NOTES
    Запускать только из D:\Dev\pepakura-next; требуется PowerShell 7+
#>
$ErrorActionPreference = "Stop"
$root = "D:\Dev\pepakura-next"
$frontend = Join-Path $root "frontend\web"
$vueApp = Join-Path $frontend "vue-app"
$electronApp = Join-Path $frontend "electron-app"

function Install-IfMissing ($tool, $wingetId) {
    Write-Host "⏳ Проверка $tool..."
    if (-not (Get-Command $tool -ErrorAction SilentlyContinue)) {
        Write-Host "➕ Установка $tool..."
        winget install --id $wingetId --silent
    } else { Write-Host "✅ $tool уже установлен" }
}

Write-Host "🔍 Проверка Node.js, npm, Python, Rust..."
Install-IfMissing node "OpenJS.NodeJS.LTS"
Install-IfMissing npm "OpenJS.NodeJS.LTS"
Install-IfMissing python "Python.Python.3.11"
Install-IfMissing cargo "Rustlang.Rustup"

Write-Host "🔧 Установка Electron/CLI frontend-зависимостей..."
if (-not (Test-Path $vueApp))    { New-Item -ItemType Directory -Path $vueApp | Out-Null }
if (-not (Test-Path $electronApp)){ New-Item -ItemType Directory -Path $electronApp | Out-Null }

Set-Location $vueApp
if (-not (Test-Path "./package.json")) {
    Write-Host "➕ Инициализация Vue..."
    npx -y create-vue@latest .
    npm install vue-router vuex three svg.js vue-i18n electron --save
    Write-Host "✅ Vue-инфраструктура развернута"
}
Write-Host "🏗️ Сборка Vue-фронта..."
npm run build

Set-Location $electronApp
if (-not (Test-Path "./main.js")) {
    Write-Host "➕ Минимальный Electron main.js"
    "
    const { app, BrowserWindow } = require('electron')
    app.whenReady().then(() => {
        const mainWindow = new BrowserWindow({ width: 1280, height: 800 })
        mainWindow.loadFile('../vue-app/dist/index.html')
    });
    " | Set-Content main.js -Encoding UTF8
}
if (-not (Test-Path "./package.json")) {
    npm init -y
    npm install electron --save
}
Write-Host "🏗️ Сборка Electron wrapper..."

Set-Location $vueApp
Write-Host "🔁 Проверка локалей..."
$localeDir = Join-Path $vueApp "src\locales"
if (-not (Test-Path $localeDir)) { New-Item -ItemType Directory -Path $localeDir | Out-Null }
Set-Content "$localeDir\ru.json" '{"gallery":"Галерея","prompt":"Описание","model":"Модель","export":"Экспорт"}' -Encoding UTF8
Set-Content "$localeDir\en.json" '{"gallery":"Gallery","prompt":"Prompt","model":"Model","export":"Export"}' -Encoding UTF8

# Создание ярлыка на рабочем столе для запуска Electron-приложения
$desktopShortcut = [Environment]::GetFolderPath("Desktop") + "\Pepakura Next.lnk"
$electronExe = "$electronApp\node_modules\.bin\electron.cmd"
$WS = New-Object -ComObject WScript.Shell
$shortcut = $WS.CreateShortcut($desktopShortcut)
$shortcut.TargetPath = $electronExe
$shortcut.Arguments = "$electronApp\main.js"
$shortcut.WorkingDirectory = $electronApp
$shortcut.IconLocation = "$vueApp\src\assets\icon.ico"
$shortcut.Save()

Set-Location $root
Write-Host "🎉 Electron+Vue Pepakura Next готов к запуску! Ярлык: $desktopShortcut"
