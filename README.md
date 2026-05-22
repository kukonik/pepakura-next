# Pepakura Next

**Архитектура:** Hybrid v4.0: Web-First + Native Bridge + Modular Addons

Pepakura Next — это система для создания развёрток 3D моделей для бумажного моделирования (паперакрафт).

## 📁 Структура проекта

```
pepakura-next/
├── crates/                      # Rust крейты
│   ├── pepakura_core/           # Ядро системы (Web-First)
│   ├── pepakura_wasm/           # WASM bindings для web
│   ├── pepakura_platform/       # Platform абстракции (FS, Storage)
│   ├── pepakura_addons/         # Фреймворк для расширений
│   └── ai_bridge/               # AI интеграция
│
├── platform/                    # Платформенные приложения
│   ├── desktop/                 # Desktop приложение (Tauri)
│   │   └── ui-desktop/
│   │       └── src-tauri/
│   └── web/                     # Web приложение (WASM)
│       ├── src/
│       └── package.json
│
├── packages/                    # JavaScript/TypeScript пакеты
│   ├── shared/                  # Общие компоненты и утилиты
│   │   └── src/
│   │       └── platform/
│   │           └── platform-bridge.ts
│   ├── core/                    # JS ядро (Three.js загрузчики)
│   └── ui-desktop/              # UI для desktop
│
├── addons/                      # Модульные расширения
│   ├── example-rust-addon/      # Пример Rust аддона
│   └── README.md
│
└── src-tauri/                   # Tauri приложение (legacy, в процессе перемещения)
```

## 🚀 Быстрый старт

### Desktop (Tauri)

```bash
# Установка зависимостей
pnpm install

# Запуск desktop приложения
pnpm tauri dev

# Сборка релиза
pnpm tauri build
```

### Web (WASM)

```bash
# Сборка WASM модуля
cd crates/pepakura_wasm
wasm-pack build --target web --out-dir ../platform/web/public/wasm

# Запуск web приложения
cd platform/web
pnpm install
pnpm dev
```

### Разработка Rust ядра

```bash
# Проверка компиляции
cargo check -p pepakura_core

# Тесты
cargo test -p pepakura_core

# WASM сборка
cargo check -p pepakura_wasm --target wasm32-unknown-unknown
```

## 🏗️ Архитектура

### Hybrid v4.0

Проект использует архитектуру **Web-First**, что означает:

1. **Ядро (pepakura_core)** не зависит от платформы — нет прямого `std::fs`, `std::net`
2. **Platform абстракции** (`pepakura_platform`) предоставляют traits для I/O операций
3. **Две реализации**:
   - `NativeFileSystem` — для desktop (через `tokio::fs`)
   - `WebFileSystem` — для web (через browser APIs)
4. **Platform Bridge** — единый TypeScript API для frontend

### Ключевые компоненты

#### 1. Pepakura Core (`crates/pepakura_core`)

```rust
use pepakura_core::geometry::Mesh;
use pepakura_core::unfold::unfold_mds;
use pepakura_platform::fs::FileSystem;

// Ядро не зависит от платформы
fn process_mesh<F: FileSystem>(fs: &F, path: &str) {
    let mesh = Mesh::load(path);  // Через FileSystem trait
    let unfolded = unfold_mds(&mesh, &config);
    export_textures(&unfolded, &config, fs, output_dir);
}
```

#### 2. Platform Bridge (`packages/shared/src/platform/platform-bridge.ts`)

```typescript
import { platformBridge } from '@pepakura/shared';

// Единый API для desktop и web
const result = await platformBridge.loadProject(path);
await platformBridge.saveProject(project, path);
await platformBridge.unfoldModel(modelId, config);
```

#### 3. Addons System (`crates/pepakura_addons`)

```rust
use pepakura_addons::{Addon, AddonManifest, AddonRegistry};

// Создание аддона
pub struct MyAddon;

impl Addon for MyAddon {
    fn manifest(&self) -> AddonManifest { /* ... */ }
    fn initialize(&self) -> Result<(), AddonError> { /* ... */ }
}

// Регистрация
let registry = AddonRegistry::new();
registry.register(Box::new(MyAddon))?;
```

## 📦 Зависимости

### Rust

- `nalgebra` — линейная алгебра для геометрии
- `image` — обработка изображений
- `printpdf`, `svg`, `dxf` — экспорт в различные форматы
- `tokio` — асинхронные операции
- `wasm-bindgen` — WASM интеграция

### TypeScript

- `vue@3` — frontend фреймворк
- `three` — 3D рендеринг
- `@tauri-apps/api` — Tauri IPC
- `pinia` — state management

## 🧪 Тестирование

```bash
# Rust тесты
cargo test --workspace

# TypeScript тесты
pnpm test

# E2E тесты (Tauri)
cd platform/desktop/ui-desktop
pnpm test:e2e
```

## 📝 Документация

- [HYBRID_V4_MIGRATION_REPORT.md](./HYBRID_V4_MIGRATION_REPORT.md) — отчёт о миграции
- [addons/README.md](./addons/README.md) — документация по аддонам
- [crates/pepakura_core/README.md](./crates/pepakura_core/README.md) — документация ядра

## 🔧 Конфигурация

### Переменные окружения

```bash
# .env.development
TAURI_ENV_PLATFORM=desktop
VITE_API_URL=http://localhost:8000

# .env.production
TAURI_ENV_PLATFORM=desktop
VITE_API_URL=https://api.pepakura.next
```

### Tauri config

См. `platform/desktop/ui-desktop/src-tauri/tauri.conf.json`

## 🤝 Вклад

1. Fork репозиторий
2. Создай ветку (`git checkout -b feature/my-feature`)
3. Закоммить изменения (`git commit -m 'Add my feature'`)
4. Push в ветку (`git push origin feature/my-feature`)
5. Открой Pull Request

## 📄 Лицензия

MIT License — см. [LICENSE](./LICENSE)

## 📞 Контакты

- Repository: https://github.com/pepakura-next/pepakura-next
- Issues: https://github.com/pepakura-next/pepakura-next/issues
