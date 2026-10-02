# Быстрый старт: Интеграция AI-модулей в Pepakura Next

## Краткое руководство за 10 минут

### 1. Установка зависимостей

```bash
# Установите Ollama (если ещё не установлен)
# Windows: скачайте с https://ollama.com/download/windows
# Linux/Mac: curl -fsSL https://ollama.com/install.sh | sh

# Запустите Ollama
ollama serve &

# Установите модель (рекомендуется)
ollama pull qwen2.5:7b
```

### 2. Клонирование и настройка проекта

```bash
# Клонируйте проект (если ещё не сделано)
git clone https://github.com/pepakura-next/pepakura-next.git
cd pepakura-next

# Проверьте структуру проекта
ls crates/pepakura_core/src/ai/
```

### 3. Быстрая сборка

```bash
# Сборка с поддержкой LLM
cargo build --features llm --package pepakura_core

# Сборка CLI-утилиты
cargo build --package pepakura_debug
```

### 4. Тестирование интеграции

```bash
# Проверка статуса LLM
cargo run --package pepakura_debug -- llm-status

# Создайте тестовую модель для анализа
cat > test_cube.obj << 'EOF'
# Simple cube
v -1 -1 -1
v -1 -1 1
v -1 1 -1
v -1 1 1
v 1 -1 -1
v 1 -1 1
v 1 1 -1
v 1 1 1
f 1 2 4 3
f 5 6 8 7
f 1 2 6 5
f 3 4 8 7
f 1 3 7 5
f 2 4 8 6
EOF

# Анализ тестовой модели
cargo run --package pepakura_debug -- analyze test_cube.obj --explain
```

### 5. Интеграция в ваш код

```rust
// Добавьте в ваш Cargo.toml
[dependencies]
pepakura_core = { path = "../pepakura_core", features = ["llm"] }

// Использование в коде
use pepakura_core::analysis::mesh_analyzer::MeshAnalyzer;
use pepakura_core::geometry::Mesh;

fn analyze_model(path: &str) {
    let mesh = Mesh::load_from_file(path).unwrap();
    let analyzer = MeshAnalyzer::new();
    let result = analyzer.analyze(&mesh);
    
    println!("Статистика: {}", result.stats.summary());
    
    if let Some(ai) = result.ai_analysis {
        println!("Сложность: {}", ai.difficulty);
        println!("Оценка: {:.2}/1.0", ai.overall_score);
    }
}
```

### 6. Решение распространённых проблем

#### Проблема: "LLM недоступен"
```bash
# Решение 1: Проверьте, запущен ли Ollama
ollama list

# Решение 2: Запустите Ollama с CORS
OLLAMA_ORIGINS="*" ollama serve

# Решение 3: Используйте другой порт
cargo run --package pepakura_debug -- llm-status --url http://localhost:11435
```

#### Проблема: "Модель не найдена"
```bash
# Установите модель
ollama pull qwen2.5:7b

# Или используйте другую модель
cargo run --package pepakura_debug -- analyze model.obj --model llama3:8b
```

#### Проблема: "Высокое потребление памяти"
```bash
# Используйте меньшую модель
ollama pull qwen2.5:3b

# Ограничьте использование потоков
export OLLAMA_NUM_THREADS=2
ollama serve
```

### 7. Готовые примеры использования

#### Пример 1: Пакетный анализ моделей
```rust
use pepakura_core::analysis::mesh_analyzer::MeshAnalyzer;
use pepakura_core::geometry::Mesh;
use std::fs;

fn batch_analyze(models_dir: &str) {
    let analyzer = MeshAnalyzer::new();
    
    for entry in fs::read_dir(models_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|e| e == "obj").unwrap_or(false) {
            let mesh = Mesh::load_from_file(&path.to_string_lossy()).unwrap();
            let result = analyzer.analyze(&mesh);
            println!("{}: {}", path.display(), result.stats.summary());
        }
    }
}
```

#### Пример 2: Интеграция с Tauri
```rust
#[tauri::command]
async fn analyze_model_with_ai(path: String) -> Result<AnalysisResult, String> {
    let mesh = Mesh::load_from_file(&path)
        .map_err(|e| format!("Ошибка загрузки: {}", e))?;
    
    let analyzer = MeshAnalyzer::new();
    let result = analyzer.analyze(&mesh);
    
    Ok(result)
}
```

#### Пример 3: Веб-интерфейс с JavaScript
```javascript
async function analyzeModel(file) {
    const formData = new FormData();
    formData.append('model', file);
    
    const response = await fetch('http://localhost:11434/api/generate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            model: 'qwen2.5:7b',
            prompt: `Проанализируй эту 3D модель для бумажного моделирования`,
            stream: false
        })
    });
    
    return await response.json();
}
```

### 8. Дальнейшие шаги

1. **Изучите полное руководство**: `docs/ai/ML_INTEGRATION_GUIDE.md`
2. **Настройте промпты**: `crates/pepakura_core/src/ai/prompts.rs`
3. **Добавьте кэширование**: Используйте `AiCache` для производительности
4. **Интегрируйте в UI**: Добавьте AI-функции в интерфейс пользователя

### 9. Полезные команды для отладки

```bash
# Проверка всех зависимостей
cargo tree --features llm

# Запуск тестов
cargo test --features llm --package pepakura_core

# Проверка WASM-сборки
cargo build --package pepakura_wasm --target wasm32-unknown-unknown

# Генерация документации
cargo doc --features llm --open
```

### 10. Готово!

Вы успешно интегрировали AI-модули в Pepakura Next. Теперь вы можете:

- ✅ Анализировать 3D-модели с помощью LLM
- ✅ Получать рекомендации по улучшению
- ✅ Использовать CLI-утилиту для отладки
- ✅ Интегрировать AI в свой код

Для вопросов и поддержки обратитесь к документации или создайте issue в репозитории.

---

*Версия: 1.0 | Последнее обновление: $(date)*