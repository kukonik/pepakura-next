Write-Host "=== ИСПРАВЛЕНИЕ RUST-КОДА ==="

# Список файлов для исправления
$files = @(
    "D:\Dev\pepakura-next\core\src\unfold\mod.rs",
    "D:\Dev\pepakura-next\core\src\unfold\unwrap3d.rs", 
    "D:\Dev\pepakura-next\core\src\unfold\layout.rs",
    "D:\Dev\pepakura-next\core\src\lib.rs",
    "D:\Dev\pepakura-next\core\src\model\mod.rs",
    "D:\Dev\pepakura-next\core\src\model\io_obj.rs",
    "D:\Dev\pepakura-next\core\src\export\mod.rs",
    "D:\Dev\pepakura-next\core\src\export\export_png.rs",
    "D:\Dev\pepakura-next\core\src\export\export_jpg.rs",
    "D:\Dev\pepakura-next\core\src\export\export_obj.rs",
    "D:\Dev\pepakura-next\core\src\export\export_stl.rs"
)

foreach ($file in $files) {
    if (Test-Path $file) {
        Write-Host "Исправляю: $file"
        $content = Get-Content $file -Raw -Encoding UTF8
        # Удаляем все обратные слеши
        $fixedContent = $content -replace '\\', ''
        $fixedContent | Set-Content $file -Encoding UTF8
    }
}

Write-Host "✅ Все файлы исправлены"
