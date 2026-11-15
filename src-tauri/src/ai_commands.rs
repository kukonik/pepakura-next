//! AI-команды для Tauri IPC (Этап A: детерминированные стабы).
//!
//! Все команды сохраняют IPC-имена (ai_*), чтобы фронтенд продолжал работать.
//! Реальный AI-бэкенд (HTTP-клиент, стриминг, кэш LLM) подключается на
//! отдельном этапе; сейчас команды возвращают осмысленные заглушки и ведут
//! простой in-memory кэш, чтобы UI оставался живым.

use pepakura_core::ai::{AiConfig, ChatMessage};
use pepakura_core::geometry::Mesh;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

/// Глобальное состояние AI.
#[derive(Default)]
pub struct AiState {
    /// Конфигурация AI.
    pub config: Mutex<AiConfig>,
    /// Включён ли кэш.
    pub cache_enabled: Mutex<bool>,
    /// Простой in-memory кэш ключей запросов.
    pub cache: Mutex<Vec<String>>,
}

/// Рекомендации по развёртке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldAdviceDto {
    pub algorithm: String,
    pub tips: Vec<String>,
    pub potential_issues: Vec<String>,
    pub max_iterations: u32,
    pub tolerance: f64,
}

/// Шаг инструкции по сборке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyStepDto {
    pub step_number: usize,
    pub description: String,
    pub part_ids: Vec<usize>,
    pub estimated_time_minutes: usize,
}

/// Инструкция по сборке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyInstructionDto {
    pub model_name: String,
    pub difficulty: String,
    pub total_time_minutes: usize,
    pub steps: Vec<AssemblyStepDto>,
    pub tips: Vec<String>,
}

/// Рекомендация по бумаге.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperRecommendationDto {
    pub paper_gsm: u32,
    pub reason: String,
    pub alternatives: Vec<String>,
}

/// Статус AI-модуля.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiStatusDto {
    pub available: bool,
    pub mode: String,
    pub model: String,
}

/// Статистика кэша.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStatsDto {
    pub entries: usize,
    pub enabled: bool,
}

/// Клонирует текущую конфигурацию из состояния.
fn lock_config(state: &State<'_, AiState>) -> Result<AiConfig, String> {
    Ok(state.config.lock().map_err(|e| e.to_string())?.clone())
}

/// Проверка статуса AI.
#[tauri::command]
pub fn ai_check_status(state: State<'_, AiState>) -> Result<AiStatusDto, String> {
    let config = lock_config(&state)?;
    let model = if config.model.is_empty() { "stub".to_string() } else { config.model };
    Ok(AiStatusDto { available: true, mode: "stub".to_string(), model })
}

/// Рекомендация алгоритма развёртки.
#[tauri::command]
pub fn ai_get_unfold_advice(mesh: Option<Mesh>) -> Result<UnfoldAdviceDto, String> {
    let face_count = mesh.as_ref().map(|m| m.faces.len()).unwrap_or(0);
    let algorithm = if face_count < 100 { "greedy-face-growth" } else { "lscm" };
    Ok(UnfoldAdviceDto {
        algorithm: algorithm.to_string(),
        tips: vec![
            "Проверьте замкнутость меша перед развёрткой".to_string(),
            "Начните с отсечения перекрытий островов".to_string(),
        ],
        potential_issues: if face_count > 500 {
            vec!["Модель содержит более 500 граней — развёртка может занять время".to_string()]
        } else {
            vec![]
        },
        max_iterations: 200,
        tolerance: 0.001,
    })
}

/// Генерирует инструкцию по сборке (локальная заглушка).
#[tauri::command]
pub fn ai_generate_instructions(mesh: Option<Mesh>) -> Result<AssemblyInstructionDto, String> {
    let face_count = mesh.as_ref().map(|m| m.faces.len()).unwrap_or(0);
    let difficulty = match face_count {
        n if n < 20 => "easy",
        n if n < 50 => "medium",
        n if n < 100 => "hard",
        _ => "expert",
    };
    let mut steps: Vec<AssemblyStepDto> = Vec::new();
    if face_count > 0 {
        steps.push(AssemblyStepDto {
            step_number: 1,
            description: format!("Вырежьте все детали ({face_count} шт.) по контуру"),
            part_ids: (0..face_count).collect(),
            estimated_time_minutes: (face_count / 10).max(5),
        });
        steps.push(AssemblyStepDto {
            step_number: 2,
            description: "Согните по линиям сгиба и склейте крупные детали".to_string(),
            part_ids: vec![],
            estimated_time_minutes: 15,
        });
    }
    steps.push(AssemblyStepDto {
        step_number: steps.len() + 1,
        description: "Соберите основание и проверьте устойчивость модели".to_string(),
        part_ids: vec![],
        estimated_time_minutes: 10,
    });
    let total: usize = steps.iter().map(|s| s.estimated_time_minutes).sum();
    let model_name = match mesh {
        Some(m) if !m.name.is_empty() => m.name,
        _ => "Модель".to_string(),
    };
    Ok(AssemblyInstructionDto {
        model_name,
        difficulty: difficulty.to_string(),
        total_time_minutes: total,
        steps,
        tips: vec!["Склеивайте от основания к вершине".to_string()],
    })
}

/// Отправляет сообщение в AI-чат (заглушка-эхо).
#[tauri::command]
pub fn ai_chat(message: Option<String>, state: State<'_, AiState>) -> Result<ChatMessage, String> {
    let text = message.unwrap_or_default();
    let config = lock_config(&state)?;
    let model = if config.model.is_empty() { "stub".to_string() } else { config.model };
    Ok(ChatMessage {
        role: "assistant".to_string(),
        content: format!(
            "[AI-заглушка, модель: {model}] Сообщение получено: \"{text}\". Полный AI-бэкенд будет подключён на следующих этапах."
        ),
    })
}

/// Обновляет конфигурацию AI (принимает произвольный JSON-объект).
#[tauri::command]
pub fn ai_update_config(
    config: Option<serde_json::Value>,
    state: State<'_, AiState>,
) -> Result<AiConfig, String> {
    let mut cfg = lock_config(&state)?;
    if let Some(obj) = config {
        if let Some(v) = obj.get("model").and_then(|v| v.as_str()) {
            cfg.model = v.to_string();
        }
        if let Some(v) = obj.get("api_url").and_then(|v| v.as_str()) {
            cfg.api_url = v.to_string();
        }
    }
    *state.config.lock().map_err(|e| e.to_string())? = cfg.clone();
    Ok(cfg)
}

/// Возвращает текущую конфигурацию AI.
#[tauri::command]
pub fn ai_get_config(state: State<'_, AiState>) -> Result<AiConfig, String> {
    lock_config(&state)
}

/// Рекомендует плотность бумаги по числу граней.
#[tauri::command]
pub fn ai_recommend_paper(mesh: Option<Mesh>) -> Result<PaperRecommendationDto, String> {
    let face_count = mesh.as_ref().map(|m| m.faces.len()).unwrap_or(0);
    let (gsm, reason) = if face_count < 50 {
        (160u32, "Мелкая модель: подходит стандартная бумага".to_string())
    } else if face_count < 200 {
        (200u32, "Средняя модель: рекомендуется плотная бумага".to_string())
    } else {
        (250u32, "Крупная модель: рекомендуется картон".to_string())
    };
    Ok(PaperRecommendationDto {
        paper_gsm: gsm,
        reason,
        alternatives: vec![
            "160 г/м²".to_string(),
            "200 г/м²".to_string(),
            "250 г/м²".to_string(),
        ],
    })
}

/// Статистика кэша.
#[tauri::command]
pub fn ai_get_cache_stats(state: State<'_, AiState>) -> Result<CacheStatsDto, String> {
    let enabled = *state.cache_enabled.lock().map_err(|e| e.to_string())?;
    let entries = state.cache.lock().map_err(|e| e.to_string())?.len();
    Ok(CacheStatsDto { entries, enabled })
}

/// Очищает кэш, возвращает число удалённых записей.
#[tauri::command]
pub fn ai_clear_cache(state: State<'_, AiState>) -> Result<usize, String> {
    let mut cache = state.cache.lock().map_err(|e| e.to_string())?;
    let cleared = cache.len();
    cache.clear();
    Ok(cleared)
}

/// Включает/выключает кэш.
#[tauri::command]
pub fn ai_set_cache_enabled(enabled: Option<bool>, state: State<'_, AiState>) -> Result<(), String> {
    let value = enabled.unwrap_or(true);
    *state.cache_enabled.lock().map_err(|e| e.to_string())? = value;
    Ok(())
}

/// Проверяет наличие ключа в кэше.
#[tauri::command]
pub fn ai_cache_contains(key: Option<String>, state: State<'_, AiState>) -> Result<bool, String> {
    let key = key.unwrap_or_default();
    let cache = state.cache.lock().map_err(|e| e.to_string())?;
    Ok(cache.iter().any(|k| *k == key))
}

/// Стриминговый чат (Этап A: без событий, управление возвращается сразу).
#[tauri::command]
pub async fn ai_chat_stream(message: Option<String>) -> Result<(), String> {
    let _ = message;
    Ok(())
}

/// Полный ответ чата одной строкой (заглушка-эхо).
#[tauri::command]
pub async fn ai_chat_complete(message: Option<String>) -> Result<String, String> {
    let text = message.unwrap_or_default();
    Ok(format!("[AI-заглушка] Ответ на: \"{text}\""))
}
