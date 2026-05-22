# Архитектура Pepakura Next

## Обзор

Pepakura Next — монорепозиторий для 3D-разворачивания моделей в бумажные развёртки.

## Структура проекта

pepakura-next/
├── packages/
│ ├── ui-desktop/ # Основной десктопный интерфейс (Vue 3 + Tauri)
│ ├── shared/ # Общие компоненты и утилиты
│ ├── core/ # Rust-ядро разворачивания
│ └── backend-python/ # Python-бэкенд (AI, экспорт)
├── docs/ # Документация (этот каталог)
├── tools/ # Инструменты разработки
└── addons/ # Аддоны и интеграции

## Основные компоненты

### UI Desktop (Vue 3 + Tauri)
- **Главная страница**: `packages/ui-desktop/src/views/HomeView.vue`
- **3D-просмотрщик**: `packages/ui-desktop/src/components/ThreeDViewer.vue`
- **Уведомления**: `packages/ui-desktop/src/composables/useNotifications.ts`
- **Загрузка моделей**: `packages/ui-desktop/src/composables/useImport3DModel.ts`

### Shared Components
- Общие компоненты для всех пакетов
- Типы и интерфейсы
- Утилиты

### Core (Rust)
- Алгоритмы разворачивания: `core/src/unwrap3d.rs`
- Оптимизация под бумагу: `core/src/paper_optimize.rs`
- Экспорт форматов: `core/src/export/`

### Backend Python
- AI-генерация швов: `backend/ai_seams/`
- Экспорт в PDF: `services/backend-python/exporters/instruction_pdf_generator.py`

## Технологический стек

- **Frontend**: Vue 3, TypeScript, Vite
- **Desktop**: Tauri (Rust)
- **3D Rendering**: Three.js
- **Backend**: Python (FastAPI/Flask)
- **Core**: Rust
- **Build**: pnpm, Cargo

## Поток данных
Пользователь → UI (Vue) → Tauri Commands → Rust Core → Результат → UI
↓
Three.js (визуализация)
↓
Python Backend (AI, экспорт)

## См. также

- [Быстрый старт](./getting-started.md)
- [Решение проблем](./troubleshooting.md)
- [Соглашения по коду](./contributing.md)
