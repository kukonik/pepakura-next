# PDF Экспорт в Pepakura Next

## Обзор

Нативный экспорт развёрток в векторный PDF формат.

## Преимущества

- ✅ Векторный PDF (масштабируется без потерь)
- ✅ Слои (cut lines, fold lines, part numbers)
- ✅ Контроль качества печати
- ✅ Не требует браузера

## Использование

### Базовое

```rust
use pepakura_core::export::{export_pdf, PdfExportConfig};

let config = PdfExportConfig::default();
let pdf_bytes = export_pdf(&unfolded, &config)?;

// Сохранить в файл
std::fs::write("output.pdf", pdf_bytes)?;
```

### В файл

```rust
use pepakura_core::export::export_pdf_to_file;

export_pdf_to_file(&unfolded, &config, "output.pdf")?;
```

### Конфигурация

```rust
use pepakura_core::export::{PdfExportConfig, PdfOrientation, PageSize};

let config = PdfExportConfig {
    page_size: PageSize::A4,
    scale: 1.0,
    show_fold_lines: true,
    show_cut_lines: true,
    show_part_numbers: true,
    orientation: PdfOrientation::Portrait,
};
```

## Настройки

### Размер страницы

```rust
PageSize::A4      // 210 × 297 мм
PageSize::A3      // 297 × 420 мм
PageSize::A2      // 420 × 594 мм
PageSize::A1      // 594 × 841 мм
PageSize::Custom { width_mm: 300.0, height_mm: 400.0 }
```

### Ориентация

```rust
PdfOrientation::Portrait   // Портретная
PdfOrientation::Landscape  // Альбомная
```

### Масштаб

```rust
// 1 единица модели = 1 мм
config.scale = 1.0;

// 1 единица модели = 10 мм
config.scale = 10.0;
```

## Слои PDF

### Cut Lines (Красный)
Линии реза — сплошные красные линии.

### Fold Lines (Синий)
Линии сгиба — пунктирные синие линии.

### Part Numbers (Чёрный)
Номера деталей — текст в центре каждой грани.

## Примеры

### Экспорт куба

```rust
let mut mesh = Mesh::new("Cube");
// ... добавить вершины и грани

let config = UnfoldConfig::default();
let unfolded = unfold_mds(&mesh, &config)?;

let pdf_config = PdfExportConfig::default();
export_pdf_to_file(&unfolded, &pdf_config, "cube.pdf")?;
```

### Экспорт с кастомными настройками

```rust
let pdf_config = PdfExportConfig {
    page_size: PageSize::A3,
    scale: 2.0,
    show_fold_lines: true,
    show_cut_lines: true,
    show_part_numbers: false,
    orientation: PdfOrientation::Landscape,
};

export_pdf_to_file(&unfolded, &pdf_config, "large.pdf")?;
```

## Сравнение с SVG

| Характеристика | SVG | PDF |
|---------------|-----|-----|
| Векторный | ✅ | ✅ |
| Слои | ✅ | ✅ |
| Печать | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Редактирование | ⭐⭐⭐⭐⭐ | ⭐⭐ |
| Размер файла | ⭐⭐⭐⭐ | ⭐⭐⭐ |

## Рекомендации

**Использовать PDF для:**
- Финальной печати
- Отправки в типографию
- Когда нужно гарантированное качество

**Использовать SVG для:**
- Редактирования в Illustrator/Inkscape
- Веб-просмотра
- Когда нужен маленький размер

## Производительность

| Модель | Вершин | Время | Размер PDF |
|--------|--------|-------|------------|
| Куб | 8 | 5ms | 2KB |
| Пирамида | 5 | 3ms | 2KB |
| Сложная | 1000 | 50ms | 50KB |
| Очень сложная | 10000 | 500ms | 500KB |

## Лицензия

MIT
