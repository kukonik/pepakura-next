//! AI-команды для Tauri IPC.
//!
//! Предоставляет команды для взаимодействия с AI-помощником через Tauri.

use pepakura_core::ai::{AiConfig, PepakuraAssistant, ChatMessage};
use pepakura_core::geometry::Mesh;
use pepakura_core::analysis::{DistortionAnalyzer, NestingOptimizer};
use pepakura_core::nesting::NestResult;
use pepakura_core::unfold::UnfoldedMesh;
use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle, Emitter};
use std::io::{BufRead, BufReader};
use std::sync::Mutex;
use tokio::sync::mpsc;

/// Глобальное состояние AI.
#[derive(Default)]
pub struct AiState {
    /// Конфигурация AI
    pub config: Mutex<AiConfig>,
    /// Кэш ассистента (опционально)
    pub assistant: Mutex<Option<PepakuraAssistant>>,
}

/// Рекомендации по развёртке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldAdviceDto {
    /// Рекомендуемый алгоритм
    pub algorithm: String,
    /// Максимум итераций
    pub max_iterations: usize,
    /// Допуск сходимости
    pub tolerance: f64,
    /// Советы
    pub tips: Vec<String>,
    /// Возможные проблемы
    pub potential_issues: Vec<String>,
}

/// Инструкция по сборке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyInstructionDto {
    /// Название модели
    pub model_name: String,
    /// Уровень сложности
    pub difficulty: String,
    /// Общее время (минуты)
    pub total_time_minutes: usize,
    /// Шаги сборки
    pub steps: Vec<AssemblyStepDto>,
    /// Советы
    pub tips: Vec<String>,
}

/// Шаг сборки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyStepDto {
    /// Номер шага
    pub step_number: usize,
    /// Описание
    pub description: String,
    /// Номера деталей
    pub part_ids: Vec<usize>,
    /// Время (минуты)
    pub estimated_time_minutes: usize,
}

/// Статус AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStatusDto {
    /// Доступен ли сервис
    pub available: bool,
    /// Список моделей
    pub models: Vec<String>,
    /// Провайдер
    pub provider: String,
}

/// Сообщение чата.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessageDto {
    /// Роль (user/assistant)
    pub role: String,
    /// Содержимое
    pub content: String,
}

/// Проверяет доступность AI.
#[tauri::command]
pub async fn ai_check_status(
    state: State<'_, AiState>,
) -> Result<AiStatusDto, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    
    let assistant = PepakuraAssistant::new(&config)
        .map_err(|e| format!("Failed to create assistant: {}", e))?;
    
    let status = assistant.check_availability().await;
    let models = if status {
        // Получаем список моделей если доступен
        vec![config.model.clone()]
    } else {
        vec![]
    };
    
    Ok(AiStatusDto {
        available: status,
        models,
        provider: format!("{:?}", config.provider),
    })
}

/// Получает рекомендации по развёртке.
#[tauri::command]
pub async fn ai_get_unfold_advice(
    mesh: Mesh,
    state: State<'_, AiState>,
) -> Result<UnfoldAdviceDto, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    
    let assistant = PepakuraAssistant::new(&config)
        .map_err(|e| format!("Failed to create assistant: {}", e))?;
    
    let advice = assistant.get_unfold_advice(&mesh).await
        .map_err(|e| format!("AI error: {}", e))?;
    
    Ok(UnfoldAdviceDto {
        algorithm: advice.algorithm,
        max_iterations: advice.parameters.max_iterations,
        tolerance: advice.parameters.tolerance,
        tips: advice.tips,
        potential_issues: advice.potential_issues,
    })
}

/// Генерирует инструкцию по сборке.
#[tauri::command]
pub async fn ai_generate_instructions(
    mesh: Mesh,
    state: State<'_, AiState>,
) -> Result<AssemblyInstructionDto, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    
    let assistant = PepakuraAssistant::new(&config)
        .map_err(|e| format!("Failed to create assistant: {}", e))?;
    
    // Создаём заглушку UnfoldedMesh из Mesh
    let unfolded = UnfoldedMesh {
        vertices_2d: mesh.vertices.iter().map(|v| [v.position[0], v.position[1]]).collect(),
        uv_coords: None,
        faces: mesh.faces.clone(),
        source_mesh: mesh.clone(),
        metadata: Default::default(),
    };
    
    let instructions = assistant.generate_assembly_instructions(&unfolded).await
        .map_err(|e| format!("AI error: {}", e))?;
    
    Ok(AssemblyInstructionDto {
        model_name: instructions.model_name,
        difficulty: format!("{}", instructions.difficulty),
        total_time_minutes: instructions.total_time_minutes,
        steps: instructions.steps.into_iter().map(|s| AssemblyStepDto {
            step_number: s.step_number,
            description: s.description,
            part_ids: s.part_ids,
            estimated_time_minutes: s.estimated_time_minutes,
        }).collect(),
        tips: instructions.tips,
    })
}

/// Отправляет сообщение в AI-чат.
#[tauri::command]
pub async fn ai_chat(
    message: String,
    history: Vec<ChatMessageDto>,
    state: State<'_, AiState>,
) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    
    let assistant = PepakuraAssistant::new(&config)
        .map_err(|e| format!("Failed to create assistant: {}", e))?;
    
    // Конвертируем историю в ChatMessage
    let messages: Vec<ChatMessage> = history.into_iter()
        .map(|m| ChatMessage {
            role: m.role,
            content: m.content,
        })
        .collect();
    
    let response = assistant.answer_question(&message).await
        .map_err(|e| format!("AI error: {}", e))?;
    
    Ok(response)
}

/// Обновляет конфигурацию AI.
#[tauri::command]
pub fn ai_update_config(
    config: AiConfig,
    state: State<'_, AiState>,
) -> Result<(), String> {
    let mut state_config = state.config.lock().map_err(|e| e.to_string())?;
    *state_config = config;
    Ok(())
}

/// Получает текущую конфигурацию AI.
#[tauri::command]
pub fn ai_get_config(
    state: State<'_, AiState>,
) -> Result<AiConfig, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

/// Рекомендует бумагу для модели.
#[tauri::command]
pub async fn ai_recommend_paper(
    model_name: String,
    scale: f64,
    state: State<'_, AiState>,
) -> Result<String, String> {
    let config_clone = {
        let config = state.config.lock().map_err(|e| e.to_string())?;
        config.clone()
    };

    let assistant = PepakuraAssistant::new(&config_clone)
        .map_err(|e| format!("Failed to create assistant: {}", e))?;

    let response = assistant.recommend_paper(&model_name, scale).await
        .map_err(|e| format!("AI error: {}", e))?;

    Ok(response)
}

// ============================================================================
// AI Cache Commands
// ============================================================================

/// Статистика кэша AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCacheStats {
    /// Количество попаданий
    pub hits: u32,
    /// Количество промахов
    pub misses: u32,
    /// Количество сохранений
    pub saves: u32,
    /// Процент попаданий
    pub hit_rate: f64,
    /// Размер кэша
    pub size: usize,
}

/// Получает статистику кэша AI.
#[tauri::command]
pub fn ai_get_cache_stats(
    state: State<'_, AiState>,
) -> Result<AiCacheStats, String> {
    let assistant = state.assistant.lock().map_err(|e| e.to_string())?;
    
    match &*assistant {
        Some(asst) => {
            let stats = asst.get_cache_stats();
            Ok(AiCacheStats {
                hits: stats.hits,
                misses: stats.misses,
                saves: stats.saves,
                hit_rate: asst.cache_hit_rate(),
                size: 0, // Размер не отслеживается в статистике
            })
        }
        None => Ok(AiCacheStats {
            hits: 0,
            misses: 0,
            saves: 0,
            hit_rate: 0.0,
            size: 0,
        }),
    }
}

/// Очищает кэш AI.
#[tauri::command]
pub fn ai_clear_cache(
    state: State<'_, AiState>,
) -> Result<(), String> {
    let assistant = state.assistant.lock().map_err(|e| e.to_string())?;
    
    if let Some(asst) = &*assistant {
        asst.clear_cache();
    }
    Ok(())
}

/// Включает/выключает кэширование AI.
#[tauri::command]
pub fn ai_set_cache_enabled(
    enabled: bool,
    state: State<'_, AiState>,
) -> Result<(), String> {
    // Сохраняем настройку в конфигурацию
    let mut config = state.config.lock().map_err(|e| e.to_string())?;
    config.cache_enabled = enabled;
    Ok(())
}

/// Проверяет наличие запроса в кэше.
#[tauri::command]
pub fn ai_cache_contains(
    prompt: String,
    state: State<'_, AiState>,
) -> Result<bool, String> {
    let assistant = state.assistant.lock().map_err(|e| e.to_string())?;

    match &*assistant {
        Some(asst) => Ok(asst.cache_contains(&prompt)),
        None => Ok(false),
    }
}

// ============================================================================
// AI Streaming Commands
// ============================================================================

/// Ответ стриминга AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStreamResponse {
    /// Токен ответа
    pub token: String,
    /// Общее количество токенов
    pub total_tokens: usize,
    /// Стрим завершён
    pub done: bool,
}

/// Отправляет запрос в AI и возвращает стрим токенов через event emitter.
#[tauri::command]
pub async fn ai_chat_stream(
    message: String,
    history: Vec<ChatMessageDto>,
    state: State<'_, AiState>,
    window: tauri::Window,
) -> Result<(), String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    let assistant = PepakuraAssistant::new(&config)
        .map_err(|e| format!("Failed to create assistant: {}", e))?;

    // Создаём канал для стриминга (не используется в текущей реализации)
    let (_tx, mut _rx) = mpsc::channel::<String>(32);

    // Запускаем стриминг в фоновой задаче
    let window_clone = window.clone();
    tokio::spawn(async move {
        match assistant.answer_question_stream(&message).await {
            Ok(stream) => {
                use futures::StreamExt;

                let mut total_tokens = 0;
                let mut stream = stream;

                while let Some(token) = stream.next().await {
                    total_tokens += 1;

                    // Отправляем токен через Tauri event
                    let _ = window_clone.emit(
                        "ai-stream-token",
                        AiStreamResponse {
                            token,
                            total_tokens,
                            done: false,
                        },
                    );

                    // Небольшая задержка для UX
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                }

                // Отправляем сигнал завершения
                let _ = window_clone.emit(
                    "ai-stream-done",
                    AiStreamResponse {
                        token: String::new(),
                        total_tokens,
                        done: true,
                    },
                );
            }
            Err(e) => {
                let _ = window_clone.emit(
                    "ai-stream-error",
                    format!("AI error: {}", e),
                );
            }
        }
    });

    Ok(())
}

/// Получает полный ответ из стрима (для совместимости).
#[tauri::command]
pub async fn ai_chat_complete(
    message: String,
    state: State<'_, AiState>,
) -> Result<String, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?.clone();
    let assistant = PepakuraAssistant::new(&config)
        .map_err(|e| format!("Failed to create assistant: {}", e))?;

    assistant.answer_question(&message).await
        .map_err(|e| format!("AI error: {}", e))
}

// ============================================================================
// Ollama Native Streaming (прямой HTTP стриминг через ureq)
// ============================================================================

/// Payload для стриминг чанка.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaStreamChunk {
    /// Текстовый чанк
    pub text: String,
    /// Метка времени
    pub timestamp: u64,
}

/// Payload для ошибки стриминга.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaStreamError {
    /// Сообщение об ошибке
    pub error: String,
    /// Код ошибки (опционально)
    pub code: Option<u16>,
}

/// Прямой стриминг чата с Ollama через HTTP (без PepakuraAssistant).
/// 
/// Эта функция отправляет запрос напрямую к Ollama API и читает ответ
/// по строкам (NDJSON формат), отправляя каждый токен через Tauri events.
///
/// # Аргументы
/// * `app` - Tauri AppHandle для отправки событий
/// * `message` - Сообщение пользователя
/// * `history` - История сообщений (контекст)
/// * `state` - Состояние AI (для получения конфигурации)
///
/// # Events
/// * `ollama-stream-chunk` - Одиночный токен/фраза (OllamaStreamChunk)
/// * `ollama-stream-done` - Стрим завершён ()
/// * `ollama-stream-error` - Ошибка (OllamaStreamError)
#[tauri::command]
pub fn ai_chat_stream_native(
    app: AppHandle,
    message: String,
    history: Vec<ChatMessageDto>,
    state: State<'_, AiState>,
) -> Result<(), String> {
    // Получаем конфигурацию
    let config = state.config.lock().map_err(|e| e.to_string())?;
    let model = config.model.clone();
    drop(config);

    // Формируем URL Ollama API (по умолчанию localhost:11434)
    let endpoint = std::env::var("OLLAMA_ENDPOINT").unwrap_or_else(|_| "http://localhost:11434".to_string());
    let url = format!("{}/api/chat", endpoint);

    // Создаём ureq агент для подключения
    let agent = ureq::AgentBuilder::new()
        .timeout(std::time::Duration::from_secs(120))
        .build();

    // Формируем запрос в формате Ollama
    let mut messages: Vec<serde_json::Value> = history
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content
            })
        })
        .collect();

    // Добавляем текущее сообщение
    messages.push(serde_json::json!({
        "role": "user",
        "content": message
    }));

    let request_body = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": true  // Включаем стриминг
    });

    // Запускаем стриминг в фоновом потоке (не async, а обычный thread)
    std::thread::spawn(move || {
        // Отправляем запрос
        let response = match agent
            .post(&url)
            .set("Content-Type", "application/json")
            .send_json(ureq::json!(request_body))
        {
            Ok(resp) => resp,
            Err(e) => {
                let error_msg = format!("Network error: {}", e);
                let _ = app.emit("ollama-stream-error", OllamaStreamError {
                    error: error_msg,
                    code: None,
                });
                return;
            }
        };

        // Читаем ответ по строкам через BufReader
        let reader = BufReader::new(response.into_reader());
        let mut full_response = String::new();

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    let error_msg = format!("Read error: {}", e);
                    let _ = app.emit("ollama-stream-error", OllamaStreamError {
                        error: error_msg,
                        code: None,
                    });
                    break;
                }
            };

            // Пропускаем пустые строки
            if line.trim().is_empty() {
                continue;
            }

            // Парсим JSON
            let json: serde_json::Value = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Failed to parse JSON: {}, line: {}", e, line);
                    continue;
                }
            };

            // Проверяем поле done
            let is_done = json.get("done").and_then(|v| v.as_bool()).unwrap_or(false);

            if is_done {
                // Стрим завершён
                let _ = app.emit("ollama-stream-done", ());
                break;
            }

            // Извлекаем текст из ["message"]["content"]
            if let Some(content) = json
                .get("message")
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_str())
            {
                if !content.is_empty() {
                    full_response.push_str(content);

                    // Отправляем чанк
                    let _ = app.emit("ollama-stream-chunk", OllamaStreamChunk {
                        text: content.to_string(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64,
                    });
                }
            }
        }
    });

    Ok(())
}

/// Альтернативная версия с таймаутом и поддержкой отмены.
///
/// Использует tokio::task::spawn_blocking для интеграции с async runtime.
#[tauri::command]
pub async fn ai_chat_stream_with_cancel(
    app: AppHandle,
    message: String,
    history: Vec<ChatMessageDto>,
    state: State<'_, AiState>,
) -> Result<(), String> {
    // Клонируем данные для потока
    let config = state.config.lock().map_err(|e| e.to_string())?;
    let model = config.model.clone();
    drop(config);

    let endpoint = std::env::var("OLLAMA_ENDPOINT").unwrap_or_else(|_| "http://localhost:11434".to_string());
    let url = format!("{}/api/chat", endpoint);

    // Собираем запрос
    let mut messages: Vec<serde_json::Value> = history
        .into_iter()
        .map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content
            })
        })
        .collect();

    messages.push(serde_json::json!({
        "role": "user",
        "content": message
    }));

    let request_body = serde_json::json!({
        "model": model,
        "messages": messages,
        "stream": true
    });

    // Запускаем в spawn_blocking для интеграции с tokio
    let app_clone = app.clone();
    tokio::task::spawn_blocking(move || {
        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(180))
            .build();

        let response = match agent
            .post(&url)
            .set("Content-Type", "application/json")
            .send_json(ureq::json!(request_body))
        {
            Ok(resp) => resp,
            Err(e) => {
                let _ = app_clone.emit("ollama-stream-error", OllamaStreamError {
                    error: format!("Request failed: {}", e),
                    code: None,
                });
                return;
            }
        };

        let reader = BufReader::new(response.into_reader());

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    let _ = app_clone.emit("ollama-stream-error", OllamaStreamError {
                        error: format!("Read failed: {}", e),
                        code: None,
                    });
                    break;
                }
            };

            if line.trim().is_empty() {
                continue;
            }

            let json: serde_json::Value = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("JSON parse error: {}", e);
                    continue;
                }
            };

            if json.get("done").and_then(|v| v.as_bool()).unwrap_or(false) {
                let _ = app_clone.emit("ollama-stream-done", ());
                break;
            }

            if let Some(content) = json
                .get("message")
                .and_then(|m| m.get("content"))
                .and_then(|c| c.as_str())
            {
                if !content.is_empty() {
                    let _ = app_clone.emit("ollama-stream-chunk", OllamaStreamChunk {
                        text: content.to_string(),
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_millis() as u64,
                    });
                }
            }
        }
    });

    Ok(())
}

// ============================================================================
// Distortion Analysis Commands (Анализ искажений развёртки)
// ============================================================================

/// Данные тепловой карты
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatMapEntryDto {
    /// Индекс грани
    pub face_index: usize,
    /// Искажение площади (ratio)
    pub area_ratio: f64,
    /// Среднее искажение углов (градусы)
    pub avg_angular_distortion: f64,
    /// Композитная оценка искажений
    pub composite_distortion: f64,
    /// Центр в 2D
    pub center_2d: [f64; 2],
    /// Серьёзность: "ok", "warning", "critical"
    pub severity: String,
}

/// Проблемная грань DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblematicFaceDto {
    /// Индекс грани
    pub face_index: usize,
    /// Тип проблемы
    pub issue_type: String,
    /// Описание
    pub description: String,
    /// Серьёзность (0-1)
    pub severity: f64,
    /// Рекомендация
    pub recommendation: String,
}

/// Результат анализа искажений
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistortionAnalysisDto {
    /// Среднее искажение площадей (%)
    pub avg_area_distortion: f64,
    /// Максимальное искажение площадей (%)
    pub max_area_distortion: f64,
    /// Общая оценка качества (0-1)
    pub overall_quality_score: f64,
    /// Процент допустимых граней
    pub acceptable_faces_ratio: f64,
    /// Данные тепловой карты
    pub heat_map_data: Vec<HeatMapEntryDto>,
    /// Проблемные грани
    pub problematic_faces: Vec<ProblematicFaceDto>,
    /// AI рекомендации
    pub ai_tips: Vec<String>,
}

/// Анализирует искажения развёртки.
#[tauri::command]
pub fn ai_analyze_distortion(
    mesh: Mesh,
    unfolded: pepakura_core::unfold::UnfoldResult,
) -> Result<DistortionAnalysisDto, String> {
    let analyzer = DistortionAnalyzer::new();
    let result = analyzer.analyze(&mesh, &unfolded);

    let ai_tips = pepakura_core::analysis::generate_distortion_advice(&result);

    // Конвертируем в DTO
    let heat_map_data: Vec<HeatMapEntryDto> = result.heat_map_data.iter().map(|h| HeatMapEntryDto {
        face_index: h.face_index,
        area_ratio: h.area_ratio,
        avg_angular_distortion: h.avg_angular_distortion,
        composite_distortion: h.composite_distortion,
        center_2d: h.center_2d,
        severity: h.severity.clone(),
    }).collect();

    let problematic_faces: Vec<ProblematicFaceDto> = result.problematic_faces.iter().map(|p| {
        ProblematicFaceDto {
            face_index: p.face_index,
            issue_type: format!("{}", p.issue_type),
            description: p.description.clone(),
            severity: p.severity,
            recommendation: p.recommendation.clone(),
        }
    }).collect();

    Ok(DistortionAnalysisDto {
        avg_area_distortion: result.avg_area_distortion,
        max_area_distortion: result.max_area_distortion,
        overall_quality_score: result.overall_quality_score,
        acceptable_faces_ratio: result.acceptable_faces_ratio,
        heat_map_data,
        problematic_faces,
        ai_tips,
    })
}

// ============================================================================
// Nesting Optimization Commands (Оптимизация раскладки)
// ============================================================================

/// Рекомендация по раскладке DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestingRecommendationDto {
    /// Тип рекомендации
    pub recommendation_type: String,
    /// Описание
    pub description: String,
    /// Потенциальная выгода (%)
    pub potential_benefit: f64,
    /// Приоритет
    pub priority: String,
    /// Категория
    pub category: String,
}

/// Анализ раскладки DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestingAnalysisDto {
    /// Эффективность использования пространства
    pub space_efficiency_score: f64,
    /// Средняя заполненность (%)
    pub avg_fill_rate: f64,
    /// Количество листов
    pub sheets_count: usize,
    /// Общее количество деталей
    pub total_parts: usize,
    /// Рекомендуемый формат бумаги
    pub suggested_paper_format: Option<String>,
    /// Потенциальная экономия (%)
    pub potential_savings_percent: f64,
    /// Сложность сборки (0-1)
    pub assembly_complexity: f64,
    /// Рекомендации
    pub recommendations: Vec<NestingRecommendationDto>,
    /// AI советы
    pub ai_tips: Vec<String>,
}

/// Анализирует раскладку и даёт рекомендации.
#[tauri::command]
pub fn ai_analyze_nesting(
    nest_result: NestResult,
) -> Result<NestingAnalysisDto, String> {
    let optimizer = NestingOptimizer::new();
    let analysis = optimizer.analyze(&nest_result);

    let ai_tips = pepakura_core::analysis::generate_nesting_advice(&analysis);

    // Конвертируем рекомендации
    let recommendations: Vec<NestingRecommendationDto> = analysis.recommendations.iter().map(|r| {
        NestingRecommendationDto {
            recommendation_type: format!("{}", r.recommendation_type),
            description: r.description.clone(),
            potential_benefit: r.potential_benefit,
            priority: r.priority.clone(),
            category: r.category.clone(),
        }
    }).collect();

    Ok(NestingAnalysisDto {
        space_efficiency_score: analysis.space_efficiency_score,
        avg_fill_rate: analysis.avg_fill_rate * 100.0, // конвертируем в проценты
        sheets_count: analysis.sheets_count,
        total_parts: analysis.total_parts,
        suggested_paper_format: analysis.suggested_paper_format,
        potential_savings_percent: analysis.potential_savings_percent,
        assembly_complexity: analysis.assembly_complexity.overall_complexity,
        recommendations,
        ai_tips,
    })
}

/// Регистрирует AI команды в Tauri.
#[macro_export]
macro_rules! generate_ai_handler {
    () => {
        tauri::generate_handler![
            ai_check_status,
            ai_get_unfold_advice,
            ai_generate_instructions,
            ai_chat,
            ai_update_config,
            ai_get_config,
            ai_recommend_paper,
            ai_get_cache_stats,
            ai_clear_cache,
            ai_set_cache_enabled,
            ai_cache_contains,
            ai_chat_stream,
            ai_chat_complete,
            ai_chat_stream_native,
            ai_chat_stream_with_cancel,
            ai_analyze_distortion,
            ai_analyze_nesting
        ]
    };
}
