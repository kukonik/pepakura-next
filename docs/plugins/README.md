# Система плагинов Pepakura Core

## Обзор

Система плагинов позволяет расширять функциональность pepakura_core без изменения основного кода.

## Типы плагинов

### ImportPlugin

Плагин импорта моделей из файлов.

```rust
use pepakura_core::plugins::ImportPlugin;
use pepakura_core::geometry::Mesh;
use pepakura_core::PepakuraError;
use std::path::Path;

pub struct ObjImporter;

impl ImportPlugin for ObjImporter {
    fn name(&self) -> &str { "Wavefront OBJ" }
    
    fn supported_extensions(&self) -> &[&str] { &["obj"] }
    
    fn import(&self, path: &Path) -> Result<Mesh, PepakuraError> {
        // реализация
    }
}
```

### ExportPlugin

Плагин экспорта развёрток в файлы.

```rust
use pepakura_core::plugins::ExportPlugin;
use pepakura_core::export::UnfoldedMesh;
use pepakura_core::PepakuraError;
use std::path::Path;

pub struct PdfExporter;

impl ExportPlugin for PdfExporter {
    fn name(&self) -> &str { "PDF Exporter" }
    
    fn supported_extensions(&self) -> &[&str] { &["pdf"] }
    
    fn export(&self, unfolded: &UnfoldedMesh, path: &Path) -> Result<(), PepakuraError> {
        // реализация
    }
}
```

### UnfoldPlugin

Плагин альтернативного алгоритма развёртки.

```rust
use pepakura_core::plugins::UnfoldPlugin;
use pepakura_core::geometry::Mesh;
use pepakura_core::export::UnfoldedMesh;
use pepakura_core::unfold::UnfoldConfig;
use pepakura_core::PepakuraError;

pub struct LscmUnfolder;

impl UnfoldPlugin for LscmUnfolder {
    fn name(&self) -> &str { "LSCM" }
    
    fn description(&self) -> &str { 
        "Least Squares Conformal Maps" 
    }
    
    fn unfold(&self, mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh, PepakuraError> {
        // реализация LSCM
    }
}
```

## Реестр плагинов

### Регистрация

```rust
use pepakura_core::plugins::PluginRegistry;

let mut registry = PluginRegistry::new();

// Регистрация встроенных плагинов
registry.register_importer(Box::new(ObjImporter));
registry.register_exporter(Box::new(SvgExporter));
registry.register_unfolder(Box::new(LscmUnfolder));
```

### Использование

```rust
// Автоматический выбор плагина по расширению
let mesh = registry.import(Path::new("model.obj"))?;

// Экспорт
registry.export(&unfolded, Path::new("output.svg"))?;

// Выбор конкретного алгоритма развёртки
let unfolded = registry.unfold_with(&mesh, "LSCM", &config)?;
```

### Встроенные плагины

```rust
use pepakura_core::plugins::create_builtin_registry;

// Реестр со всеми встроенными плагинами
let registry = create_builtin_registry();

// Доступные плагины:
// - Import: OBJ
// - Export: SVG
// - Unfold: Simple Projection
```

## Создание внешнего плагина

### Структура проекта

```
my-plugin/
├── Cargo.toml
└── src/
    └── lib.rs
```

### Cargo.toml

```toml
[package]
name = "my-lscm-plugin"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
pepakura_core = { version = "0.1" }
```

### lib.rs

```rust
use pepakura_core::plugins::{UnfoldPlugin, PluginMetadata};
use pepakura_core::geometry::Mesh;
use pepakura_core::export::UnfoldedMesh;
use pepakura_core::unfold::UnfoldConfig;
use pepakura_core::PepakuraError;

pub struct LscmPlugin;

impl UnfoldPlugin for LscmPlugin {
    fn name(&self) -> &str { "LSCM Pro" }
    
    fn unfold(&self, mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh, PepakuraError> {
        // реализация
    }
}

// Экспорт функции для создания плагина
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn UnfoldPlugin> {
    Box::new(LscmPlugin)
}
```

## Тестирование плагинов

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_importer() {
        let importer = ObjImporter;
        assert!(importer.supports_extension("obj"));
        
        let mesh = importer.import(Path::new("test.obj")).unwrap();
        assert!(!mesh.vertices.is_empty());
    }
    
    #[test]
    fn test_exporter() {
        let exporter = SvgExporter;
        let unfolded = create_test_unfolded();
        
        let result = exporter.export(&unfolded, Path::new("test.svg"));
        assert!(result.is_ok());
    }
}
```

## API Reference

### PluginRegistry

| Метод | Описание |
|-------|----------|
| `new()` | Создать пустой реестр |
| `register_importer(plugin)` | Зарегистрировать импортёр |
| `register_exporter(plugin)` | Зарегистрировать экспортёр |
| `register_unfolder(plugin)` | Зарегистрировать развёртку |
| `import(path)` | Импортировать модель |
| `export(unfolded, path)` | Экспортировать развёртку |
| `unfold_with(mesh, name, config)` | Развернуть алгоритмом |
| `list_importers()` | Список импортёров |
| `list_exporters()` | Список экспортёров |
| `list_unfolders()` | Список развёрток |

### ImportPlugin

| Метод | Описание |
|-------|----------|
| `name()` | Имя плагина |
| `supported_extensions()` | Поддерживаемые расширения |
| `import(path)` | Импортировать из файла |
| `supports_extension(ext)` | Проверка расширения |

### ExportPlugin

| Метод | Описание |
|-------|----------|
| `name()` | Имя плагина |
| `supported_extensions()` | Поддерживаемые расширения |
| `export(unfolded, path)` | Экспортировать в файл |
| `supports_extension(ext)` | Проверка расширения |

### UnfoldPlugin

| Метод | Описание |
|-------|----------|
| `name()` | Имя плагина |
| `description()` | Описание алгоритма |
| `unfold(mesh, config)` | Развернуть меш |

## Примеры

### Добавление поддержки STL

```rust
pub struct StlImporter;

impl ImportPlugin for StlImporter {
    fn name(&self) -> &str { "STL Importer" }
    
    fn supported_extensions(&self) -> &[&str] { &["stl"] }
    
    fn import(&self, path: &Path) -> Result<Mesh, PepakuraError> {
        let content = std::fs::read(path)?;
        // Парсинг STL (binary или ASCII)
        todo!()
    }
}
```

### Добавление экспорта в PDF

```rust
pub struct PdfExporter;

impl ExportPlugin for PdfExporter {
    fn name(&self) -> &str { "PDF Exporter" }
    
    fn supported_extensions(&self) -> &[&str] { &["pdf"] }
    
    fn export(&self, unfolded: &UnfoldedMesh, path: &Path) -> Result<(), PepakuraError> {
        use printpdf::*;
        // Генерация PDF
        todo!()
    }
}
```

## Лучшие практики

1. **Thread-safe**: Плагины должны реализовывать `Send + Sync`
2. **Без паник**: Возвращайте `Result` вместо `unwrap()`
3. **Валидация**: Проверяйте входные данные
4. **Документация**: Документируйте публичный API
5. **Тесты**: Пишите тесты для всех плагинов

## Лицензия

MIT
