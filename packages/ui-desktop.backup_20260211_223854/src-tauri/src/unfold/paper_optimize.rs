//! Модуль интеграции оптимизации для бумаги с Tauri
//! 
//! Этот модуль предоставляет команды Tauri для вызова функций оптимизации
//! из фронтенд-приложения.

use tauri::command;
use pepakura_core::model::Model;
use pepakura_core::unfold::paper_optimize::{optimize_for_paper, PaperOptimizeParams, PaperOptimizeResult};

/// Оптимизирует 3D модель для печати на бумаге
#[command]
pub fn optimize_model_for_paper(
    model: Model,
    params: PaperOptimizeParams,
) -> PaperOptimizeResult {
    optimize_for_paper(&model, &params)
}

/// Возвращает параметры оптимизации по умолчанию
#[command]
pub fn get_default_paper_optimize_params() -> PaperOptimizeParams {
    PaperOptimizeParams::default()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_default_paper_optimize_params() {
        let params = get_default_paper_optimize_params();
        assert_eq!(params.sheet_width, 210.0);
        assert_eq!(params.sheet_height, 297.0);
    }
}