
# 📋 Промпты для Qwen Code в VS Code — Pepakura Next

Ниже представлена серия промптов для пошаговой реализации проекта. Сохраните их в файл `PROMPTS.md` в корне репозитория и используйте последовательно.

---

## 📁 Этап 1: Инициализация проекта

### Промпт 1.1: Создание структуры монорепозитория
```markdown
# Задача: Инициализация структуры проекта Pepakura Next

## Контекст
Создаю приложение для генерации развёрток бумажных моделей (papercraft). 
Стек: Rust (ядро) + Tauri (desktop) + Vue 3 (frontend).

## Требования
1. Создай структуру монорепозитория:
   ```
   pepakura-next/
   ├── core/pepakura_core/    # Rust библиотека
   ├── desktop/src-tauri/     # Tauri приложение
   ├── frontend/              # Vue 3 приложение
   ├── scripts/               # PowerShell скрипты
   └── docs/                  # Документация
   ```

2. В корне создай:
   - `.gitignore` (Rust + Node + Tauri)
   - `README.md` с описанием проекта
   - `LICENSE` (MIT)
   - `pnpm-workspace.yaml` для управления зависимостями

3. Для каждого пакета создай базовые конфиги:
   - `core/Cargo.toml` (lib type, name = "pepakura_core")
   - `desktop/package.json` + `tauri.conf.json`
   - `frontend/package.json` + `vite.config.ts`

## Acceptance Criteria
- [ ] `cargo check` проходит в core/
- [ ] `pnpm install` работает в корне
- [ ] Структура папок соответствует схеме выше
- [ ] Все .gitignore правила актуальны для стека

## Ограничения
- Не устанавливай сложные зависимости пока
- Используй стабильные версии (Rust 1.75+, Node 20+)
- Комментарии в конфигах на русском языке
```

---

### Промпт 1.2: Настройка Rust ядра
```markdown
# Задача: Базовая настройка pepakura_core

## Контекст
Ядро приложения на Rust. Должно быть независимым от Tauri, 
потенциально компилируемым в WASM для веб-версии.

## Требования
1. В `core/pepakura_core/Cargo.toml` добавь зависимости:
   ```toml
   [dependencies]
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   thiserror = "1.0"
   log = "0.4"
   nalgebra = "0.32"        # линейная алгебра для геометрии
   rustc-hash = "1.1"       # быстрые хеш-мапы
   
   [dev-dependencies]
   criterion = "0.5"        # бенчмарки
   ```

2. Создай базовую структуру модулей:
   ```
   src/
   ├── lib.rs           # публичный API
   ├── error.rs         # типы ошибок (thiserror)
   ├── geometry/        # работа с мешами
   │   ├── mod.rs
   │   ├── mesh.rs
   │   └── vertex.rs
   ├── unfold/          # алгоритмы развёртки
   │   ├── mod.rs
   │   └── mds.rs
   └── export/          # экспорт форматов
       ├── mod.rs
       └── svg.rs
   ```

3. В `lib.rs` экспортируй публичные типы:
   - `PepakuraError`
   - `Mesh`
   - `UnfoldedMesh`
   - `ExportFormat`

## Acceptance Criteria
- [ ] `cargo build` проходит без предупреждений
- [ ] `cargo doc` генерирует документацию
- [ ] Все модули имеют `#[cfg(test)]` тесты-заглушки
- [ ] Публичный API минимален (только необходимое)

## Примечания
- Используй `thiserror` для всех ошибок
- Все публичные типы должны иметь документацию (`///`)
- Избегай `unwrap()` в публичном API
```

---

### Промпт 1.3: Настройка Tauri бэкенда
```markdown
# Задача: Настройка Tauri приложения

## Контекст
Desktop-оболочка для pepakura_core. 
Минимальная конфигурация для запуска.

## Требования
1. В `desktop/src-tauri/Cargo.toml` добавь:
   ```toml
   [dependencies]
   tauri = "2.0"
   pepakura_core = { path = "../../core/pepakura_core" }
   tauri-plugin-store = "2.0"
   serde = { version = "1.0", features = ["derive"] }
   ```

2. Создай `src-tauri/src/`:
   ```
   ├── main.rs          # точки входа
   ├── commands.rs      # Tauri IPC команды
   └── state.rs         # глобальное состояние
   ```

3. В `commands.rs` создай заглушки команд:
   - `create_project(name: String) -> Result<ProjectId>`
   - `import_model(path: String) -> Result<Mesh>`
   - `unfold_mesh(mesh: Mesh) -> Result<UnfoldedMesh>`
   - `export_svg(unfolded: UnfoldedMesh, path: String) -> Result<()>`

4. В `tauri.conf.json` настрой:
   - `identifier = "com.pepakura.next"`
   - `windows[0].title = "Pepakura Next"`
   - `bundle.resources` для ассетов

## Acceptance Criteria
- [ ] `cargo tauri dev` запускает окно
- [ ] Команды регистрируются через `invoke()`
- [ ] Ошибки корректно мапятся в Tauri-формат
- [ ] Консоль браузера показывает подключение

## Примечания
- Все команды асинхронные (`async fn`)
- Используй `tauri::State` для передачи pepakura_core
- Логирование через `log::info!` с префиксом "[TAURI]"
```

---

### Промпт 1.4: Настройка Vue 3 фронтенда
```markdown
# Задача: Настройка Vue 3 приложения

## Контекст
Frontend для Tauri. Composition API + TypeScript + Pinia.

## Требования
1. В `frontend/package.json` добавь:
   ```json
   {
     "dependencies": {
       "vue": "^3.4",
       "pinia": "^2.1",
       "vue-i18n": "^9.8",
       "@tauri-apps/api": "^2.0"
     },
     "devDependencies": {
       "vite": "^5.0",
       "@vitejs/plugin-vue": "^5.0",
       "typescript": "^5.3",
       "tailwindcss": "^3.4",
       "vitest": "^1.0"
     }
   }
   ```

2. Создай структуру `frontend/src/`:
   ```
   ├── main.ts
   ├── App.vue
   ├── components/
   │   ├── dashboard/
   │   ├── viewer/
   │   └── common/
   ├── stores/
   │   ├── project.store.ts
   │   └── settings.store.ts
   ├── composables/
   │   ├── useTauri.ts
   │   └── useUnfold.ts
   ├── i18n/
   │   ├── ru.json
   │   └── en.json
   └── types/
       └── index.ts
   ```

3. В `types/index.ts` создай TypeScript-типы, 
   зеркальные к Rust-структурам (Mesh, UnfoldedMesh, etc.)

4. Настрой `vite.config.ts` с алиасами:
   - `@/` → `src/`
   - `@components/` → `src/components/`

## Acceptance Criteria
- [ ] `pnpm dev` запускает Vite сервер
- [ ] TypeScript компилируется без ошибок
- [ ] Pinia store инициализируется
- [ ] i18n переключает язык (ru/en)

## Примечания
- Все компоненты с `<script setup lang="ts">`
- Стили через Tailwind + CSS variables для тем
- Tauri API только через composables (не напрямую в компонентах)
```

---

## 📁 Этап 2: Реализация ядра

### Промпт 2.1: Геометрия меша
```markdown
# Задача: Реализация базовой геометрии в pepakura_core

## Контекст
Нужны структуры для представления 3D-меша и операций с ним.
Используем half-edge структуру для удобства развёртки.

## Требования
1. В `geometry/vertex.rs` создай:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct Vertex {
       pub id: usize,
       pub position: [f64; 3],
       pub normal: Option<[f64; 3]>,
       pub uv: Option<[f64; 2]>,
   }
   ```

2. В `geometry/mesh.rs` создай:
   ```rust
   #[derive(Debug, Clone, Serialize, Deserialize)]
   pub struct Mesh {
       pub vertices: Vec<Vertex>,
       pub faces: Vec<Face>,  // треугольники (индексы вершин)
       pub name: String,
       pub metadata: MeshMetadata,
   }
   
   pub struct Face {
       pub vertices: [usize; 3],
       pub material_id: Option<usize>,
   }
   ```

3. Реализуй методы для `Mesh`:
   - `fn bounding_box(&self) -> BoundingBox`
   - `fn centroid(&self) -> [f64; 3]`
   - `fn scale(&mut self, factor: f64)`
   - `fn translate(&mut self, offset: [f64; 3])`

4. В `error.rs` добавь ошибки:
   - `InvalidMesh`
   - `NonManifoldEdge`
   - `DegenerateFace`

## Acceptance Criteria
- [ ] Все структуры сериализуются в JSON
- [ ] Методы имеют юнит-тесты
- [ ] Обработка ошибок через `thiserror`
- [ ] Документация для публичных методов

## Примечания
- Используй `nalgebra` для векторных операций если нужно
- Индексы вершин валидируй при создании меша
- Избегай паник, возвращай `Result`
```

---

### Промпт 2.2: Алгоритм развёртки (MDS)
```markdown
# Задача: Базовый алгоритм развёртки через MDS

## Контекст
Упрощённая версия алгоритма из статьи "Mesh Unfolding via Semidefinite Programming".
Используем Multidimensional Scaling для проекции 3D → 2D.

## Требования
1. В `unfold/mds.rs` создай:
   ```rust
   pub struct UnfoldConfig {
       pub preserve_detail: bool,
       pub max_iterations: usize,
       pub tolerance: f64,
   }
   
   pub fn unfold_mds(mesh: &Mesh, config: &UnfoldConfig) 
       -> Result<UnfoldedMesh, PepakuraError>
   ```

2. Алгоритм:
   - Вычисли матрицу попарных расстояний между вершинами (3D)
   - Примени классический MDS для получения 2D-координат
   - Сохрани топологию (грани) из исходного меша
   - Верни `UnfoldedMesh` с 2D-координатами

3. Создай `UnfoldedMesh`:
   ```rust
   pub struct UnfoldedMesh {
       pub vertices_2d: Vec<[f64; 2]>,
       pub faces: Vec<Face>,  // те же индексы что в исходном
       pub source_mesh: Mesh, // ссылка на оригинал
       pub metadata: UnfoldMetadata,
   }
   ```

4. Добавь тесты на простых мешах:
   - Куб (6 граней)
   - Пирамида (4 грани)
   - Плоский квадрат (1 грань)

## Acceptance Criteria
- [ ] Алгоритм сходится за < 100 итераций для простых мешей
- [ ] 2D-координаты не имеют наложений (для выпуклых мешей)
- [ ] Топология сохраняется (те же грани)
- [ ] Бенчмарк для меша 1000 вершин < 100ms

## Примечания
- Используй `nalgebra` для матричных операций
- MDS реализуй через собственное разложение (или готовую функцию)
- Для начала без оптимизации искажений (добавим позже)
```

---

### Промпт 2.3: Экспорт в SVG
```markdown
# Задача: Экспорт развёртки в SVG

## Контекст
Нужен генератор SVG для печати и редактирования в векторных редакторах.

## Требования
1. В `export/svg.rs` создай:
   ```rust
   pub struct SvgExportConfig {
       pub page_size: PageSize,  // A4, A3, custom
       pub scale: f64,           // мм на единицу
       pub show_vertex_ids: bool,
       pub show_fold_lines: bool,
       pub show_cut_lines: bool,
   }
   
   pub fn export_svg(unfolded: &UnfoldedMesh, config: &SvgExportConfig) 
       -> Result<String, PepakuraError>
   ```

2. SVG должен содержать:
   - Слой для линий реза (cut lines) — сплошные
   - Слой для линий сгиба (fold lines) — пунктирные
   - Слой для номеров деталей
   - Слой для текстур (если есть)

3. Добавь оптимизацию путей:
   - Объединяй смежные отрезки в один path
   - Удаляй дубликаты вершин
   - Минифицируй вывод (без pretty-print)

4. Создай тесты:
   - Экспорт куба → проверка количества путей
   - Экспорт с текстурами → проверка clip-path
   - Проверка валидности SVG через валидатор

## Acceptance Criteria
- [ ] SVG валиден (открывается в браузере/Inkscape)
- [ ] Слои разделены логически
- [ ] Размеры соответствуют page_size + scale
- [ ] Файл < 100KB для меша 1000 граней

## Примечания
- Используй `xml-rs` или ручную генерацию строк
- Координаты SVG: Y инвертирован (0 сверху)
- Добавь метаданные в SVG комментарий (версия, дата)
```

---

## 📁 Этап 3: Интеграция

### Промпт 3.1: Tauri команды для ядра
```markdown
# Задача: Связка Tauri команд с pepakura_core

## Контекст
Frontend вызывает Rust-функции через Tauri IPC.

## Требования
1. В `commands.rs` реализуй полные команды:
   ```rust
   #[tauri::command]
   pub async fn create_project(
       name: String,
       state: State<'_, AppState>
   ) -> Result<ProjectId, String>
   
   #[tauri::command]
   pub async fn import_model(
       path: String,
       format: String,
       state: State<'_, AppState>
   ) -> Result<Mesh, String>
   
   #[tauri::command]
   pub async fn unfold_mesh(
       mesh_id: usize,
       config: UnfoldConfig,
       state: State<'_, AppState>
   ) -> Result<UnfoldedMesh, String>
   
   #[tauri::command]
   pub async fn export_svg(
       unfolded_id: usize,
       path: String,
       config: SvgExportConfig,
       state: State<'_, AppState>
   ) -> Result<(), String>
   ```

2. Создай `AppState` в `state.rs`:
   ```rust
   pub struct AppState {
       pub projects: DashMap<ProjectId, Project>,
       pub meshes: DashMap<usize, Mesh>,
       pub unfolded: DashMap<usize, UnfoldedMesh>,
       pub config: AppConfiguration,
   }
   ```

3. Обработай ошибки:
   - Маппинг `PepakuraError` → `String` для Tauri
   - Логирование ошибок с контекстом
   - Валидация входных параметров

4. Добавь команды управления:
   - `get_recent_projects() -> Vec<ProjectInfo>`
   - `delete_project(id: ProjectId) -> Result