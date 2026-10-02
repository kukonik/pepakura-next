//! Мост между Tauri и ядром разворачивания

use super::{UnfoldParams, UnfoldResult, UnfoldedFace, Seam};
use pepakura_core;
use std::path::Path;

/// Разворачивает 3D модель через ядро
pub fn unfold_model(
    model_path: &str,
    params: &UnfoldParams,
) -> Result<UnfoldResult, String> {
    // Загружаем модель через ядро
    let mesh = pepakura_core::load_obj(model_path)?;
    
    // Преобразуем параметры
    let core_params = pepakura_core::unfold::UnwrapParams {
        min_seam_angle: params.min_seam_angle,
        max_seam_length: params.max_seam_length,
    };
    
    // Выполняем разворачивание
    let core_result = pepakura_core::unwrap_3d_model(&mesh, &core_params);
    
    // Преобразуем результат
    let result = UnfoldResult {
        unfolded_faces: core_result.unfolded_faces.into_iter().map(|face| UnfoldedFace {
            face_index: face.face_index,
            vertices_2d: face.vertices_2d,
            normal: face.normal,
        }).collect(),
        seams: core_result.seams.into_iter().map(|seam| Seam {
            start_point: seam.start_point,
            end_point: seam.end_point,
            face_a_index: seam.face_a_index,
            face_b_index: seam.face_b_index,
        }).collect(),
        width: core_result.width,
        height: core_result.height,
    };
    
    Ok(result)
}

/// Укладывает развернутые элементы на лист через ядро
pub fn layout_unfolded_pieces(
    unfolded_result: UnfoldResult,
    sheet_width: f32,
    sheet_height: f32,
) -> Result<UnfoldResult, String> {
    // Преобразуем данные в формат ядра
    let core_faces: Vec<pepakura_core::unfold::UnfoldedFace> = unfolded_result.unfolded_faces
        .into_iter()
        .map(|face| pepakura_core::unfold::UnfoldedFace {
            face_index: face.face_index,
            vertices_2d: face.vertices_2d,
            normal: face.normal,
        })
        .collect();
    
    let core_seams: Vec<pepakura_core::unfold::Seam> = unfolded_result.seams
        .into_iter()
        .map(|seam| pepakura_core::unfold::Seam {
            start_point: seam.start_point,
            end_point: seam.end_point,
            face_a_index: seam.face_a_index,
            face_b_index: seam.face_b_index,
        })
        .collect();
    
    // Создаем параметры укладки
    let layout_params = pepakura_core::unfold::LayoutParams {
        sheet_width,
        sheet_height,
        min_gap: 5.0, // Минимальный зазор 5 мм
    };
    
    // Выполняем укладку
    let core_result = pepakura_core::layout_unfolded_pieces(core_faces, core_seams, &layout_params);
    
    // Преобразуем результат
    let result = UnfoldResult {
        unfolded_faces: core_result.laid_out_faces.into_iter().map(|face| UnfoldedFace {
            face_index: face.face_index,
            vertices_2d: face.vertices_2d,
            normal: face.normal,
        }).collect(),
        seams: core_result.updated_seams.into_iter().map(|seam| Seam {
            start_point: seam.start_point,
            end_point: seam.end_point,
            face_a_index: seam.face_a_index,
            face_b_index: seam.face_b_index,
        }).collect(),
        width: core_result.sheet_width,
        height: core_result.sheet_height,
    };
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bridge_functions_exist() {
        // Просто проверяем, что функции компилируются
        // Реальное тестирование требует файлов моделей
        assert!(true);
    }
}