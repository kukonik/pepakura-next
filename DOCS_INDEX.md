# 📖 Полная документация Pepakura Next

**Сводный указатель всей документации**

---

## 🚀 Быстрый старт

| Документ | Описание |
|----------|----------|
| [README.md](README.md) | Обзор проекта |
| [QUICKSTART.md](QUICKSTART.md) | Установка и запуск за 5 минут |
| [INSTALL.md](INSTALL.md) | Подробная установка |

---

## 📚 Для пользователей

| Документ | Описание |
|----------|----------|
| [docs/user-guide/README.md](docs/user-guide/README.md) | Руководство пользователя |
| [docs/user-guide/getting-started.md](docs/user-guide/getting-started.md) | Начало работы |
| [docs/user-guide/import-model.md](docs/user-guide/import-model.md) | Импорт моделей |
| [docs/user-guide/unfolding.md](docs/user-guide/unfolding.md) | Развёртка |
| [docs/user-guide/export.md](docs/user-guide/export.md) | Экспорт |
| [docs/user-guide/settings.md](docs/user-guide/settings.md) | Настройки |
| [docs/user-guide/troubleshooting.md](docs/user-guide/troubleshooting.md) | Решение проблем |

---

## 👨‍💻 Для разработчиков

| Документ | Описание |
|----------|----------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Архитектура проекта |
| [docs/api/README.md](docs/api/README.md) | API документация (Rust) |
| [docs/api/LSCM.md](docs/api/LSCM.md) | LSCM алгоритм |
| [docs/api/PDF_EXPORT.md](docs/api/PDF_EXPORT.md) | PDF экспорт |
| [docs/plugins/README.md](docs/plugins/README.md) | Система плагинов |
| [docs/ai/README.md](docs/ai/README.md) | AI-модуль |
| [docs/ai/CACHING.md](docs/ai/CACHING.md) | AI кэширование |
| [docs/ai/STREAMING.md](docs/ai/STREAMING.md) | AI стриминг |
| [docs/ui/VIEWER_EDITOR.md](docs/ui/VIEWER_EDITOR.md) | 3D Viewer и 2D Editor |

---

## 📋 Отчёты о реализации

| Документ | Описание |
|----------|----------|
| [IMPLEMENTATION_REPORT.md](IMPLEMENTATION_REPORT.md) | Общий отчёт |
| [PHASE1_COMPLETE.md](PHASE1_COMPLETE.md) | Phase 1: Критичные улучшения |
| [PHASE2_COMPLETE.md](PHASE2_COMPLETE.md) | Phase 2: UI улучшения |
| [PHASEB_COMPLETE.md](PHASEB_COMPLETE.md) | Phase B: Доп. улучшения |
| [FINAL_COMPLETE.md](FINAL_COMPLETE.md) | Финальный отчёт |
| [TEST_COVERAGE_REPORT.md](TEST_COVERAGE_REPORT.md) | Отчёт о тестировании |

---

## 🛠️ Разработка

| Документ | Описание |
|----------|----------|
| [PROMPTS.md](PROMPTS.md) | Промпты для разработки |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Вклад в проект |
| [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) | Кодекс поведения |
| [CHANGELOG.md](CHANGELOG.md) | История изменений |

---

## 📊 Статистика проекта

```
Код:
├── Rust: 4350+ строк
├── TypeScript: 1750+ строк
└── Всего: 6100+ строк

Файлы: 92+
Тесты: 190+
Покрытие: 83%
Документация: 23 файла
```

---

## 🎯 Ключевые возможности

### Ядро (pepakura_core)

- ✅ **geometry** — Vertex, Face, Mesh, BoundingBox
- ✅ **unfold** — MDS, LSCM, оптимизированный MDS
- ✅ **export** — SVG, PDF
- ✅ **plugins** — ImportPlugin, ExportPlugin, UnfoldPlugin
- ✅ **ai** — Ollama, OpenAI, кэширование, стриминг
- ✅ **persistence** — SQLite хранилище

### Tauri приложение

- ✅ 15+ IPC команд
- ✅ Глобальное состояние (DashMap)
- ✅ Интеграция с pepakura_core
- ✅ AI команды

### Frontend (Vue 3)

- ✅ Viewer3D — интерактивный 3D вьювер
- ✅ UnfoldEditor — 2D редактор развёрток
- ✅ Workspace — объединённый вид
- ✅ AiAssistantPanel — AI чат
- ✅ Pinia stores
- ✅ i18n (ru/en)

---

## 🧪 Тестирование

### Запуск тестов

```bash
# Rust тесты
cd crates/pepakura_core
cargo test --lib

# TypeScript тесты
cd ui-desktop
pnpm test:unit

# E2E тесты
cd ui-desktop
pnpm test:e2e

# С покрытием
cargo tarpaulin --all-features --out Html
pnpm test:unit --coverage
```

### Покрытие

| Модуль | Покрытие |
|--------|----------|
| geometry | 95% |
| unfold | 90% |
| export | 85% |
| plugins | 90% |
| ai | 85% |
| persistence | 90% |
| nesting | 80% |

---

## 📦 Сборка

```bash
# Windows
.\scripts\build-windows.ps1

# Linux
./scripts/build-linux.sh

# macOS
./scripts/build-macos.sh

# Или через Tauri
cd src-tauri
cargo tauri build
```

---

## 🤝 Вклад в проект

1. Fork репозиторий
2. Создай ветку (`git checkout -b feature/amazing-feature`)
3. Закоммить изменения (`git commit -m 'Add amazing feature'`)
4. Push (`git push origin feature/amazing-feature`)
5. Открой Pull Request

### Требования к коду

- **Rust**: следуй Rust API Guidelines
- **TypeScript/Vue**: ESLint + Prettier
- **Коммиты**: Conventional Commits
- **Тесты**: обязательно для нового функционала

---

## 📞 Поддержка

- **GitHub Issues**: [Сообщить о проблеме](https://github.com/pepakura-next/pepakura-next/issues)
- **Discussions**: [Обсуждения](https://github.com/pepakura-next/pepakura-next/discussions)
- **Email**: support@pepakura.next (TODO)

---

## 📄 Лицензия

MIT — см. [LICENSE](LICENSE)

---

## 🙏 Благодарности

- [Tauri](https://tauri.app/) — Desktop framework
- [Vue 3](https://vuejs.org/) — Frontend framework
- [Three.js](https://threejs.org/) — 3D графика
- [nalgebra](https://nalgebra.org/) — Линейная алгебра
- [Ollama](https://ollama.ai/) — AI интеграция
- [Playwright](https://playwright.dev/) — E2E тесты

---

*Полная документация*  
*Версия: 0.2.0*  
*21 марта 2026 г.*
