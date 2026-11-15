//! Экспорт развёртки в формат SVG.
//!
//! Генерирует SVG с слоями для:
//! - Линий реза (cut lines) — сплошные красные линии
//! - Линий сгиба (fold lines) — пунктирные синие линии
//! - Номеров деталей

use serde::{Deserialize, Serialize};
use crate::unfold::UnfoldedMesh;

/// Размер страницы для экспорта.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PageSize {
    A4,
    A3,
    A2,
    A1,
    Custom { width_mm: f64, height_mm: f64 },
}

impl PageSize {
    pub fn size_mm(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (210.0, 297.0),
            PageSize::A3 => (297.0, 420.0),
            PageSize::A2 => (420.0, 594.0),
            PageSize::A1 => (594.0, 841.0),
            PageSize::Custom { width_mm, height_mm } => (*width_mm, *height_mm),
        }
    }
}

/// Конфигурация экспорта в SVG.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgExportConfig {
    pub page_size: PageSize,
    pub scale: f64,
    pub show_vertex_ids: bool,
    pub show_fold_lines: bool,
    pub show_cut_lines: bool,
    pub show_part_numbers: bool,
}

impl Default for SvgExportConfig {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            scale: 1.0,
            show_vertex_ids: false,
            show_fold_lines: true,
            show_cut_lines: true,
            show_part_numbers: true,
        }
    }
}

/// Упрощённые настройки экспорта SVG (для Alpha версии).
#[derive(Debug, Clone, Copy)]
pub struct SvgExportOptions {
    /// Толщина линий (в пикселях).
    pub stroke_width: f32,
    /// Коэффициент масштабирования (пиксели на мм).
    pub scale: f32,
}

impl Default for SvgExportOptions {
    fn default() -> Self {
        Self {
            stroke_width: 1.0,
            scale: 10.0, // 10px = 1mm
        }
    }
}

/// Ошибки экспорта.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("Ошибка записи файла: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Пустой меш для экспорта")]
    EmptyMesh,
    #[error("Отсутствуют 2D-координаты вершин")]
    No2DCoordinates,
}

/// Экспортирует развёрнутый меш в SVG.
pub fn export_svg(
    unfolded: &UnfoldedMesh,
    config: &SvgExportConfig,
) -> Result<String, ExportError> {
    if unfolded.vertices_2d.is_empty() {
        return Err(ExportError::EmptyMesh);
    }

    let (width_mm, height_mm) = config.page_size.size_mm();
    let scale = config.scale;

    let (min_x, min_y, max_x, max_y) = calculate_bounding_box(unfolded);
    let model_width = max_x - min_x;
    let model_height = max_y - min_y;

    let model_center_x = (min_x + max_x) / 2.0;
    let model_center_y = (min_y + max_y) / 2.0;

    let page_width_px = width_mm * 3.78;
    let page_height_px = height_mm * 3.78;

    let scale_x = (page_width_px * 0.9) / (model_width * scale).max(1.0);
    let scale_y = (page_height_px * 0.9) / (model_height * scale).max(1.0);
    let view_scale = scale_x.min(scale_y);

    let transform_x = page_width_px / 2.0 - model_center_x * view_scale;
    let transform_y = page_height_px / 2.0 + model_center_y * view_scale;

    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg"
     width="{:.2}mm" height="{:.2}mm"
     viewBox="0 0 {:.2} {:.2}">
<!-- Pepakura Next SVG Export -->
"#,
        width_mm, height_mm, page_width_px, page_height_px
    ));

    svg.push_str(
        r#"<defs>
    <style type="text/css"><![CDATA[
        .cut-line { stroke: #ff0000; stroke-width: 0.5; fill: none; }
        .fold-line { stroke: #0000ff; stroke-width: 0.3; fill: none; stroke-dasharray: 2,2; }
        .part-number { font-family: Arial, sans-serif; font-size: 10px; fill: #000000; }
        .vertex-id { font-family: monospace; font-size: 8px; fill: #666666; }
    ]]></style>
</defs>
"#,
    );

    if config.show_cut_lines {
        svg.push_str("<g id=\"cut-lines\">\n");
        for face in &unfolded.faces {
            let v0 = &unfolded.vertices_2d[face.vertices[0]];
            let v1 = &unfolded.vertices_2d[face.vertices[1]];
            let v2 = &unfolded.vertices_2d[face.vertices[2]];

            let x0 = v0[0] * view_scale + transform_x;
            let y0 = -v0[1] * view_scale + transform_y;
            let x1 = v1[0] * view_scale + transform_x;
            let y1 = -v1[1] * view_scale + transform_y;
            let x2 = v2[0] * view_scale + transform_x;
            let y2 = -v2[1] * view_scale + transform_y;

            svg.push_str(&format!(
                "  <path class=\"cut-line\" d=\"M {:.2} {:.2} L {:.2} {:.2} L {:.2} {:.2} Z\"/>\n",
                x0, y0, x1, y1, x2, y2
            ));
        }
        svg.push_str("</g>\n");
    }

    if config.show_fold_lines {
        svg.push_str("<g id=\"fold-lines\">\n");
        let mut edges: std::collections::HashSet<(usize, usize)> = std::collections::HashSet::new();

        for face in &unfolded.faces {
            for i in 0..3 {
                let j = (i + 1) % 3;
                let v1 = face.vertices[i];
                let v2 = face.vertices[j];
                let edge = if v1 < v2 { (v1, v2) } else { (v2, v1) };

                if edges.contains(&edge) {
                    let vert1 = &unfolded.vertices_2d[edge.0];
                    let vert2 = &unfolded.vertices_2d[edge.1];

                    let x1 = vert1[0] * view_scale + transform_x;
                    let y1 = -vert1[1] * view_scale + transform_y;
                    let x2 = vert2[0] * view_scale + transform_x;
                    let y2 = -vert2[1] * view_scale + transform_y;

                    svg.push_str(&format!(
                        "  <line class=\"fold-line\" x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\"/>\n",
                        x1, y1, x2, y2
                    ));
                } else {
                    edges.insert(edge);
                }
            }
        }
        svg.push_str("</g>\n");
    }

    if config.show_part_numbers {
        svg.push_str("<g id=\"part-numbers\">\n");
        for (i, face) in unfolded.faces.iter().enumerate() {
            let v0 = &unfolded.vertices_2d[face.vertices[0]];
            let v1 = &unfolded.vertices_2d[face.vertices[1]];
            let v2 = &unfolded.vertices_2d[face.vertices[2]];

            let cx = (v0[0] + v1[0] + v2[0]) / 3.0;
            let cy = (v0[1] + v1[1] + v2[1]) / 3.0;

            let x = cx * view_scale + transform_x;
            let y = -cy * view_scale + transform_y;

            svg.push_str(&format!(
                "  <text class=\"part-number\" x=\"{:.2}\" y=\"{:.2}\" text-anchor=\"middle\">{}</text>\n",
                x, y, i + 1
            ));
        }
        svg.push_str("</g>\n");
    }

    if config.show_vertex_ids {
        svg.push_str("<g id=\"vertex-ids\">\n");
        for (i, &[x, y]) in unfolded.vertices_2d.iter().enumerate() {
            let px = x * view_scale + transform_x;
            let py = -y * view_scale + transform_y;

            svg.push_str(&format!(
                "  <text class=\"vertex-id\" x=\"{:.2}\" y=\"{:.2}\">{}</text>\n",
                px, py, i
            ));
        }
        svg.push_str("</g>\n");
    }

    svg.push_str("</svg>");
    Ok(svg)
}

/// Экспортирует развёрнутый меш в SVG файл.
pub fn export_svg_to_file(
    unfolded: &UnfoldedMesh,
    config: &SvgExportConfig,
    path: &str,
) -> Result<(), ExportError> {
    let svg = export_svg(unfolded, config)?;
    std::fs::write(path, svg)?;
    Ok(())
}

fn calculate_bounding_box(unfolded: &UnfoldedMesh) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for &[x, y] in &unfolded.vertices_2d {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    (min_x, min_y, max_x, max_y)
}

/// Генерирует XML-строку SVG с упрощёнными настройками (Alpha версия).
///
/// # Аргументы
/// * `mesh` - развёрнутый меш
/// * `opts` - настройки экспорта
///
/// # Возвращает
/// * `Result<String, ExportError>` - SVG строка или ошибка
///
/// # Пример
/// ```
/// use pepakura_core::export::svg::{export_to_svg, SvgExportOptions};
/// use pepakura_core::unfold::UnfoldedMesh;
///
/// let unfolded = UnfoldedMesh::default();
/// let opts = SvgExportOptions::default();
/// let svg = export_to_svg(&unfolded, &opts).unwrap();
/// ```
pub fn export_to_svg(mesh: &UnfoldedMesh, opts: &SvgExportOptions) -> Result<String, ExportError> {
    // Преобразуем упрощённые настройки в полный конфиг
    let config = SvgExportConfig {
        page_size: PageSize::A4,
        scale: opts.scale as f64,
        show_vertex_ids: false,
        show_fold_lines: true,
        show_cut_lines: true,
        show_part_numbers: false,
    };
    
    // Вызываем существующий экспортёр
    let svg_string = export_svg(mesh, &config)?;
    
    // Примечание: stroke_width из opts не используется в текущей реализации,
    // но можно добавить пост-обработку SVG для изменения толщины линий.
    // Для простоты MVP игнорируем.
    
    Ok(svg_string)
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
    fn test_export_svg_basic() {
        let unfolded = create_test_triangle();
        let config = SvgExportConfig::default();
        let svg = export_svg(&unfolded, &config).unwrap();

        assert!(svg.contains("<?xml version=\"1.0\""));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_export_svg_empty_mesh() {
        let unfolded = UnfoldedMesh {
            vertices_2d: vec![],
            faces: vec![],
            source_mesh: Mesh::new("Empty"),
            metadata: Default::default(),
        };
        let config = SvgExportConfig::default();
        let result = export_svg(&unfolded, &config);
        assert!(matches!(result, Err(ExportError::EmptyMesh)));
    }

    #[test]
    fn test_page_size() {
        assert_eq!(PageSize::A4.size_mm(), (210.0, 297.0));
        assert_eq!(PageSize::A3.size_mm(), (297.0, 420.0));
    }
}
