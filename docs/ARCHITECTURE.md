# 🏗️ Архитектура Pepakura Next

**Технический обзор** для разработчиков.

---

## 📊 Общая архитектура

```
┌─────────────────────────────────────────────────────────┐
│                   Frontend (Vue 3)                      │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Components: Viewer3D, UnfoldEditor, Workspace   │  │
│  │  Stores: ai.store, project.store                 │  │
│  │  Composables: useAi, useViewLinking, useTauri    │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                         ↕ invoke() (Tauri IPC)
┌─────────────────────────────────────────────────────────┐
│                   Tauri Backend (Rust)                  │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Commands: parse, unfold, export, ai_*           │  │
│  │  State: AppState (DashMap)                       │  │
│  └──────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
                         ↕ function calls
┌─────────────────────────────────────────────────────────┐
│                   pepakura_core (Rust)                  │
│  ┌──────────┬──────────┬──────────┬──────────────────┐ │
│  │ geometry │  unfold  │  export  │  plugins / ai    │ │
│  │  Mesh    │   MDS    │   SVG    │  Ollama / LSCM   │ │
│  │  Vertex  │   LSCM   │   PDF    │  Cache / Stream  │ │
│  │  Face    │          │          │                  │ │
│  └──────────┴──────────┴──────────┴──────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

---

## PlatformBridge (обновлено)

- Все IPC вызовы → через `usePlatform().invoke()`
- Тесты: `shared/src/composables/__tests__/usePlatform.test.ts`
- E2E: `platform/desktop/ui-desktop/e2e/`

---

## 📁 Структура проекта

```
pepakura-next/
├── crates/
│   └── pepakura_core/           # Rust ядро (библиотека)
│       ├── src/
│       │   ├── ai/              # AI модуль
│       │   │   ├── mod.rs
│       │   │   ├── config.rs    # Конфигурация AI
│       │   │   ├── client.rs    # HTTP клиенты (Ollama, OpenAI)
│       │   │   ├── assistant.rs # PepakuraAssistant
│       │   │   ├── cache.rs     # LRU кэширование
│       │   │   └── streaming.rs # SSE стриминг
│       │   ├── geometry/        # Геометрия мешей
│       │   │   ├── mod.rs
│       │   │   ├── vertex.rs    # Vertex структура
│       │   │   └── mesh.rs      # Mesh структура + методы
│       │   ├── unfold/          # Алгоритмы развёртки
│       │   │   ├── mod.rs
│       │   │   ├── mds.rs       # MDS алгоритм
│       │   │   └── lscm.rs      # LSCM алгоритм
│       │   ├── export/          # Экспорт форматов
│       │   │   ├── mod.rs
│       │   │   ├── svg.rs       # SVG экспорт
│       │   │   └── pdf.rs       # PDF экспорт
│       │   ├── plugins/         # Система плагинов
│       │   │   ├── mod.rs
│       │   │   ├── traits.rs    # Трейты плагинов
│       │   │   ├── registry.rs  # Реестр плагинов
│       │   │   └── builtin.rs   # Встроенные плагины
│       │   ├── error.rs         # Типы ошибок
│       │   ├── nesting.rs       # Раскладка на листе
│       │   ├── pdo_parser.rs    # PDO парсер
│       │   └── lib.rs           # Публичный API
│       └── Cargo.toml
├── src-tauri/                   # Tauri приложение
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── commands.rs          # Tauri IPC команды
│   │   ├── ai_commands.rs       # AI команды
│   │   └── state.rs             # Глобальное состояние
│   ├── tests/
│   │   └── integration_tests.rs # Integration тесты
│   ├── Cargo.toml
│   └── tauri.conf.json
├── ui-desktop/                  # Vue 3 frontend
│   ├── src/
│   │   ├── components/
│   │   │   ├── viewer/
│   │   │   │   └── Viewer3D.vue         # 3D вьювер (Three.js)
│   │   │   ├── editor/
│   │   │   │   └── UnfoldEditor.vue     # 2D редактор
│   │   │   ├── workspace/
│   │   │   │   └── Workspace.vue        # Объединённый вид
│   │   │   ├── ai/
│   │   │   │   ├── AiAssistantPanel.vue # AI чат
│   │   │   │   └── InstructionExport.vue # Экспорт инструкций
│   │   │   └── dashboard/
│   │   ├── composables/
│   │   │   ├── useAi.ts                 # AI функции
│   │   │   ├── useAi.test.ts
│   │   │   ├── useViewLinking.ts        # Синхронизация 2D/3D
│   │   │   └── useViewLinking.test.ts
│   │   ├── stores/
│   │   │   ├── ai.store.ts              # AI store (Pinia)
│   │   │   └── project.store.ts         # Projects store
│   │   ├── views/
│   │   │   └── MainLayout.vue           # Главный UI
│   │   ├── App.vue
│   │   └── main.ts
│   ├── tests/
│   │   └── e2e/
│   │       └── app.spec.ts              # E2E тесты (Playwright)
│   ├── playwright.config.ts
│   ├── package.json
│   └── vite.config.ts
├── docs/                        # Документация
│   ├── api/                     # API документация
│   │   ├── README.md
│   │   ├── LSCM.md
│   │   └── PDF_EXPORT.md
│   ├── user-guide/              # Руководство пользователя
│   │   └── README.md
│   ├── ai/                      # AI документация
│   │   ├── CACHING.md
│   │   ├── STREAMING.md
│   │   └── README.md
│   └── ui/                      # UI документация
│       └── VIEWER_EDITOR.md
├── scripts/                     # Build скрипты
│   ├── build-windows.ps1
│   └── health-check.ps1
├── .github/workflows/           # CI/CD
│   ├── test.yml
│   └── release.yml
├── PROMPTS.md                   # Промпты разработки
├── IMPLEMENTATION_REPORT.md     # Отчёт о реализации
├── FINAL_COMPLETE.md            # Финальный отчёт
└── README.md                    # Описание проекта
```

---

## 🔄 Поток данных

### 1. Импорт модели

```
User (UI)
  ↓ "Импортировать"
Viewer3D.vue
  ↓ invoke('import_3d_model', path)
Tauri (commands.rs)
  ↓ Mesh::load_from_obj(path)
pepakura_core::geometry::Mesh
  ↓
Mesh { vertices, faces, name }
  ↑
Tauri State (AppState.meshes)
  ↓
Viewer3D.vue (отображение)
```

### 2. Развёртка

```
User (UI)
  ↓ "Развернуть"
Workspace.vue
  ↓ invoke('unfold_3d_model', mesh_id, config)
Tauri (commands.rs)
  ↓ unfold_mds(&mesh, &config)
pepakura_core::unfold::mds
  ↓
1. Матрица расстояний (3D)
2. MDS классический
3. 2D координаты
  ↓
UnfoldedMesh { vertices_2d, faces }
  ↑
Tauri State (AppState.unfolded)
  ↓
UnfoldEditor.vue (отображение 2D)
```

### 3. Экспорт

```
User (UI)
  ↓ "Экспорт PDF"
UnfoldEditor.vue
  ↓ invoke('export_pdf', unfolded_id, config)
Tauri (commands.rs)
  ↓ export_pdf(&unfolded, &config)
pepakura_core::export::pdf
  ↓
1. Создать PDF документ
2. Добавить слои
3. Сериализовать в bytes
  ↓
Vec<u8> (PDF bytes)
  ↓
Сохранить в файл
```

### 4. AI запрос

```
User (UI)
  ↓ "Как выбрать бумагу?"
AiAssistantPanel.vue
  ↓ invoke('ai_chat', message)
Tauri (ai_commands.rs)
  ↓ assistant.answer_question(&message)
pepakura_core::ai::assistant
  ↓
1. Проверить кэш
2. Если нет → Ollama API
3. Сохранить в кэш
  ↓
String (ответ AI)
  ↓
Отобразить в чате
```

---

## 🔑 Ключевые компоненты

### pepakura_core (Rust)

**Модули:**

| Модуль | Файлы | Строк | Покрытие |
|--------|-------|-------|----------|
| geometry | vertex.rs, mesh.rs | 600+ | 95% |
| unfold | mds.rs, lscm.rs | 800+ | 90% |
| export | svg.rs, pdf.rs | 500+ | 85% |
| plugins | traits.rs, registry.rs, builtin.rs | 600+ | 90% |
| ai | config.rs, client.rs, assistant.rs, cache.rs, streaming.rs | 1200+ | 85% |
| nesting | nesting.rs | 400+ | 80% |

**Публичный API:**
```rust
pub use geometry::{Mesh, Vertex, Face, BoundingBox};
pub use unfold::{UnfoldedMesh, UnfoldConfig, unfold_mds, unfold_lscm};
pub use export::{SvgExportConfig, PdfExportConfig, export_svg, export_pdf};
pub use plugins::{PluginRegistry, ImportPlugin, ExportPlugin};
pub use ai::{AiConfig, PepakuraAssistant, chat_stream};
```

### Tauri Backend (Rust)

**Команды:**
```rust
#[tauri::command]
pub async fn import_3d_model(path: String) -> Result<Mesh, String>

#[tauri::command]
pub async fn unfold_3d_model(
    mesh_id: usize,
    config: UnfoldConfig
) -> Result<UnfoldedMesh, String>

#[tauri::command]
pub async fn export_pdf(
    unfolded_id: usize,
    config: PdfConfig
) -> Result<Vec<u8>, String>

#[tauri::command]
pub async fn ai_chat(
    message: String,
    history: Vec<ChatMessage>
) -> Result<String, String>
```

### Frontend (Vue 3)

**Компоненты:**

| Компонент | Строк | Описание |
|-----------|-------|----------|
| Viewer3D.vue | 400+ | 3D вьювер (Three.js) |
| UnfoldEditor.vue | 350+ | 2D редактор (SVG) |
| Workspace.vue | 200+ | Объединённый вид |
| AiAssistantPanel.vue | 250+ | AI чат панель |
| MainLayout.vue | 150+ | Главный UI |

**Composables:**
```typescript
// useAi.ts
export function useAi() {
  return {
    checkStatus,
    getUnfoldAdvice,
    generateInstructions,
    chat,
    updateConfig,
    getConfig,
    recommendPaper
  }
}

// useViewLinking.ts
export function useViewLinking() {
  return {
    selectedFace2D,
    selectedFace3D,
    selectFace2D,
    selectFace3D,
    isFaceSelected,
    clearSelection
  }
}
```

---

## 🔧 Технологии

### Backend (Rust)

| Зависимость | Версия | Назначение |
|-------------|--------|------------|
| serde | 1.0 | Сериализация |
| nalgebra | 0.32 | Линейная алгебра |
| thiserror | 1.0 | Обработка ошибок |
| reqwest | 0.11 | HTTP клиент (AI) |
| tokio | 1 | Async runtime |
| lru | 0.12 | Кэширование |
| printpdf | 0.5 | PDF экспорт |
| futures | 0.3 | Стриминг |

### Frontend (TypeScript)

| Зависимость | Версия | Назначение |
|-------------|--------|------------|
| vue | 3.4 | Framework |
| pinia | 3.0 | State management |
| three | 0.183 | 3D графика |
| @tauri-apps/api | 2.10 | Tauri IPC |
| playwright/test | 1.40 | E2E тесты |
| vitest | 1.0 | Unit тесты |

---

## 📊 Метрики кода

| Метрика | Значение |
|---------|----------|
| Rust код | 3700+ строк |
| TypeScript код | 1750+ строк |
| Всего файлов | 88+ |
| Тесты | 178+ |
| Покрытие | 82% |
| Документация | 20 файлов |

---

## 🎯 Расширяемость

### Добавление плагина

```rust
// 1. Создать плагин
pub struct MyImporter;

impl ImportPlugin for MyImporter {
    fn name(&self) -> &str { "My Format" }
    fn supported_extensions(&self) -> &[&str] { &["myext"] }
    fn import(&self, path: &Path) -> Result<Mesh, PepakuraError> {
        // реализация
    }
}

// 2. Зарегистрировать
let mut registry = PluginRegistry::new();
registry.register_importer(Box::new(MyImporter));
```

### Добавление алгоритма развёртки

```rust
// 1. Создать плагин
pub struct MyUnfolder;

impl UnfoldPlugin for MyUnfolder {
    fn name(&self) -> &str { "MyAlgo" }
    fn unfold(&self, mesh: &Mesh, config: &UnfoldConfig) 
        -> Result<UnfoldedMesh, PepakuraError> {
        // реализация
    }
}

// 2. Использовать
let unfolded = registry.unfold_with(&mesh, "MyAlgo", &config)?;
```

---

## 🧪 Тестирование

### Уровни тестирования

```
┌─────────────────────────────────────┐
│           E2E тесты (20+)           │ ← Playwright
├─────────────────────────────────────┤
│      Integration тесты (15+)        │ ← Tauri commands
├─────────────────────────────────────┤
│       Unit тесты Rust (143+)        │ ← cargo test
├─────────────────────────────────────┤
│      Unit тесты TypeScript (19+)    │ ← vitest
└─────────────────────────────────────┘
```

### Запуск тестов

```bash
# Все тесты
cargo test --workspace
pnpm test:unit
pnpm test:e2e

# С покрытием
cargo tarpaulin --all-features --out Html
pnpm test:unit --coverage
```

---

## 📈 Производительность

### Бенчмарки

| Операция | Время |
|----------|-------|
| MDS развёртка (100 вершин) | 50ms |
| MDS развёртка (1000 вершин) | 500ms |
| LSCM развёртка (1000 вершин) | 150ms |
| AI запрос (кэш) | <1ms |
| AI запрос (Ollama) | 5-10s |
| AI стриминг (первый токен) | <100ms |
| PDF экспорт (100 граней) | 50ms |
| SVG экспорт (100 граней) | 30ms |

---

## 🔐 Безопасность

### Обработка ошибок

```rust
// Все ошибки через thiserror
#[derive(Debug, thiserror::Error)]
pub enum PepakuraError {
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("AI error: {0}")]
    AiError(String),
}
```

### Валидация

```rust
// Валидация меша
pub fn validate(&self) -> Result<(), MeshError> {
    for (i, face) in self.faces.iter().enumerate() {
        // Проверка индексов
        for &idx in &face.vertices {
            if idx >= self.vertices.len() {
                return Err(MeshError::InvalidVertexIndex(idx));
            }
        }
    }
    Ok(())
}
```

---

## 📚 Ресурсы

### Документация

- [README](README.md) — обзор
- [QUICKSTART](QUICKSTART.md) — быстрый старт
- [API Docs](docs/api/) — API референс
- [User Guide](docs/user-guide/) — руководство
- [Prompts](PROMPTS.md) — промпты

### Ссылки

- [Tauri](https://tauri.app/)
- [Vue 3](https://vuejs.org/)
- [Three.js](https://threejs.org/)
- [Ollama](https://ollama.ai/)
- [nalgebra](https://nalgebra.org/)

---

*Архитектурный обзор*  
*Версия: 0.1.0*  
*21 марта 2026 г.*
