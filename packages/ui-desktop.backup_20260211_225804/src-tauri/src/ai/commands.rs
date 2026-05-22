//! Команды Tauri для работы с AI

use super::{AiBackendConfig, ChatMessage, AiResponse};
use crate::ai::ollama_client;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;

/// Состояние AI бэкенда
pub struct AiState {
    pub config: Mutex<Option<AiBackendConfig>>,
}

/// Отправляет сообщение в AI
#[tauri::command]
pub fn send_ai_message(
    message: String,
    state: State<AiState>,
) -> Result<AiResponse, String> {
    // Получаем конфигурацию
    let config = state.config.lock().unwrap();
    let config = config.as_ref().ok_or("AI backend not configured")?;
    
    // В зависимости от типа бэкенда вызываем соответствующий клиент
    match config.backend_type {
        super::AiBackendType::Ollama => {
            ollama_client::send_message(&message, config)
        }
        super::AiBackendType::OpenAI => {
            // TODO: Реализовать клиент для OpenAI
            Err("OpenAI backend not implemented yet".to_string())
        }
        super::AiBackendType::Custom => {
            // TODO: Реализовать клиент для пользовательского бэкенда
            Err("Custom backend not implemented yet".to_string())
        }
    }
}

/// Получает предложения от AI для работы с швами
#[tauri::command]
pub fn get_ai_suggestions(
    model_description: String,
    current_seams: Vec<super::super::unfold::Seam>,
    state: State<AiState>,
) -> Result<Vec<super::super::unfold::Seam>, String> {
    // Получаем конфигурацию
    let config = state.config.lock().unwrap();
    let config = config.as_ref().ok_or("AI backend not configured")?;
    
    // Формируем сообщение для AI
    let prompt = format!(
        "Model: {}\nCurrent seams: {:?}\n\nSuggest optimal seams for this 3D model unfolding.",
        model_description,
        current_seams.len()
    );
    
    // Отправляем сообщение в AI
    let response = match config.backend_type {
        super::AiBackendType::Ollama => {
            ollama_client::send_message(&prompt, config)?
        }
        super::AiBackendType::OpenAI => {
            // TODO: Реализовать клиент для OpenAI
            return Err("OpenAI backend not implemented yet".to_string());
        }
        super::AiBackendType::Custom => {
            // TODO: Реализовать клиент для пользовательского бэкенда
            return Err("Custom backend not implemented yet".to_string());
        }
    };
    
    // TODO: Парсим ответ и возвращаем новые швы
    // Пока возвращаем пустой вектор
    Ok(Vec::new())
}

/// Настраивает AI бэкенд
#[tauri::command]
pub fn configure_ai_backend(
    config: AiBackendConfig,
    state: State<AiState>,
) -> Result<(), String> {
    // Сохраняем конфигурацию в состоянии
    let mut state_config = state.config.lock().unwrap();
    *state_config = Some(config);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ai_state() {
        let state = AiState {
            config: Mutex::new(None),
        };
        
        assert!(state.config.lock().unwrap().is_none());
    }
}