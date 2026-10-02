# 📋 Релизный чеклист Pepakura Next v0.2.0

**Дата релиза**: 21 марта 2026 г.  
**Версия**: 0.2.0  
**Статус**: ✅ Готов к релизу

---

## ✅ Предрелизные проверки

### Код

- [x] ✅ Все тесты проходят (190+ тестов)
- [x] ✅ Покрытие тестами >80% (83%)
- [x] ✅ Rust компилируется без ошибок
- [x] ✅ TypeScript компилируется без ошибок
- [x] ✅ Clippy предупреждения исправлены
- [x] ✅ Код отформатирован (rustfmt, prettier)
- [x] ✅ Нет TODO/FIXME в коде

### Документация

- [x] ✅ README.md обновлён
- [x] ✅ CHANGELOG.md заполнен
- [x] ✅ QUICKSTART.md актуален
- [x] ✅ API документация сгенерирована
- [x] ✅ User Guide проверен

### Инфраструктура

- [x] ✅ CI/CD pipeline работает
- [x] ✅ Pre-commit хуки настроены
- [x] ✅ Build скрипты работают
- [x] ✅ Health check проходит

---

## 📦 Сборка релиза

### Windows

```powershell
# Проверка
.\ci-check.ps1

# Сборка
.\scripts\build-windows.ps1

# Проверка бинарников
Get-ChildItem src-tauri\target\release\bundle\ -Recurse -File | 
  Select-Object FullName, Length
```

**Ожидаемый результат:**
- `*.msi` инсталлятор (~50 MB)
- `*.exe` portable версия (~45 MB)

### Linux

```bash
# Проверка
./ci-check.sh

# Сборка
./scripts/build-linux.sh

# Проверка бинарников
ls -lh src-tauri/target/release/bundle/
```

**Ожидаемый результат:**
- `*.deb` пакет (~45 MB)
- `*.rpm` пакет (~45 MB)
- `*.AppImage` (~50 MB)

### macOS

```bash
# Проверка
./ci-check.sh

# Сборка
./scripts/build-macos.sh

# Проверка бинарников
ls -lh src-tauri/target/release/bundle/
```

**Ожидаемый результат:**
- `*.dmg` инсталлятор (~50 MB)
- `*.app` (~45 MB)

---

## 🧪 Тестирование релиза

### Быстрые тесты

```bash
# 1. Запуск приложения
./target/release/pepakura-next

# 2. Импорт тестовой модели
# Файл: test_files/cube.obj

# 3. Развёртка
# Алгоритм: MDS / LSCM

# 4. Экспорт
# Формат: SVG / PDF

# 5. AI помощник
# Вопрос: "Как выбрать бумагу?"
```

### Полные тесты

```bash
# Rust тесты
cargo test --release --lib

# TypeScript тесты
pnpm test:unit

# E2E тесты
pnpm test:e2e
```

---

## 📝 Публикация

### GitHub Release

1. **Создать тег**
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```

2. **Создать релиз на GitHub**
   - Перейти на https://github.com/pepakura-next/pepakura-next/releases/new
   - Выбрать тег v0.2.0
   - Заполнить описание (см. шаблон ниже)
   - Прикрепить бинарники

3. **Шаблон описания релиза**

```markdown
## Pepakura Next v0.2.0

🎉 Все этапы реализованы!

### Что нового

#### Phase 1: Критичные улучшения
- 🚀 AI кэширование (5000-10000x быстрее)
- 🚀 LSCM алгоритм (3-10x быстрее MDS)
- 🚀 PDF экспорт (нативный)
- 🚀 AI стриминг (<100ms до токена)

#### Phase 2: UI улучшения
- 🎨 Интерактивный 3D Viewer
- 🎨 2D Editor развёрток
- 🎨 Синхронизация 2D ↔ 3D

#### Phase 3: Тесты
- ✅ 83% покрытие тестами
- ✅ 190+ тестов

#### Phase B: Оптимизация
- ⚡ MDS оптимизация (параллелизм)
- 💾 Персистентное состояние (SQLite)

### Статистика

- 14100+ строк кода
- 92+ файла
- 190+ тестов
- 83% покрытие

### Установка

#### Windows
- Скачайте `Pepakura.Next_0.2.0_x64.msi`
- Запустите установщик

#### Linux
- Debian/Ubuntu: `dpkg -i pepakura-next_0.2.0_amd64.deb`
- Fedora: `rpm -i pepakura-next-0.2.0.x86_64.rpm`

#### macOS
- Скачайте `Pepakura.Next_0.2.0_x64.dmg`
- Перетащите в Applications

### Известные проблемы

- [ ] Мобильное приложение (в разработке)
- [ ] Облачная синхронизация (в разработке)

### Благодарности

Спасибо всем контрибьюторам! 🙏

### Лицензия

MIT
```

---

## 📢 Анонс релиза

### Соцсети

**Twitter:**
```
🎉 Pepakura Next v0.2.0 released!

✨ AI быстрее в 5000-10000 раз
✨ MDS оптимизация (3-10x)
✨ Интерактивные 3D/2D редакторы
✨ Нативный PDF экспорт

#rust #vuejs #papercraft #opensource

👉 https://github.com/pepakura-next/pepakura-next/releases/tag/v0.2.0
```

**Reddit (r/rust):**
```
Title: Pepakura Next v0.2.0 - Rust + Tauri + Vue 3 papercraft app

Body:
Excited to announce Pepakura Next v0.2.0!

Key features:
- AI-powered assistance (Ollama integration)
- MDS/LSCM unfolding algorithms
- Interactive 3D/2D editors
- Native PDF export
- 83% test coverage

Built with Rust, Tauri, Vue 3, Three.js

GitHub: https://github.com/pepakura-next/pepakura-next
```

---

## 📊 Пострелизные задачи

### Мониторинг

- [ ] Отслеживать GitHub Issues
- [ ] Мониторить Crash Reports
- [ ] Собирать фидбэк пользователей

### Обновления

- [ ] Исправлять критичные баги
- [ ] Обновлять зависимости
- [ ] Готовить v0.2.1 (bugfixes)

### Метрики

- [ ] Количество загрузок
- [ ] Количество звёзд на GitHub
- [ ] Активность в Issues/Discussions

---

## ✅ Финальный чеклист

- [x] Код готов
- [x] Тесты проходят
- [x] Документация обновлена
- [x] CI/CD работает
- [x] Бинарники собраны
- [ ] Релиз создан на GitHub
- [ ] Анонс опубликован
- [ ] Метрики настроены

---

**Готово к релизу!** 🚀

*Релизный чеклист*  
*Версия: 0.2.0*  
*21 марта 2026 г.*
