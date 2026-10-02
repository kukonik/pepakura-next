# Руководство по внедрению модулей машинного обучения в Pepakura Next

## Введение

Данное руководство описывает полный процесс интеграции модулей машинного обучения (ML) в проект Pepakura Next для расширения функционала анализа 3D-моделей с помощью локальных LLM (Ollama и llama.cpp).

## Цели внедрения

1. **Анализ 3D-моделей**: Автоматическая оценка сложности развёртки
2. **Рекомендации по улучшению**: AI-советы по оптимизации моделей для бумажного моделирования
3. **Объяснение проблем**: Детальное описание проблем развёртки на естественном языке
4. **Интеграция в UI**: Встроенные AI-функции в пользовательский интерфейс

## Требования к окружению

### Обязательные
- Установленный Rust и Cargo (версия 1.70+)
- Проект Pepakura Next (актуальная версия)
- Git для управления версиями

### Опциональные (для LLM)
- Установленный Ollama (рекомендуется) или совместимый сервер LLM
- Доступ к интернету для загрузки моделей
- 8+ ГБ оперативной памяти для работы с моделями

## Структура проекта после интеграции

```
pepakura-next/
├── crates/
│   ├── pepakura_core/
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── Cargo.toml
│   │       ├── ai/                    # Модули машинного обучения
│   │       │   ├── mod.rs
│   │       │   ├── local_llm.rs       # Интеграция с локальными LLM
│   │       │   ├── prompts.rs         # Промпты для анализа моделей
│   │       │   ├── cache.rs           # Кэширование ответов LLM
│   │       │   └── streaming.rs       # Стриминг ответов
│   │       └── analysis/              # Анализ 3D-моделей
│   │           ├── mod.rs
│   │           ├── mesh_stats.rs      # Статистика мешей
│   │           └── mesh_analyzer.rs   # Анализатор с LLM
│   └── pepakura_debug/                # CLI-утилита для отладки
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           └── commands.rs
└── workspace Cargo.toml
```

## Пошаговое руководство по внедрению

### Шаг 1: Подготовка проекта

1. **Клонирование репозитория** (если не сделано):
   ```bash
   git clone https://github.com/pepakura-next/pepakura-next.git
   cd pepakura-next
   ```

2. **Проверка структуры проекта**:
   ```bash
   ls -la crates/
   ```

### Шаг 2: Создание директорий для новых модулей

Модули уже существуют в проекте. Проверьте их наличие:
```bash
ls crates/pepakura_core/src/ai/
ls crates/pepakura_core/src/analysis/
```

Если модули отсутствуют, создайте их:
```bash
mkdir -p crates/pepakura_core/src/ai
mkdir -p crates/pepakura_core/src/analysis
```

### Шаг 3: Копирование файлов модулей

Файлы модулей уже присутствуют в проекте. Убедитесь в их корректности:

1. **Основные файлы AI-модуля**:
   - `crates/pepakura_core/src/ai/mod.rs` - точка входа модуля
   - `crates/pepakura_core/src/ai/local_llm.rs` - интеграция с Ollama/llama.cpp
   - `crates/pepakura_core/src/ai/prompts.rs` - промпты для анализа
   - `crates/pepakura_core/src/ai/cache.rs` - кэширование
   - `crates/pepakura_core/src/ai/streaming.rs` - стриминг

2. **Модули анализа**:
   - `crates/pepakura_core/src/analysis/mod.rs` - точка входа
   - `crates/pepakura_core/src/analysis/mesh_stats.rs` - статистика
   - `crates/pepakura_core/src/analysis/mesh_analyzer.rs` - анализатор с LLM

3. **CLI-утилита**:
   - `crates/pepakura_debug/src/main.rs` - основной файл CLI
   - `crates/pepakura_debug/src/commands.rs` - команды

### Шаг 4: Обновление конфигурационных файлов

#### 4.1. Обновление `pepakura_core/Cargo.toml`

Убедитесь, что в файле `crates/pepakura_core/Cargo.toml` присутствуют следующие зависимости:

```toml
[features]
default = ["sqlite", "native"]
sqlite = ["rusqlite"]
native = ["reqwest", "tokio", "tokio-stream", "parallel"]
wasm = []
parallel = ["rayon"]
llm = ["ureq"]  # Функция для включения LLM

[dependencies]
# ... другие зависимости ...

# LLM-зависимости (синхронный HTTP для простоты)
ureq = { version = "2", features = ["json"], optional = true }
```

#### 4.2. Обновление `pepakura_core/src/lib.rs`

Добавьте модули в корневой файл библиотеки. Проверьте наличие:

```rust
// В начале файла или в соответствующем месте
pub mod ai;
pub mod analysis;
```

#### 4.3. Обновление workspace `Cargo.toml`

Убедитесь, что `pepakura_debug` включен в workspace:

```toml
[workspace]
members = [
    "crates/pepakura_core",
    "crates/pepakura_wasm",
    "crates/pepakura_platform",
    "crates/pepakura_addons",
    "crates/ai_bridge",
    "crates/pepakura_debug",  # Убедитесь, что эта строка есть
    # ... другие члены ...
]
```

### Шаг 5: Адаптация типов данных

#### 5.1. Проверка совместимости `MeshStats`

Файл `mesh_stats.rs` уже адаптирован для работы с существующей структурой `Mesh`. Убедитесь, что:

1. Импорты корректны:
   ```rust
   use crate::geometry::{Mesh, Vertex, Face};
   ```

2. Метод `from_mesh` принимает правильный тип:
   ```rust
   pub fn from_mesh(mesh: &Mesh) -> Self
   ```

#### 5.2. Реализация трейта `MeshProvider` (если требуется)

Если в вашем проекте используется трейт `MeshProvider`, убедитесь, что тип `Mesh` его реализует:

```rust
impl MeshProvider for Mesh {
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }
    
    fn faces(&self) -> &[Face] {
        &self.faces
    }
}
```

### Шаг 6: Сборка и проверка работоспособности

#### 6.1. Сборка с активированной функцией `llm`

```bash
cd pepakura-next
cargo build --features llm --package pepakura_core
```

#### 6.2. Проверка сборки CLI-утилиты

```bash
cargo build --package pepakura_debug
```

#### 6.3. Тестирование LLM-статуса

```bash
# Запустите Ollama перед тестированием
ollama serve &

# Проверьте статус LLM
cargo run --package pepakura_debug -- llm-status
```

Ожидаемый вывод при успешном подключении:
```
✅ LLM бэкенд доступен
  Название: Ollama
  Версия: 0.1.0
  Модели: ["qwen2.5:7b", "llama3:8b", "mistral:7b"]
```

#### 6.4. Проверка WASM-сборки

Убедитесь, что модуль `ai` исключен из WASM-сборки с помощью условной компиляции:

```bash
cargo build --package pepakura_wasm --target wasm32-unknown-unknown
```

### Шаг 7: Интеграция в пользовательский интерфейс

#### 7.1. Использование CLI-утилиты

```bash
# Анализ модели с объяснением проблем
cargo run --package pepakura_debug -- analyze tests/models/cube.obj --explain

# Развёртка с LLM-помощью
cargo run --package pepakura_debug -- unfold tests/models/complex.obj --explain
```

#### 7.2. Интеграция `MeshAnalyzer` в Tauri бэкенд

Добавьте в ваш Tauri-бэкенд:

```rust
use pepakura_core::analysis::mesh_analyzer::MeshAnalyzer;
use pepakura_core::geometry::Mesh;

#[tauri::command]
async fn analyze_model(path: String) -> Result<AnalysisResult, String> {
    let mesh = Mesh::load_from_file(&path)?;
    let mut analyzer = MeshAnalyzer::new();
    let result = analyzer.analyze(&mesh);
    Ok(result)
}
```

#### 7.3. Вызов Ollama API из браузерного интерфейса

Для прямого вызова Ollama из JavaScript:

```javascript
async function analyzeWithOllama(meshData) {
    const response = await fetch('http://localhost:11434/api/generate', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            model: 'qwen2.5:7b',
            prompt: `Проанализируй 3D модель для бумажного моделирования: ${JSON.stringify(meshData.stats)}`,
            stream: false
        })
    });
    
    return await response.json();
}
```

**Важно**: Настройте CORS политику Ollama для доступа из веб-браузера:
```bash
# Запуск Ollama с CORS
OLLAMA_ORIGINS="*" ollama serve
```

## Критические замечания и рекомендации

### 1. **Избегайте выполнения `cargo fix`**
   - Автоматическое исправление может нарушить работоспособность кода
   - Вместо этого используйте `cargo clippy` для рекомендаций

### 2. **Используйте синхронный HTTP-клиент `ureq`**
   - Для совместимости с Tauri и простоты интеграции
   - Асинхронные клиенты могут требовать дополнительной настройки

### 3. **Исключение модуля `ai` из WASM-сборки**
   - Используйте условную компиляцию `#[cfg(feature = "llm")]`
   - WASM-сборка должна работать без модуля AI

### 4. **Настройка CORS политики Ollama**
   ```bash
   # Linux/Mac
   export OLLAMA_ORIGINS="*"
   ollama serve
   
   # Windows (PowerShell)
   $env:OLLAMA_ORIGINS="*"
   ollama serve
   ```

### 5. **Управление памятью**
   - Ollama + WASM-сборка могут потреблять много оперативной памяти
   - Рекомендуется не запускать их одновременно на слабых машинах
   - Используйте флаг `--num-threads` для ограничения использования CPU

### 6. **Кэширование ответов LLM**
   - Включите кэширование для повторяющихся запросов
   - Используйте `AiCache` для уменьшения нагрузки на LLM-бэкенд

## Тестирование

### Базовые тесты
```bash
# Тест статистики меша
cargo test --package pepakura_core --test mesh_stats

# Тест анализатора
cargo test --package pepakura_core --test mesh_analyzer

# Тест CLI-команд
cargo test --package pepakura_debug
```

### Интеграционные тесты
1. **Тест с реальной моделью**:
   ```bash
   cargo run --package pepakura_debug -- analyze test_files/cube.obj
   ```

2. **Тест LLM-интеграции**:
   ```bash
   # Требует запущенного Ollama
   cargo run --package pepakura_debug -- llm-status
   ```

## Устранение неполадок

### Проблема: LLM бэкенд недоступен
**Решение**:
1. Проверьте, запущен ли Ollama: `ollama list`
2. Проверьте URL: по умолчанию `http://localhost:11434`
3. Проверьте CORS настройки

### Проблема: Ошибка сборки WASM
**Решение**:
1. Убедитесь, что модуль `ai` исключен из WASM-сборки
2. Проверьте условную компиляцию `#[cfg(not(target_arch = "wasm32"))]`

### Проблема: Высокое потребление памяти
**Решение**:
1. Используйте меньшие модели (например, `qwen2.5:3b` вместо `7b`)
2. Ограничьте количество потоков: `OLLAMA_NUM_THREADS=4`
3. Закрывайте неиспользуемые соединения

## Расширенные возможности

### 1. **Поддержка нескольких LLM-бэкендов**
   - Ollama (рекомендуется)
   - llama.cpp через HTTP-сервер
   - OpenAI API (облачный)

### 2. **Кастомные промпты**
   - Редактируйте `prompts.rs` для настройки анализа
   - Добавьте доменно-специфичные промпты

### 3. **Пакетный анализ**
   ```rust
   let analyzer = MeshAnalyzer::new();
   for model in models {
       let result = analyzer.analyze(&model);
       // Обработка результатов
   }
   ```

### 4. **Экспорт отчетов**
   - JSON-отчеты для интеграции с CI/CD
   - HTML-отчеты для визуализации
   - PDF-отчеты для документации

## Заключение

Данное руководство обеспечивает полный цикл внедрения модулей машинного обучения в проект Pepakura Next. После успешной интеграции вы получите:

1. **Автоматический анализ 3D-моделей** с оценкой сложности
2. **Интеллектуальные рекомендации** по улучшению моделей
3. **Интеграцию с UI** через Tauri и JavaScript
4. **CLI-инструменты** для отладки и пакетной обработки

Для дальнейшего развития системы рекомендуется:
- Добавление поддержки большего количества форматов 3D-моделей
- Интеграция с облачными LLM-сервисами
- Развитие системы кэширования и оптимизации
- Создание плагинов для популярных 3D-редакторов

## Полезные ссылки

- [Ollama документация](https://github.com/ollama/ollama)
- [llama.cpp](https://github.com/ggerganov/llama.cpp)
- [Rust и WASM](https://rustwasm.github.io/docs/book/)
- [Tauri документация](https://tauri.app/v1/guides/)

---

*Последнее обновление: $(date)*  
*Версия руководства: 1.0*