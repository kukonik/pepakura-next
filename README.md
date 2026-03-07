# Pepakura Next

> Кроссплатформенное приложение для генерации бумажных развёрток 3D-моделей с AI-ассистентом

## 🚀 Возможности

- **AI-генерация 3D моделей** из текста и изображений (Shape-E, TripoSR)
- **Импорт форматов**: OBJ, STL, GLTF, PDO, DXF
- **Развёртка 3D → 2D** с оптимизацией для печати
- **Экспорт**: PDF, SVG, DXF, PNG
- **Интеграции**: Blender, CAD системы
- **Мультиязычность**: 🇷🇺 🇬🇧 🇩🇪 🇫🇷
- **Кроссплатформенность**: Windows, macOS, Linux

## 📦 Установка

```bash
# Клонирование
git clone https://github.com/kukonik/pepakura-next.git
cd pepakura-next

# Установка зависимостей
pnpm install

# Запуск в режиме разработки
pnpm tauri dev
```

## 🏗️ Структура проекта

```
pepakura-next/
├── src-tauri/      # Rust backend (Tauri)
├── ui-desktop/     # Vue 3 frontend
├── shared/         # Общий TypeScript код
├── core/           # Core Rust library
├── services/       # AI сервисы (Python)
├── integrations/   # Blender, CAD, etc.
└── docs/           # Документация
```

## 📚 Документация

- [ROADMAP](docs/ROADMAP.md) — План развития
- [Архитектура](docs/architecture/) — Технические решения
- [API](docs/api/) — Документация API

## 🛠️ Разработка

```bash
# Проверка типов
pnpm typecheck

# Сборка
pnpm build

# Тесты
pnpm test
```

## 📄 Лицензия

MIT

## 🤝 Участие

PR приветствуются! См. [CONTRIBUTING.md](CONTRIBUTING.md)