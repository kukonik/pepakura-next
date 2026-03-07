<#
.SYNOPSIS
    Диагностика проекта Pepakura Next
.DESCRIPTION
    Проверяет наличие ключевых файлов, конфигурацию и зависимости.
    Фокус на реальных частых проблемах из практики.
.PARAMETER Verbose
    Подробный вывод
.PARAMETER CheckAliases
    Проверить только алиасы @
.PARAMETER CheckComponents
    Проверить только компоненты Vue
.PARAMETER CheckThreeJS
    Проверить только Three.js
#>

param(
    [switch]$Verbose,
    [switch]$CheckAliases,
    [switch]$CheckComponents,
    [switch]$CheckThreeJS
)

$ErrorActionPreference = "Stop"
$projectRoot = Split-Path $PSScriptRoot -Parent

function Write-CheckResult {
    param($Name, $Status, $Critical = $false, $Details = "")
    $icon = if ($Status) { "✅" } else { "❌" }
    $color = if ($Status) { "Green" } else { if ($Critical) { "Red" } else { "Yellow" } }
    
    $line = "  $icon $Name"
    if ($Details) { $line += " ($Details)" }
    Write-Host $line -ForegroundColor $color
}

Write-Host "`n🔍 Диагностика проекта Pepakura Next" -ForegroundColor Magenta
Write-Host "   Путь: $projectRoot`n" -ForegroundColor DarkGray

$checks = @()
$passed = 0
$failed = 0
$criticalFailed = 0

# ============================================================================
# Проверка файлов
# ============================================================================
if (-not $CheckAliases -and -not $CheckComponents -and -not $CheckThreeJS) {
    Write-Host "📁 Проверка файлов:" -ForegroundColor Cyan
    
    $fileChecks = @(
        @{Name="App.vue"; Path="packages\ui-desktop\src\App.vue"; Critical=$true},
        @{Name="HomeView.vue"; Path="packages\ui-desktop\src\views\HomeView.vue"; Critical=$true},
        @{Name="ThreeDViewer.vue"; Path="packages\ui-desktop\src\components\ThreeDViewer.vue"; Critical=$true},
        @{Name="main.ts"; Path="packages\ui-desktop\src\main.ts"; Critical=$true},
        @{Name="vite.config.ts"; Path="packages\ui-desktop\vite.config.ts"; Critical=$true},
        @{Name="package.json"; Path="packages\ui-desktop\package.json"; Critical=$true},
        @{Name="tsconfig.json"; Path="packages\ui-desktop\tsconfig.json"; Critical=$true}
    )
    
    foreach ($check in $fileChecks) {
        $fullPath = Join-Path $projectRoot $check.Path
        $exists = Test-Path $fullPath
        
        if ($exists) {
            # Дополнительная проверка: не пустой ли файл
            $content = Get-Content $fullPath -Raw
            if ([string]::IsNullOrWhiteSpace($content)) {
                Write-CheckResult $check.Name $false $check.Critical "ФАЙЛ ПУСТОЙ!"
                $failed++; if ($check.Critical) { $criticalFailed++ }
                continue
            }
            
            # Проверка для App.vue: наличие <script setup lang="ts">
            if ($check.Name -eq "App.vue") {
                $hasScriptSetup = $content -match '<script setup lang="ts">'
                if (-not $hasScriptSetup) {
                    Write-CheckResult $check.Name $false $check.Critical "Нет <script setup lang='ts'>"
                    $failed++; $criticalFailed++
                    continue
                }
            }
        }
        
        Write-CheckResult $check.Name $exists $check.Critical
        if ($exists) { $passed++ } else { $failed++; if ($check.Critical) { $criticalFailed++ } }
    }
    Write-Host ""
}

# ============================================================================
# Проверка алиасов @
# ============================================================================
if ($CheckAliases -or (-not $CheckComponents -and -not $CheckThreeJS)) {
    Write-Host "🔗 Проверка алиасов @:" -ForegroundColor Cyan
    
    $viteConfigPath = Join-Path $projectRoot "packages\ui-desktop\vite.config.ts"
    if (Test-Path $viteConfigPath) {
        $viteConfig = Get-Content $viteConfigPath -Raw
        
        $hasAlias = $viteConfig -match 'alias:\s*\{[^}]*@'
        $hasPathImport = $viteConfig -match 'import.*path'
        
        Write-CheckResult "Импорт path" $hasPathImport $true
        Write-CheckResult "Алиас @" $hasAlias $true
        
        if ($hasAlias -and $Verbose) {
            if ($viteConfig -match 'alias:\s*\{([^}]+)\}') {
                Write-Host "     Найденные алиасы:" -ForegroundColor DarkGray
                $matches[1] -split ',' | ForEach-Object {
                    Write-Host "       $_" -ForegroundColor DarkGray
                }
            }
        }
        
        if ($hasAlias) { $passed++ } else { $failed++; $criticalFailed++ }
        if ($hasPathImport) { $passed++ } else { $failed++; $criticalFailed++ }
    } else {
        Write-CheckResult "vite.config.ts" $false $true "Файл не найден"
        $failed += 2; $criticalFailed += 2
    }
    Write-Host ""
}

# ============================================================================
# Проверка компонентов Vue
# ============================================================================
if ($CheckComponents -or (-not $CheckAliases -and -not $CheckThreeJS)) {
    Write-Host "🎨 Проверка компонентов Vue:" -ForegroundColor Cyan
    
    $componentChecks = @(
        @{Name="HomeView.vue"; Path="packages\ui-desktop\src\views\HomeView.vue"},
        @{Name="ThreeDViewer.vue"; Path="packages\ui-desktop\src\components\ThreeDViewer.vue"}
    )
    
    foreach ($check in $componentChecks) {
        $fullPath = Join-Path $projectRoot $check.Path
        if (Test-Path $fullPath) {
            $content = Get-Content $fullPath -Raw
            
            # Проверка структуры
            $hasTemplate = $content -match '<template>'
            $hasScriptSetup = $content -match '<script setup'
            $hasStyle = $content -match '<style'
            
            # Проверка парности тегов
            $templateOpen = ([regex]::Matches($content, '<template>')).Count
            $templateClose = ([regex]::Matches($content, '</template>')).Count
            $tagsBalanced = $templateOpen -eq $templateClose
            
            $valid = $hasTemplate -and $hasScriptSetup -and $hasStyle -and $tagsBalanced
            
            $details = @()
            if (-not $hasTemplate) { $details += "нет <template>" }
            if (-not $hasScriptSetup) { $details += "нет <script setup>" }
            if (-not $hasStyle) { $details += "нет <style>" }
            if (-not $tagsBalanced) { $details += "непарные теги ($templateOpen/$templateClose)" }
            
            Write-CheckResult $check.Name $valid $true ($details -join ", ")
            if ($valid) { $passed++ } else { $failed++; $criticalFailed++ }
        } else {
            Write-CheckResult $check.Name $false $true "Файл не найден"
            $failed++; $criticalFailed++
        }
    }
    Write-Host ""
}

# ============================================================================
# Проверка Three.js
# ============================================================================
if ($CheckThreeJS -or (-not $CheckAliases -and -not $CheckComponents)) {
    Write-Host "🎮 Проверка Three.js:" -ForegroundColor Cyan
    
    # Проверка установки three.js
    $packageJsonPath = Join-Path $projectRoot "packages\ui-desktop\package.json"
    if (Test-Path $packageJsonPath) {
        $packageJson = Get-Content $packageJsonPath | ConvertFrom-Json
        $hasThree = $null -ne $packageJson.dependencies.three
        
        Write-CheckResult "three.js в package.json" $hasThree $true
        if ($hasThree) { $passed++ } else { $failed++; $criticalFailed++ }
        
        # Проверка наличия в node_modules
        $threeModulePath = Join-Path $projectRoot "packages\ui-desktop\node_modules\three"
        $threeInstalled = Test-Path $threeModulePath
        Write-CheckResult "three.js в node_modules" $threeInstalled $false
        if ($threeInstalled) { $passed++ } else { $failed++ }
    }
    
    # Проверка импортов в компонентах
    $viewerPath = Join-Path $projectRoot "packages\ui-desktop\src\components\ThreeDViewer.vue"
    if (Test-Path $viewerPath) {
        $viewerContent = Get-Content $viewerPath -Raw
        $hasThreeImport = $viewerContent -match "import \* as THREE from 'three'"
        $hasLoaders = $viewerContent -match "OBJLoader|FBXLoader|GLTFLoader"
        
        Write-CheckResult "Импорт THREE" $hasThreeImport $true
        Write-CheckResult "Загрузчики форматов" $hasLoaders $false
        
        if ($hasThreeImport) { $passed++ } else { $failed++; $criticalFailed++ }
        if ($hasLoaders) { $passed++ } else { $failed++ }
    }
    Write-Host ""
}

# ============================================================================
# Проверка зависимостей
# ============================================================================
if (-not $CheckAliases -and -not $CheckComponents -and -not $CheckThreeJS) {
    Write-Host "📦 Проверка зависимостей:" -ForegroundColor Cyan
    
    $pkgPath = Join-Path $projectRoot "packages\ui-desktop\package.json"
    if (Test-Path $pkgPath) {
        $pkg = Get-Content $pkgPath | ConvertFrom-Json
        
        $depChecks = @(
            @{Name="vue"; Required=$true},
            @{Name="three"; Required=$true},
            @{Name="@tauri-apps/api"; Required=$false},
            @{Name="typescript"; Required=$true},
            @{Name="vite"; Required=$true}
        )
        
        foreach ($dep in $depChecks) {
            $exists = $null -ne $pkg.dependencies."$($dep.Name)" -or $null -ne $pkg.devDependencies."$($dep.Name)"
            Write-CheckResult $dep.Name $exists $dep.Required
            if ($exists) { $passed++ } else { $failed++; if ($dep.Required) { $criticalFailed++ } }
        }
    }
    Write-Host ""
}

# ============================================================================
# Итоговый отчёт
# ============================================================================
Write-Host "📊 Итоговый отчёт:" -ForegroundColor Cyan
Write-Host "  ✅ Успешно: $passed" -ForegroundColor Green
Write-Host "  ❌ Ошибок: $failed" -ForegroundColor $(if ($criticalFailed -gt 0) { "Red" } else { "Yellow" })
Write-Host "  🔴 Критических: $criticalFailed" -ForegroundColor Red

if ($criticalFailed -gt 0) {
    Write-Host "`n⚠️  Найдены критические ошибки. Рекомендуется запустить:" -ForegroundColor Red
    Write-Host "   .\tools\fix-common-issues.ps1 --all" -ForegroundColor Yellow
    exit 1
} else {
    Write-Host "`n✅ Проект в рабочем состоянии" -ForegroundColor Green
    exit 0
}
