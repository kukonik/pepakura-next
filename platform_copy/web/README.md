# 🌐 Pepakura Next Web

**Веб-версия Pepakura Next для работы в браузере**

[![Demo](https://img.shields.io/badge/demo-online-blue.svg)](https://web.pepakura-next.com)
[![WASM](https://img.shields.io/badge/powered-WASM-orange.svg)](https://webassembly.org)

---

## 🚀 Быстрый старт

### 1. Установка зависимостей

```bash
cd web
pnpm install
```

### 2. Сборка WASM

```bash
# Установка wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Сборка
wasm-pack build ../crates/pepakura_wasm --target web --out-dir ../web/public/wasm
```

### 3. Запуск dev-сервера

```bash
pnpm dev
# http://localhost:5173
```

### 4. Production сборка

```bash
pnpm build
pnpm preview
```

---

## 📦 Возможности

- ✅ **Импорт моделей**: OBJ, STL, PLY, PDO
- ✅ **Развёртка**: MDS, LSCM алгоритмы
- ✅ **3D Viewer**: интерактивный просмотр
- ✅ **2D Редактор**: редактирование развёртки
- ✅ **Экспорт**: SVG, PDF, DXF
- ✅ **Nesting**: оптимизация раскладки

---

## 🎯 Использование

### Импорт модели

1. Нажмите "Выбрать файл"
2. Выберите 3D модель (OBJ, STL, PLY)
3. Модель загрузится в 3D viewer

### Создание развёртки

1. Выберите алгоритм (MDS или LSCM)
2. Настройте параметры
3. Нажмите "Создать развёртку"

### Экспорт

1. Выберите формат (SVG, PDF, DXF)
2. Скачайте файл

---

## 🛠️ Технологии

- **Vue 3** — frontend framework
- **Vite** — build tool
- **Three.js** — 3D графика
- **WASM** — Rust ядро
- **Pinia** — state management

---

## 📁 Структура

```
web/
├── public/
│   └── wasm/              # WASM модули
├── src/
│   ├── components/        # Vue компоненты
│   ├── composables/       # Composables
│   ├── App.vue           # Главный компонент
│   └── main.ts           # Entry point
├── index.html
├── package.json
└── vite.config.js
```

---

## 🔧 Конфигурация

### Vite

```javascript
// vite.config.js
export default defineConfig({
  plugins: [vue()],
  server: {
    port: 5173,
  },
})
```

### WASM

```toml
# crates/pepakura_wasm/Cargo.toml
[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```

---

## 🧪 Тесты

```bash
# WASM тесты
wasm-pack test --headless

# Frontend тесты
pnpm test
```

---

## 📊 Производительность

| Метрика | Значение |
|---------|----------|
| Размер WASM | ~500 KB |
| Время загрузки | <2 сек |
| Время развёртки | <500 мс |
| FPS (3D viewer) | 60 |

---

## 🐛 Известные ограничения

- Нет доступа к файловой системе
- Ограниченная память (~2GB)
- Нет многопоточности

---

## 🤝 Вклад

1. Fork репозиторий
2. Создай ветку (`git checkout -b feature/web-improvement`)
3. Закоммить изменения (`git commit -m 'Add web feature'`)
4. Push (`git push origin feature/web-improvement`)
5. Открой Pull Request

---

## 📄 Лицензия

MIT — см. [LICENSE](../LICENSE)

---

## 🙏 Благодарности

- [Tauri](https://tauri.app/) — Desktop framework
- [Vue 3](https://vuejs.org/) — Frontend framework
- [Three.js](https://threejs.org/) — 3D графика
- [wasm-bindgen](https://rustwasm.github.io/) — WASM bindings

---

**Pepakura Next Team**  
*22 марта 2026 г.*
