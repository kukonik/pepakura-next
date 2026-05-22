Write-Host "=== ИСПРАВЛЕНИЕ RUST-КОДА ==="

 = @(
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

foreach ( in ) {
    if (Test-Path ) {
        Write-Host "Исправляю: "
         = Get-Content  -Raw -Encoding UTF8
        # Удаляем все обратные слеши
         =  -replace '\\', ''
         | Set-Content  -Encoding UTF8
    }
}

Write-Host "✅ Все файлы исправлены"
