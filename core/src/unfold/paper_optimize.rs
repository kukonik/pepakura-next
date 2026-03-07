//! Модуль оптимизации 3D моделей для печати на бумаге
//! 
//! Этот модуль содержит алгоритмы для оптимизации моделей с целью
//! минимизации отходов бумаги и улучшения удобства сборки.

use crate::unfold::{UnfoldedFace, Seam, Point2D, LayoutResult, PlacedFace, GlueTab};
use crate::model::Model;

/// Параметры оптимизации для бумаги
#[derive(Debug, Clone)]
pub struct PaperOptimizeParams {
    /// Минимальный зазор между элементами (мм)
    pub min_gap: f64,
    /// Ширина листа бумаги (мм)
    pub sheet_width: f64,
    /// Высота листа бумаги (мм)
    pub sheet_height: f64,
    /// Минимальная ширина вкладыша для склеивания (мм)
    pub min_tab_width: f64,
    /// Максимальный угол для автоматического создания вкладышей
    pub max_auto_tab_angle: f64,
    /// Добавлять ли поля для печати
    pub add_print_margins: bool,
    /// Размер полей для печати (мм)
    pub margin_size: f64,
}

impl Default for PaperOptimizeParams {
    fn default() -> Self {
        Self {
            min_gap: 2.0,
            sheet_width: 210.0,  // A4 ширина
            sheet_height: 297.0, // A4 высота
            min_tab_width: 5.0,
            max_auto_tab_angle: 60.0,
            add_print_margins: true,
            margin_size: 5.0,
        }
    }
}

/// Результат оптимизации для бумаги
#[derive(Debug, Clone)]
pub struct PaperOptimizeResult {
    /// Оптимизированные развернутые грани
    pub optimized_faces: Vec<UnfoldedFace>,
    /// Оптимизированные швы с вкладышами
    pub optimized_seams: Vec<Seam>,
    /// Информация об использовании бумаги
    pub paper_usage: PaperUsageInfo,
    /// Рекомендации по сборке
    pub assembly_tips: Vec<String>,
}

/// Информация об использовании бумаги
#[derive(Debug, Clone)]
pub struct PaperUsageInfo {
    /// Общая площадь использованной бумаги (мм²)
    pub used_area: f64,
    /// Общая площадь модели (мм²)
    pub model_area: f64,
    /// Процент использования бумаги
    pub usage_percentage: f64,
    /// Количество листов бумаги
    pub sheet_count: usize,
    /// Общая длина швов (мм)
    pub total_seam_length: f64,
}

/// Оптимизирует 3D модель для печати на бумаге
pub fn optimize_for_paper(
    model: &Model,
    params: &PaperOptimizeParams,
) -> PaperOptimizeResult {
    // TODO: Реализовать алгоритмы оптимизации
    
    // 1. Анализ модели для определения оптимальной стратегии разворачивания
    let analysis = analyze_model_for_paper(model);
    
    // 2. Разворачивание модели с учетом оптимизаций
    let (unfolded_faces, seams) = unfold_with_paper_optimizations(model, params);
    
    // 3. Добавление вкладышей для склеивания
    let (faces_with_tabs, updated_seams) = add_glue_tabs(unfolded_faces, seams, params);
    
    // 4. Укладка элементов на лист с минимальными отходами
    let layout_result = layout_for_paper_efficiency(faces_with_tabs, params);
    
    // 5. Генерация рекомендаций по сборке
    let assembly_tips = generate_assembly_tips(&layout_result, &analysis);
    
    // 6. Расчет информации об использовании бумаги
    let paper_usage = calculate_paper_usage(&layout_result, params);
    
    PaperOptimizeResult {
        optimized_faces: layout_result.faces.into_iter().map(|pf| pf.face).collect(),
        optimized_seams: updated_seams,
        paper_usage,
        assembly_tips,
    }
}

/// Анализирует модель для определения оптимальной стратегии разворачивания
fn analyze_model_for_paper(model: &Model) -> ModelAnalysis {
    // Подсчитываем количество граней
    let face_count = model.faces.len();
    
    // Оцениваем сложность модели на основе количества граней
    let complexity_score = (face_count as f64).min(100.0);
    
    // Оцениваем количество листов бумаги (упрощенно)
    let estimated_sheet_count = (face_count as f64 / 10.0).ceil() as usize;
    
    ModelAnalysis {
        face_count,
        estimated_sheet_count,
        complexity_score,
    }
}

/// Разворачивает модель с оптимизациями для бумаги
fn unfold_with_paper_optimizations(
    model: &Model,
    _params: &PaperOptimizeParams,
) -> (Vec<UnfoldedFace>, Vec<Seam>) {
    // Используем существующую функцию разворачивания
    let unfold_result = crate::unfold::unfold_model(model);
    
    // Возвращаем развернутые грани и швы
    (unfold_result.faces, unfold_result.seams)
}

/// Добавляет вкладыши для склеивания
fn add_glue_tabs(
    faces: Vec<UnfoldedFace>,
    seams: Vec<Seam>,
    params: &PaperOptimizeParams,
) -> (Vec<UnfoldedFace>, Vec<Seam>) {
    let mut faces_with_tabs = faces;
    let mut updated_seams = Vec::new();
    
    for seam in seams {
        // Проверяем, нужно ли добавлять вкладыш для этого шва
        if seam.angle_degrees <= params.max_auto_tab_angle {
            // Создаем вкладыш для шва
            let tab = create_glue_tab(&seam, params);
            
            // Добавляем вкладыш к соответствующей грани
            if let Some(face_index) = find_face_for_seam(&faces_with_tabs, &seam) {
                faces_with_tabs[face_index].tabs.push(tab);
            }
        }
        
        updated_seams.push(seam);
    }
    
    (faces_with_tabs, updated_seams)
}

/// Создает вкладыш для шва
fn create_glue_tab(seam: &Seam, params: &PaperOptimizeParams) -> GlueTab {
    // Определяем положение и размер вкладыша
    let midpoint = Point2D {
        x: (seam.start.x + seam.end.x) / 2.0,
        y: (seam.start.y + seam.end.y) / 2.0,
    };
    
    // Определяем направление перпендикулярное шву
    let dx = seam.end.x - seam.start.x;
    let dy = seam.end.y - seam.start.y;
    let length = (dx * dx + dy * dy).sqrt();
    
    // Нормализованный перпендикулярный вектор
    let perp_x = -dy / length;
    let perp_y = dx / length;
    
    // Создаем точки вкладыша
    let tab_width = params.min_tab_width;
    let tab_points = vec![
        seam.start,
        Point2D {
            x: midpoint.x + perp_x * tab_width,
            y: midpoint.y + perp_y * tab_width,
        },
        seam.end,
    ];
    
    GlueTab {
        points: tab_points,
        seam_id: seam.id,
    }
}

/// Находит индекс грани, к которой принадлежит шов
fn find_face_for_seam(faces: &[UnfoldedFace], seam: &Seam) -> Option<usize> {
    faces.iter().position(|face| {
        face.original_face_index == seam.face1_index ||
        face.original_face_index == seam.face2_index
    })
}

/// Укладывает элементы на лист с минимальными отходами
fn layout_for_paper_efficiency(
    faces: Vec<UnfoldedFace>,
    params: &PaperOptimizeParams,
) -> LayoutResult {
    // Простая реализация укладки - укладываем элементы в ряд
    let mut placed_faces = Vec::new();
    let mut current_x = params.margin_size;
    let mut current_y = params.margin_size;
    let mut max_row_height = 0.0;
    
    for face in faces {
        // Определяем размеры грани (упрощенно)
        let face_width = 50.0; // Примерная ширина
        let face_height = 50.0; // Примерная высота
        
        // Проверяем, помещается ли элемент в текущий ряд
        if current_x + face_width + params.margin_size > params.sheet_width {
            // Переходим к следующему ряду
            current_x = params.margin_size;
            current_y += max_row_height + params.min_gap;
            max_row_height = 0.0;
        }
        
        // Создаем размещённую грань
        let placed_face = PlacedFace {
            face,
            position: Point2D { x: current_x, y: current_y },
            rotation: 0.0,
        };
        
        placed_faces.push(placed_face);
        
        // Обновляем позицию для следующего элемента
        current_x += face_width + params.min_gap;
        max_row_height = max_row_height.max(face_height);
    }
    
    LayoutResult {
        faces: placed_faces,
        width: params.sheet_width,
        height: (current_y + max_row_height + params.margin_size).min(params.sheet_height),
    }
}

/// Генерирует рекомендации по сборке
fn generate_assembly_tips(
    _layout: &LayoutResult,
    analysis: &ModelAnalysis,
) -> Vec<String> {
    let mut tips = vec![
        "Начните сборку с центральных элементов".to_string(),
        "Склейте все вкладыши перед финальной сборкой".to_string(),
        "Используйте линейку для точного сгибания".to_string(),
    ];
    
    // Добавляем рекомендации на основе сложности модели
    if analysis.complexity_score > 70.0 {
        tips.push("Рекомендуется собирать модель по частям".to_string());
        tips.push("Используйте клей-карандаш для точечной фиксации".to_string());
    }
    
    if analysis.estimated_sheet_count > 9 {
        tips.push("Рекомендуется нумеровать элементы перед сборкой".to_string());
    }
    
    tips
}

/// Рассчитывает информацию об использовании бумаги
fn calculate_paper_usage(
    layout: &LayoutResult,
    params: &PaperOptimizeParams,
) -> PaperUsageInfo {
    // Рассчитываем общую площадь модели
    let model_area = layout.faces.iter().map(|pf| calculate_face_area(&pf.face)).sum();
    
    // Рассчитываем площадь листа бумаги
    let sheet_area = params.sheet_width * params.sheet_height;
    
    // Рассчитываем количество листов бумаги
    let sheet_count = ((model_area / sheet_area) + 0.99) as usize; // Округление вверх
    
    // Рассчитываем использованную площадь
    let used_area = sheet_count as f64 * sheet_area;
    
    // Рассчитываем процент использования бумаги
    let usage_percentage = if used_area > 0.0 { (model_area / used_area) * 100.0 } else { 0.0 };
    
    // Рассчитываем общую длину швов
    let total_seam_length = 0.0; // TODO: Реализовать расчет длины швов
    
    PaperUsageInfo {
        used_area,
        model_area,
        usage_percentage,
        sheet_count,
        total_seam_length,
    }
}

/// Рассчитывает площадь развернутой грани (треугольника)
fn calculate_face_area(face: &UnfoldedFace) -> f64 {
    if face.vertices_2d.len() < 3 {
        return 0.0;
    }
    
    // Для треугольника используем формулу площади через координаты вершин
    // S = 0.5 * |(x1(y2-y3) + x2(y3-y1) + x3(y1-y2))|
    let v0 = &face.vertices_2d[0];
    let v1 = &face.vertices_2d[1];
    let v2 = &face.vertices_2d[2];
    
    let area = 0.5 * ((v0.x * (v1.y - v2.y)) +
                      (v1.x * (v2.y - v0.y)) +
                      (v2.x * (v0.y - v1.y))).abs();
    
    area
}

/// Структура для хранения анализа модели
#[derive(Debug, Clone)]
struct ModelAnalysis {
    /// Количество граней в модели
    face_count: usize,
    /// Оценочное количество листов бумаги
    estimated_sheet_count: usize,
    /// Оценка сложности модели (0-100)
    complexity_score: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Model, Face, Vertex, Vector3D};
    
    #[test]
    fn test_paper_optimize_params_default() {
        let params = PaperOptimizeParams::default();
        assert_eq!(params.sheet_width, 210.0);
        assert_eq!(params.sheet_height, 297.0);
        assert_eq!(params.min_gap, 2.0);
    }
    
    #[test]
    fn test_optimize_for_paper() {
        // Создаем простую тестовую модель
        let model = Model {
            vertices: vec![
                Vertex { position: Vector3D { x: 0.0, y: 0.0, z: 0.0 } },
                Vertex { position: Vector3D { x: 1.0, y: 0.0, z: 0.0 } },
                Vertex { position: Vector3D { x: 0.0, y: 1.0, z: 0.0 } },
            ],
            faces: vec![
                Face {
                    vertex_indices: vec![0, 1, 2],
                    normal: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
                    material_id: None,
                }
            ],
        };
        
        let params = PaperOptimizeParams::default();
        let result = optimize_for_paper(&model, &params);
        
        // Проверяем, что результат создан
        assert_eq!(result.optimized_faces.len(), 0); // Пока не реализовано
        assert_eq!(result.optimized_seams.len(), 0); // Пока не реализовано
    }
}