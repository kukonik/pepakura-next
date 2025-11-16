param(
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

function Write-Log {
    param(
        [string]$Message,
        [string]$Level = "Info"
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $colorMap = @{
        "Success" = "Green"
        "Info" = "Cyan"
        "Warning" = "Yellow"
        "Error" = "Red"
        "Important" = "Magenta"
    }
    $color = if ($colorMap.ContainsKey($Level)) { $colorMap[$Level] } else { "White" }
    Write-Host "[$timestamp] $Message" -ForegroundColor $color
}

Write-Log "🔍 ТЕСТИРОВАНИЕ ПОЛНОГО WORKFLOW PEPAKURA NEXT" -Level "Important"

# 1. Создание тестового GIF файла
Write-Log "✅ Шаг 1: Создание тестового GIF файла..." -Level "Info"
$testGifBase64 = "R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7"
$gifBytes = [System.Convert]::FromBase64String($testGifBase64)
$tempGif = [System.IO.Path]::GetTempFileName() -replace '\.tmp$', '.gif'
[System.IO.File]::WriteAllBytes($tempGif, $gifBytes)
Write-Log "✅ Тестовый GIF создан: $tempGif" -Level "Success"

# 2. Отправка GIF в AI Gateway для получения 3D модели
Write-Log "✅ Шаг 2: Отправка GIF в AI Gateway..." -Level "Info"
$form = @{
    file = Get-Item $tempGif
    description = "3D cube model"
}

try {
    $aiResponse = Invoke-RestMethod -Uri "http://localhost:8000/gif2mesh" -Method Post -Form $form -TimeoutSec 30
    Write-Log "✅ GIF успешно преобразован в 3D модель" -Level "Success"
    
    if ($Verbose) {
        Write-Log "📊 Результат AI Gateway:" -Level "Info"
        $aiResponse | ConvertTo-Json -Depth 4 | Write-Host -ForegroundColor Cyan
    }
    
    # Извлечение данных для Unfolding Core
    $meshVertices = $aiResponse.data.vertices
    $meshFaces = $aiResponse.data.faces
    $previewImage = $aiResponse.data.preview_image
    
    # Сохранение preview изображения
    if ($previewImage) {
        $previewBytes = [System.Convert]::FromBase64String($previewImage.Split(",")[1])
        $previewPath = Join-Path -Path (Get-Location) -ChildPath "test_preview.png"
        [System.IO.File]::WriteAllBytes($previewPath, $previewBytes)
        Write-Log "✅ Preview изображение сохранено: $previewPath" -Level "Success"
    }
} catch {
    $errorMessage = $_.ErrorDetails.Message
    Write-Log "❌ Ошибка при обработке GIF: $errorMessage" -Level "Error"
    exit 1
}

# 3. Отправка 3D модели в Unfolding Core для развёртки
Write-Log "✅ Шаг 3: Отправка 3D модели в Unfolding Core..." -Level "Info"

# Формирование запроса для развёртки
$unfoldRequest = @{
    vertices = $meshVertices
    faces = $meshFaces
    config = @{
        quality_level = "standard"
        sheet_size = @(210.0, 297.0)  # A4 формат
        optimize_folding_lines = $true
        add_tabs = $true
    }
} | ConvertTo-Json -Depth 4

try {
    $unfoldResponse = Invoke-RestMethod -Uri "http://localhost:8080/unfold" -Method Post `
        -ContentType "application/json" -Body $unfoldRequest -TimeoutSec 30
    Write-Log "✅ 3D модель успешно развёрнута в SVG" -Level "Success"
    
    if ($Verbose) {
        Write-Log "📊 Результат развёртки:" -Level "Info"
        $unfoldResponse | ConvertTo-Json -Depth 4 | Write-Host -ForegroundColor Cyan
    }
    
    # 4. Генерация SVG файла
    Write-Log "✅ Шаг 4: Генерация SVG файла..." -Level "Info"
    $svgContent = @"
<svg xmlns="http://www.w3.org/2000/svg" width="210mm" height="297mm" viewBox="0 0 210 297">
<rect width="210" height="297" fill="none" stroke="#000" stroke-width="0.5"/>
"@
    
    # Добавление полигонов из развёртки
    foreach ($sheet in $unfoldResponse.sheets) {
        $svgContent += "<g>"
        $svgContent += "<polygon points='"
        $svgContent += ($sheet | ForEach-Object { "$($_[0]),$($_[1])" }) -join " "
        $svgContent += "' fill='none' stroke='#0066cc' stroke-width='0.5'/>"
        $svgContent += "</g>"
    }
    
    $svgContent += "</svg>"
    
    $svgPath = Join-Path -Path (Get-Location) -ChildPath "test_result.svg"
    Set-Content -Path $svgPath -Value $svgContent -Encoding UTF8
    Write-Log "✅ SVG файл сохранен: $svgPath" -Level "Success"
    
    # Открытие результатов
    Write-Log "🔍 ОТКРЫТИЕ РЕЗУЛЬТАТОВ..." -Level "Info"
    if (Test-Path $previewPath) {
        Start-Process $previewPath
    }
    if (Test-Path $svgPath) {
        Start-Process $svgPath
    }
    Write-Log "✅ Результаты открыты в браузере/приложении по умолчанию" -Level "Success"
    
} catch {
    $errorMessage = $_.ErrorDetails.Message
    Write-Log "❌ Ошибка при развёртке модели: $errorMessage" -Level "Error"
    exit 1
} finally {
    # Очистка временного файла
    if (Test-Path $tempGif) {
        Remove-Item $tempGif -Force -ErrorAction SilentlyContinue
    }
}

# 5. Финальный отчет
Write-Log "🎉 ПОЛНЫЙ WORKFLOW ЗАВЕРШЕН УСПЕШНО!" -Level "Success"
Write-Log "📋 СВОДКА:" -Level "Info"
Write-Log "  ✅ GIF преобразован в 3D модель" -Level "Success"
Write-Log "  ✅ 3D модель развёрнута в SVG" -Level "Success"
Write-Log "  ✅ Preview изображение сохранено: test_preview.png" -Level "Success"
Write-Log "  ✅ SVG файл сохранен: test_result.svg" -Level "Success"
Write-Log "  ✅ Результаты открыты автоматически" -Level "Success"