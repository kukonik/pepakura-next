Write-Host "`n🔍 Углублённая диагностика: поиск компонентов во всём проекте`n" -ForegroundColor Magenta

# Искомые компоненты и их возможные пути
$components = @(
    @{Name="mesh_generator.py"; Paths=@("services\backend-python\ai\", "services\backend-python\unfolders\", "backend\ai_seams\")},
    @{Name="prompt_parser.py"; Paths=@("services\backend-python\ai\", "backend\ai_seams\")},
    @{Name="ThreeDViewerCanvas.vue"; Paths=@("packages\ui-desktop\src\components\", "packages\shared\src\components\", "packages\ui-web\src\components\")},
    @{Name="UnfoldViewer2D.vue"; Paths=@("packages\ui-desktop\src\components\", "packages\shared\src\components\")},
    @{Name="project.store.ts"; Paths=@("packages\ui-desktop\src\stores\", "packages\shared\src\stores\", "packages\ui-web\src\stores\")},
    @{Name="useAutoSave.ts"; Paths=@("packages\ui-desktop\src\composables\", "packages\shared\src\composables\")}
)

foreach ($comp in $components) {
    Write-Host "`n🔎 Поиск: $($comp.Name)" -ForegroundColor Yellow
    $found = $false
    
    foreach ($path in $comp.Paths) {
        $fullPath = "D:\Dev\pepakura-next\$path$($comp.Name)"
        if (Test-Path $fullPath) {
            Write-Host "  ✅ НАЙДЕН: $fullPath" -ForegroundColor Green
            $found = $true
            break
        }
    }
    
    if (-not $found) {
        Write-Host "  ❌ НЕ НАЙДЕН ни в одном из ожидаемых путей" -ForegroundColor Red
    }
}

# Проверить структуру пакетов
Write-Host "`n📦 Структура пакетов:`n" -ForegroundColor Cyan
Get-ChildItem "D:\Dev\pepakura-next\packages" -Directory | ForEach-Object {
    $pkgName = $_.Name
    $hasSrc = Test-Path "D:\Dev\pepakura-next\packages\$pkgName\src"
    $hasComponents = Test-Path "D:\Dev\pepakura-next\packages\$pkgName\src\components"
    $hasComposables = Test-Path "D:\Dev\pepakura-next\packages\$pkgName\src\composables"
    $hasStores = Test-Path "D:\Dev\pepakura-next\packages\$pkgName\src\stores"
    
    Write-Host "  ${pkgName}:" -ForegroundColor Yellow
    Write-Host "    components: $($hasComponents ? '✅' : '❌')" -ForegroundColor Gray
    Write-Host "    composables: $($hasComposables ? '✅' : '❌')" -ForegroundColor Gray
    Write-Host "    stores: $($hasStores ? '✅' : '❌')" -ForegroundColor Gray
}
