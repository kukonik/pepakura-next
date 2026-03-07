<#
.SYNOPSIS
    Безопасное редактирование файлов с подтверждением и бэкапом
.DESCRIPTION
    Заменяет текст в файле только после подтверждения + создаёт резервную копию
#>
param(
    [Parameter(Mandatory=$true)]
    [string]$FilePath,
    
    [Parameter(Mandatory=$true)]
    [string]$Search,
    
    [Parameter(Mandatory=$true)]
    [string]$Replace
)

$ErrorActionPreference = "Stop"

try {
    # Разрешить относительный путь от корня проекта
    $ProjectRoot = "D:\Dev\pepakura-next"
    if (-not [System.IO.Path]::IsPathRooted($FilePath)) {
        $FilePath = Join-Path $ProjectRoot $FilePath
    }
    
    $FullPath = Resolve-Path $FilePath -ErrorAction Stop
    
    Write-Host "`n📝 Планируемое изменение файла:" -ForegroundColor Yellow
    Write-Host "Путь: $($FullPath.Path)" -ForegroundColor Cyan
    Write-Host "Найти:    '$Search'" -ForegroundColor Red
    Write-Host "Заменить: '$Replace'" -ForegroundColor Green
    
    # Создать бэкап
    $backupName = "$($FullPath).bak_$(Get-Date -Format 'yyyyMMdd_HHmmss')"
    Copy-Item $FullPath $backupName
    Write-Host "✅ Резервная копия: $backupName" -ForegroundColor DarkGray
    
    # Прочитать содержимое
    $content = Get-Content $FullPath -Raw -ErrorAction Stop
    
    # Проверить наличие шаблона
    if ($content -notmatch [regex]::Escape($Search)) {
        Write-Host "`n⚠️ Шаблон '$Search' не найден в файле!" -ForegroundColor Yellow
        Remove-Item $backupName
        exit 1
    }
    
    # Подсчитать замены
    $count = ([regex]::Matches($content, [regex]::Escape($Search))).Count
    Write-Host "`nНайдено совпадений: $count" -ForegroundColor Magenta
    
    # Подтверждение
    $confirm = Read-Host "`nПодтвердите замену (yes/no)"
    if ($confirm -ne 'yes') {
        Remove-Item $backupName
        Write-Host "`n❌ Изменение отменено" -ForegroundColor Red
        exit 0
    }
    
    # Выполнить замену
    $newContent = $content -replace [regex]::Escape($Search), $Replace
    Set-Content $FullPath $newContent -NoNewline -Encoding UTF8
    
    Write-Host "`n✅ Файл успешно изменён!" -ForegroundColor Green
    
    # Показать diff первых 10 изменённых строк
    $originalLines = Get-Content $backupName
    $newLines = Get-Content $FullPath
    $diff = Compare-Object $originalLines $newLines -IncludeEqual |
        Where-Object { $_.SideIndicator -ne '==' } |
        Select-Object -First 20
    
    if ($diff) {
        Write-Host "`nИзменённые строки (первые 20):" -ForegroundColor Yellow
        $diff | Format-Table -AutoSize
    }
    
    Write-Host "`nℹ️  Резервная копия сохранена: $backupName" -ForegroundColor DarkGray
    
} catch {
    Write-Host "`n❌ Ошибка: $_" -ForegroundColor Red
    Write-Host "💡 Проверьте путь к файлу и права доступа" -ForegroundColor Yellow
    exit 1
}
