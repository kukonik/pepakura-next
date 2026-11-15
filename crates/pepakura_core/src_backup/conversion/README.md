# Конвертация PDO → PepaScene

## Обзор

Модуль `conversion` предоставляет полный набор инструментов для конвертации PDO файлов (формат Pepakura Designer) в унифицированное представление сцены `PepaScene`.

## Архитектура

```
crates/pepakura_core/src/conversion/
├── mod.rs                    # Основной API конвертации
├── traits.rs                 # Traits: FromPdoModel, ConvertWithProgress
├── config.rs                 # Конфигурация: ConversionConfig
├── error.rs                  # Ошибки: ConversionError
├── progress.rs               # Отслеживание прогресса
│
├── geometry/
│   ├── mod.rs                # Модуль геометрии
│   ├── vertex_converter.rs   # Конвертация вершин
│   ├── face_converter.rs     # Конвертация граней
│   ├── normal_calculator.rs  # Расчет нормалей
│   └── uv_mapper.rs          # UV-маппинг
│
├── materials/
│   ├── mod.rs                # Модуль материалов
│   ├── material_converter.rs # Конвертация материалов
│   └── texture_extractor.rs  # Извлечение текстур
│
└── validation/
    ├── mod.rs                # Модуль валидации
    ├── pdo_validator.rs      # Валидация PDO структуры
    ├── geometry_validator.rs # Валидация геометрии
    └── error_recovery.rs     # Восстановление после ошибок
```

## Быстрый старт

### Базовая конвертация

```rust
use pepakura_core::conversion::{convert_pdo_to_scene, ConversionConfig};

// Чтение PDO файла
let pdo_data = std::fs::read("model.pdo")?;

// Конвертация с конфигурацией по умолчанию
let config = ConversionConfig::default();
let scene = convert_pdo_to_scene(&pdo_data, &config)?;

// Доступ к данным
println!("Вершин: {}", scene.meshes[0].positions.len() / 3);
println!("Материалов: {}", scene.materials.len());
```

### Конвертация с прогрессом

```rust
use pepakura_core::conversion::{
    convert_pdo_to_scene_with_progress,
    ConversionConfig,
    SceneConversionProgress,
};

let pdo_data = std::fs::read("large_model.pdo")?;
let config = ConversionConfig::full();

let scene = convert_pdo_to_scene_with_progress(
    &pdo_data,
    &config,
    |progress| {
        println!(
            "[{:.1}%] {}: {}",
            progress.percent_complete(),
            progress.description(),
            progress.current_step()
        );
    }
)?;
```

## Конфигурация

### Режимы конвертации

```rust
// Быстрая конвертация (только геометрия)
let config = ConversionConfig::fast();

// Полная конвертация (все данные)
let config = ConversionConfig::full();

// Отладочная конвертация (строгая валидация)
let config = ConversionConfig::debug();

// Кастомная конфигурация
let config = ConversionConfig {
    extract_3d_geometry: true,
    extract_2d_unfolds: true,
    auto_generate_tabs: true,
    compute_normals: true,
    compute_uvs: true,
    create_texture_atlas: true,
    chunk_size: 10_000,
    async_threshold_vertices: 100_000,
    validation_mode: ValidationMode::Strict,
    error_handling: ErrorHandlingMode::Recover,
    max_vertices: 0,      // 0 = без лимита
    max_faces: 0,
};
```

### Режимы валидации

- `ValidationMode::None` - Пропускать проверки
- `ValidationMode::Lenient` - Предупреждать, но продолжать
- `ValidationMode::Strict` - Останавливаться при ошибках

### Режимы обработки ошибок

- `ErrorHandlingMode::FailFast` - Немедленно возвращать ошибку
- `ErrorHandlingMode::Recover` - Пытаться восстановить данные
- `ErrorHandlingMode::Skip` - Пропускать проблемные элементы

## Валидация

### Проверка PDO модели

```rust
use pepakura_core::conversion::validation::{
    validate_pdo_model,
    PdoValidator,
    GeometryValidator,
};

let pdo = PdoModel::parse_from_bytes(&data)?;

// Полная валидация
let result = validate_pdo_model(&pdo);
if !result.is_valid {
    for error in &result.errors {
        eprintln!("Ошибка: {}", error);
    }
}

// Быстрая проверка
if !PdoValidator::is_valid_quick(&pdo) {
    return Err("Invalid PDO structure".into());
}

// Проверка геометрии
let geo_result = GeometryValidator::validate(&pdo);
println!("Площадь поверхности: {}", GeometryValidator::compute_surface_area(&pdo));
println!("Объем: {}", GeometryValidator::compute_volume(&pdo));
println!("Замкнута ли модель: {}", GeometryValidator::is_watertight(&pdo));
```

### Восстановление поврежденных данных

```rust
use pepakura_core::conversion::validation::{
    try_recover_pdo,
    RecoveryStrategy,
};

// Попытка восстановления
let model = try_recover_pdo(&data)?;

// Или вручную
let mut model = PdoModel::parse_from_bytes(&data)?;
RecoveryStrategy::recover_model(&mut model);

// Дополнительные операции
RecoveryStrategy::scale_to_unit_size(&mut model);
RecoveryStrategy::center_model(&mut model);
RecoveryStrategy::simplify_model(&mut model);
```

## Работа с геометрией

### Конвертация вершин

```rust
use pepakura_core::conversion::geometry::{
    VertexConverter,
    convert_vertices,
};

let converter = VertexConverter::new(config.clone());
let vertices = converter.convert_vertices(&pdo_vertices)?;

// Вычисление bounding box
let bbox = converter.compute_bounding_box(&vertices);
```

### Конвертация граней

```rust
use pepakura_core::conversion::geometry::{
    FaceConverter,
    convert_faces,
};

let converter = FaceConverter::new(config.clone());
let faces = converter.convert_faces(&pdo_faces, vertex_count)?;

// Валидация
let errors = converter.validate_faces(&pdo_faces, vertex_count);
```

### Расчет нормалей

```rust
use pepakura_core::conversion::geometry::NormalCalculator;

let normals = NormalCalculator::compute_normals(&vertices, &faces);

// Проверка ориентации
let orientation = NormalCalculator::check_normal_orientation(
    &vertices, &normals, &faces
);
if orientation < 0.0 {
    println!("Нормали смотрят внутрь!");
}

// Инверсия нормалей
let mut flipped_normals = normals.clone();
NormalCalculator::flip_normals(&mut flipped_normals);
```

### UV-маппинг

```rust
use pepakura_core::conversion::geometry::UvMapper;

// Проекционная развертка
let uvs = UvMapper::project_uvs(&vertices);

// Сферическая развертка
let uvs = UvMapper::spherical_uvs(&vertices);

// Цилиндрическая развертка
let uvs = UvMapper::cylindrical_uvs(&vertices, 2); // Z-axis

// Box развертка
let uvs = UvMapper::box_uvs(&vertices, &faces);

// Трансформация UV
let mut uvs = UvMapper::project_uvs(&vertices);
UvMapper::scale_uvs(&mut uvs, [2.0, 2.0]);
UvMapper::translate_uvs(&mut uvs, [0.1, 0.1]);
UvMapper::rotate_uvs_90(&mut uvs, true);
```

## Работа с материалами

### Конвертация материалов

```rust
use pepakura_core::conversion::materials::{
    MaterialConverter,
    convert_materials,
};

let materials = convert_materials(&pdo.textures)?;

// Материал по умолчанию
let default_mat = MaterialConverter::create_default_material();

// Поиск материала
let mat = MaterialConverter::find_material_by_id(&materials, 42);
```

### Работа с текстурами

```rust
use pepakura_core::conversion::materials::{
    TextureExtractor,
    export_texture_to_png,
};

// Экспорт текстуры в PNG
if let Some(png_data) = export_texture_to_png(&pdo_texture) {
    std::fs::write("texture.png", png_data)?;
}

// Создание текстурного атласа
if let Some((atlas, regions)) = TextureExtractor::create_texture_atlas(&textures) {
    // atlas - изображение атласа
    // regions - информация о регионах
    for region in &regions {
        println!(
            "Texture {}: [{}x{}] at [{}, {}]",
            region.texture_id,
            region.width(),
            region.height(),
            region.uv_min[0],
            region.uv_min[1]
        );
    }
}

// Анализ текстуры
let avg_color = TextureExtractor::compute_average_color(&texture)?;
let is_transparent = TextureExtractor::is_transparent(&texture);
let is_pot = TextureExtractor::is_power_of_two(&texture);
```

## Обработка ошибок

```rust
use pepakura_core::conversion::{ConversionError, ErrorContext};

match convert_pdo_to_scene(&data, &config) {
    Ok(scene) => { /* ... */ }
    Err(ConversionError::InvalidVertexData { index, reason }) => {
        eprintln!("Ошибка вершины #{}: {}", index, reason);
    }
    Err(ConversionError::EncryptedPdo) => {
        eprintln!("Файл зашифрован! Требуется дешифровка.");
    }
    Err(ConversionError::LimitExceeded { limit_name, value, max }) => {
        eprintln!("Превышен лимит {}: {} (max {})", limit_name, value, max);
    }
    Err(e) => {
        let ctx = ErrorContext::new(e)
            .with_path("model.pdo")
            .with_suggestion("Проверьте целостность файла");
        eprintln!("{}", ctx);
    }
}
```

## Производительность

### Оптимизация для больших моделей

```rust
// Автоматическое переключение на параллельную обработку
let config = ConversionConfig {
    async_threshold_vertices: 100_000,
    chunk_size: 10_000,
    ..Default::default()
};

// Использование Rayon для параллелизма
// (встроено в конвертеры по умолчанию)
```

### Рекомендации

1. **Малые модели (< 10K вершин)**: Используйте `ConversionConfig::fast()`
2. **Средние модели (10K-100K)**: Используйте `ConversionConfig::default()`
3. **Большие модели (> 100K)**: Используйте `ConversionConfig::full()` с прогрессом
4. **Огромные модели (> 1M)**: Рассмотрите асинхронную обработку чанками

## Тестирование

```rust
#[cfg(test)]
mod tests {
    use pepakura_core::conversion::*;

    #[test]
    fn test_basic_conversion() {
        let data = include_bytes!("test_data/cube.pdo");
        let config = ConversionConfig::default();
        let scene = convert_pdo_to_scene(data, &config).unwrap();
        
        assert_eq!(scene.scene_version, "1.0");
        assert!(!scene.meshes.is_empty());
    }

    #[test]
    fn test_large_model_performance() {
        let data = include_bytes!("test_data/large.pdo");
        let config = ConversionConfig::fast();
        
        let start = std::time::Instant::now();
        let scene = convert_pdo_to_scene(data, &config).unwrap();
        let elapsed = start.elapsed();
        
        assert!(elapsed.as_secs() < 5);
        assert!(!scene.meshes.is_empty());
    }
}
```

## Интеграция с Tauri

```rust
use pepakura_core::conversion::{convert_pdo_to_scene, ConversionConfig};
use tauri::command;

#[command]
pub async fn parse_pdo_file(path: String) -> Result<PepaScene, String> {
    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| e.to_string())?;

    // Запуск в blocking thread для CPU-bound операции
    let config = ConversionConfig::default();
    tokio::task::spawn_blocking(move || {
        convert_pdo_to_scene(&data, &config)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())?
}
```

## См. также

- [`crate::pdo_parser`] - Парсинг PDO файлов
- [`crate::pepa_scene_adapter`] - Структуры PepaScene
- [`crate::geometry`] - Геометрические структуры
- [`crate::export`] - Экспорт в различные форматы
