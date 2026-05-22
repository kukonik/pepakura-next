# 📝 План улучшений для Pepakura Next

**Практическое руководство по внедрению**

---

## Быстрые улучшения (1-2 недели)

### 1. AI кэширование (3 дня)

**Файлы**:
```
crates/pepakura_core/src/ai/cache.rs (новый)
crates/pepakura_core/src/ai/assistant.rs (изменить)
```

**Реализация**:
```rust
// crates/pepakura_core/src/ai/cache.rs
use lru::LruCache;
use std::num::NonZeroUsize;
use sha2::{Sha256, Digest};

pub struct AiCache {
    cache: LruCache<String, String>,
}

impl AiCache {
    pub fn new(size: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(size).unwrap()),
        }
    }
    
    pub fn get(&mut self, prompt: &str) -> Option<&String> {
        let hash = self.hash_prompt(prompt);
        self.cache.get(&hash)
    }
    
    pub fn put(&mut self, prompt: &str, response: String) {
        let hash = self.hash_prompt(prompt);
        self.cache.put(hash, response);
    }
    
    fn hash_prompt(&self, prompt: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prompt.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
```

**Зависимости в Cargo.toml**:
```toml
[dependencies]
lru = "0.12"
sha2 = "0.10"
```

**Эффект**: Повторные запросы мгновенные.

---

### 2. Системный трей (3 дня)

**Файлы**:
```
src-tauri/src/tray.rs (новый)
src-tauri/src/main.rs (изменить)
```

**Реализация**:
```rust
// src-tauri/src/tray.rs
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
};

pub fn create_tray(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let show_i = MenuItem::with_id(app, "show", "Показать", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Выход", true, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;
    
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .build(app)?;
    
    Ok(())
}
```

**Эффект**: Удобный доступ из трея.

---

### 3. Персистентное состояние (5 дней)

**Файлы**:
```
src-tauri/src/state/persistence.rs (новый)
src-tauri/src/state/mod.rs (изменить)
```

**Реализация**:
```rust
// src-tauri/src/state/persistence.rs
use rusqlite::{Connection, Result};
use serde_json;

pub struct StatePersistence {
    conn: Connection,
}

impl StatePersistence {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    pub fn save<T: serde::Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let value_json = serde_json::to_string(value)?;
        self.conn.execute(
            "INSERT OR REPLACE INTO state (key, value) VALUES (?1, ?2)",
            [key, &value_json],
        )?;
        Ok(())
    }
    
    pub fn load<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        let value: String = self.conn.query_row(
            "SELECT value FROM state WHERE key = ?1",
            [key],
            |row| row.get(0),
        )?;
        
        Ok(serde_json::from_str(&value).ok())
    }
}
```

**Зависимости в Cargo.toml**:
```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
```

**Эффект**: Сохранение между запусками.

---

## Средние улучшения (2-4 недели)

### 4. LSCM алгоритм (2 недели)

**Файлы**:
```
crates/pepakura_core/src/unfold/lscm.rs (новый)
crates/pepakura_core/src/unfold/mod.rs (изменить)
```

**Структура**:
```rust
// crates/pepakura_core/src/unfold/lscm.rs
use nalgebra::{Matrix2, Vector2, DMatrix};

/// Least Squares Conformal Maps алгоритм
pub struct LscmUnfolder;

impl LscmUnfolder {
    pub fn unfold(mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh> {
        // 1. Построить матрицу весов рёбер
        // 2. Решить систему линейных уравнений
        // 3. Получить 2D координаты
        
        todo!()
    }
    
    fn build_laplacian(mesh: &Mesh) -> DMatrix<f64> {
        // Матрица Лапласа с котангенсными весами
        todo!()
    }
    
    fn solve_lscm(laplacian: &DMatrix<f64>) -> Vec<Vector2<f64>> {
        // Решить систему через собственные векторы
        todo!()
    }
}
```

**Эффект**: Лучшее сохранение углов.

---

### 5. PDF экспорт (1 неделя)

**Файлы**:
```
crates/pepakura_core/src/export/pdf.rs (новый)
crates/pepakura_core/src/export/mod.rs (изменить)
```

**Реализация**:
```rust
// crates/pepakura_core/src/export/pdf.rs
use printpdf::*;

pub fn export_pdf(unfolded: &UnfoldedMesh, config: &PdfConfig) -> Result<Vec<u8>> {
    let mut pdf_document = PdfDocument::new(
        "Pepakura Next Export",
        Mm(config.width),
        Mm(config.height),
        "Layer 1",
    );
    
    let layer = pdf_document.get_page(0).get_layer("Layer 1");
    
    // Добавить линии развёртки
    for face in &unfolded.faces {
        let path = create_path_for_face(unfolded, face);
        layer.add_shape(path);
    }
    
    // Сериализовать в bytes
    let mut bytes = Vec::new();
    pdf_document.save_to(&mut bytes)?;
    
    Ok(bytes)
}
```

**Зависимости в Cargo.toml**:
```toml
[dependencies]
printpdf = "0.5"
```

**Эффект**: Нативный PDF вместо browser print.

---

### 6. AI стриминг (1 неделя)

**Файлы**:
```
crates/pepakura_core/src/ai/streaming.rs (новый)
ui-desktop/src/composables/useAi.ts (изменить)
```

**Реализация (Rust)**:
```rust
// crates/pepakura_core/src/ai/streaming.rs
use futures::stream::Stream;

pub async fn chat_stream(
    client: &OllamaClient,
    messages: &[ChatMessage],
) -> Result<impl Stream<Item = String>> {
    // Использовать stream: true в Ollama API
    // Возвращать стрим токенов
    
    todo!()
}
```

**Реализация (TypeScript)**:
```typescript
// ui-desktop/src/composables/useAi.ts
export async function chatStream(message: string) {
  const response = await fetch(`${config.ollamaUrl}/api/chat`, {
    method: 'POST',
    body: JSON.stringify({
      model: config.model,
      messages: [{ role: 'user', content: message }],
      stream: true
    })
  });
  
  const reader = response.body.getReader();
  // Читать стрим
}
```

**Эффект**: Мгновенная обратная связь.

---

## Долгие улучшения (1-2 месяца)

### 7. MDS оптимизация (2 недели)

**Файлы**:
```
crates/pepakura_core/src/unfold/mds_optimizer.rs (новый)
crates/pepakura_core/src/unfold/mds.rs (изменить)
```

**Оптимизации**:
```rust
// 1. Sparse матрицы
use sprs::CsMatrix;

fn mds_sparse(distances: &CsMatrix<f64>) -> Result<Vec<[f64; 2]>> {
    // Использовать sparse матрицу для расстояний
}

// 2. Параллелизм
use rayon::prelude::*;

fn compute_distances_parallel(vertices: &[Vertex]) -> Vec<Vec<f64>> {
    vertices.par_iter().map(|v| {
        vertices.iter().map(|other| v.distance_to(other)).collect()
    }).collect()
}

// 3. Approximate nearest neighbors
use annoy::AnnoyIndex;

fn mds_approximate(vertices: &[Vertex]) -> Result<Vec<[f64; 2]>> {
    // Использовать approximate nearest neighbors
    // для больших мешей
}
```

**Эффект**: Ускорение в 3-5 раз.

---

### 8. Nesting оптимизация (2 недели)

**Файлы**:
```
crates/pepakura_core/src/nesting/genetic.rs (новый)
crates/pepakura_core/src/nesting/optimizer.rs (новый)
```

**Генетический алгоритм**:
```rust
pub struct GeneticNesting {
    population_size: usize,
    mutation_rate: f64,
    generations: usize,
}

impl GeneticNesting {
    pub fn optimize(&mut self, parts: &[Part], sheet_size: SheetSize) -> NestResult {
        // 1. Инициализировать популяцию
        // 2. Оценить fitness (меньше отходов = лучше)
        // 3. Селекция, кроссовер, мутация
        // 4. Повторять N поколений
        
        todo!()
    }
}
```

**Эффект**: Меньше отходов бумаги на 15-25%.

---

## Метрики успеха

После внедрения всех улучшений:

| Метрика | Было | Стало |
|---------|------|-------|
| Тесты покрытие | 65% | >80% |
| Время развёртки (1000 вершин) | 500ms | <100ms |
| Время AI ответа | 5-10 сек | 1-2 сек (стриминг) |
| PDF размер | N/A | <1 MB |
| Отходы бумаги | 20% | <10% |

---

## Чеклист внедрения

### Неделя 1
- [ ] AI кэширование
- [ ] Системный трей

### Неделя 2
- [ ] Персистентное состояние
- [ ] Начать LSCM

### Неделя 3
- [ ] Завершить LSCM
- [ ] Начать PDF экспорт

### Неделя 4
- [ ] Завершить PDF
- [ ] Начать AI стриминг

### Неделя 5
- [ ] Завершить AI стриминг
- [ ] Начать 3D viewer

### Неделя 6-8
- [ ] 3D viewer
- [ ] Тесты >80%

---

*План действий подготовлен на основе анализа*  
*21 марта 2026 г.*
