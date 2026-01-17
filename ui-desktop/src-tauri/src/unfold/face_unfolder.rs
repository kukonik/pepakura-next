//! Модуль для развертки отдельных граней 3D модели в 2D полигоны

use crate::unfold::types::{Line2D, LineKind, Model3D, Point2D, Rect, Vertex3D};

/// Результат развертки одной грани
#[derive(Debug, Clone)]
pub struct UnfoldedFace {
    pub face_id: u32,
    pub polygon: Vec<Point2D>,
    pub bounds: Rect,
    pub lines: Vec<Line2D>,
}

/// Основной класс для развертки граней
pub struct FaceUnfolder;

impl FaceUnfolder {
    /// Создает новый экземпляр развертывателя граней
    pub fn new() -> Self {
        Self
    }

    /// Развертывает одну грань (треугольник) в 2D полигон
    pub fn unfold_face(&self, model: &Model3D, face_index: u32) -> Result<UnfoldedFace, String> {
        let face = model.faces.get(face_index as usize)
            .ok_or_else(|| format!("Грань с индексом {} не найдена", face_index))?;

        // Получаем вершины грани
        let v0 = self.get_vertex(model, face.vertices[0])?;
        let v1 = self.get_vertex(model, face.vertices[1])?;
        let v2 = self.get_vertex(model, face.vertices[2])?;

        // Преобразуем 3D координаты в 2D (проекция на плоскость XY)
        let points_2d = vec![
            Point2D { x: v0.x, y: v0.y },
            Point2D { x: v1.x, y: v1.y },
            Point2D { x: v2.x, y: v2.y },
        ];

        // Вычисляем границы
        let bounds = self.calculate_bounds(&points_2d);

        // Создаем линии контура
        let lines = self.create_outline_lines(&points_2d);

        Ok(UnfoldedFace {
            face_id: face_index,
            polygon: points_2d,
            bounds,
            lines,
        })
    }

    /// Получает вершину по индексу
    fn get_vertex<'a>(&self, model: &'a Model3D, index: u32) -> Result<&'a Vertex3D, String> {
        model.vertices.get(index as usize)
            .ok_or_else(|| format!("Вершина с индексом {} не найдена", index))
    }

    /// Вычисляет границы полигона
    fn calculate_bounds(&self, points: &[Point2D]) -> Rect {
        if points.is_empty() {
            return Rect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 };
        }

        let mut min_x = points[0].x;
        let mut max_x = points[0].x;
        let mut min_y = points[0].y;
        let mut max_y = points[0].y;

        for point in points.iter() {
            if point.x < min_x { min_x = point.x; }
            if point.x > max_x { max_x = point.x; }
            if point.y < min_y { min_y = point.y; }
            if point.y > max_y { max_y = point.y; }
        }

        Rect {
            x: min_x,
            y: min_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }

    /// Создает линии контура полигона
    fn create_outline_lines(&self, points: &[Point2D]) -> Vec<Line2D> {
        let mut lines = Vec::new();

        for i in 0..points.len() {
            let p1 = &points[i];
            let p2 = &points[(i + 1) % points.len()];

            lines.push(Line2D {
                x1: p1.x,
                y1: p1.y,
                x2: p2.x,
                y2: p2.y,
                kind: LineKind::Cut,
            });
        }

        lines
    }

    /// Добавляет клапан к ребру
    pub fn add_glue_tab(&self, line: &Line2D, width: f32) -> Vec<Line2D> {
        let mut tab_lines = vec![line.clone()]; // Основная линия

        // Вычисляем перпендикулярный вектор
        let dx = line.x2 - line.x1;
        let dy = line.y2 - line.y1;
        let length = (dx * dx + dy * dy).sqrt();

        if length > 0.0 {
            // Нормализованный перпендикулярный вектор
            let nx = -dy / length;
            let ny = dx / length;

            // Точки клапана
            let tab_x1 = line.x1 + nx * width;
            let tab_y1 = line.y1 + ny * width;
            let tab_x2 = line.x2 + nx * width;
            let tab_y2 = line.y2 + ny * width;

            // Линии клапана
            tab_lines.push(Line2D {
                x1: line.x1,
                y1: line.y1,
                x2: tab_x1,
                y2: tab_y1,
                kind: LineKind::GlueTab,
            });

            tab_lines.push(Line2D {
                x1: tab_x1,
                y1: tab_y1,
                x2: tab_x2,
                y2: tab_y2,
                kind: LineKind::GlueTab,
            });

            tab_lines.push(Line2D {
                x1: tab_x2,
                y1: tab_y2,
                x2: line.x2,
                y2: line.y2,
                kind: LineKind::GlueTab,
            });
        }

        tab_lines
    }
}

impl Default for FaceUnfolder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold_simple_triangle() {
        let model = Model3D {
            vertices: vec![
                Vertex3D { x: 0.0, y: 0.0, z: 0.0 },
                Vertex3D { x: 1.0, y: 0.0, z: 0.0 },
                Vertex3D { x: 0.0, y: 1.0, z: 0.0 },
            ],
            faces: vec![
                Face3D {
                    id: 0,
                    vertices: [0, 1, 2],
                    normal: [0.0, 0.0, 1.0],
                },
            ],
        };

        let unfolder = FaceUnfolder::new();
        let result = unfolder.unfold_face(&model, 0);

        assert!(result.is_ok());
        let unfolded = result.unwrap();
        assert_eq!(unfolded.polygon.len(), 3);
        assert_eq!(unfolded.lines.len(), 3);
    }

    #[test]
    fn test_calculate_bounds() {
        let unfolder = FaceUnfolder::new();
        let points = vec![
            Point2D { x: 1.0, y: 2.0 },
            Point2D { x: 3.0, y: 4.0 },
            Point2D { x: 0.0, y: 1.0 },
        ];

        let bounds = unfolder.calculate_bounds(&points);
        assert_eq!(bounds.x, 0.0);
        assert_eq!(bounds.y, 1.0);
        assert_eq!(bounds.width, 3.0);
        assert_eq!(bounds.height, 3.0);
    }
}