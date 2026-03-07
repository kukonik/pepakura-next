//! Модуль для экспорта результатов разворачивания

use crate::unfold::UnfoldResult;
use pepakura_core;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Поддерживаемые форматы экспорта
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ExportFormat {
    /// SVG формат (векторный)
    Svg,
    /// PNG формат (растровый)
    Png,
    /// JPG формат (растровый)
    Jpg,
    /// OBJ формат (3D модель)
    Obj,
    /// STL формат (3D модель, бинарный)
    Stl,
}

/// Экспортирует развернутую модель в SVG
#[tauri::command]
pub fn export_to_svg(
    unfolded_result: UnfoldResult,
    file_path: String,
) -> Result<(), String> {
    // Проверяем, что путь к файлу корректный
    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err("Directory does not exist".to_string());
        }
    }
    
    // Преобразуем данные в формат ядра
    let core_result = pepakura_core::unfold::UnfoldResult {
        unfolded_faces: unfolded_result.unfolded_faces.into_iter().map(|face| {
            pepakura_core::unfold::UnfoldedFace {
                face_index: face.face_index,
                vertices_2d: face.vertices_2d,
                normal: face.normal,
            }
        }).collect(),
        seams: unfolded_result.seams.into_iter().map(|seam| {
            pepakura_core::unfold::Seam {
                start_point: seam.start_point,
                end_point: seam.end_point,
                face_a_index: seam.face_a_index,
                face_b_index: seam.face_b_index,
            }
        }).collect(),
        width: unfolded_result.width,
        height: unfolded_result.height,
    };
    
    // Экспортируем через ядро
    pepakura_core::export::export_to_svg(&core_result, &file_path)
}

/// Экспортирует развернутую модель в PNG
#[tauri::command]
pub fn export_to_png(
    unfolded_result: UnfoldResult,
    file_path: String,
) -> Result<(), String> {
    // Проверяем, что путь к файлу корректный
    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err("Directory does not exist".to_string());
        }
    }
    
    // Преобразуем данные в формат ядра
    let core_result = pepakura_core::unfold::UnfoldResult {
        unfolded_faces: unfolded_result.unfolded_faces.into_iter().map(|face| {
            pepakura_core::unfold::UnfoldedFace {
                face_index: face.face_index,
                vertices_2d: face.vertices_2d,
                normal: face.normal,
            }
        }).collect(),
        seams: unfolded_result.seams.into_iter().map(|seam| {
            pepakura_core::unfold::Seam {
                start_point: seam.start_point,
                end_point: seam.end_point,
                face_a_index: seam.face_a_index,
                face_b_index: seam.face_b_index,
            }
        }).collect(),
        width: unfolded_result.width,
        height: unfolded_result.height,
    };
    
    // Экспортируем через ядро
    pepakura_core::export::export_to_png(&core_result, &file_path)
}

/// Экспортирует развернутую модель в JPG
#[tauri::command]
pub fn export_to_jpg(
    unfolded_result: UnfoldResult,
    file_path: String,
) -> Result<(), String> {
    // Проверяем, что путь к файлу корректный
    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err("Directory does not exist".to_string());
        }
    }
    
    // Преобразуем данные в формат ядра
    let core_result = pepakura_core::unfold::UnfoldResult {
        unfolded_faces: unfolded_result.unfolded_faces.into_iter().map(|face| {
            pepakura_core::unfold::UnfoldedFace {
                face_index: face.face_index,
                vertices_2d: face.vertices_2d,
                normal: face.normal,
            }
        }).collect(),
        seams: unfolded_result.seams.into_iter().map(|seam| {
            pepakura_core::unfold::Seam {
                start_point: seam.start_point,
                end_point: seam.end_point,
                face_a_index: seam.face_a_index,
                face_b_index: seam.face_b_index,
            }
        }).collect(),
        width: unfolded_result.width,
        height: unfolded_result.height,
    };
    
    // Экспортируем через ядро
    pepakura_core::export::export_to_jpg(&core_result, &file_path)
}

/// Экспортирует развернутую модель в OBJ
#[tauri::command]
pub fn export_to_obj(
    unfolded_result: UnfoldResult,
    file_path: String,
) -> Result<(), String> {
    // Проверяем, что путь к файлу корректный
    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err("Directory does not exist".to_string());
        }
    }
    
    // Преобразуем данные в формат ядра
    let core_result = pepakura_core::unfold::UnfoldResult {
        unfolded_faces: unfolded_result.unfolded_faces.into_iter().map(|face| {
            pepakura_core::unfold::UnfoldedFace {
                face_index: face.face_index,
                vertices_2d: face.vertices_2d,
                normal: face.normal,
            }
        }).collect(),
        seams: unfolded_result.seams.into_iter().map(|seam| {
            pepakura_core::unfold::Seam {
                start_point: seam.start_point,
                end_point: seam.end_point,
                face_a_index: seam.face_a_index,
                face_b_index: seam.face_b_index,
            }
        }).collect(),
        width: unfolded_result.width,
        height: unfolded_result.height,
    };
    
    // Экспортируем через ядро
    pepakura_core::export::export_to_obj(&core_result, &file_path)
}

/// Экспортирует развернутую модель в STL
#[tauri::command]
pub fn export_to_stl(
    unfolded_result: UnfoldResult,
    file_path: String,
) -> Result<(), String> {
    // Проверяем, что путь к файлу корректный
    let path = Path::new(&file_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            return Err("Directory does not exist".to_string());
        }
    }
    
    // Преобразуем данные в формат ядра
    let core_result = pepakura_core::unfold::UnfoldResult {
        unfolded_faces: unfolded_result.unfolded_faces.into_iter().map(|face| {
            pepakura_core::unfold::UnfoldedFace {
                face_index: face.face_index,
                vertices_2d: face.vertices_2d,
                normal: face.normal,
            }
        }).collect(),
        seams: unfolded_result.seams.into_iter().map(|seam| {
            pepakura_core::unfold::Seam {
                start_point: seam.start_point,
                end_point: seam.end_point,
                face_a_index: seam.face_a_index,
                face_b_index: seam.face_b_index,
            }
        }).collect(),
        width: unfolded_result.width,
        height: unfolded_result.height,
    };
    
    // Экспортируем через ядро
    pepakura_core::export::export_to_stl(&core_result, &file_path)
}