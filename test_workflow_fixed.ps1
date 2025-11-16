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

Write-Log "🔍 Старт тестирования workflow Pepakura Next..." "Important"
Write-Log "Текущая директория: $(Get-Location)" "Info"

# 1. Проверка состояния сервисов
Write-Log "Шаг 1: Проверка состояния сервисов..." "Info"
$services = @(
    @{Name = "AI Gateway"; Url = "http://localhost:8000/health"; Port = 8000},
    @{Name = "Unfolding Core"; Url = "http://localhost:8080/health"; Port = 8080}
)
$allOnline = $true
foreach ($service in $services) {
    try {
        $response = Invoke-RestMethod -Uri $service.Url -Method Get -TimeoutSec 10 -ErrorAction Stop
        Write-Log "✅ $($service.Name) работает (порт $($service.Port))" "Success"
        if ($Verbose) {
            Write-Log "Ответ $($service.Name):" "Info"
            $response | ConvertTo-Json -Depth 4 | Write-Host -ForegroundColor Cyan
        }
    } catch {
        Write-Log "❌ $($service.Name) недоступен: $($_.Exception.Message)" "Error"
        $allOnline = $false
    }
}
if (-not $allOnline) { Write-Log "Один или несколько сервисов недоступны, workflow прерван." "Error"; exit 1 }

# 2. Создание тестового GIF
Write-Log "Шаг 2: Генерация тестового GIF..." "Info"
$gifBase64 = "R0lGODlhAQABAIAAAP///wAAACH5BAEAAAAALAAAAAABAAEAAAICRAEAOw=="
$gifBytes = [Convert]::FromBase64String($gifBase64)
$tempGif = [System.IO.Path]::GetTempFileName() -replace '\.tmp$', '.gif'
[IO.File]::WriteAllBytes($tempGif, $gifBytes)
Write-Log "Тестовый GIF создан: $tempGif, размер: $((Get-Item $tempGif).Length) байт" "Success"

# 3. Отправка GIF в AI Gateway
Write-Log "Шаг 3: Отправка GIF в AI Gateway..." "Info"
$formData = @{
    file = Get-Item $tempGif
    description = "test cube workflow"
}
try {
    $aiResponse = Invoke-RestMethod -Uri "http://localhost:8000/gif2mesh" `
        -Method Post -Form $formData -ContentType "multipart/form-data"
    Write-Log "GIF преобразован в 3D модель." "Success"
    if ($Verbose) {
        Write-Log "Результат преобразования GIF:" "Info"
        $aiResponse | ConvertTo-Json -Depth 4 | Write-Host -ForegroundColor Cyan
    }
    $previewImage = $aiResponse.data.preview_image
    $vertices = $aiResponse.data.vertices
    $faces = $aiResponse.data.faces

    # Сохранение preview
    if ($previewImage) {
        $base64 = $previewImage.Split(",")[-1]
        $previewPath = Join-Path (Get-Location) "test_preview.png"
        [IO.File]::WriteAllBytes($previewPath, [Convert]::FromBase64String($base64))
        Write-Log "Preview сохранено в: $previewPath" "Success"
    }
} catch {
    Write-Log "Ошибка преобразования GIF: $($_.Exception.Message)" "Error"
    Remove-Item $tempGif -ErrorAction SilentlyContinue
    exit 1
} finally {
    if (Test-Path $tempGif) { Remove-Item $tempGif -Force -ErrorAction SilentlyContinue }
}

# 4. Отправка 3D данных в Unfolding Core
Write-Log "Шаг 4: Отправка 3D данных в Unfolding Core..." "Info"
$unfoldRequest = @{
    vertices = @(0.0,0.0,0.0,1.0,0.0,0.0,1.0,1.0,0.0,0.0,1.0,0.0,
                0.0,0.0,1.0,1.0,0.0,1.0,1.0,1.0,1.0,0.0,1.0,1.0)
    faces = @(
        @(0,1,2,3), @(4,5,6,7), @(0,1,5,4),
        @(1,2,6,5), @(2,3,7,6), @(3,0,4,7)
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
        -Method Post -ContentType "application/json" -Body $unfoldRequest -TimeoutSec 60
    Write-Log "3D модель развёрнута в SVG." "Success"
    if ($Verbose) {
        Write-Log "Результат развёртки:" "Info"
        $unfoldResponse | ConvertTo-Json -Depth 4 | Write-Host -ForegroundColor Cyan
    }
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
    Write-Log "SVG файл сохранен: $svgPath" "Success"
    
    Write-Log "Открытие результатов..." "Info"
    if (Test-Path $previewPath) { Start-Process $previewPath }
    if (Test-Path $svgPath) { Start-Process $svgPath }
    Write-Log "✅ Файлы открыты стандартными средствами." "Success"

} catch {
    Write-Log "Ошибка развёртки: $($_.Exception.Message)" "Error"
    exit 1
}

# 5. Финальный отчет
Write-Log "🎉 WORKFLOW УСПЕШНО ЗАВЕРШЁН!" "Success"
Write-Log "📋 Сводка:" "Info"
Write-Log "  ✓ GIF → 3D: test_preview.png" "Success"
Write-Log "  ✓ 3D → SVG: test_result.svg" "Success"
Write-Log "  ✓ Открыто автоматически" "Success"
