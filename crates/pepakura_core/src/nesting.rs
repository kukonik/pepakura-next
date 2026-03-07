use crate::project::PepaProject;
use serde::{Deserialize, Serialize};

/// Параметры бумаги
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaperSettings {
    /// Формат бумаги (A4, A3, Letter и т.д.)
    pub format: String,
    /// Ширина бумаги в мм
    pub width_mm: f32,
    /// Высота бумаги в мм
    pub height_mm: f32,
    /// Отступ от края бумаги в мм
    pub margin_mm: f32,
}

impl PaperSettings {
    /// Создание настроек бумаги по формату
    pub fn from_format(format: &str) -> Self {
        match format.to_uppercase().as_str() {
            "A4" => PaperSettings {
                format: "A4".to_string(),
                width_mm: 210.0,
                height_mm: 297.0,
                margin_mm: 5.0,
            },
            "A3" => PaperSettings {
                format: "A3".to_string(),
                width_mm: 297.0,
                height_mm: 420.0,
                margin_mm: 5.0,
            },
            "LETTER" => PaperSettings {
                format: "Letter".to_string(),
                width_mm: 215.9,
                height_mm: 279.4,
                margin_mm: 5.0,
            },
            _ => PaperSettings {
                format: format.to_string(),
                width_mm: 210.0,
                height_mm: 297.0,
                margin_mm: 5.0,
            },
        }
    }
}

/// Результат размещения разверток
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestResult {
    /// Список листов с размещенными развертками
    pub sheets: Vec<NestSheet>,
    /// Метрики качества размещения
    pub metrics: NestMetrics,
    /// Снимок параметров размещения
    pub params_snapshot: NestParams,
}

/// Лист с размещенными развертками
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestSheet {
    /// Номер листа
    pub id: u32,
    /// Индекс листа
    pub index: u32,
    /// Ширина листа в мм
    pub width_mm: f32,
    /// Высота листа в мм
    pub height_mm: f32,
    /// Отступ от края листа в мм
    pub margin_mm: f32,
    /// Размещенные части
    pub parts: Vec<NestPart>,
}

/// Часть развертки на листе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestPart {
    /// ID части
    pub id: u32,
    /// Имя части
    pub name: Option<String>,
    /// Индекс соответствующей UnfoldedFace в UnfoldResult
    pub unfolded_face_index: usize,
    /// Позиция X на листе (в мм)
    pub x_mm: f32,
    /// Позиция Y на листе (в мм)
    pub y_mm: f32,
    /// Ширина части (в мм)
    pub width_mm: f32,
    /// Высота части (в мм)
    pub height_mm: f32,
    /// Угол поворота в градусах
    pub rotation: f32,
}

/// Переопределение позиции и поворота части
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartOverride {
    /// ID части
    pub part_id: u32,
    /// Изменение позиции X (в мм)
    pub delta_x: Option<f32>,
    /// Изменение позиции Y (в мм)
    pub delta_y: Option<f32>,
    /// Изменение угла поворота (в градусах)
    pub delta_rotation: Option<f32>,
    /// Флаг, указывающий, что часть была изменена пользователем
    pub is_manual: Option<bool>,
}

/// Параметры размещения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestParams {
    /// Настройки бумаги
    pub paper: PaperSettings,
    /// Максимальное количество листов
    pub max_sheets: u32,
    /// Масштаб
    pub scale: f32,
    /// Шаг вращения в градусах
    pub rotation_step_deg: f32,
}

impl Default for NestParams {
    fn default() -> Self {
        NestParams {
            paper: PaperSettings::from_format("A4"),
            max_sheets: 4,
            scale: 1.0,
            rotation_step_deg: 45.0,
        }
    }
}

/// Метрики качества размещения
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestMetrics {
    /// Общее количество листов
    pub total_sheets: u32,
    /// Общее количество частей
    pub total_parts: u32,
    /// Средняя заполненность листов (%)
    pub avg_fill_rate: f32,
    /// Общая площадь всех частей (мм²)
    pub total_parts_area: f32,
    /// Общая площадь использованных листов (мм²)
    pub total_sheets_area: f32,
}

/// Структура для представления прямоугольника
#[derive(Debug, Clone)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Rectangle { x, y, width, height }
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn intersects(&self, other: &Rectangle) -> bool {
        self.x < other.x + other.width &&
        self.x + self.width > other.x &&
        self.y < other.y + other.height &&
        self.y + self.height > other.y
    }

    pub fn contains(&self, other: &Rectangle) -> bool {
        self.x <= other.x &&
        self.x + self.width >= other.x + other.width &&
        self.y <= other.y &&
        self.y + self.height >= other.y + other.height
    }
}

/// Свободный прямоугольник для размещения деталей
#[derive(Debug, Clone)]
pub struct FreeRect {
    pub rect: Rectangle,
}

impl FreeRect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        FreeRect { rect: Rectangle::new(x, y, width, height) }
    }
}

/// Позиция размещения детали
#[derive(Debug, Clone)]
pub struct PlacementPosition {
    pub sheet_index: usize,
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub score: f32,
}

/// MaxRects алгоритм размещения
pub fn nest_unfolds_maxrects(_project: &PepaProject, params: &NestParams) -> NestResult {
    let mut sheets: Vec<NestSheet> = Vec::new();
    let mut free_rects: Vec<Vec<FreeRect>> = Vec::new();

    // Создаем первый лист
    let sheet_width = params.paper.width_mm;
    let sheet_height = params.paper.height_mm;
    let margin = params.paper.margin_mm;

    let first_sheet = NestSheet {
        id: 0,
        index: 0,
        width_mm: sheet_width,
        height_mm: sheet_height,
        margin_mm: margin,
        parts: Vec::new(),
    };

    sheets.push(first_sheet);

    // Инициализируем свободные прямоугольники для первого листа
    let initial_free_rect = FreeRect::new(
        margin,
        margin,
        sheet_width - 2.0 * margin,
        sheet_height - 2.0 * margin,
    );
    free_rects.push(vec![initial_free_rect]);

    // Для демонстрации создаем несколько тестовых частей
    let test_parts = vec![
        NestPart {
            id: 1,
            name: Some("Part 1".to_string()),
            unfolded_face_index: 0,
            x_mm: 0.0,
            y_mm: 0.0,
            width_mm: 50.0,
            height_mm: 30.0,
            rotation: 0.0,
        },
        NestPart {
            id: 2,
            name: Some("Part 2".to_string()),
            unfolded_face_index: 1,
            x_mm: 0.0,
            y_mm: 0.0,
            width_mm: 40.0,
            height_mm: 25.0,
            rotation: 0.0,
        },
        NestPart {
            id: 3,
            name: Some("Part 3".to_string()),
            unfolded_face_index: 2,
            x_mm: 0.0,
            y_mm: 0.0,
            width_mm: 60.0,
            height_mm: 35.0,
            rotation: 0.0,
        },
    ];

    // Применяем масштаб к частям
    let scaled_parts: Vec<NestPart> = test_parts
        .iter()
        .map(|part| NestPart {
            id: part.id,
            name: part.name.clone(),
            unfolded_face_index: part.unfolded_face_index,
            x_mm: part.x_mm * params.scale,
            y_mm: part.y_mm * params.scale,
            width_mm: part.width_mm * params.scale,
            height_mm: part.height_mm * params.scale,
            rotation: part.rotation,
        })
        .collect();

    // Размещаем части на листах
    for part in &scaled_parts {
        let best_position = find_best_position(part, &sheets, &free_rects, params);

        if let Some(position) = best_position {
            // Обновляем лист с размещенной частью
            let sheet_index = position.sheet_index;
            let placed_part = NestPart {
                id: part.id,
                name: part.name.clone(),
                unfolded_face_index: part.unfolded_face_index,
                x_mm: position.x,
                y_mm: position.y,
                width_mm: part.width_mm,
                height_mm: part.height_mm,
                rotation: position.rotation,
            };

            sheets[sheet_index].parts.push(placed_part);

            // Обновляем свободные прямоугольники
            // TODO: Реализовать правильное обновление free_rects
        } else {
            // Создаем новый лист
            let new_sheet_id = sheets.len() as u32;
            let new_sheet = NestSheet {
                id: new_sheet_id,
                index: new_sheet_id,
                width_mm: sheet_width,
                height_mm: sheet_height,
                margin_mm: margin,
                parts: vec![part.clone()],
            };

            sheets.push(new_sheet);

            // Инициализируем свободные прямоугольники для нового листа
            // TODO: Реализовать правильную инициализацию free_rects
        }
    }

    // Вычисляем метрики качества
    let total_sheets = sheets.len() as u32;
    let total_parts = sheets.iter().map(|s| s.parts.len() as u32).sum();

    // Вычисляем площади
    let total_parts_area: f32 = sheets
        .iter()
        .map(|s| {
            s.parts
                .iter()
                .map(|p| p.width_mm * p.height_mm)
                .sum::<f32>()
        })
        .sum();

    let total_sheets_area = total_sheets as f32 * sheet_width * sheet_height;

    // Вычисляем среднюю заполненность
    let avg_fill_rate = if total_sheets_area > 0.0 {
        (total_parts_area / total_sheets_area) * 100.0
    } else {
        0.0
    };

    // Создаем метрики
    let metrics = NestMetrics {
        total_sheets,
        total_parts,
        avg_fill_rate,
        total_parts_area,
        total_sheets_area,
    };

    // Создаем снимок параметров
    let params_snapshot = params.clone();

    NestResult {
        sheets,
        metrics,
        params_snapshot,
    }
}

/// Найти лучшую позицию для размещения детали
fn find_best_position(
    part: &NestPart,
    sheets: &[NestSheet],
    free_rects: &[Vec<FreeRect>],
    params: &NestParams,
) -> Option<PlacementPosition> {
    let mut best_position: Option<PlacementPosition> = None;
    let mut best_score = f32::MAX;

    // Генерируем список углов для проверки
    let angles = generate_rotation_angles(params.rotation_step_deg);

    // Проверяем каждый лист
    for (sheet_index, sheet) in sheets.iter().enumerate() {
        // Проверяем каждый угол поворота
        for &angle in &angles {
            // Вычисляем размеры детали с учетом поворота
            let (width, height) = rotate_dimensions(part.width_mm, part.height_mm, angle);

            // Проверяем каждый свободный прямоугольник на листе
            for free_rect in &free_rects[sheet_index] {
                // Проверяем, помещается ли деталь в свободный прямоугольник
                if width <= free_rect.rect.width && height <= free_rect.rect.height {
                    // Вычисляем позицию размещения (в левом верхнем углу свободного прямоугольника)
                    let x = free_rect.rect.x;
                    let y = free_rect.rect.y;

                    // Вычисляем оценку размещения (чем меньше, тем лучше)
                    let score = compute_placement_score(
                        x,
                        y,
                        width,
                        height,
                        sheet,
                        free_rects[sheet_index].len(),
                    );

                    // Проверяем, является ли это лучшей позицией
                    if score < best_score {
                        best_score = score;
                        best_position = Some(PlacementPosition {
                            sheet_index,
                            x,
                            y,
                            rotation: angle,
                            score,
                        });
                    }
                }
            }
        }
    }

    best_position
}

/// Генерировать список углов поворота с заданным шагом
fn generate_rotation_angles(step: f32) -> Vec<f32> {
    let mut angles = Vec::new();
    let mut angle = 0.0;
    while angle < 180.0 {
        angles.push(angle);
        angle += step;
    }
    angles
}

/// Вычислить размеры прямоугольника после поворота
fn rotate_dimensions(width: f32, height: f32, angle: f32) -> (f32, f32) {
    // Преобразуем угол в радианы
    let angle_rad = angle.to_radians();

    // Вычисляем новые размеры
    let sin = angle_rad.sin().abs();
    let cos = angle_rad.cos().abs();

    let new_width = width * cos + height * sin;
    let new_height = width * sin + height * cos;

    (new_width, new_height)
}

/// Вычислить оценку размещения
fn compute_placement_score(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    _sheet: &NestSheet,
    free_rect_count: usize,
) -> f32 {
    // Простая эвристика: предпочитаем размещения ближе к левому верхнему углу
    // и стараемся минимизировать количество свободных прямоугольников
    let distance_score = x + y;
    let size_score = width * height;
    let fragmentation_score = free_rect_count as f32;

    distance_score + size_score * 0.01 + fragmentation_score * 10.0
}

/// Экспорт листа в SVG формат
/// Создает SVG строку с контурами для каждой части на листе
///
/// # Аргументы
/// * `sheet` - Лист с размещенными частями
/// * `unfold_result` - Результат развертки модели
///
/// # Возвращает
/// SVG строку с изображением листа и размещенных частей
pub fn export_sheet_to_svg(
    sheet: &NestSheet,
    unfold_result: &crate::unfold::UnfoldResult,
) -> String {
    let mut svg = String::new();

    // Добавляем заголовок SVG с размерами листа
    svg.push_str(&format!(
        "<svg width=\"{}mm\" height=\"{}mm\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">\n",
        sheet.width_mm, sheet.height_mm, sheet.width_mm, sheet.height_mm
    ));

    // Добавляем прямоугольник для границ листа
    svg.push_str(&format!(
        "  <rect x=\"0\" y=\"0\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"black\" stroke-width=\"0.5\"/>\n",
        sheet.width_mm, sheet.height_mm
    ));

    // Добавляем контуры для каждой части
    for part in &sheet.parts {
        // Получаем соответствующую UnfoldedFace
        if let Some(unfolded_face) = unfold_result.faces.get(part.unfolded_face_index) {
            let center = Point2D::new(unfolded_face.center.x, unfolded_face.center.y);
            let position = Point2D::new(part.x_mm as f64, part.y_mm as f64);

            // Трансформируем вершины контура
            let transformed_vertices = transform_points(
                &unfolded_face.vertices_2d,
                &center,
                part.rotation as f64,
                &position,
            );

            // Преобразуем точки в SVG path
            let path_data = points_to_svg_path(&transformed_vertices);

            // Добавляем path в SVG
            svg.push_str(&format!(
                "  <path d=\"{}\" fill=\"none\" stroke=\"blue\" stroke-width=\"0.2\"/>\n",
                path_data
            ));
        }
    }

    // Закрываем тег SVG
    svg.push_str("</svg>");

    svg
}

/// Экспорт результата размещения в вектор SVG строк
/// Каждый лист экспортируется в отдельный SVG
///
/// # Аргументы
/// * `result` - Результат размещения разверток
/// * `unfold_result` - Результат развертки модели
///
/// # Возвращает
/// Вектор SVG строк, по одной для каждого листа
pub fn export_nest_result_to_svgs(
    result: &NestResult,
    unfold_result: &crate::unfold::UnfoldResult,
) -> Vec<String> {
    result
        .sheets
        .iter()
        .map(|sheet| export_sheet_to_svg(sheet, unfold_result))
        .collect()
}

/// Простой алгоритм размещения разверток
/// Размещает части по листам сверху вниз, слева направо
pub fn nest_unfolds(project: &PepaProject, params: &NestParams) -> NestResult {
    // Используем новый MaxRects алгоритм
    nest_unfolds_maxrects(project, params)
}

/// Применить переопределения к результату размещения
/// Создает новый NestResult с учетом ручных изменений пользователя
///
/// # Аргументы
/// * `result` - Исходный результат размещения
/// * `overrides` - Список переопределений позиций и поворотов частей
///
/// # Возвращает
/// Новый NestResult с примененными переопределениями
pub fn apply_overrides_to_nest_result(
    result: &NestResult,
    overrides: &[PartOverride],
) -> NestResult {
    // Создаем хеш-таблицу переопределений для быстрого поиска
    let overrides_map: std::collections::HashMap<u32, &PartOverride> = overrides
        .iter()
        .map(|override_| (override_.part_id, override_))
        .collect();

    // Клонируем исходный результат
    let mut new_result = result.clone();

    // Применяем переопределения к частям на листах
    for sheet in &mut new_result.sheets {
        for part in &mut sheet.parts {
            if let Some(override_) = overrides_map.get(&part.id) {
                // Применяем изменения позиции
                if let Some(delta_x) = override_.delta_x {
                    part.x_mm += delta_x;
                }
                if let Some(delta_y) = override_.delta_y {
                    part.y_mm += delta_y;
                }

                // Применяем изменение угла поворота
                if let Some(delta_rotation) = override_.delta_rotation {
                    part.rotation += delta_rotation;
                }
            }
        }
    }

    new_result
}

/// Структура для представления 2D точки
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Создать новую точку
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }

    /// Применить трансформацию к точке
    pub fn transform(
        &self,
        center: &Point2D,
        rotation_deg: f64,
        position: &Point2D,
    ) -> Point2D {
        // Сначала центрируем точку относительно центра
        let centered_x = self.x - center.x;
        let centered_y = self.y - center.y;

        // Затем применяем поворот
        let rotation_rad = rotation_deg.to_radians();
        let cos = rotation_rad.cos();
        let sin = rotation_rad.sin();
        let rotated_x = centered_x * cos - centered_y * sin;
        let rotated_y = centered_x * sin + centered_y * cos;

        // Наконец, применяем позицию
        Point2D {
            x: rotated_x + position.x,
            y: rotated_y + position.y,
        }
    }
}

/// Преобразовать вектор точек с применением трансформации
pub fn transform_points(
    points: &[Point2D],
    center: &Point2D,
    rotation_deg: f64,
    position: &Point2D,
) -> Vec<Point2D> {
    points
        .iter()
        .map(|point| point.transform(center, rotation_deg, position))
        .collect()
}

/// Преобразовать вектор точек в SVG path строку
pub fn points_to_svg_path(points: &[Point2D]) -> String {
    if points.is_empty() {
        return String::new();
    }

    let mut path = String::new();

    // Начальная точка
    path.push_str(&format!("M {:.2} {:.2} ", points[0].x, points[0].y));

    // Линии к остальным точкам
    for point in &points[1..] {
        path.push_str(&format!("L {:.2} {:.2} ", point.x, point.y));
    }

    // Замыкаем контур
    path.push_str("Z");

    path
}
