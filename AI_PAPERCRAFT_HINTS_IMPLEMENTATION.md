# AI Подсказки для Papercraft - Реализация

## 📋 Обзор

Реализованы 4 новые AI-функции для улучшения работы с papercraft моделями:

1. ✅ **Анализ искажений развёртки (тепловая карта)**
2. ✅ **Рекомендации по оптимизации раскладки на листе**
3. ✅ **Авто-генерация инструкций по сборке** (расширено)
4. ✅ **Детекция проблемных граней** (слишком маленькие, острые углы)

---

## 🔍 1. Анализ искажений развёртки (тепловая карта)

### Файлы реализации
- **Модуль ядра:** `crates/pepakura_core/src/analysis/distortion_analysis.rs`
- **Tauri команда:** `src-tauri/src/ai_commands.rs` → `ai_analyze_distortion()`
- **Экспорт:** `crates/pepakura_core/src/analysis/mod.rs`

### Функционал

#### Вычисляемые метрики:
- **Area Distortion** - искажение площадей граней при развёртке 3D→2D
- **Angular Distortion** - искажение углов (в градусах)
- **Edge Length Distortion** - искажение длин рёбер
- **Composite Distortion** - композитная оценка (0.0 = нет искажений)

#### Тепловая карта:
- Генерация SVG с цветовой индикацией (blue → green → yellow → red)
- Каждая грань окрашена по уровню искажений
- Интерактивные подсказки при наведении

#### Статистика:
- Среднее, медиана, стандартное отклонение
- 95-й перцентиль
- Min/Max значения

### Пример использования

```rust
use pepakura_core::analysis::{DistortionAnalyzer, generate_distortion_advice};
use pepakura_core::geometry::Mesh;
use pepakura_core::unfold::UnfoldResult;

let mesh: Mesh = /* загрузить модель */;
let unfolded: UnfoldResult = /* развернуть */;

let analyzer = DistortionAnalyzer::new();
let result = analyzer.analyze(&mesh, &unfolded);

// Метрики качества
println!("Качество развёртки: {:.1}%", result.overall_quality_score * 100.0);
println!("Среднее искажение: {:.1}%", result.avg_area_distortion);
println!("Допустимых граней: {:.1}%", result.acceptable_faces_ratio * 100.0);

// SVG тепловая карта
let svg = analyzer.generate_heatmap_svg(&result, "Анализ модели");

// AI рекомендации
let tips = generate_distortion_advice(&result);
for tip in &tips {
    println!("{}", tip);
}
```

### Tauri IPC вызов

```javascript
const result = await invoke('ai_analyze_distortion', {
  mesh: meshData,
  unfolded: unfoldedData
});

console.log(`Качество: ${result.overall_quality_score * 100}%`);
console.log(`Проблемных граней: ${result.problematic_faces.length}`);
console.log(`Советы: ${result.ai_tips}`);
```

---

## 📊 2. Рекомендации по оптимизации раскладки на листе

### Файлы реализации
- **Модуль ядра:** `crates/pepakura_core/src/analysis/nesting_optimization.rs`
- **Tauri команда:** `src-tauri/src/ai_commands.rs` → `ai_analyze_nesting()`
- **Экспорт:** `crates/pepakura_core/src/analysis/mod.rs`

### Функционал

#### Анализ заполненности:
- Процент использования пространства каждого листа
- Выявление "от孤岛" деталей (далеко от других)
- Оценка компактности размещения

#### Оптимизация формата бумаги:
- Автоматический подбор формата (A4, A3, A2, A1)
- Расчёт потенциальной экономии бумаги
- Рекомендации по изменению масштаба модели

#### Оценка сложности сборки:
- Фактор количества листов
- Разброс размеров деталей
- Компактность раскладки

#### AI рекомендации:
- Изменить формат бумаги
- Оптимизировать масштаб
- Группировать детали
- Использовать генетический алгоритм
- Настроить угол поворота

### Пример использования

```rust
use pepakura_core::analysis::{NestingOptimizer, generate_nesting_advice};
use pepakura_core::nesting::NestResult;

let nest_result: NestResult = /* результат раскладки */;

let optimizer = NestingOptimizer::new();
let analysis = optimizer.analyze(&nest_result);

// Метрики
println!("Заполненность: {:.1}%", analysis.avg_fill_rate * 100.0);
println!("Листов: {}", analysis.sheets_count);
println!("Рекомендуемый формат: {:?}", analysis.suggested_paper_format);
println!("Экономия: {:.1}%", analysis.potential_savings_percent);

// AI советы
let tips = generate_nesting_advice(&analysis);
for tip in &tips {
    println!("{}", tip);
}
```

### Tauri IPC вызов

```javascript
const analysis = await invoke('ai_analyze_nesting', {
  nestResult: layoutData
});

console.log(`Заполненность: ${analysis.avg_fill_rate}%`);
console.log(`Формат бумаги: ${analysis.suggested_paper_format}`);
console.log(`Рекомендации:`, analysis.recommendations);
```

---

## 📋 3. Авто-генерация инструкций по сборке

### Файлы реализации
- **Модуль ядра:** `crates/pepakura_core/src/ai/assistant.rs` → `generate_assembly_instructions()`
- **Tauri команда:** `src-tauri/src/ai_commands.rs` → `ai_generate_instructions()`

### Функционал (расширено)

#### Генерация пошаговых инструкций:
- Автоматическое разбиение на логические шаги
- Оценка времени сборки
- Группировка связанных деталей

#### Оценка сложности:
- **Easy** - < 20 деталей
- **Medium** - 20-50 деталей
- **Hard** - 50-100 деталей
- **Expert** - > 100 деталей

#### Советы по сборке:
- Выбор клея
- Порядок сборки
- Советы по покраске

### Пример использования

```rust
use pepakura_core::ai::PepakuraAssistant;
use pepakura_core::ai::AiConfig;

let config = AiConfig::default();
let assistant = PepakuraAssistant::new(&config)?;

let instructions = assistant.generate_assembly_instructions(&unfolded).await?;

println!("Модель: {}", instructions.model_name);
println!("Сложность: {}", instructions.difficulty);
println!("Время сборки: {} мин", instructions.total_time_minutes);

for step in &instructions.steps {
    println!("{}. {}", step.step_number, step.description);
}
```

### Tauri IPC вызов

```javascript
const instructions = await invoke('ai_generate_instructions', {
  mesh: meshData
});

console.log(`Сложность: ${instructions.difficulty}`);
console.log(`Шагов: ${instructions.steps.length}`);
```

---

## ⚠️ 4. Детекция проблемных граней

### Файлы реализации
- **Модуль ядра:** `crates/pepakura_core/src/analysis/distortion_analysis.rs`
- **Интегрировано в:** `ai_analyze_distortion()` команду

### Типы обнаруживаемых проблем

| Тип проблемы | Описание | Рекомендации |
|-------------|----------|--------------|
| **TooSmallArea** | Слишком маленькая площадь грани (< 1 мм²) | Увеличить масштаб или объединить грани |
| **SharpAngle** | Острый угол (< 15°) | Сгладить углы или увеличить масштаб |
| **HighAreaDistortion** | Сильное искажение площади (> 30%) | Изменить алгоритм развёртки |
| **HighAngularDistortion** | Сильное искажение углов (> 15°) | Использовать LSCM алгоритм |
| **LongEdge** | Очень длинное ребро | Разбить на более мелкие грани |
| **DegenerateFace** | Вырожденная грань | Удалить из модели |

### Пример использования

```rust
let result = analyzer.analyze(&mesh, &unfolded);

// Проверка проблемных граней
if !result.problematic_faces.is_empty() {
    println!("Найдено {} проблем:", result.problematic_faces.len());
    
    for problem in &result.problematic_faces {
        println!("\n[{}] {}", problem.issue_type, problem.description);
        println!("  Серьёзность: {:.0}%", problem.severity * 100.0);
        println!("  Решение: {}", problem.recommendation);
    }
}
```

### Tauri IPC вызов

```javascript
const result = await invoke('ai_analyze_distortion', {
  mesh: meshData,
  unfolded: unfoldedData
});

const critical = result.problematic_faces.filter(
  p => p.severity > 0.7
);

console.log(`Критических проблем: ${critical.length}`);
```

---

## 🎨 Обновления UI

### AiAssistantPanel

Обновлены подсказки в `src/components/AiAssistantPanel.vue`:

```javascript
const suggestions = ref([
  '🔍 Анализировать искажения развёртки',
  '📊 Оптимизировать раскладку на листе',
  '📋 Сгенерировать инструкции по сборке',
  '⚠️ Найти проблемные грани',
  '💡 Предложить улучшения модели',
  '📄 Рекомендовать формат бумаги',
  '🎯 Оценить качество развёртки',
  '🔧 Оптимизировать сложность сборки'
])
```

---

## 📡 Tauri Команды

### Новые команды

| Команда | Описание | Входные данные | Возвращает |
|---------|----------|----------------|------------|
| `ai_analyze_distortion` | Анализ искажений развёртки | `mesh`, `unfolded` | `DistortionAnalysisDto` |
| `ai_analyze_nesting` | Анализ раскладки | `nestResult` | `NestingAnalysisDto` |

### DTO структуры

```typescript
interface DistortionAnalysisDto {
  avg_area_distortion: number;      // Среднее искажение (%)
  max_area_distortion: number;      // Макс. искажение (%)
  overall_quality_score: number;    // Общая оценка (0-1)
  acceptable_faces_ratio: number;   // % допустимых граней
  heat_map_data: HeatMapEntryDto[]; // Тепловая карта
  problematic_faces: ProblematicFaceDto[]; // Проблемы
  ai_tips: string[];                // AI советы
}

interface NestingAnalysisDto {
  space_efficiency_score: number;   // Эффективность (0-1)
  avg_fill_rate: number;            // Заполненность (%)
  sheets_count: number;             // Количество листов
  total_parts: number;              // Общее число деталей
  suggested_paper_format?: string;  // Рекомендуемый формат
  potential_savings_percent: number; // Экономия (%)
  assembly_complexity: number;      // Сложность сборки (0-1)
  recommendations: NestingRecommendationDto[];
  ai_tips: string[];
}
```

---

## 🏗 Архитектура

### Структура модулей

```
crates/pepakura_core/src/
├── analysis/
│   ├── mod.rs                     # Экспорт модулей
│   ├── mesh_stats.rs              # Статистика меша
│   ├── mesh_analyzer.rs           # LLM анализ
│   ├── distortion_analysis.rs     # ✨ НОВОЕ: Анализ искажений
│   └── nesting_optimization.rs    # ✨ НОВОЕ: Оптимизация раскладки
├── ai/
│   └── assistant.rs               # Генерация инструкций (расширено)

src-tauri/src/
└── ai_commands.rs                 # Tauri команды (расширено)

src/components/
└── AiAssistantPanel.vue           # UI подсказки (обновлено)
```

---

## 🧪 Тестирование

### Юнит-тесты

```bash
# Тесты анализа искажений
cargo test --package pepakura_core distortion_analysis

# Тесты оптимизации раскладки
cargo test --package pepakura_core nesting_optimization

# Все тесты ядра
cargo test --package pepakura_core
```

### Покрытие тестами

- ✅ `DistortionStats::from_values()` - статистика
- ✅ `DistortionAnalyzer::analyze()` - основной анализ
- ✅ `compute_face_area_3d()` - площадь в 3D
- ✅ `compute_face_area_2d()` - площадь в 2D
- ✅ `compute_angular_distortion()` - угловые искажения
- ✅ `compute_edge_distortion()` - искажения рёбер
- ✅ `distortion_to_color()` - цветовая карта
- ✅ `generate_distortion_advice()` - AI советы
- ✅ `NestingOptimizer::analyze()` - анализ раскладки
- ✅ `evaluate_compactness()` - оценка компактности
- ✅ `suggest_paper_format()` - подбор бумаги
- ✅ `generate_nesting_advice()` - рекомендации

---

## 📊 Метрики качества

### Оценка развёртки

| Метрика | Отлично (>0.8) | Хорошо (0.5-0.8) | Плохо (<0.5) |
|---------|----------------|-------------------|--------------|
| **Area Distortion** | < 10% | 10-30% | > 30% |
| **Angular Distortion** | < 5° | 5-15° | > 15° |
| **Acceptable Faces** | > 90% | 70-90% | < 70% |

### Оценка раскладки

| Метрика | Отлично | Хорошо | Плохо |
|---------|---------|--------|-------|
| **Fill Rate** | > 75% | 50-75% | < 50% |
| **Paper Optimality** | > 0.9 | 0.7-0.9 | < 0.7 |
| **Assembly Complexity** | < 0.3 | 0.3-0.7 | > 0.7 |

---

## 🚀 Будущие улучшения

- [ ] Интеграция LLM для персонализированных рекомендаций
- [ ] Визуализация тепловой карты в 3D viewer
- [ ] Автоматическое исправление проблемных граней
- [ ] Оптимизация раскладки через AI
- [ ] Генерация видео-инструкций по сборке
- [ ] Экспорт отчётов в PDF

---

## 📝 Примеры использования

### Полный анализ модели

```rust
// 1. Загрузка модели
let mesh = load_mesh("model.obj")?;

// 2. Развёртка
let config = UnfoldConfig::default();
let unfolded = unfold_mds(&mesh, &config)?;

// 3. Анализ искажений
let analyzer = DistortionAnalyzer::new();
let distortion = analyzer.analyze(&mesh, &unfolded);

// 4. Раскладка
let nest_result = nest_unfolds(&project, &nest_params);
let optimizer = NestingOptimizer::new();
let nesting = optimizer.analyze(&nest_result);

// 5. Генерация инструкций
let assistant = PepakuraAssistant::new(&ai_config)?;
let instructions = assistant.generate_assembly_instructions(&unfolded).await?;

// 6. Вывод результатов
println!("=== АНАЛИЗ МОДЕЛИ ===");
println!("Качество развёртки: {:.1}%", distortion.overall_quality_score * 100.0);
println!("Заполненность бумаги: {:.1}%", nesting.avg_fill_rate);
println!("Сложность сборки: {}", instructions.difficulty);
println!("\nРекомендации:");
for tip in distortion.ai_tips.iter().chain(nesting.ai_tips.iter()) {
    println!("  {}", tip);
}
```

---

## 🎯 Резюме

Реализованы все 4 запланированные AI-функции:

✅ **Анализ искажений** - полная метрика + тепловая карта SVG  
✅ **Оптимизация раскладки** - анализ + рекомендации по бумаге  
✅ **Инструкции по сборке** - автоматическая генерация шагов  
✅ **Детекция проблем** - 6 типов проблем + решения  

**Общий объём кода:** ~1500 строк (Rust)  
**Tauri команды:** 2 новые  
**UI обновления:** 8 новых подсказок  

Все функции протестированы и готовыы к использованию!
