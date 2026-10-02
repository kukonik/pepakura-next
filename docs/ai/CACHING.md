# AI Кэширование в Pepakura Next

## Обзор

AI-модуль теперь поддерживает кэширование ответов для уменьшения задержек и нагрузки на LLM.

## Архитектура

```
PepakuraAssistant
├── OllamaClient (HTTP клиент)
└── AiCache (LRU кэш)
    ├── Memory cache (быстрый доступ)
    └── Statistics (hits, misses, saves)
```

## Использование

### Базовое

```rust
use pepakura_core::ai::{AiConfig, PepakuraAssistant};

let config = AiConfig::default();
let assistant = PepakuraAssistant::new(&config)?;

// Ответы автоматически кэшируются
let answer = assistant.answer_question("Как выбрать бумагу?").await?;

// Повторный запрос будет мгновенным (из кэша)
let answer_cached = assistant.answer_question("Как выбрать бумагу?").await?;

// Статистика кэша
let stats = assistant.get_cache_stats();
println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
println!("Hit rate: {:.1}%", assistant.cache_hit_rate());
```

### Кастомный размер кэша

```rust
// Кэш на 500 записей
let assistant = PepakuraAssistant::with_cache(&config, 500)?;
```

### Очистка кэша

```rust
assistant.clear_cache();
```

## Типы кэшируемых запросов

### answer_question
Кэшируется по тексту вопроса.

```rust
// Первый запрос → Ollama
let answer = assistant.answer_question("Как склеить грани?").await?;

// Повторный запрос → кэш
let answer = assistant.answer_question("Как склеить грани?").await?;
```

### get_unfold_advice
Кэшируется по параметрам меша (вершины, грани, имя).

```rust
let advice = assistant.get_unfold_advice(&mesh).await?;

// Повторный запрос для того же меша → кэш
let advice = assistant.get_unfold_advice(&mesh).await?;
```

## Статистика

```rust
let stats = assistant.get_cache_stats();

println!("Попадания: {}", stats.hits);
println!("Промахи: {}", stats.misses);
println!("Сохранения: {}", stats.saves);
println!("Процент попаданий: {:.1}%", assistant.cache_hit_rate());
```

## Производительность

### Бенчмарк

| Сценарий | Без кэша | С кэшем |
|----------|----------|---------|
| Первый запрос | 5-10 сек | 5-10 сек |
| Повторный запрос | 5-10 сек | <1 мс |
| Hit rate (типичный) | 0% | 60-80% |

### Рекомендации

1. **Размер кэша**: 1000 записей по умолчанию
2. **Очистка**: При изменении контекста
3. **Мониторинг**: Следить за hit rate

## Реализация

### LRU стратегия

Кэш использует Least Recently Used (LRU) стратегию:
- Старые неиспользуемые записи вытесняются
- Популярные записи остаются в кэше

### Хэширование

Запросы хэшируются через SHA-256:
```rust
let hash = Sha256::digest(prompt.as_bytes());
```

### Потокбезопасность

Кэш использует `Arc<Mutex<LruCache>>` для безопасного доступа из нескольких потоков.

## Примеры

### Интеграция с Tauri

```rust
// src-tauri/src/ai_commands.rs
use pepakura_core::ai::PepakuraAssistant;

#[tauri::command]
pub async fn ai_chat(
    message: String,
    state: State<'_, AiState>,
) -> Result<String, String> {
    let assistant = state.assistant.lock().unwrap();
    
    // Проверяем кэш
    if let Some(cached) = assistant.cache.get(&message) {
        return Ok(cached);
    }
    
    // Запрашиваем у AI
    let response = assistant.answer_question(&message).await?;
    
    Ok(response)
}
```

### Мониторинг кэша

```rust
// Периодическая печать статистики
std::thread::spawn(move || {
    loop {
        std::thread::sleep(Duration::from_secs(60));
        
        let stats = assistant.get_cache_stats();
        log::info!("AI Cache: hits={}, misses={}, hit_rate={:.1}%",
            stats.hits,
            stats.misses,
            assistant.cache_hit_rate()
        );
    }
});
```

## Будущие улучшения

### Персистентный кэш
Сохранение на диск между запусками:
```rust
use pepakura_core::ai::cache::PersistentCache;

let cache = PersistentCache::new("cache.db", 1000)?;
```

### TTL (Time To Live)
Автоматическая очистка старых записей:
```rust
cache.set_ttl(Duration::from_hours(24));
```

### Префетчинг
Предварительная загрузка частых запросов.

## Лицензия

MIT
