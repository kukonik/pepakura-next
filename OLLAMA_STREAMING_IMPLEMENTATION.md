# Ollama Streaming Integration - Backend

## Обзор
Реализован прямой HTTP стриминг с Ollama через Tauri Events, без ожидания полного ответа.

## Архитектура

```
Frontend (Vue)          Tauri IPC                Ollama API
    │                      │                        │
    ├── invoke("ai_chat_stream_native") ───────────►│
    │                      │                        │
    │                      ├── ureq POST /api/chat ─►│
    │                      │                        │
    │                      │◄── NDJSON stream ──────┤
    │                      │    (line by line)      │
    │                      │                        │
    │◄── emit("ollama-stream-chunk") ───────────────┤
    │◄── emit("ollama-stream-chunk") ───────────────┤
    │◄── emit("ollama-stream-chunk") ───────────────┤
    │                      │                        │
    │◄── emit("ollama-stream-done") ────────────────┤
    │                      │                        │
```

## Реализованные функции

### 1. `ai_chat_stream_native`

**Сигнатура:**
```rust
#[tauri::command]
pub fn ai_chat_stream_native(
    app: AppHandle,
    message: String,
    history: Vec<ChatMessageDto>,
    state: State<'_, AiState>,
) -> Result<(), String>
```

**Описание:**
- Отправляет запрос напрямую к Ollama API через `ureq`
- Читает ответ по строкам (NDJSON) через `BufReader`
- Парсит каждую строку как `serde_json::Value`
- Извлекает текст из `["message"]["content"]`
- Отправляет каждый токен через `app.emit("ollama-stream-chunk", payload)`
- При `"done": true` отправляет `app.emit("ollama-stream-done", ())`
- При ошибках сети отправляет `app.emit("ollama-stream-error", error_msg)`

**Использование в фоне:**
```rust
std::thread::spawn(move || {
    // Синхронный HTTP вызов + чтение стрима
    // ...
});
```

### 2. `ai_chat_stream_with_cancel`

**Сигнатура:**
```rust
#[tauri::command]
pub async fn ai_chat_stream_with_cancel(
    app: AppHandle,
    message: String,
    history: Vec<ChatMessageDto>,
    state: State<'_, AiState>,
) -> Result<(), String>
```

**Описание:**
- То же самое, но через `tokio::task::spawn_blocking`
- Лучшая интеграция с async runtime Tauri
- Потенциальная поддержка отмены (future work)

**Использование в фоне:**
```rust
tokio::task::spawn_blocking(move || {
    // Синхронный HTTP вызов + чтение стрима
    // ...
});
```

## Tauri Events

### Входящие события (Rust → Frontend)

#### 1. `ollama-stream-chunk`
Отправляется для каждого токена/фрагмента текста.

**Payload:**
```typescript
interface OllamaStreamChunk {
  text: string       // Токен/фрагмент текста
  timestamp: number  // Unix timestamp (milliseconds)
}
```

**Пример:**
```json
{
  "text": "Привет",
  "timestamp": 1712345678901
}
```

#### 2. `ollama-stream-done`
Отправляется при завершении стрима.

**Payload:** `null` (empty)

#### 3. `ollama-stream-error`
Отправляется при ошибке сети или парсинга.

**Payload:**
```typescript
interface OllamaStreamError {
  error: string
  code?: number
}
```

**Пример:**
```json
{
  "error": "Network error: connection refused",
  "code": null
}
```

## Использование на фронтенде (Vue/TypeScript)

### Подписка на события

```typescript
import { listen } from '@tauri-apps/api/event'

// В компоненте Vue
onMounted(async () => {
  // Подписка на чанки
  const unlistenChunk = await listen<OllamaStreamChunk>(
    'ollama-stream-chunk',
    (event) => {
      streamingText.value += event.payload.text
      console.log('Received chunk:', event.payload.text)
    }
  )

  // Подписка на завершение
  const unlistenDone = await listen(
    'ollama-stream-done',
    () => {
      isStreaming.value = false
      console.log('Stream complete')
    }
  )

  // Подписка на ошибки
  const unlistenError = await listen<OllamaStreamError>(
    'ollama-stream-error',
    (event) => {
      isStreaming.value = false
      error.value = event.payload.error
      console.error('Stream error:', event.payload.error)
    }
  )

  // Сохраняем функции отписки для cleanup
  onUnmounted(() => {
    unlistenChunk()
    unlistenDone()
    unlistenError()
  })
})
```

### Запуск стриминга

```typescript
import { invoke } from '@tauri-apps/api/core'

async function startStreaming(message: string) {
  isStreaming.value = true
  streamingText.value = ''
  
  try {
    await invoke('ai_chat_stream_native', {
      message,
      history: chatHistory.value
    })
  } catch (error) {
    isStreaming.value = false
    console.error('Failed to start streaming:', error)
  }
}
```

### Полный пример компонента

```vue
<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

interface OllamaStreamChunk {
  text: string
  timestamp: number
}

interface OllamaStreamError {
  error: string
  code?: number
}

const streamingText = ref('')
const isStreaming = ref(false)
const error = ref<string | null>(null)
const chatHistory = ref<Array<{role: string, content: string}>>([])

let unlistenChunk: (() => void) | null = null
let unlistenDone: (() => void) | null = null
let unlistenError: (() => void) | null = null

onMounted(async () => {
  unlistenChunk = await listen<OllamaStreamChunk>(
    'ollama-stream-chunk',
    (event) => {
      streamingText.value += event.payload.text
    }
  )

  unlistenDone = await listen('ollama-stream-done', () => {
    isStreaming.value = false
  })

  unlistenError = await listen<OllamaStreamError>(
    'ollama-stream-error',
    (event) => {
      isStreaming.value = false
      error.value = event.payload.error
    }
  )
})

onUnmounted(() => {
  unlistenChunk?.()
  unlistenDone?.()
  unlistenError?.()
})

async function sendMessage(message: string) {
  error.value = null
  isStreaming.value = true
  streamingText.value = ''
  
  // Добавляем сообщение пользователя в историю
  chatHistory.value.push({ role: 'user', content: message })
  
  try {
    await invoke('ai_chat_stream_native', {
      message,
      history: chatHistory.value
    })
    
    // После запуска стрима, ответ будет приходить через events
    // Когда придёт "ollama-stream-done", добавим ответ в историю
    const unlistenDone = await listen('ollama-stream-done', () => {
      chatHistory.value.push({ 
        role: 'assistant', 
        content: streamingText.value 
      })
    })
  } catch (err) {
    isStreaming.value = false
    error.value = String(err)
  }
}
</script>

<template>
  <div class="streaming-chat">
    <div v-if="error" class="error">{{ error }}</div>
    <div class="streaming-text">{{ streamingText }}</div>
    <div v-if="isStreaming" class="typing-indicator">
      <span></span><span></span><span></span>
    </div>
    <button @click="sendMessage('Hello')" :disabled="isStreaming">
      Send
    </button>
  </div>
</template>
```

## Конфигурация

### Переменные окружения

```bash
# URL Ollama API (по умолчанию: http://localhost:11434)
OLLAMA_ENDPOINT=http://localhost:11434
```

### Зависимости (Cargo.toml)

```toml
[dependencies]
ureq = { version = "2.9", features = ["json"] }
serde_json = "1.0"
tauri = { version = "2.3.5", features = [] }
```

## Преимущества этого подхода

1. **Низкая задержка**: Пользователь видит ответ сразу, по мере генерации
2. **Эффективность памяти**: Не нужно хранить полный ответ в памяти перед отправкой
3. **UX**: Показываем typing indicator и прогресс генерации
4. **Надёжность**: Обработка ошибок сети и парсинга
5. **Гибкость**: Можно отменить стрим (future work)

## Отличия от `ai_chat_stream`

| Функция | Подход | Зависимости |
|---------|--------|-------------|
| `ai_chat_stream` | Через `PepakuraAssistant` | `pepakura_core::ai` |
| `ai_chat_stream_native` | Прямой HTTP к Ollama | `ureq` только |

`ai_chat_stream_native` более прямой и контролируемый подход, без абстракций core.

## Troubleshooting

### "Network error: connection refused"
- Убедитесь, что Ollama запущен: `ollama serve`
- Проверьте порт: `curl http://localhost:11434/api/tags`

### "Failed to parse JSON"
- Проверьте формат ответа Ollama (должен быть NDJSON)
- Включите логирование: `RUST_LOG=debug`

### Стрим не завершается
- Проверьте, что Ollama отправляет `"done": true`
- Включите таймаут: `ureq::AgentBuilder::new().timeout(Duration::from_secs(120))`
