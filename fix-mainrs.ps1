# D:\Dev\pepakura-next\fix-mainrs.ps1

$projectRoot = "D:\Dev\pepakura-next"
$mainRsPath  = Join-Path $projectRoot "ui-desktop\src-tauri\src\main.rs"

Write-Host "Исправляю файл:" $mainRsPath

if (!(Test-Path $mainRsPath)) {
    Write-Host "Файл main.rs не найден, проверь путь." -ForegroundColor Red
    exit 1
}

# Читаем исходный текст
$code = Get-Content $mainRsPath -Raw

# 1) Подключаем нужные типы (Rgb, Color, Point, Line, BufWriter)
if ($code -notmatch "use printpdf::Rgb as PdfRgb") {
    $code = $code -replace "use printpdf::\{([^}]+)\};", "use printpdf::{`$1, Rgb as PdfRgb, Point, Line as PdfLine};"
}

if ($code -notmatch "use std::io::BufWriter") {
    $code = $code -replace "use std::fs::File;", "use std::fs::File;`r`nuse std::io::BufWriter;"
}

# 2) Все Mm(.. as f64) -> Mm(.. as f32)
$code = $code -replace "Mm\(([^)]+) as f64\)", "Mm($1 as f32)"

# 3) Color::Rgb( r, g, b, None ) -> Color::Rgb(PdfRgb::new(r, g, b, None))
$code = $code -replace "Color::Rgb\(\s*([0-9\.]+)\s*,\s*([0-9\.]+)\s*,\s*([0-9\.]+)\s*,\s*None\s*\)",
                       "Color::Rgb(PdfRgb::new($1, $2, $3, None))"

# 4) Создание start/end как Point, а не (Mm, Mm)
$code = $code -replace "let start = \(Mm\(([^)]+)\), Mm\(([^)]+)\)\);",
                       "let start = Point::new(Mm($1 as f32), Mm($2 as f32));"
$code = $code -replace "let end = \(Mm\(([^)]+)\), Mm\(([^)]+)\)\);",
                       "let end = Point::new(Mm($1 as f32), Mm($2 as f32));"

# 5) Структура PdfLine без has_fill / has_stroke / is_clipping_path
$code = $code -replace "PdfLine\s*\{\s*points:\s*vec!\[\(start,\s*false\),\s*\(end,\s*false\)\],\s*is_closed:\s*false,\s*has_fill:\s*false,\s*has_stroke:\s*true,\s*is_clipping_path:\s*false,\s*\}",
                       "PdfLine { points: vec![(start, false), (end, false)], is_closed: false }"

# 6) set_outline_thickness ожидает число, а не Pt(...)
$code = $code -replace "layer\.set_outline_thickness\(Pt\(([^)]+)\)\);",
                       "layer.set_outline_thickness($1 as f32);"

# 7) save с BufWriter
$code = $code -replace "\.save\(&mut fs::File::create\(&path\)\.map_err\(\|e\| e\.to_string\(\)\)\?\)",
                       ".save(&mut BufWriter::new(File::create(&path).map_err(|e| e.to_string())?))"

# Сохраняем изменения
Set-Content -Path $mainRsPath -Value $code -Encoding UTF8

Write-Host "Готово. Попробуй снова:" -ForegroundColor Green
Write-Host '  cd "D:\Dev\pepakura-next\ui-desktop"' -ForegroundColor Yellow
Write-Host '  pnpm tauri dev' -ForegroundColor Yellow
