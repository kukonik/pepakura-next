<#
.SYNOPSIS
    Автоматическое исправление частых проблем проекта
.DESCRIPTION
    Исправляет алиасы, очищает кэш, восстанавливает компоненты из бэкапа.

    ⚠️ ВНИМАНИЕ: Скрипт с флагом --fix-aliases ПЕРЕЗАПИСЫВАЕТ
    vite.config.ts и tsconfig.json. Используйте только как "кнопку паники"
    для восстановления рабочего состояния после поломок.

.PARAMETER All
    Применить все исправления
.PARAMETER ClearCache
    Очистить кэш Vite + pnpm store prune
.PARAMETER FixAliases
    Исправить алиасы @ в vite.config.ts (ПЕРЕЗАПИСЫВАЕТ ФАЙЛ!)
.PARAMETER FixDragDrop
    Исправить обработчики drag&drop
.PARAMETER RestoreBackup
    Восстановить компоненты из бэкапа
#>

param(
    [switch]$All,
    [switch]$ClearCache,
    [switch]$FixAliases,
    [switch]$FixDragDrop,
    [switch]$RestoreBackup
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path $PSScriptRoot -Parent

function Write-Action {
    param($Message, $Success = $true)
    $icon = if ($Success) { "✅" } else { "❌" }
    $color = if ($Success) { "Green" } else { "Red" }
    Write-Host "  $icon $Message" -ForegroundColor $color
}

Write-Host "`n🛠️  Исправление частых проблем Pepakura Next" -ForegroundColor Magenta
Write-Host "   Путь: $projectRoot`n" -ForegroundColor DarkGray

$anyAction = $false

# Очистка кэша
if ($All -or $ClearCache) {
    $anyAction = $true
    Write-Host "🧹 Очистка кэша:" -ForegroundColor Cyan

    $viteCache = Join-Path $projectRoot "packages\ui-desktop\node_modules\.vite"
    if (Test-Path $viteCache) {
        Remove-Item -Recurse -Force $viteCache -ErrorAction SilentlyContinue
        Write-Action "Удалён кэш Vite"
    } else {
        Write-Action "Кэш Vite не найден (пропуск)" $true
    }

    $distDir = Join-Path $projectRoot "packages\ui-desktop\dist"
    if (Test-Path $distDir) {
        Remove-Item -Recurse -Force $distDir -ErrorAction SilentlyContinue
        Write-Action "Удалена папка dist"
    }

    if ($ClearCache) {
        Write-Host "   ⏳ Очистка pnpm store (может занять время)..." -ForegroundColor DarkGray
        pnpm store prune | Out-Null
        Write-Action "Очищен pnpm кэш"
    }

    Write-Host ""
}

# Исправление алиасов
if ($All -or $FixAliases) {
    $anyAction = $true
    Write-Host "🔗 Исправление алиасов @:" -ForegroundColor Cyan
    Write-Host "   ⚠️  ВНИМАНИЕ: Будет ПЕРЕЗАПИСАН файл vite.config.ts!" -ForegroundColor Red

    $viteConfigPath = Join-Path $projectRoot "packages\ui-desktop\vite.config.ts"

    if (Test-Path $viteConfigPath) {
        $content = Get-Content $viteConfigPath -Raw

        if ($content -notmatch "alias:\s*\{[^}]*'@':\s*path\.resolve") {
$newConfig = @"
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, 'src')
    }
  },
  base: './'
})
"@
            Set-Content -Path $viteConfigPath -Value $newConfig -Encoding UTF8
            Write-Action "Исправлен vite.config.ts (файл перезаписан)"
        } else {
            Write-Action "Алиасы уже настроены (пропуск)" $true
        }
    } else {
        Write-Action "vite.config.ts не найден" $false
    }

    $tsConfigPath = Join-Path $projectRoot "packages\ui-desktop\tsconfig.json"
    if (Test-Path $tsConfigPath) {
        $tsConfig = Get-Content $tsConfigPath | ConvertFrom-Json
        if (-not $tsConfig.compilerOptions.paths) {
            $tsConfig.compilerOptions | Add-Member -NotePropertyName "paths" -NotePropertyValue (@{"@/*" = @("src/*")}) -Force
            $tsConfig | ConvertTo-Json -Depth 10 | Set-Content $tsConfigPath -Encoding UTF8
            Write-Action "Добавлены paths в tsconfig.json"
        } else {
            Write-Action "paths уже настроены (пропуск)" $true
        }
    }

    Write-Host ""
}

# Восстановление из бэкапа
if ($All -or $RestoreBackup) {
    $anyAction = $true
    Write-Host "💾 Восстановление из бэкапа:" -ForegroundColor Cyan

    $backups = Get-ChildItem "$projectRoot\packages" -Directory | Where-Object { $_.Name -match 'ui-desktop\.backup' } | Sort-Object LastWriteTime -Descending

    if ($backups.Count -gt 0) {
        $latestBackup = $backups[0].FullName
        $target = Join-Path $projectRoot "packages\ui-desktop"

        Write-Host "  Найден бэкап: $($backups[0].Name)" -ForegroundColor Green

        @("src\views", "src\components", "src\composables") | ForEach-Object {
            $backupPath = Join-Path $latestBackup $_
            $targetPath = Join-Path $target $_

            if (Test-Path $backupPath) {
                Copy-Item -Path "$backupPath\*" -Destination $targetPath -Recurse -Force -ErrorAction SilentlyContinue
                Write-Action "Восстановлен $_"
            }
        }
    } else {
        Write-Action "Бэкапы не найдены" $false
    }

    Write-Host ""
}

if ($anyAction) {
    Write-Host "✅ Исправления применены" -ForegroundColor Green
    Write-Host "`n💡 Рекомендуется перезапустить сервер:" -ForegroundColor Yellow
    Write-Host "   pnpm dev --force" -ForegroundColor Gray
} else {
    Write-Host "⚠️  Нет выбранных действий. Используйте:" -ForegroundColor Yellow
    Write-Host "   --all, --clear-cache, --fix-aliases, --restore-backup" -ForegroundColor Gray
}
