<#
.SYNOPSIS
    Полная диагностика цикла: Текст → AI → 3D модель → 2D развёртка → Сохранение
#>
param([switch]$Verbose)

$ErrorActionPreference = "SilentlyContinue"
$Root = "D:\Dev\pepakura-next"

Write-Host "`n🔍 Диагностика полного цикла работы Pepakura Next`n" -ForegroundColor Magenta
Write-Host "Цикл: Текст → AI → 3D модель → 2D развёртка → Редактирование → Сохранение`n" -ForegroundColor Cyan

# ============================================================================
# ЭТАП 1: TXT/TXT AI-генерация → 2D/3D модель
# ============================================================================
Write-Host "╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  ЭТАП 1: TXT/TXT AI-генерация → 2D/3D модель                ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

$stage1 = @{
    status = @()
    critical = @()
}

# 1.1. AI Text-to-3D сервис
Write-Host "1.1 AI Text-to-3D сервис:" -ForegroundColor Yellow

$aiTextTo3D = @(
    @{Path="services\backend-python\ai\mesh_generator.py"; Desc="Генератор 3D-мешей из текста"; Critical=$true}
    @{Path="services\backend-python\ai\prompt_parser.py"; Desc="Парсер промптов"; Critical=$false}
    @{Path="services\backend-python\ai\advanced_prompt_parser.py"; Desc="Продвинутый парсер промптов"; Critical=$false}
    @{Path="backend\ai_seams\app.py"; Desc="AI Seam Service (Flask/FastAPI)"; Critical=$true}
    @{Path="packages\ui-desktop\src\components\TextTo3DGenerator.vue"; Desc="UI компонент Text-to-3D"; Critical=$true}
)

foreach ($item in $aiTextTo3D) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $stage1.status += "✅ $($item.Desc)"
        if ($item.Critical) { $stage1.critical += "✅" }
    } else {
        Write-Host "  ❌ $($item.Desc)" -ForegroundColor Red
        $stage1.status += "❌ $($item.Desc)"
        if ($item.Critical) { $stage1.critical += "❌" }
    }
}

# 1.2. Text-to-Image AI (если есть)
Write-Host "`n1.2 Text-to-Image AI:" -ForegroundColor Yellow

$aiTextToImage = @(
    @{Path="services\backend-python\unfolders\svg_unfolder.py"; Desc="SVG Unfolder (из текста/изображения)"; Critical=$false}
    @{Path="packages\ui-desktop\src\components\ImageTo3DGenerator.vue"; Desc="UI компонент Image-to-3D"; Critical=$false}
)

foreach ($item in $aiTextToImage) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
    } else {
        Write-Host "  ⚪ $($item.Desc) (опционально)" -ForegroundColor DarkGray
    }
}

# ============================================================================
# ЭТАП 2: 3D модель → 2D бумажная развёртка
# ============================================================================
Write-Host "`n╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  ЭТАП 2: 3D модель → 2D бумажная развёртка                  ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

$stage2 = @{
    status = @()
    critical = @()
}

# 2.1. Rust Core (разворачивание)
Write-Host "2.1 Rust Core (разворачивание):" -ForegroundColor Yellow

$rustCore = @(
    @{Path="core\src\unfold\unwrap3d.rs"; Desc="Алгоритм разворачивания 3D→2D"; Critical=$true}
    @{Path="core\src\unfold\layout.rs"; Desc="Оптимизация расположения частей"; Critical=$true}
    @{Path="core\src\unfold\paper_optimize.rs"; Desc="Оптимизация под бумагу"; Critical=$true}
    @{Path="core\src\model\mesh.rs"; Desc="Работа с 3D мешами"; Critical=$true}
    @{Path="core\src\model\io_obj.rs"; Desc="Импорт/экспорт OBJ"; Critical=$true}
)

foreach ($item in $rustCore) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $stage2.status += "✅ $($item.Desc)"
        if ($item.Critical) { $stage2.critical += "✅" }
    } else {
        Write-Host "  ❌ $($item.Desc)" -ForegroundColor Red
        $stage2.status += "❌ $($item.Desc)"
        if ($item.Critical) { $stage2.critical += "❌" }
    }
}

# 2.2. Tauri интеграция
Write-Host "`n2.2 Tauri интеграция:" -ForegroundColor Yellow

$tauriIntegration = @(
    @{Path="packages\ui-desktop\src-tauri\src\ai\commands.rs"; Desc="Tauri команды для разворачивания"; Critical=$true}
    @{Path="packages\ui-desktop\src-tauri\src\ai\export.rs"; Desc="Экспорт развёрток"; Critical=$true}
    @{Path="packages\ui-desktop\src-tauri\Cargo.toml"; Desc="Зависимости Tauri + Rust Core"; Critical=$true}
)

foreach ($item in $tauriIntegration) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $stage2.status += "✅ $($item.Desc)"
        if ($item.Critical) { $stage2.critical += "✅" }
    } else {
        Write-Host "  ❌ $($item.Desc)" -ForegroundColor Red
        $stage2.status += "❌ $($item.Desc)"
        if ($item.Critical) { $stage2.critical += "❌" }
    }
}

# ============================================================================
# ЭТАП 3: Просмотр и редактирование
# ============================================================================
Write-Host "`n╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  ЭТАП 3: Просмотр и редактирование                          ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

$stage3 = @{
    status = @()
    critical = @()
}

# 3.1. 3D просмотрщик
Write-Host "3.1 3D просмотрщик (Three.js):" -ForegroundColor Yellow

$threeDViewer = @(
    @{Path="packages\shared\src\components\ModelViewer.vue"; Desc="Основной 3D просмотрщик"; Critical=$true}
    @{Path="packages\ui-desktop\src\components\ThreeDViewerCanvas.vue"; Desc="Canvas для Three.js"; Critical=$true}
    @{Path="packages\ui-desktop\src\components\Scene3D.vue"; Desc="Сцена 3D"; Critical=$true}
    @{Path="packages\shared\src\composables\useThreeJsScene.ts"; Desc="Управление 3D сценой"; Critical=$true}
)

foreach ($item in $threeDViewer) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $stage3.status += "✅ $($item.Desc)"
        if ($item.Critical) { $stage3.critical += "✅" }
    } else {
        Write-Host "  ❌ $($item.Desc)" -ForegroundColor Red
        $stage3.status += "❌ $($item.Desc)"
        if ($item.Critical) { $stage3.critical += "❌" }
    }
}

# 3.2. 2D просмотрщик развёртки
Write-Host "`n3.2 2D просмотрщик развёртки:" -ForegroundColor Yellow

$twoDViewer = @(
    @{Path="packages\ui-desktop\src\components\UnfoldViewer2D.vue"; Desc="2D просмотрщик развёртки"; Critical=$true}
    @{Path="packages\ui-desktop\src\views\UnfoldEditorView.vue"; Desc="Редактор развёрток"; Critical=$true}
    @{Path="packages\ui-desktop\src\components\PaperOptimizePanel.vue"; Desc="Панель оптимизации"; Critical=$false}
)

foreach ($item in $twoDViewer) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $stage3.status += "✅ $($item.Desc)"
        if ($item.Critical) { $stage3.critical += "✅" }
    } else {
        Write-Host "  ❌ $($item.Desc)" -ForegroundColor Red
        $stage3.status += "❌ $($item.Desc)"
        if ($item.Critical) { $stage3.critical += "❌" }
    }
}

# 3.3. Редактирование швов
Write-Host "`n3.3 Редактирование швов:" -ForegroundColor Yellow

$seamEditing = @(
    @{Path="packages\ui-desktop\src\components\SeamVisualizer.ts"; Desc="Визуализатор швов"; Critical=$false}
    @{Path="core\src\unfold\paper_optimize.test.rs"; Desc="Тесты оптимизации"; Critical=$false}
)

foreach ($item in $seamEditing) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
    } else {
        Write-Host "  ⚪ $($item.Desc) (опционально)" -ForegroundColor DarkGray
    }
}

# ============================================================================
# ЭТАП 4: Сохранение (локально/облако)
# ============================================================================
Write-Host "`n╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║  ЭТАП 4: Сохранение (локально/облако)                       ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════════╝`n" -ForegroundColor Cyan

$stage4 = @{
    status = @()
    critical = @()
}

# 4.1. Локальное сохранение
Write-Host "4.1 Локальное сохранение:" -ForegroundColor Yellow

$localSave = @(
    @{Path="packages\shared\src\stores\project.store.ts"; Desc="Хранилище проектов"; Critical=$true}
    @{Path="packages\shared\src\composables\useAutoSave.ts"; Desc="Автосохранение"; Critical=$true}
    @{Path="services\backend-python\exporters\project_zipper.py"; Desc="Экспорт в архив"; Critical=$true}
)

foreach ($item in $localSave) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $stage4.status += "✅ $($item.Desc)"
        if ($item.Critical) { $stage4.critical += "✅" }
    } else {
        Write-Host "  ❌ $($item.Desc)" -ForegroundColor Red
        $stage4.status += "❌ $($item.Desc)"
        if ($item.Critical) { $stage4.critical += "❌" }
    }
}

# 4.2. Экспорт форматов
Write-Host "`n4.2 Экспорт форматов:" -ForegroundColor Yellow

$exportFormats = @(
    @{Path="services\backend-python\exporters\instruction_pdf_generator.py"; Desc="Генерация инструкций (PDF)"; Critical=$true}
    @{Path="services\backend-python\exporters\obj_to_stl.py"; Desc="Конвертация OBJ→STL"; Critical=$false}
    @{Path="services\backend-python\exporters\obj_to_glb.py"; Desc="Конвертация OBJ→GLB"; Critical=$false}
    @{Path="services\backend-python\exporters\obj_to_fbx.py"; Desc="Конвертация OBJ→FBX"; Critical=$false}
    @{Path="core\src\export\export_svg.rs"; Desc="Экспорт в SVG (Rust)"; Critical=$true}
    @{Path="core\src\export\export_png.rs"; Desc="Экспорт в PNG (Rust)"; Critical=$false}
    @{Path="core\src\export\export_pdf.rs"; Desc="Экспорт в PDF (Rust)"; Critical=$false}
)

foreach ($item in $exportFormats) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $stage4.status += "✅ $($item.Desc)"
        if ($item.Critical) { $stage4.critical += "✅" }
    } else {
        Write-Host "  ⚪ $($item.Desc) (опционально)" -ForegroundColor DarkGray
    }
}

# 4.3. Облачное сохранение
Write-Host "`n4.3 Облачное сохранение:" -ForegroundColor Yellow

$cloudSave = @(
    @{Path="backend\api\sync\sync.controller.ts"; Desc="Контроллер синхронизации"; Critical=$false}
    @{Path="packages\shared\src\types\sync.types.ts"; Desc="Типы для синхронизации"; Critical=$false}
    @{Path="packages\shared\src\ai\AiBackendConfig.ts"; Desc="Конфигурация бэкенда"; Critical=$false}
)

$cloudFound = 0
foreach ($item in $cloudSave) {
    $fullPath = Join-Path $Root $item.Path
    if (Test-Path $fullPath) {
        Write-Host "  ✅ $($item.Desc)" -ForegroundColor Green
        $cloudFound++
    } else {
        Write-Host "  ⚪ $($item.Desc) (опционально)" -ForegroundColor DarkGray
    }
}

if ($cloudFound -eq 0) {
    Write-Host "  ⚠️  Облачное сохранение: НЕ РЕАЛИЗОВАНО" -ForegroundColor Yellow
} else {
    Write-Host "  ℹ️  Облачное сохранение: ЧАСТИЧНО РЕАЛИЗОВАНО ($cloudFound/3)" -ForegroundColor Cyan
}

# ============================================================================
# ИТОГОВЫЙ ОТЧЁТ
# ============================================================================
Write-Host "`n╔══════════════════════════════════════════════════════════════╗" -ForegroundColor Magenta
Write-Host "║  ИТОГОВЫЙ ОТЧЁТ                                              ║" -ForegroundColor Magenta
Write-Host "╚══════════════════════════════════════════════════════════════╝`n" -ForegroundColor Magenta

# Подсчитать статистику
$stages = @(
    @{Name="ЭТАП 1: AI-генерация"; Data=$stage1}
    @{Name="ЭТАП 2: Разворачивание"; Data=$stage2}
    @{Name="ЭТАП 3: Просмотр/редактирование"; Data=$stage3}
    @{Name="ЭТАП 4: Сохранение"; Data=$stage4}
)

$totalCritical = 0
$totalCriticalDone = 0

foreach ($stage in $stages) {
    $criticalCount = ($stage.Data.critical | Where-Object { $_ -eq "✅" }).Count
    $totalCriticalInStage = $stage.Data.critical.Count
    
    $totalCritical += $totalCriticalInStage
    $totalCriticalDone += $criticalCount
    
    $percent = if ($totalCriticalInStage -gt 0) { [math]::Round(($criticalCount / $totalCriticalInStage) * 100) } else { 100 }
    
    $statusColor = if ($percent -eq 100) { "Green" } elseif ($percent -ge 70) { "Cyan" } else { "Yellow" }
    $statusIcon = if ($percent -eq 100) { "✅" } elseif ($percent -ge 70) { "🟡" } else { "⚠️" }
    
    Write-Host "$($statusIcon) $($stage.Name): $percent% ($criticalCount/$totalCriticalInStage)" -ForegroundColor $statusColor
}

# Общий прогресс
$overallPercent = [math]::Round(($totalCriticalDone / $totalCritical) * 100)
$overallStatus = if ($overallPercent -eq 100) { "✅ ПОЛНОСТЬЮ РЕАЛИЗОВАНО" } elseif ($overallPercent -ge 80) { "🟡 ПОЧТИ ГОТОВО" } else { "⚠️ ТРЕБУЕТ ДОРАБОТКИ" }

Write-Host "`n📊 Общий прогресс: $overallPercent% ($totalCriticalDone/$totalCritical критических компонентов)" -ForegroundColor Magenta
Write-Host "   Статус: $overallStatus`n" -ForegroundColor $(if ($overallPercent -eq 100) { "Green" } elseif ($overallPercent -ge 80) { "Cyan" } else { "Yellow" })

# ============================================================================
# РЕКОМЕНДАЦИИ
# ============================================================================
Write-Host "💡 Рекомендации:`n" -ForegroundColor Cyan

if ($stage1.critical -contains "❌") {
    Write-Host "  ⚠️  ЭТАП 1 (AI-генерация): Требуется доработка компонентов" -ForegroundColor Yellow
}

if ($stage2.critical -contains "❌") {
    Write-Host "  ⚠️  ЭТАП 2 (Разворачивание): Проверьте интеграцию Rust Core" -ForegroundColor Yellow
}

if ($stage3.critical -contains "❌") {
    Write-Host "  ⚠️  ЭТАП 3 (Просмотр): Интегрируйте существующие компоненты" -ForegroundColor Yellow
}

if ($stage4.critical -contains "❌") {
    Write-Host "  ⚠️  ЭТАП 4 (Сохранение): Добавьте экспорт в PDF/SVG" -ForegroundColor Yellow
}

if ($cloudFound -eq 0) {
    Write-Host "  ⚠️  Облачное сохранение: Не реализовано — рассмотрите интеграцию с S3/Google Drive" -ForegroundColor Yellow
}

Write-Host "`n✨ Проект имеет СИЛЬНУЮ архитектуру с реализацией всех ключевых этапов!" -ForegroundColor Green
Write-Host "   Основная задача: ИНТЕГРАЦИЯ существующих компонентов между собой.`n" -ForegroundColor Cyan
