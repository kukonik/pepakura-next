<#
.SYNOPSIS
    Сканирование структуры проекта Pepakura Next
.DESCRIPTION
    Показывает древовидную структуру проекта с фильтрацией по важным файлам
#>
param(
    [string]$Path = "D:\Dev\pepakura-next",
    [int]$Depth = 3,
    [switch]$Full
)

$ErrorActionPreference = "Stop"
$ProjectRoot = Resolve-Path $Path

Write-Host "`n🔍 Сканирование проекта: $($ProjectRoot.Path)" -ForegroundColor Cyan
Write-Host "Глубина: $Depth`n" -ForegroundColor DarkGray

try {
    $excludePatterns = @(
        'node_modules', 'target', '.git', 'env', '__pycache__',
        '.vscode', '.idea', 'dist', 'build', 'coverage'
    )
    
    $items = Get-ChildItem -Path $ProjectRoot -Recurse -Depth $Depth -ErrorAction SilentlyContinue |
        Where-Object {
            $relativePath = $_.FullName.Replace($ProjectRoot.Path, '').Replace('\', '/')
            $excluded = $false
            foreach ($pattern in $excludePatterns) {
                if ($relativePath -match "/$pattern(/|$)") { $excluded = $true; break }
            }
            -not $excluded
        } |
        Sort-Object FullName

    if ($Full) {
        # Полное содержимое файлов
        $items | Where-Object { -not $_.PSIsContainer } | ForEach-Object {
            Write-Host "`n=== $($_.Name) ===" -ForegroundColor Yellow
            Write-Host "Путь: $($_.FullName.Replace($ProjectRoot.Path, ''))" -ForegroundColor DarkGray
            try {
                $content = Get-Content $_.FullName -Raw -ErrorAction Stop
                if ($content.Length -gt 5000) {
                    Write-Host ($content.Substring(0, 5000) + "`n... [обрезано]") -ForegroundColor Gray
                } else {
                    Write-Host $content -ForegroundColor Gray
                }
            } catch {
                Write-Host "⚠️ Не удалось прочитать файл: $_" -ForegroundColor Red
            }
        }
    } else {
        # Древовидная структура
        $rootLength = $ProjectRoot.Path.Length + 1
        $items | ForEach-Object {
            $relative = $_.FullName.Substring($rootLength)
            $indent = "  " * ($relative.Split('\').Count - 1)
            
            if ($_.PSIsContainer) {
                Write-Host "${indent}📂 $($_.Name)" -ForegroundColor DarkCyan
            } elseif ($_.Name -match '\.(json|toml|lock)$') {
                Write-Host "${indent}📄 $($_.Name)" -ForegroundColor Magenta
            } elseif ($_.Name -match '\.(ts|js|vue)$') {
                Write-Host "${indent}💻 $($_.Name)" -ForegroundColor Cyan
            } elseif ($_.Name -match '\.(rs|py)$') {
                Write-Host "${indent}⚙️  $($_.Name)" -ForegroundColor Yellow
            } elseif ($_.Name -match '\.(md|txt)$') {
                Write-Host "${indent}📝 $($_.Name)" -ForegroundColor Green
            } else {
                Write-Host "${indent}📄 $($_.Name)" -ForegroundColor Gray
            }
        }
    }

    Write-Host "`n💡 Совет: скопируйте вывод и вставьте в чат для анализа" -ForegroundColor Green
    Write-Host "Команды:`n  pnpm scan          # быстрая структура`n  pnpm scan:full     # с содержимым файлов`n" -ForegroundColor DarkGray

} catch {
    Write-Host "`n❌ Ошибка: $_" -ForegroundColor Red
    exit 1
}
