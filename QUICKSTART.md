# 🚀 Быстрый старт Pepakura Next

**Для разработчиков** — настройка и запуск за 5 минут.

---

## 📋 Требования

### Обязательно
- ✅ **Rust** 1.75+ ([установить](https://rustup.rs/))
- ✅ **Node.js** 20+ ([установить](https://nodejs.org/))
- ✅ **pnpm** 8+ (`npm install -g pnpm`)

### Опционально
- 🤖 **Ollama** для AI ([установить](https://ollama.ai/))
- 🐍 **Python** 3.8+ (для скриптов)

---

## ⚡ Установка за 5 минут

### Шаг 1: Клонирование

```bash
git clone https://github.com/pepakura-next/pepakura-next.git
cd pepakura-next
```

### Шаг 2: Установка зависимостей

```bash
# Frontend зависимости
pnpm install

# Rust зависимости (автоматически при сборке)
cd crates/pepakura_core
cargo build
```

### Шаг 3: Проверка окружения

```powershell
# Windows
.\scripts\health-check.ps1

# Linux/Mac
./scripts/health-check.sh
```

---

## 🎯 Запуск разработки

### Вариант 1: Полный стек (рекомендуется)

```bash
# Terminal 1: Frontend
cd ui-desktop
pnpm dev

# Terminal 2: Tauri
cd src-tauri
cargo tauri dev
```

### Вариант 2: Только Rust ядро

```bash
cd crates/pepakura_core
cargo test --lib
```

### Вариант 3: Только Frontend

```bash
cd ui-desktop
pnpm dev
pnpm test:unit
```

---

## 🧪 Запуск тестов

### Все тесты

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
```

### С покрытием

```bash
# Rust coverage
cargo tarpaulin --all-features --out Html
# Отчёт: target/tarpaulin-report.html

# TypeScript coverage
cd ui-desktop
pnpm test:unit --coverage
# Отчёт: coverage/index.html
```

---

## 📦 Сборка релиза

### Windows

```powershell
.\scripts\build-windows.ps1
```

### Linux

```bash
./scripts/build-linux.sh
```

### macOS

```bash
./scripts/build-macos.sh
```

### Кроссплатформенная сборка

```bash
cd src-tauri
cargo tauri build
```

**Результат:**
- `src-tauri/target/release/bundle/`
  - `.msi` (Windows)
  - `.deb` / `.rpm` (Linux)
  - `.dmg` (macOS)

---

## 🤖 Настройка AI

### Ollama (локально)

```bash
# Установить Ollama
# https://ollama.ai

# Скачать модель
ollama pull llama3.2

# Запустить сервис
ollama serve
```

### Проверка AI

```bash
# Тест через CLI
ollama run llama3.2 "Привет!"

# В приложении
# Откройте AI-панель → проверьте статус
```

### OpenAI (облако)

1. Получите API ключ: https://platform.openai.com/api-keys
2. В приложении: Настройки → AI → OpenAI
3. Введите ключ: `sk-...`

---

## 🐛 Решение проблем

### "cargo: command not found"

```bash
# Установите Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Перезапустите терминал
```

### "pnpm: command not found"

```bash
# Установите pnpm
npm install -g pnpm

# Проверьте
pnpm --version
```

### Ошибка компиляции Tauri

```bash
# Очистите кэш
cd src-tauri
cargo clean

# Пересоберите
cargo build
```

### AI не доступен

```bash
# Проверьте Ollama
ollama list

# Если пусто - скачайте модель
ollama pull llama3.2

# Перезапустите сервис
ollama serve
```

---

## 📚 Полезные команды

### Разработка

```bash
# Проверка кода
cargo clippy -- -D warnings
pnpm lint

# Форматирование
cargo fmt
pnpm format

# Типизация
pnpm typecheck
```

### Отладка

```bash
# Логирование
RUST_LOG=debug cargo run

# Профилирование
cargo flamegraph

# Benchmark
cargo bench
```

### Очистка

```bash
# Очистка Rust
cargo clean

# Очистка Node
rm -rf node_modules
pnpm install

# Полная очистка
cargo clean
rm -rf ui-desktop/node_modules
rm -rf ui-desktop/dist
```

---

## 🎯 Первый проект

### 1. Импорт модели

```
1. Откройте приложение
2. Нажмите "Импортировать"
3. Выберите .obj файл
```

### 2. Развёртка

```
1. Нажмите "Развернуть"
2. Выберите алгоритм (MDS/LSCM)
3. Нажмите "OK"
```

### 3. Экспорт

```
1. Нажмите "Экспорт"
2. Выберите формат (SVG/PDF)
3. Сохраните файл
```

### 4. Печать

```
1. Откройте PDF в браузере
2. Печать → "Actual Size"
3. Используйте бумагу 120-200 g/m²
```

---

## 📞 Поддержка

### Документация

- [README](README.md) — обзор проекта
- [API Docs](docs/api/) — документация API
- [User Guide](docs/user-guide/) — руководство пользователя
- [Prompts](PROMPTS.md) — промпты разработки

### Контакты

- **GitHub Issues**: [Сообщить о проблеме](https://github.com/pepakura-next/pepakura-next/issues)
- **Discussions**: [Обсуждения](https://github.com/pepakura-next/pepakura-next/discussions)

---

## 🎓 Обучение

### Для новых разработчиков

1. **Изучите архитектуру**
   - [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
   - [docs/api/README.md](docs/api/README.md)

2. **Запустите тесты**
   ```bash
   cargo test --lib
   pnpm test:unit
   ```

3. **Сделайте первое изменение**
   - Найдите issue с меткой "good first issue"
   - Создайте ветку
   - Внесите изменения
   - Отправьте PR

### Туториалы

- [Создание плагина](docs/plugins/getting-started.md)
- [AI интеграция](docs/ai/README.md)
- [3D Viewer](docs/ui/VIEWER_EDITOR.md)

---

## ✅ Чеклист готовности

- [ ] Rust 1.75+ установлен
- [ ] Node.js 20+ установлен
- [ ] pnpm 8+ установлен
- [ ] `pnpm install` прошёл успешно
- [ ] `cargo test --lib` проходит
- [ ] `pnpm test:unit` проходит
- [ ] Приложение запускается
- [ ] AI подключён (опционально)

**Готово!** 🎉

---

*Quick Start Guide*  
*Версия: 0.1.0*  
*21 марта 2026 г.*
