# AI-модуль Pepakura Core

## Обзор

AI-модуль предоставляет интеграцию с языковыми моделями для:
- Генерации инструкций по сборке
- Рекомендаций по параметрам развёртки
- Ответов на вопросы по papercraft

## Поддерживаемые провайдеры

### Ollama (локальная LLM)

**Преимущества:**
- Работает локально (без интернета)
- Бесплатно
- Конфиденциально

**Требования:**
- Установленный Ollama: https://ollama.ai
- Запущенный сервис: `ollama serve`

### OpenAI (облачная LLM)

**Преимущества:**
- Высокое качество ответов
- Много моделей (GPT-4, GPT-3.5)

**Требования:**
- API ключ OpenAI

## Быстрый старт

### 1. Установка Ollama

```bash
# Windows/Mac/Linux
# Скачайте с https://ollama.ai

# Установите модель
ollama pull llama3.2
```

### 2. Использование в коде

```rust
use pepakura_core::ai::{AiConfig, PepakuraAssistant};

// Создаём конфигурацию
let config = AiConfig::default(); // Ollama, localhost:11434, llama3.2

// Создаём ассистента
let assistant = PepakuraAssistant::new(&config)?;

// Проверяем доступность
if assistant.check_availability().await {
    // Получаем рекомендации
    let advice = assistant.get_unfold_advice(&mesh).await?;
    println!("Алгоритм: {}", advice.algorithm);
    
    // Генерируем инструкцию
    let instructions = assistant.generate_assembly_instructions(&unfolded).await?;
    println!("Сложность: {}", instructions.difficulty);
}
```

## API Reference

### AiConfig

Конфигурация AI-провайдера.

```rust
let config = AiConfig {
    provider: AiProvider::Ollama,
    ollama_url: "http://localhost:11434".to_string(),
    model: "llama3.2".to_string(),
    temperature: 0.7,
    max_tokens: 2048,
    timeout_sec: 60,
};
```

**Методы:**
- `AiConfig::ollama()` — конфигурация Ollama по умолчанию
- `AiConfig::ollama_with_url(url)` — Ollama с кастомным URL
- `AiConfig::openai(api_key)` — конфигурация OpenAI
- `with_model(model)` — установить модель
- `with_temperature(temp)` — установить температуру (0.0-1.0)
- `with_timeout(sec)` — установить таймаут

### PepakuraAssistant

AI-ассистент для papercraft.

#### check_availability

```rust
async fn check_availability(&self) -> bool
```

Проверяет доступность AI-сервиса.

#### get_unfold_advice

```rust
async fn get_unfold_advice(&self, mesh: &Mesh) -> Result<UnfoldAdvice>
```

Получает рекомендации по развёртке для меша.

**Возвращает:**
- `algorithm` — рекомендуемый алгоритм
- `parameters` — параметры развёртки
- `potential_issues` — возможные проблемы
- `tips` — советы

#### get_model_description

```rust
async fn get_model_description(&self, mesh: &Mesh) -> Result<String>
```

Генерирует описание модели.

#### generate_assembly_instructions

```rust
async fn generate_assembly_instructions(&self, unfolded: &UnfoldedMesh) -> Result<AssemblyInstruction>
```

Генерирует пошаговую инструкцию сборки.

**Возвращает:**
- `model_name` — название модели
- `difficulty` — уровень сложности
- `total_time_minutes` — общее время
- `steps` — шаги сборки
- `tips` — советы

#### answer_question

```rust
async fn answer_question(&self, question: &str) -> Result<String>
```

Отвечает на вопрос по papercraft.

#### recommend_paper

```rust
async fn recommend_paper(&self, model_name: &str, scale: f64) -> Result<String>
```

Рекомендует бумагу для модели.

## Примеры

### Рекомендации по развёртке

```rust
use pepakura_core::ai::{AiConfig, PepakuraAssistant};
use pepakura_core::geometry::Mesh;

let mesh = Mesh::load("model.obj")?;

let config = AiConfig::default();
let assistant = PepakuraAssistant::new(&config)?;

let advice = assistant.get_unfold_advice(&mesh).await?;

println!("Рекомендуемый алгоритм: {}", advice.algorithm);
println!("Итерации: {}", advice.parameters.max_iterations);
println!("Допуск: {}", advice.parameters.tolerance);

for issue in &advice.potential_issues {
    println!("Возможная проблема: {}", issue);
}
```

### Генерация инструкции сборки

```rust
use pepakura_core::ai::PepakuraAssistant;
use pepakura_core::unfold::unfold_mds;

let unfolded = unfold_mds(&mesh, &config)?;

let instructions = assistant.generate_assembly_instructions(&unfolded).await?;

println!("Модель: {}", instructions.model_name);
println!("Сложность: {}", instructions.difficulty);
println!("Время: {} мин", instructions.total_time_minutes);

for step in &instructions.steps {
    println!("{}. {}", step.step_number, step.description);
}
```

### Ответ на вопрос

```rust
let question = "Какую бумагу выбрать для архитектурной модели?";
let answer = assistant.answer_question(question).await?;
println!("{}", answer);
```

### Работа с OpenAI

```rust
use pepakura_core::ai::AiConfig;

let config = AiConfig::openai("sk-your-api-key");
let assistant = PepakuraAssistant::new(&config)?;

let answer = assistant.answer_question("Как склеить сложные грани?").await?;
```

## Настройка Ollama

### Установка

```bash
# Windows: скачайте установщик с ollama.ai
# macOS:
brew install ollama

# Linux:
curl -fsSL https://ollama.ai/install.sh | sh
```

### Запуск

```bash
# Запуск сервиса
ollama serve

# В другом терминале - загрузка модели
ollama pull llama3.2
```

### Проверка

```bash
# Список моделей
ollama list

# Тестовый запрос
ollama run llama3.2 "Привет!"
```

### Рекомендуемые модели

| Модель | Размер | Качество | Скорость |
|--------|--------|----------|----------|
| llama3.2 | 3GB | Хорошее | Быстро |
| mistral | 4GB | Хорошее | Средне |
| gemma:7b | 5GB | Отличное | Средне |
| llama3:8b | 5GB | Отличное | Медленно |

## Обработка ошибок

```rust
use pepakura_core::PepakuraError;

match assistant.get_unfold_advice(&mesh).await {
    Ok(advice) => println!("Советы получены"),
    Err(PepakuraError::AiError(msg)) => println!("AI ошибка: {}", msg),
    Err(e) => println!("Другая ошибка: {}", e),
}
```

## Тестирование

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_assistant_available() {
        let config = AiConfig::default();
        let assistant = PepakuraAssistant::new(&config).unwrap();
        
        // Будет true если Ollama запущен
        let available = assistant.check_availability().await;
        println!("AI доступен: {}", available);
    }
}
```

## Лучшие практики

1. **Проверяйте доступность** перед использованием
2. **Кэшируйте ответы** для частых запросов
3. **Используйте таймауты** для долгих запросов
4. **Логируйте запросы** для отладки
5. **Обрабатывайте ошибки** gracefully

## Лицензия

MIT
