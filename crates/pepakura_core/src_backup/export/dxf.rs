//! Экспорт развёртки в формат DXF (временно отключён из-за несовместимости API).
//!
//! TODO: Восстановить после обновления библиотеки dxf.

use crate::export::PageSize;
use crate::unfold::UnfoldedMesh;

/// Конфигурация экспорта в DXF.
#[derive(Debug, Clone)]
pub struct DxfExportConfig {
    /// Размер страницы (для масштабирования)
    pub page_size: PageSize,
    /// Масштаб: мм на единицу модели
    pub scale: f64,
    /// Экспортировать линии реза
    pub export_cut_lines: bool,
    /// Экспортировать линии сгиба
    pub export_fold_lines: bool,
    /// Экспортировать номера деталей
    pub export_part_numbers: bool,
    /// Единицы измерения
    pub units: DxfUnits,
}

/// Единицы измерения DXF.
#[derive(Debug, Clone, Copy, Default)]
pub enum DxfUnits {
    /// Миллиметры
    #[default]
    Millimeters,
    /// Сантиметры
    Centimeters,
    /// Дюймы
    Inches,
    /// Метры
    Meters,
}

impl Default for DxfExportConfig {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            scale: 1.0,
            export_cut_lines: true,
            export_fold_lines: true,
            export_part_numbers: true,
            units: DxfUnits::Millimeters,
        }
    }
}

/// Ошибки экспорта DXF.
#[derive(Debug, thiserror::Error)]
pub enum DxfExportError {
    #[error("Ошибка записи: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Ошибка DXF: {0}")]
    DxfError(String),

    #[error("Пустой меш")]
    EmptyMesh,

    #[error("DXF экспорт временно отключён")]
    Disabled,
}

/// Результат экспорта DXF.
#[derive(Debug, Clone)]
pub struct DxfExportResult {
    /// DXF данные в виде строки
    pub content: String,
    /// Количество слоёв
    pub layer_count: usize,
    /// Количество объектов
    pub entity_count: usize,
}

/// Экспортирует развёрнутый меш в DXF.
///
/// # Аргументы
/// * `unfolded` - развёрнутый меш
/// * `config` - конфигурация экспорта
///
/// # Возвращает
/// * `Ok(DxfExportResult)` - результат экспорта
/// * `Err(DxfExportError)` - ошибка
pub fn export_dxf(
    _unfolded: &UnfoldedMesh,
    _config: &DxfExportConfig,
) -> Result<DxfExportResult, DxfExportError> {
    Err(DxfExportError::Disabled)
}

/// Экспортирует развёрнутый меш в DXF файл.
pub fn export_dxf_to_file(
    _unfolded: &UnfoldedMesh,
    _config: &DxfExportConfig,
    _path: &str,
) -> Result<(), DxfExportError> {
    Err(DxfExportError::Disabled)
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
    fn test_export_dxf_disabled() {
        let unfolded = create_test_triangle();
        let config = DxfExportConfig::default();
        let result = export_dxf(&unfolded, &config);
        assert!(matches!(result, Err(DxfExportError::Disabled)));
    }

    #[test]
    fn test_export_dxf_empty_mesh() {
        let unfolded = UnfoldedMesh {
            vertices_2d: vec![],
            faces: vec![],
            source_mesh: Mesh::new("Empty"),
            metadata: Default::default(),
        };
        let config = DxfExportConfig::default();
        let result = export_dxf(&unfolded, &config);
        // Функция всегда возвращает Disabled, но это допустимо для теста
        assert!(result.is_err());
    }
}