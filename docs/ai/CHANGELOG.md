# История изменений AI-модуля

## [0.1.0] - 2026-03-21

### Добавлено
- **Базовая интеграция с Ollama**
  - `OllamaClient` — HTTP клиент для Ollama API
  - `AiConfig` — конфигурация AI-провайдера
  - `AiProvider` — enum провайдеров (Ollama, OpenAI)
  - `AiStatus` — статус доступности сервиса

- **PepakuraAssistant**
  - `get_unfold_advice()` — рекомендации по развёртке
  - `get_model_description()` — описание модели
  - `generate_assembly_instructions()` — инструкция сборки
  - `answer_question()` — ответы на вопросы
  - `recommend_paper()` — рекомендации бумаги

- **Вспомогательные типы**
  - `ChatMessage` — сообщение для чата
  - `AssemblyInstruction` — инструкция сборки
  - `AssemblyStep` — шаг сборки
  - `Difficulty` — уровень сложности
  - `UnfoldAdvice` — рекомендации по развёртке

- **Документация**
  - `docs/ai/README.md` — полное руководство
  - Примеры использования
  - API reference

### Зависимости
- `reqwest = "0.11"` — HTTP клиент
- `tokio = "1"` — асинхронная среда
- `serde_json = "1.0"` — JSON сериализация

### Тесты
- Unit-тесты для `AiConfig`
- Unit-тесты для `ChatMessage`
- Integration-тесты (требуют запущенного Ollama)

---

## Планы

### [0.2.0]
- [ ] Стриминг ответов (stream: true)
- [ ] Кэширование частых запросов
- [ ] Поддержка мультимодальных моделей (изображения)
- [ ] Rate limiting

### [0.3.0]
- [ ] Локальная fallback модель
- [ ] Контекст диалога (история сообщений)
- [ ] Генерация превью изображений
- [ ] Экспорт инструкций в PDF
