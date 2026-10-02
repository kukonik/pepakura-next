//! Математические утилиты для работы с 3D векторами и матрицами.
//! Этот модуль содержит чистые функции для операций с векторами и матрицами.

/// Возвращает квадрат расстояния между двумя точками (для проверки пересечения шва).
pub fn dist_sq(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> f64 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    let dz = z1 - z2;
    dx * dx + dy * dy + dz * dz
}

/// Безопасное деление (защита от деления на ноль)
pub fn safe_div(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        f64::INFINITY
    } else {
        a / b
    }
}

/// Логарифм для отладки.
pub fn log(message: &str) {
    println!("[MathUtil] {}", message);
}

/// Сложение двух векторов (Float32Array представлен как срез)
pub fn add_vec(a: &[f32], b: &[f32], result: &mut [f32]) {
    if a.len() >= 5 && b.len() >= 5 && result.len() >= 5 {
        result[0] = a[0] + b[0];
        result[1] = a[1] + b[1];
        result[2] = a[2] + b[2];
        result[3] = a[3] + b[3];
        result[4] = a[4] + b[4];
    }
}

/// Разность векторов
pub fn sub_vec(a: &[f32], b: &[f32], result: &mut [f32]) {
    if a.len() >= 5 && b.len() >= 5 && result.len() >= 5 {
        result[0] = a[0] - b[0];
        result[1] = a[1] - b[1];
        result[2] = a[2] - b[2];
        result[3] = a[3] - b[3];
        result[4] = a[4] - b[4];
    }
}

/// Умножение вектора на число
pub fn mul_vec(v: &[f32], s: f32, result: &mut [f32]) {
    if v.len() >= 5 && result.len() >= 5 {
        result[0] = v[0] * s;
        result[1] = v[1] * s;
        result[2] = v[2] * s;
        result[3] = v[3] * s;
        result[4] = v[4] * s;
    }
}

/// Нормализация вектора (деление на длину)
pub fn normalize_vec(v: &[f32], result: &mut [f32]) -> bool {
    if v.len() < 5 || result.len() < 5 {
        return false;
    }
    let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2] + v[3] * v[3] + v[4] * v[4]).sqrt();
    if length == 0.0 {
        eprintln!("[MathUtil] Пытаюсь разделить вектор на ноль!");
        return false;
    }
    result[0] = v[0] / length;
    result[1] = v[1] / length;
    result[2] = v[2] / length;
    result[3] = v[3] / length;
    result[4] = v[4] / length;
    true
}

/// Квадрат расстояния
pub fn dist(v: &[f32]) -> f32 {
    if v.len() < 5 {
        return 0.0;
    }
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2] + v[3] * v[3] + v[4] * v[4]).sqrt()
}

/// Группирует грани (triangles).
/// Принимает массив `indices` [0, 1, 2, 3, 5, 6, ...] (всего по 3 вершины в лице).
pub fn group_faces(indices: &[u32]) -> Vec<[u32; 3]> {
    let mut faces = Vec::new();
    for i in (0..indices.len()).step_by(3) {
        if i + 2 < indices.len() {
            faces.push([indices[i], indices[i + 1], indices[i + 2]]);
        }
    }
    faces
}

/// Подсчёт количества уникальных вершин (упрощённо)
pub fn count_vertices(indices: &[u32]) -> usize {
    indices.len() / 3
}

/// Тип матрицы 4x4 (Column-Major)
pub type Mat4 = [f64; 16];

/// Тип матрицы 3x3
pub type Mat3 = [f64; 9];

/// Единичная матрица 4x4
pub const IDENTITY_MAT4: Mat4 = [
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    0.0, 0.0, 0.0, 1.0,
];

/// Умножает 3D-вектор на 4x4 матрицу.
pub fn mat4_multiply_vec(m: &Mat4, v: &[f64; 4]) -> [f64; 4] {
    [
        m[0] * v[0] + m[4] * v[1] + m[8] * v[2] + m[12] * v[3],
        m[1] * v[0] + m[5] * v[1] + m[9] * v[2] + m[13] * v[3],
        m[2] * v[0] + m[6] * v[1] + m[10] * v[2] + m[14] * v[3],
        m[3] * v[0] + m[7] * v[1] + m[11] * v[2] + m[15] * v[3],
    ]
}

/// Константа PI
pub const PI: f64 = std::f64::consts::PI;

/// Константа TWO_PI
pub const TWO_PI: f64 = 2.0 * std::f64::consts::PI;

/// Вычисляет длину окружности
pub fn calculate_circumference(radius: f64, degrees: f64) -> f64 {
    (2.0 * PI * radius) * (degrees / 360.0)
}

/// Преобразует радианы в градусы
pub fn to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}

/// Эпсилон для сравнения чисел с плавающей точкой
pub const EPSILON: f64 = 1e-4;

/// Проверяет, близко ли число к нулю
pub fn is_zero(x: f64) -> bool {
    x.abs() < EPSILON
}

/// Информация о геометрии
pub struct GeomState {
    pub indices: Vec<u32>,
    pub center: (f64, f64, f64),
    pub size: (f64, f64, f64),
}

/// Расчёт ограничивающего прямоугольника
pub fn get_bbox_geometry(verts: &[f32]) -> GeomState {
    if verts.is_empty() {
        return GeomState {
            indices: Vec::new(),
            center: (0.0, 0.0, 0.0),
            size: (0.0, 0.0, 0.0),
        };
    }

    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut min_z = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut max_z = f32::NEG_INFINITY;

    for i in (0..verts.len()).step_by(3) {
        let x = verts[i];
        let y = verts[i + 1];
        let z = verts[i + 2];

        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
        if z < min_z { min_z = z; }
        if z > max_z { max_z = z; }
    }

    GeomState {
        indices: Vec::new(),
        center: (
            (min_x as f64 + max_x as f64) / 2.0,
            (min_y as f64 + max_y as f64) / 2.0,
            (min_z as f64 + max_z as f64) / 2.0,
        ),
        size: (
            (max_x - min_x) as f64,
            (max_y - min_y) as f64,
            (max_z - min_z) as f64,
        ),
    }
}