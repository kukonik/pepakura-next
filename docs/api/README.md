# Pepakura Core API Documentation

## Обзор

`pepakura_core` — это Rust-библиотека, предоставляющая ядро для генерации развёрток бумажных моделей (papercraft).

## Модули

### geometry

Модуль для работы с 3D-геометрией.

#### Vertex

```rust
pub struct Vertex {
    pub id: usize,
    pub position: [f64; 3],
    pub normal: Option<[f64; 3]>,
    pub uv: Option<[f64; 2]>,
}
```

**Методы:**
- `new(id, position)` — создать вершину
- `with_normal(id, position, normal)` — создать вершину с нормалью
- `with_normal_and_uv(id, position, normal, uv)` — создать вершину с нормалью и UV
- `distance_to(&other)` — расстояние до другой вершины
- `midpoint(&other)` — середина между вершинами

#### Face

```rust
pub struct Face {
    pub vertices: [usize; 3],
    pub material_id: Option<usize>,
}
```

**Методы:**
- `new(v1, v2, v3)` — создать грань
- `with_material(v1, v2, v3, material_id)` — создать грань с материалом
- `area(&vertices)` — вычислить площадь грани

#### Mesh

```rust
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub name: String,
    pub metadata: MeshMetadata,
}
```

**Методы:**
- `new(name)` — создать пустой меш
- `bounding_box()` — ограничивающий короб
- `centroid()` — центроид (средняя точка)
- `scale(factor)` — масштабировать
- `translate(offset)` — транслировать
- `center()` — центрировать в начале координат
- `validate()` — валидировать меш

### unfold

Модуль алгоритмов развёртки.

#### UnfoldConfig

```rust
pub struct UnfoldConfig {
    pub preserve_detail: bool,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub algorithm: UnfoldAlgorithm,
}
```

#### UnfoldAlgorithm

```rust
pub enum UnfoldAlgorithm {
    MDS,    // Multidimensional Scaling
    LSCM,   // Least Squares Conformal Maps
}
```

#### UnfoldedMesh

```rust
pub struct UnfoldedMesh {
    pub vertices_2d: Vec<[f64; 2]>,
    pub faces: Vec<Face>,
    pub source_mesh: Mesh,
    pub metadata: UnfoldMetadata,
}
```

#### Функции

##### unfold_mds

```rust
pub fn unfold_mds(mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh, UnfoldError>
```

Разворачивает меш используя MDS (Multidimensional Scaling).

**Алгоритм:**
1. Вычисляет матрицу попарных расстояний между вершинами (3D)
2. Применяет классический MDS для получения 2D-координат
3. Сохраняет топологию (грани) из исходного меша

##### unfold_simple_projection

```rust
pub fn unfold_simple_projection(mesh: &Mesh) -> Result<UnfoldedMesh, UnfoldError>
```

Простая развёртка через проекцию на плоскость.

### export

Модуль экспорта в различные форматы.

#### SvgExportConfig

```rust
pub struct SvgExportConfig {
    pub page_size: PageSize,
    pub scale: f64,
    pub show_vertex_ids: bool,
    pub show_fold_lines: bool,
    pub show_cut_lines: bool,
    pub show_part_numbers: bool,
}
```

#### PageSize

```rust
pub enum PageSize {
    A4,     // 210 × 297 мм
    A3,     // 297 × 420 мм
    A2,     // 420 × 594 мм
    A1,     // 594 × 841 мм
    Custom { width_mm: f64, height_mm: f64 },
}
```

#### Функции

##### export_svg

```rust
pub fn export_svg(unfolded: &UnfoldedMesh, config: &SvgExportConfig) -> Result<String, ExportError>
```

Экспортирует развёрнутый меш в SVG строку.

##### export_svg_to_file

```rust
pub fn export_svg_to_file(unfolded: &UnfoldedMesh, config: &SvgExportConfig, path: &str) -> Result<(), ExportError>
```

Экспортирует развёрнутый меш в SVG файл.

## Примеры использования

### Базовый пример

```rust
use pepakura_core::geometry::{Mesh, Vertex, Face};
use pepakura_core::unfold::{unfold_mds, UnfoldConfig};
use pepakura_core::export::{export_svg, SvgExportConfig};

// Создаём меш (треугольник)
let mut mesh = Mesh::new("Triangle");
mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));
mesh.add_face(Face::new(0, 1, 2));

// Разворачиваем
let config = UnfoldConfig::default();
let unfolded = unfold_mds(&mesh, &config).unwrap();

// Экспортируем в SVG
let svg_config = SvgExportConfig::default();
let svg = export_svg(&unfolded, &svg_config).unwrap();

// Сохраняем в файл
std::fs::write("output.svg", svg).unwrap();
```

### Импорт OBJ

```rust
use pepakura_core::geometry::Mesh;

let mesh = Mesh::load_from_obj("model.obj").unwrap();
```

### Работа с ошибками

```rust
use pepakura_core::unfold::{unfold_mds, UnfoldError};
use pepakura_core::geometry::Mesh;

let mesh = Mesh::new("Empty");
let config = UnfoldConfig::default();

match unfold_mds(&mesh, &config) {
    Ok(unfolded) => println!("Развёртка успешна!"),
    Err(UnfoldError::EmptyMesh) => println!("Меш пуст!"),
    Err(UnfoldError::TooFewVertices(n)) => println!("Мало вершин: {}", n),
    Err(UnfoldError::NoConvergence(iter)) => println!("Не сошлось за {} итераций", iter),
    Err(UnfoldError::NumericalError(msg)) => println!("Ошибка: {}", msg),
}
```

## Тестирование

```bash
cd crates/pepakura_core
cargo test
```

## Бенчмарки

```bash
cargo bench
```

## Лицензия

MIT
