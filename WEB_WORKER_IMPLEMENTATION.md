# Web Worker Архитектура для Развёртки

## Обзор

Реализована архитектура на основе Web Workers для выноса тяжелых геометрических расчетов (развёртка) в отдельный поток. Это предотвращает блокировку UI во время выполнения операций.

## Компоненты

### 1. Worker Script (`frontend/src/workers/unfold.worker.ts`)

Web Worker для выполнения развёртки в фоновом потоке.

**Сообщения (входящие):**
- `UNFOLD_REQUEST` - запрос на развёртку
- `TERMINATE` - завершение работы воркера

**Сообщения (исходящие):**
- `UNFOLD_SUCCESS` - успешное завершение развёртки
- `UNFOLD_ERROR` - ошибка при развёртке
- `UNFOLD_PROGRESS` - прогресс выполнения

**Пример использования:**
```typescript
const worker = new Worker(
  new URL('../workers/unfold.worker.ts', import.meta.url),
  { type: 'module' }
);

worker.postMessage({
  type: 'UNFOLD_REQUEST',
  payload: { mesh, config }
});

worker.onmessage = (event) => {
  const { type, payload } = event.data;
  // Обработка результатов
};
```

### 2. Composable (`frontend/src/composables/useAsyncUnfold.ts`)

Vue 3 composable для удобной работы с воркером.

**Реактивные переменные:**
- `isProcessing` - выполняется ли развёртка
- `progress` - прогресс (0-100)
- `progressMessage` - текстовое описание прогресса
- `result` - результат развёртки
- `error` - ошибка

**Методы:**
- `runUnfold(mesh, config)` - запустить развёртку
- `cancel()` - отменить развёртку
- `reset()` - сбросить состояние
- `dispose()` - очистить ресурсы

**Пример использования:**
```typescript
import { useAsyncUnfold } from '@frontend/composables/useAsyncUnfold'

const { isProcessing, progress, result, error, runUnfold, cancel } = useAsyncUnfold()

await runUnfold(meshData, {
  algorithm: 'mds',
  max_iterations: 100,
  tolerance: 1e-6,
  preserve_detail: true
})
```

### 3. Обновлённый UI (`src/components/UnfoldButton.vue`)

Компонент кнопки с индикатором прогресса и кнопкой отмены.

**Функции:**
- Отображение прогресса выполнения
- Анимация во время работы
- Кнопка отмены операции
- Сообщения об успехе/ошибке

### 4. Обновлённый Store (`src/stores/projectStore.ts`)

Добавлены типы и методы для поддержки новой архитектуры:

**Новые типы:**
- `Vertex3D` - 3D вершина
- `Face3D` - грань меша
- `MeshData` - данные меша
- `UnfoldedResult` - результат развёртки

**Новые методы:**
- `setCurrentMesh(mesh)` - установить текущий меш
- `setUnfoldedResult(result)` - установить результат развёртки
- `setError(message)` - установить ошибку

## Настройка Vite

### `vite.config.ts`

Добавлены настройки для поддержки Web Workers и WASM:

```typescript
export default defineConfig({
  resolve: {
    alias: {
      '@pepakura_wasm': path.resolve(__dirname, './crates/pepakura_wasm'),
      '@frontend': path.resolve(__dirname, './frontend/src'),
    }
  },
  server: {
    headers: {
      // Требуется для SharedArrayBuffer и кросс-оригинных воркеров
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
  worker: {
    format: 'es',
  },
})
```

## Критерии приемки

### ✅ UI остается отзывчивым во время расчета

- Курсор меняется на `wait` во время обработки
- Кнопки остаются нажимаемыми (кроме кнопки запуска)
- Анимация прогресс-бара работает плавно
- Кнопка "Отмена" доступна во время выполнения

### ✅ Настройка CORS для WASM в воркере

Vite настроен с заголовками:
- `Cross-Origin-Opener-Policy: same-origin`
- `Cross-Origin-Embedder-Policy: require-corp`

Эти заголовки необходимы для загрузки WASM модулей в Web Workers без ошибок CORS.

## Архитектура

```
┌─────────────────────────────────────────────────────────────┐
│                     Vue Component                            │
│                   (UnfoldButton.vue)                         │
└────────────────────────┬────────────────────────────────────┘
                         │ использует
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                  useAsyncUnfold Composable                   │
│  - initWorker()                                              │
│  - runUnfold(mesh, config)                                   │
│  - cancel()                                                  │
│  - Реактивные: isProcessing, progress, result, error         │
└────────────────────────┬────────────────────────────────────┘
                         │ отправляет сообщения
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Web Worker (unfold.worker.ts)                   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  1. Инициализация WASM: await init()                 │   │
│  │  2. Конвертация данных в WASM-формат                 │   │
│  │  3. Выполнение: unfold_mesh(meshWasm, configWasm)    │   │
│  │  4. Отправка результата в главный поток              │   │
│  └──────────────────────────────────────────────────────┘   │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              WASM Module (pepakura_wasm)                     │
│  - unfold_mesh()                                             │
│  - export_to_svg()                                           │
│  - optimize_nesting()                                        │
└─────────────────────────────────────────────────────────────┘
```

## Поток данных

1. **Пользователь нажимает "Развернуть модель"**
2. **Component → Composable**: Вызов `runUnfold(mesh, config)`
3. **Composable → Worker**: `postMessage({ type: 'UNFOLD_REQUEST', ... })`
4. **Worker**: Инициализация WASM, выполнение расчетов
5. **Worker → Composable**: `postMessage({ type: 'UNFOLD_SUCCESS', ... })`
6. **Composable → Component**: Обновление реактивных переменных
7. **Component → Store**: Вызов `setUnfoldedResult(result)`

## Отмена операции

1. Пользователь нажимает "Отмена"
2. Вызывается `cancel()` в composable
3. Отправляется `{ type: 'TERMINATE' }` воркеру
4. Вызывается `worker.terminate()` для немедленной остановки
5. Состояние сбрасывается

## Преимущества

- **Неблокирующий UI**: Интерфейс остается отзывчивым
- **Прогресс**: Пользователь видит прогресс выполнения
- **Отмена**: Возможность прервать длительную операцию
- **Изоляция**: Ошибки в воркере не ломают основное приложение
- **Масштабируемость**: Легко добавить новые типы расчетов

## Требования к сборке WASM

Перед запуском необходимо скомпилировать WASM модуль:

```bash
cd crates/pepakura_wasm
wasm-pack build --target web
```

### Важные примечания

**pepakura_core Cargo.toml:**
- `tokio` и `reqwest` вынесены в `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]`
- Добавлен feature `native = ["reqwest", "tokio", "tokio-stream"]`
- Для WASM используются `wasm-bindgen-futures` и `gloo-timers`

Это необходимо потому что:
- `tokio` с `features = ["full"]` не поддерживает WASM target
- `reqwest` зависит от `mio` и `tokio`, которые не работают в WASM
- Для WASM версии эти зависимости исключаются через `cfg(target_arch)`

## Примечания

- Воркер использует `type: 'module'` для загрузки ES модулей
- WASM загружается внутри воркера для избежания CORS проблем
- Для production сборки может потребоваться дополнительная настройка `worker-loader`
