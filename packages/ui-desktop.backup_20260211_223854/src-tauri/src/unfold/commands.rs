//! Команды Tauri для работы с разворачиванием

use super::{UnfoldParams, UnfoldResult};
use crate::unfold::bridge;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Разворачивает 3D модель
#[tauri::command]
pub fn unfold_model(
    model_path: String,
    params: UnfoldParams,
) -> Result<UnfoldResult, String> {
    // Проверяем, что файл существует
    if !Path::new(&model_path).exists() {
        return Err("Model file does not exist".to_string());
    }
    
    // Выполняем разворачивание через мост к ядру
    bridge::unfold_model(&model_path, &params)
}

/// Укладывает развернутые элементы на лист
#[tauri::command]
pub fn layout_unfolded_pieces(
    unfolded_result: UnfoldResult,
    sheet_width: f32,
    sheet_height: f32,
) -> Result<UnfoldResult, String> {
    // Выполняем укладку через мост к ядру
    bridge::layout_unfolded_pieces(unfolded_result, sheet_width, sheet_height)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_unfold_params_default() {
        let params = UnfoldParams::default();
        assert_eq!(params.min_seam_angle, 75.0);
        assert_eq!(params.max_seam_length, 10.0);
        assert_eq!(params.auto_seams, true);
    }
}