# Примеры использования модуля конвертации

## 1. Базовая конвертация PDO → PepaScene

```rust
use pepakura_core::conversion::{convert_pdo_to_scene, ConversionConfig};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Чтение PDO файла
    let pdo_data = fs::read("model.pdo")?;
    
    // Конвертация с конфигурацией по умолчанию
    let config = ConversionConfig::default();
    let scene = convert_pdo_to_scene(&pdo_data, &config)?;
    
    // Вывод информации о сцене
    println!("Версия сцены: {}", scene.scene_version);
    println!("Количество мешей: {}", scene.meshes.len());
    println!("Количество материалов: {}", scene.materials.len());
    
    if let Some(bbox) = &scene.bounding_box {
        println!("Bounding box: min={:?}, max={:?}", bbox.min, bbox.max);
    }
    
    // Доступ к данным первого меша
    if let Some(mesh) = scene.meshes.first() {
        let vertex_count = mesh.positions.len() / 3;
        let face_count = mesh.indices.len() / 3;
        println!("Вершин: {}, Граней: {}", vertex_count, face_count);
    }
    
    Ok(())
}
```

## 2. Конвертация с прогрессом

```rust
use pepakura_core::conversion::{
    convert_pdo_to_scene_with_progress,
    ConversionConfig,
    SceneConversionProgress,
    ConversionStage,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdo_data = std::fs::read("large_model.pdo")?;
    let config = ConversionConfig::full();
    
    println!("Начинается конвертация большой модели...");
    
    let scene = convert_pdo_to_scene_with_progress(
        &pdo_data,
        &config,
        |progress: &SceneConversionProgress| {
            // Обновление прогресс-бара в UI
            println!(
                "[{:5.1}%] {:<30} (этап {}/{})",
                progress.percent_complete(),
                progress.description(),
                progress.current_step(),
                progress.total_steps()
            );
            
            // Проверка на конкретный этап
            if progress.current_stage == ConversionStage::NormalCalculation {
                println!("  → Вычисление нормалей...");
            }
        }
    )?;
    
    println!("Конвертация завершена!");
    Ok(())
}
```

## 3. Валидация перед конвертацией

```rust
use pepakura_core::{
    conversion::{
        validate_pdo_model,
        PdoValidator,
        GeometryValidator,
        ValidationResult,
    },
    pdo_parser::PdoModel,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdo_data = std::fs::read("model.pdo")?;
    let pdo = PdoModel::parse_from_bytes(&pdo_data)?;
    
    // Быстрая проверка
    if !PdoValidator::is_valid_quick(&pdo) {
        eprintln!("Модель не прошла быструю проверку!");
        return Err("Invalid model".into());
    }
    
    // Полная валидация
    let result = validate_pdo_model(&pdo);
    
    if !result.is_valid {
        eprintln!("Ошибки валидации:");
        for error in &result.errors {
            eprintln!("  ❌ {}", error);
        }
    }
    
    if !result.warnings.is_empty() {
        println!("Предупреждения:");
        for warning in &result.warnings {
            println!("  ⚠️  {}", warning);
        }
    }
    
    // Детальная проверка геометрии
    let geo_result = GeometryValidator::validate(&pdo);
    
    // Метрики геометрии
    let area = GeometryValidator::compute_surface_area(&pdo);
    let volume = GeometryValidator::compute_volume(&pdo);
    let is_watertight = GeometryValidator::is_watertight(&pdo);
    
    println!("Площадь поверхности: {:.2} кв.ед.", area);
    println!("Объем: {:.2} куб.ед.", volume);
    println!("Модель замкнута: {}", if is_watertight { "да" } else { "нет" });
    
    Ok(())
}
```

## 4. Восстановление поврежденной модели

```rust
use pepakura_core::{
    conversion::{
        try_recover_pdo,
        RecoveryStrategy,
        ConversionConfig,
        convert_pdo_to_scene,
    },
    pdo_parser::PdoModel,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdo_data = std::fs::read("damaged_model.pdo")?;
    
    // Попытка автоматического восстановления
    match try_recover_pdo(&pdo_data) {
        Ok(mut model) => {
            println!("Модель успешно восстановлена!");
            
            // Дополнительные операции восстановления
            RecoveryStrategy::remove_duplicate_vertices(&mut model);
            RecoveryStrategy::remove_isolated_vertices(&mut model);
            RecoveryStrategy::simplify_model(&mut model);
            
            // Нормализация модели
            RecoveryStrategy::center_model(&mut model);
            RecoveryStrategy::scale_to_unit_size(&mut model);
            
            // Конвертация восстановленной модели
            let config = ConversionConfig::default();
            let scene = convert_pdo_to_scene(&pdo_data, &config)?;
            
            println!("Конвертация успешна!");
        }
        Err(e) => {
            eprintln!("Не удалось восстановить модель: {}", e);
            return Err(e.into());
        }
    }
    
    Ok(())
}
```

## 5. Работа с материалами и текстурами

```rust
use pepakura_core::{
    conversion::{
        materials::{
            MaterialConverter,
            TextureExtractor,
            export_texture_to_png,
        },
        ConversionConfig,
        convert_pdo_to_scene,
    },
    pdo_parser::PdoModel,
};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdo_data = fs::read("textured_model.pdo")?;
    let pdo = PdoModel::parse_from_bytes(&pdo_data)?;
    
    // Конвертация материалов
    let materials = MaterialConverter::convert_materials(&pdo.textures)?;
    
    println!("Материалы:");
    for mat in &materials {
        println!("  - {} (ID: {})", mat.name, mat.id);
        if let Some(tex_id) = mat.texture_id {
            println!("    Текстура: #{}", tex_id);
        }
    }
    
    // Экспорт текстур в PNG
    for (idx, texture) in pdo.textures.iter().enumerate() {
        if let Some(png_data) = export_texture_to_png(texture) {
            let filename = format!("texture_{}.png", idx);
            fs::write(&filename, png_data)?;
            println!("Экспортирована текстура: {}", filename);
        }
        
        // Анализ текстуры
        if let Some(avg_color) = TextureExtractor::compute_average_color(texture) {
            println!(
                "  Средний цвет: R={:.0}, G={:.0}, B={:.0}, A={:.0}",
                avg_color[0] * 255.0,
                avg_color[1] * 255.0,
                avg_color[2] * 255.0,
                avg_color[3] * 255.0
            );
        }
        
        println!("  Прозрачная: {}", TextureExtractor::is_transparent(texture));
        println!("  Power-of-two: {}", TextureExtractor::is_power_of_two(texture));
    }
    
    // Создание текстурного атласа
    if let Some((atlas, regions)) = TextureExtractor::create_texture_atlas_grid(&pdo.textures, 2) {
        println!("\nТекстурный атлас:");
        println!("  Размер: {}x{}", atlas.width(), atlas.height());
        println!("  Регионов: {}", regions.len());
        
        for region in &regions {
            println!(
                "    Текстура #{}: [{}x{}] @ [{}, {}]",
                region.texture_id,
                region.width(),
                region.height(),
                region.x,
                region.y
            );
        }
        
        // Экспорт атласа
        let mut atlas_png = Vec::new();
        atlas.write_to(
            &mut std::io::Cursor::new(&mut atlas_png),
            image::ImageFormat::Png
        )?;
        fs::write("texture_atlas.png", atlas_png)?;
    }
    
    Ok(())
}
```

## 6. UV-маппинг

```rust
use pepakura_core::{
    conversion::geometry::UvMapper,
    geometry::{Mesh, Vertex, Face},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создание тестового меша
    let mut mesh = Mesh::new("TestCube");
    
    // Добавление вершин куба
    mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
    mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
    mesh.add_vertex(Vertex::new(2, [1.0, 1.0, 0.0]));
    mesh.add_vertex(Vertex::new(3, [0.0, 1.0, 0.0]));
    mesh.add_vertex(Vertex::new(4, [0.0, 0.0, 1.0]));
    mesh.add_vertex(Vertex::new(5, [1.0, 0.0, 1.0]));
    mesh.add_vertex(Vertex::new(6, [1.0, 1.0, 1.0]));
    mesh.add_vertex(Vertex::new(7, [0.0, 1.0, 1.0]));
    
    // Добавление граней
    mesh.add_face(Face::new(0, 1, 2));
    mesh.add_face(Face::new(0, 2, 3));
    // ... остальные грани
    
    // Проекционная развертка (XY плоскость)
    let uvs = UvMapper::project_uvs(&mesh.vertices);
    println!("Проекционные UV: {:?}", uvs);
    
    // Сферическая развертка
    let uvs = UvMapper::spherical_uvs(&mesh.vertices);
    println!("Сферические UV: {:?}", uvs);
    
    // Цилиндрическая развертка (Z ось)
    let uvs = UvMapper::cylindrical_uvs(&mesh.vertices, 2);
    println!("Цилиндрические UV: {:?}", uvs);
    
    // Box развертка
    let uvs = UvMapper::box_uvs(&mesh.vertices, &mesh.faces);
    println!("Box UV: {:?}", uvs);
    
    // Трансформация UV
    let mut uvs = UvMapper::project_uvs(&mesh.vertices);
    
    // Масштабирование
    UvMapper::scale_uvs(&mut uvs, [2.0, 2.0]);
    
    // Сдвиг
    UvMapper::translate_uvs(&mut uvs, [0.1, 0.1]);
    
    // Поворот на 90 градусов
    UvMapper::rotate_uvs_90(&mut uvs, true);
    
    // Упаковка
    UvMapper::pack_uvs(&mut uvs, 0.01);
    
    Ok(())
}
```

## 7. Конвертация с различными конфигурациями

```rust
use pepakura_core::conversion::{ConversionConfig, ValidationMode, ErrorHandlingMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pdo_data = std::fs::read("model.pdo")?;
    
    // Быстрая конвертация (только геометрия)
    let config = ConversionConfig::fast();
    let _scene = pepakura_core::conversion::convert_pdo_to_scene(&pdo_data, &config)?;
    
    // Полная конвертация (все данные)
    let config = ConversionConfig::full();
    let _scene = pepakura_core::conversion::convert_pdo_to_scene(&pdo_data, &config)?;
    
    // Отладочная конвертация (строгая валидация)
    let config = ConversionConfig::debug();
    match pepakura_core::conversion::convert_pdo_to_scene(&pdo_data, &config) {
        Ok(scene) => println!("Конвертация успешна"),
        Err(e) => eprintln!("Ошибка (ожидаемо в debug режиме): {}", e),
    }
    
    // Кастомная конфигурация
    let config = ConversionConfig {
        extract_3d_geometry: true,
        extract_2d_unfolds: false,
        auto_generate_tabs: false,
        compute_normals: true,
        compute_uvs: true,
        create_texture_atlas: false,
        chunk_size: 5000,
        async_threshold_vertices: 50000,
        validation_mode: ValidationMode::Lenient,
        error_handling: ErrorHandlingMode::Recover,
        max_vertices: 1_000_000,
        max_faces: 5_000_000,
    };
    
    let scene = pepakura_core::conversion::convert_pdo_to_scene(&pdo_data, &config)?;
    
    Ok(())
}
```

## 8. Обработка ошибок

```rust
use pepakura_core::{
    conversion::{
        convert_pdo_to_scene,
        ConversionConfig,
        ConversionError,
        ErrorContext,
    },
    PepakuraError,
};

fn main() {
    let pdo_data = std::fs::read("model.pdo").unwrap();
    let config = ConversionConfig::default();
    
    match convert_pdo_to_scene(&pdo_data, &config) {
        Ok(scene) => {
            println!("Успех!");
        }
        Err(ConversionError::InvalidVertexData { index, reason }) => {
            eprintln!("Ошибка вершины #{}: {}", index, reason);
        }
        Err(ConversionError::InvalidFaceData { index, reason }) => {
            eprintln!("Ошибка грани #{}: {}", index, reason);
        }
        Err(ConversionError::InvalidVertexIndex { face_index, vertex_index, max_valid }) => {
            eprintln!(
                "Грань #{} ссылается на несуществующую вершину #{} (max: {})",
                face_index, vertex_index, max_valid
            );
        }
        Err(ConversionError::EncryptedPdo) => {
            eprintln!("Файл зашифрован! Используйте инструмент дешифровки.");
        }
        Err(ConversionError::LimitExceeded { limit_name, value, max }) => {
            eprintln!(
                "Превышен лимит {}: {} (максимум: {})",
                limit_name, value, max
            );
        }
        Err(e) => {
            // Контекстуализированная ошибка
            let ctx = ErrorContext::new(e)
                .with_path("model.pdo")
                .with_suggestion("Проверьте целостность файла")
                .with_suggestion("Попробуйте режим Recovery");
            
            eprintln!("{}", ctx);
        }
    }
}
```

## 9. Интеграция с Tauri

```rust
// src-tauri/src/commands.rs

use pepakura_core::conversion::{convert_pdo_to_scene, ConversionConfig};
use pepakura_core::PepaScene;
use tauri::command;

#[command]
pub async fn parse_pdo_file(path: String) -> Result<PepaScene, String> {
    // Чтение файла
    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    // Конвертация в blocking thread (CPU-bound операция)
    let config = ConversionConfig::default();
    tokio::task::spawn_blocking(move || {
        convert_pdo_to_scene(&data, &config)
            .map_err(|e| format!("Conversion error: {}", e))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[command]
pub async fn parse_pdo_with_progress(
    path: String,
    window: tauri::Window,
) -> Result<PepaScene, String> {
    let data = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let config = ConversionConfig::full();
    
    // Конвертация с обновлением прогресса в UI
    tokio::task::spawn_blocking(move || {
        convert_pdo_to_scene_with_progress(
            &data,
            &config,
            |progress| {
                // Отправка события в frontend
                let _ = window.emit(
                    "conversion-progress",
                    serde_json::json!({
                        "percent": progress.percent_complete(),
                        "description": progress.description(),
                        "stage": format!("{:?}", progress.current_stage),
                    })
                );
            }
        )
        .map_err(|e| format!("Conversion error: {}", e))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}
```

## 10. Benchmark тест

```rust
// benches/conversion_bench.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pepakura_core::conversion::{convert_pdo_to_scene, ConversionConfig};

fn bench_small_model(c: &mut Criterion) {
    let data = include_bytes!("../test_data/cube.pdo");
    let config = ConversionConfig::default();
    
    c.bench_function("convert_small_model", |b| {
        b.iter(|| {
            convert_pdo_to_scene(black_box(data), black_box(&config)).unwrap()
        })
    });
}

fn bench_large_model(c: &mut Criterion) {
    let data = include_bytes!("../test_data/large_model.pdo");
    let config = ConversionConfig::fast();
    
    c.bench_function("convert_large_model", |b| {
        b.iter(|| {
            convert_pdo_to_scene(black_box(data), black_box(&config)).unwrap()
        })
    });
}

fn bench_full_conversion(c: &mut Criterion) {
    let data = include_bytes!("../test_data/medium_model.pdo");
    let config = ConversionConfig::full();
    
    c.bench_function("full_conversion", |b| {
        b.iter(|| {
            convert_pdo_to_scene(black_box(data), black_box(&config)).unwrap()
        })
    });
}

criterion_group!(
    benches,
    bench_small_model,
    bench_large_model,
    bench_full_conversion
);
criterion_main!(benches);
```

---

Эти примеры покрывают все основные сценарии использования модуля конвертации.
