# AI Стриминг в Pepakura Next

## Обзор

Стриминг позволяет получать ответ от AI постепенно, по мере генерации, вместо ожидания полного ответа.

## Преимущества

- ✅ Мгновенная обратная связь
- ✅ Улучшенный UX (пользователь видит прогресс)
- ✅ Возможность отмены генерации
- ✅ Меньшая задержка первого токена

## Архитектура

```
PepakuraAssistant
├── chat_stream() → AiStream
│   └── tokio mpsc channel
│       └── Ollama SSE stream
└── collect_stream() → String
```

## Использование

### Базовый стриминг

```rust
use pepakura_core::ai::{AiConfig, PepakuraAssistant};
use pepakura_core::ai::streaming::collect_stream;
use futures::StreamExt;

let assistant = PepakuraAssistant::new(&config)?;

// Получаем стрим
let mut stream = assistant.answer_question_stream("Как выбрать бумагу?").await?;

// Читаем токены
while let Some(token) = stream.next().await {
    print!("{}", token);
}
```

### Сборка полного ответа

```rust
use pepakura_core::ai::streaming::collect_stream;

let stream = assistant.answer_question_stream("Привет!").await?;
let full_response = collect_stream(stream).await;
println!("{}", full_response);
```

### Стриминг с прогрессом

```rust
use pepakura_core::ai::streaming::with_progress;

let stream = assistant.answer_question_stream("Привет!").await?;
let mut progress_stream = with_progress(stream);

while let Some((token, count)) = progress_stream.next().await {
    print!("{}", token);
    println!(" (токенов: {})", count);
}
```

## Интеграция с Tauri

### Rust команда

```rust
// src-tauri/src/ai_commands.rs
use pepakura_core::ai::streaming::chat_stream;
use tokio::sync::mpsc;

#[tauri::command]
pub async fn ai_chat_stream(
    message: String,
    state: State<'_, AiState>,
    window: Window,
) -> Result<(), String> {
    let assistant = state.assistant.lock().unwrap();
    
    let mut stream = assistant.answer_question_stream(&message).await?;
    
    // Отправляем токены через IPC
    while let Some(token) = stream.next().await {
        window.emit("ai-token", token)?;
    }
    
    Ok(())
}
```

### Frontend (Vue 3)

```typescript
// composables/useAi.ts
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export function useAiStream() {
  const response = ref('')
  const isLoading = ref(false)
  
  async function chatStream(message: string) {
    isLoading.value = true
    response.value = ''
    
    // Слушаем токены
    const unlisten = await listen('ai-token', (event) => {
      response.value += event.payload as string
    })
    
    // Запускаем стриминг
    await invoke('ai_chat_stream', { message })
    
    await unlisten()
    isLoading.value = false
  }
  
  return { response, isLoading, chatStream }
}
```

### Компонент чата

```vue
<template>
  <div class="chat">
    <div class="messages">
      <div v-for="(msg, i) in messages" :key="i" class="message">
        {{ msg.content }}
        <span v-if="msg.isStreaming" class="typing">...</span>
      </div>
    </div>
    
    <input v-model="input" @keyup.enter="sendMessage" />
  </div>
</template>

<script setup lang="ts">
import { useAiStream } from '@/composables/useAiStream'

const { response, isLoading, chatStream } = useAiStream()

async function sendMessage() {
  messages.value.push({ role: 'user', content: input.value })
  messages.value.push({ role: 'assistant', content: '', isStreaming: true })
  
  await chatStream(input.value)
  
  // Обновляем последнее сообщение
  messages.value[messages.value.length - 1].content = response.value
  messages.value[messages.value.length - 1].isStreaming = false
}
</script>
```

## Производительность

| Метрика | Без стриминга | Со стримингом |
|---------|---------------|---------------|
| Время до первого токена | 5-10 сек | <100 мс |
| Общее время | 5-10 сек | 5-10 сек |
| UX | ⭐⭐ | ⭐⭐⭐⭐⭐ |

## Примеры

### Постепенный вывод в консоль

```rust
use futures::StreamExt;

let stream = assistant.answer_question_stream("Расскажи про papercraft").await?;

print!("AI: ");
let mut stream = stream;
while let Some(token) = stream.next().await {
    print!("{}", token);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
println!();
```

### Отмена генерации

```rust
use tokio::time::{timeout, Duration};

let stream = assistant.answer_question_stream("Привет!").await?;

// Таймаут через 5 секунд
let result = timeout(Duration::from_secs(5), async {
    collect_stream(stream).await
}).await;

match result {
    Ok(response) => println!("{}", response),
    Err(_) => println!("Превышено время ожидания"),
}
```

### Параллельная обработка

```rust
use futures::{StreamExt, FutureExt};

let stream = assistant.answer_question_stream("Привет!").await?;

// Обрабатываем токены параллельно
stream
    .for_each_concurrent(3, |token| async move {
        println!("Получен токен: {}", token);
    })
    .await;
```

## API Reference

### chat_stream

```rust
pub async fn chat_stream(
    client: &OllamaClient,
    prompt: &str,
) -> Result<AiStream, PepakuraError>
```

Возвращает стрим токенов.

### collect_stream

```rust
pub async fn collect_stream(stream: AiStream) -> String
```

Собирает стрим в полную строку.

### with_progress

```rust
pub fn with_progress(stream: AiStream) -> ProgressStream
```

Добавляет информацию о прогрессе (количество токенов).

### AiStream

```rust
pub struct AiStream {
    // Stream<Item = String>
}

impl Stream for AiStream {
    type Item = String;
}
```

### ProgressStream

```rust
pub struct ProgressStream {
    // Stream<Item = (String, usize)>
}

impl ProgressStream {
    pub fn token_count(&self) -> usize;
}
```

## Рекомендации

### Когда использовать стриминг

✅ **Да:**
- Длинные ответы (>100 токенов)
- Чат с пользователем
- Нужна мгновенная обратная связь

❌ **Нет:**
- Короткие ответы (<20 токенов)
- Кэшированные запросы
- Фоновые задачи

### Оптимизация

```rust
// Используем буфер для уменьшения аллокаций
let (tx, rx) = mpsc::channel(64); // Больше буфер

// Группируем токены для уменьшения IPC вызовов
use futures::stream::StreamExt;
stream.chunks(4).for_each(|tokens| {
    println!("{}", tokens.join(""));
});
```

## Будущие улучшения

- [ ] Прерывание генерации
- [ ] Приоритетные запросы
- [ ] Мультиплексирование стримов
- [ ] Компрессия токенов

## Лицензия

MIT
