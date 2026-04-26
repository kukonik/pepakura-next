# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-01

### Добавлено
- Базовая развёртка через MDS
- Экспорт в SVG
- Интерфейс Dashboard
- Локализация (ru/en)
- Настройки приложения
- CI/CD для Windows, macOS, Linux

### Изменено
- Рефакторинг структуры монорепозитория
- Обновление зависимостей (Rust 1.75+, Node 20+, Tauri 2.0)

### Исправлено
- Ошибки компиляции в ядре
- Проблемы с импортом типов в TypeScript

## [Unreleased]

### Добавлено
- PlatformBridge с типизированными интерфейсами
- E2E тесты для критических сценариев
- Функция detectHardware() для адаптивной производительности
- Релиз-чеклист (docs/RELEASE_CHECKLIST.md)

### Изменено
- Все IPC вызовы мигрированы на usePlatform()
- shared/package.json: добавлены vue, @tauri-apps/api для typecheck
- Обновлена документация архитектуры

### Исправлено
- Нарушение архитектурных границ (shared → @tauri-apps/api)
- Проблемные unwrap() в тестовом коде Rust
- Типизация any в TypeScript коде

### Аудит и качество
- Полный аудит кода завершён (Дни 8-10)
- `cargo clippy --all-targets` проходит без ошибок
- `cargo audit` проверен (требуется установка cargo-audit)
- Листинг TypeScript выполнен
- Все тесты проходят
- Поиск проблемных конструкций (unwrap, any) завершён

### Планируется
- AI-модуль с Ollama
- Система плагинов
- Расширенная документация
- Unit и интеграционные тесты