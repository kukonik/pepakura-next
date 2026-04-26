//! Экспорт развёртки в PDF (временно отключён из-за несовместимости API).
//!
//! TODO: Восстановить после обновления библиотеки printpdf.

use crate::export::PageSize;
use crate::unfold::UnfoldedMesh;

/// Конфигурация экспорта в PDF.
#[derive(Debug, Clone)]
pub struct PdfExportConfig {
    /// Размер страницы
    pub page_size: PageSize,
    /// Масштаб: мм на единицу модели
    pub scale: f64,
    /// Показывать линии сгиба
    pub show_fold_lines: bool,
    /// Показывать линии реза
    pub show_cut_lines: bool,
    /// Показывать номера деталей
    pub show_part_numbers: bool,
    /// Ориентация страницы
    pub orientation: PdfOrientation,
    /// Добавлять сетку
    pub show_grid: bool,
    /// Размер сетки в мм
    pub grid_size_mm: f64,
}

/// Ориентация страницы.
#[derive(Debug, Clone, Copy, Default)]
pub enum PdfOrientation {
    #[default]
    Portrait,
    Landscape,
}

impl Default for PdfExportConfig {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            scale: 1.0,
            show_fold_lines: true,
            show_cut_lines: true,
            show_part_numbers: true,
            orientation: PdfOrientation::Portrait,
            show_grid: false,
            grid_size_mm: 10.0,
        }
    }
}

/// Ошибки экспорта PDF.
#[derive(Debug, thiserror::Error)]
pub enum PdfExportError {
    /// Ошибка записи
    #[error("Ошибка записи: {0}")]
    IoError(#[from] std::io::Error),

    /// Ошибка printpdf
    #[error("Ошибка PDF: {0}")]
    PdfError(String),

    /// Пустой меш
    #[error("Пустой меш")]
    EmptyMesh,

    /// Нет вершин
    #[error("Нет 2D-координат вершин")]
    NoVertices,

    /// PDF экспорт временно отключён
    #[error("PDF экспорт временно отключён")]
    Disabled,
}

/// Результат экспорта PDF.
#[derive(Debug, Clone)]
pub struct PdfExportResult {
    /// PDF данные в bytes
    pub bytes: Vec<u8>,
    /// Количество страниц
    pub page_count: usize,
    /// Размер файла в байтах
    pub file_size: usize,
}

/// Экспортирует развёрнутый меш в PDF.
pub fn export_pdf(
    _unfolded: &UnfoldedMesh,
    _config: &PdfExportConfig,
) -> Result<PdfExportResult, PdfExportError> {
    Err(PdfExportError::Disabled)
}

/// Экспортирует развёрнутый меш в PDF файл.
pub fn export_pdf_to_file(
    _unfolded: &UnfoldedMesh,
    _config: &PdfExportConfig,
    _path: &str,
) -> Result<(), PdfExportError> {
    Err(PdfExportError::Disabled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Mesh, Vertex, Face};

    fn create_test_triangle() -> UnfoldedMesh {
        let mut mesh = Mesh::new("Triangle");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));

        UnfoldedMesh {
            vertices_2d: vec![[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]],
            faces: mesh.faces.clone(),
            source_mesh: mesh,
            metadata: Default::default(),
        }
    }

    #[test]
    fn test_export_pdf_disabled() {
        let unfolded = create_test_triangle();
        let config = PdfExportConfig::default();
        let result = export_pdf(&unfolded, &config);
        assert!(matches!(result, Err(PdfExportError::Disabled)));
    }

    #[test]
    fn test_export_pdf_empty_mesh() {
        let unfolded = UnfoldedMesh {
            vertices_2d: vec![],
            faces: vec![],
            source_mesh: Mesh::new("Empty"),
            metadata: Default::default(),
        };
        let config = PdfExportConfig::default();
        let result = export_pdf(&unfolded, &config);
        assert!(result.is_err());
    }
}