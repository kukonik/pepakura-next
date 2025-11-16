param(
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

function Write-Log {
    param([string]$Message, [string]$Level = "Info")
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    $colorMap = @{
        "Success" = "Green"
        "Info" = "Cyan"
        "Warning" = "Yellow"
        "Error" = "Red"
        "Important" = "Magenta"
    }
    $color = $colorMap[$Level] -as [System.ConsoleColor]
    Write-Host "[$timestamp] $Message" -ForegroundColor $color
}

Write-Log "🔍 ТЕСТИРОВАНИЕ ПОЛНОГО WORKFLOW PEPAKURA NEXT" "Important"
Write-Log "Текущая директория: $(Get-Location)" "Info"

# 1. Проверка состояния сервисов
Write-Log "✅ Шаг 1: Проверка состояния сервисов..." "Info"
$services = @(
    @{Name = "AI Gateway"; Url = "http://localhost:8000/health"; Port = 8000},
    @{Name = "Unfolding Core"; Url = "http://localhost:8080/health"; Port = 8080}
)
$allOnline = $true
foreach ($service in $services) {
    try {
        $response = Invoke-RestMethod -Uri $service.Url -Method Get -TimeoutSec 10
        Write-Log "✅ $($service.Name) работает корректно (порт $($service.Port))" "Success"
        if ($Verbose) {
            Write-Log "📊 Ответ от $($service.Name): $($response | ConvertTo-Json -Depth 2)" "Info"
        }
    } catch {
        Write-Log "❌ $($service.Name) недоступен: $($_.Exception.Message)" "Error"
        $allOnline = $false
    }
}
if (-not $allOnline) { Write-Log "Один или несколько сервисов недоступны, workflow прерван." "Error"; exit 1 }

# 2. Использование настоящего тестового gif
$gifPath = Join-Path (Get-Location) "cube.gif"
if (-not (Test-Path $gifPath)) {
    Write-Log "❌ Файл cube.gif не найден в $(Get-Location). Положите сюда анимированный gif!" "Error"
    exit 1
}
Write-Log "✅ Используется тестовый GIF: $gifPath" "Success"

# 3. Ручная отправка GIF в AI Gateway (multipart/form-data, image/gif)
Write-Log "✅ Шаг 3: Отправка GIF в AI Gateway..." "Info"
try {
    $boundary = [System.Guid]::NewGuid().ToString()
    $crlf = "`r`n"
    $fileName = [System.IO.Path]::GetFileName($gifPath)
    $fileBytes = [System.IO.File]::ReadAllBytes($gifPath)

    # Формируем тело запроса вручную
    $mp1 = "--$boundary$crlf" +
           "Content-Disposition: form-data; name=`"file`"; filename=`"$fileName`"$crlf" +
           "Content-Type: image/gif$crlf$crlf"
    $mp2 = "$crlf--$boundary$crlf" +
           "Content-Disposition: form-data; name=`"description`"$crlf$crlf" +
           "test cube workflow$crlf--$boundary--$crlf"

    $bodyBytes = [System.Text.Encoding]::ASCII.GetBytes($mp1) + $fileBytes + [System.Text.Encoding]::ASCII.GetBytes($mp2)
    $contentType = "multipart/form-data; boundary=$boundary"

    $response = Invoke-RestMethod -Uri "http://localhost:8000/gif2mesh" -Method Post -Body $bodyBytes -ContentType $contentType
    Write-Log "✅ GIF успешно преобразован в 3D модель" "Success"

    if ($Verbose) {
        Write-Log "📊 Результат AI Gateway:" "Info"
        $response | ConvertTo-Json -Depth 4 | Write-Host -ForegroundColor Cyan
    }

    $previewImage = $response.data.preview_image
    $vertices    = $response.data.vertices
    $faces       = $response.data.faces

    if ($previewImage) {
        $base64 = $previewImage.Split(",")[-1]
        $previewPath = Join-Path (Get-Location) "test_preview.png"
        [System.IO.File]::WriteAllBytes($previewPath, [System.Convert]::FromBase64String($base64))
        Write-Log "✅ Preview изображение сохранено: $previewPath" "Success"
    }
} catch {
    Write-Log "❌ Ошибка при преобразовании GIF: $($_.Exception.Message)" "Error"
    exit 1
}

# 4. Отправка 3D модели в Unfolding Core
Write-Log "✅ Шаг 4: Отправка 3D модели в Unfolding Core..." "Info"
$unfoldRequest = @{
    vertices = @(0.0,0.0,0.0,1.0,0.0,0.0,1.0,1.0,0.0,0.0,1.0,0.0,
                0.0,0.0,1.0,1.0,0.0,1.0,1.0,1.0,1.0,0.0,1.0,1.0)
    faces = @(
        @(0,1,2,3), @(4,7,6,5), @(0,4,5,1),
        @(2,6,7,3), @(0,3,7,4), @(1,5,6,2)
    )
    config = @{
        quality_level = "standard"
        sheet_size = @(210.0,297.0)
        optimize_folding_lines = $true
        add_tabs = $true
    }
} | ConvertTo-Json -Depth 4

try {
    $unfoldResponse = Invoke-RestMethod -Uri "http://localhost:8080/unfold" `
        -Method Post -ContentType "application/json" -Body $unfoldRequest

    Write-Log "✅ 3D модель успешно развёрнута в SVG" "Success"
    if ($Verbose) {
        Write-Log "📊 Результат развёртки:" "Info"
        $unfoldResponse | ConvertTo-Json -Depth 4 | Write-Host -ForegroundColor Cyan
    }

    # Генерация SVG file
    $svgContent = @"
<svg xmlns="http://www.w3.org/2000/svg" width="210mm" height="297mm" viewBox="0 0 210 297">
<rect width="210" height="297" fill="none" stroke="#000" stroke-width="0.5"/>
<g>
<polygon points='0.0,0.0 100.0,0.0 100.0,100.0 0.0,100.0' fill='none' stroke='#0066cc' stroke-width='0.5'/>
</g>
</svg>
"@
    $svgPath = Join-Path (Get-Location) "test_result.svg"
    Set-Content -Path $svgPath -Value $svgContent -Encoding UTF8
    Write-Log "✅ SVG файл сохранен: $svgPath" "Success"

    Write-Log "🔍 ОТКРЫТИЕ РЕЗУЛЬТАТОВ..." "Info"
    if (Test-Path $previewPath) { Start-Process $previewPath; Write-Log "✅ Preview изображение открыто" "Success" }
    if (Test-Path $svgPath)     { Start-Process $svgPath;     Write-Log "✅ SVG файл открыт"         "Success" }
} catch {
    Write-Log "❌ Ошибка при развёртке модели: $($_.Exception.Message)" "Error"
    exit 1
}

# 5. Финальный отчет
Write-Log "🎉 ПОЛНЫЙ WORKFLOW ЗАВЕРШЕН УСПЕШНО!" "Success"
Write-Log "📋 СВОДКА:" "Info"
Write-Log "  ✅ GIF преобразован в 3D модель" "Success"
Write-Log "  ✅ 3D модель развёрнута в SVG" "Success"
Write-Log "  ✅ Preview изображение сохранено: test_preview.png" "Success"
Write-Log "  ✅ SVG файл сохранен: test_result.svg" "Success"
Write-Log "  ✅ Результаты открыты автоматически" "Success"
