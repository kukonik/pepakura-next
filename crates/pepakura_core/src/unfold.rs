//! Модуль развёртки мешей.
//!
//! Предоставляет алгоритмы для проекции 3D-мешей на 2D-плоскость:
//! - MDS (Multidimensional Scaling) — классический алгоритм
//! - MDS Optimized — параллельная версия с rayon
//! - LSCM (Least Squares Conformal Maps) — сохранение углов

pub mod lscm;
pub mod mds_optimized;

use serde::{Deserialize, Serialize};

use crate::geometry::{Face, Mesh};

/// Результат развёртки грани.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedFace {
    /// Центр грани в 2D
    pub center: crate::nesting::Point2D,
    /// 2D-координаты вершин
    pub vertices_2d: Vec<crate::nesting::Point2D>,
    /// Индекс исходной грани
    pub face_index: usize,
}

/// Результат раскладки.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutResult {
    /// Развёрнутые грани
    pub faces: Vec<UnfoldedFace>,
    /// Ширина раскладки
    pub width: f32,
    /// Высота раскладки
    pub height: f32,
}

/// Результат развёртки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldResult {
    /// Развёрнутые грани
    pub faces: Vec<UnfoldedFace>,
    /// Швы (рёбра для разреза)
    pub seams: Vec<(usize, usize)>,
    /// Раскладка
    pub layout: LayoutResult,
}

/// Конфигурация развёртки MDS.
/// 
/// # Примеры
/// 
/// ```
/// use pepakura_core::unfold::UnfoldConfig;
/// 
/// let config = UnfoldConfig::default();
/// assert_eq!(config.max_iterations, 100);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldConfig {
    /// Сохранять детали (текстуры, UV)
    pub preserve_detail: bool,
    /// Максимальное количество итераций
    pub max_iterations: usize,
    /// Допуск сходимости
    pub tolerance: f64,
    /// Алгоритм развёртки
    pub algorithm: UnfoldAlgorithm,
}

/// Алгоритм развёртки.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum UnfoldAlgorithm {
    /// MDS (Multidimensional Scaling)
    #[default]
    MDS,
    /// LSCM (Least Squares Conformal Maps)
    LSCM,
}

impl Default for UnfoldConfig {
    fn default() -> Self {
        Self {
            preserve_detail: true,
            max_iterations: 100,
            tolerance: 1e-6,
            algorithm: UnfoldAlgorithm::MDS,
        }
    }
}

/// Метаданные развёртки.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnfoldMetadata {
    /// Название алгоритма
    pub algorithm: String,
    /// Время выполнения (мс)
    pub unfold_time_ms: f64,
    /// Количество итераций
    pub iterations: usize,
    /// Метрика сходимости
    pub convergence: Option<f64>,
}

/// Развёрнутый меш.
///
/// Содержит 2D-координаты вершин и исходный меш.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedMesh {
    /// 2D-координаты вершин
    pub vertices_2d: Vec<[f64; 2]>,
    /// UV-координаты для текстур (опционально)
    pub uv_coords: Option<Vec<[f64; 2]>>,
    /// Грани (те же индексы что в исходном меше)
    pub faces: Vec<Face>,
    /// Исходный меш
    pub source_mesh: Mesh,
    /// Метаданные развёртки
    pub metadata: UnfoldMetadata,
}

/// Ошибки развёртки.
#[derive(Debug, thiserror::Error)]
pub enum UnfoldError {
    /// Пустой меш
    #[error("Пустой меш для развёртки")]
    EmptyMesh,
    /// Слишком мало вершин
    #[error("Недостаточно вершин: {0}, минимум 3")]
    TooFewVertices(usize),
    /// Слишком много вершин (ограничение производительности)
    #[error("Слишком много вершин: {0}, максимум {1}")]
    TooManyVertices(usize, usize),
    /// Алгоритм не сошёлся
    #[error("Алгоритм не сошёлся за {0} итераций")]
    NoConvergence(usize),
    /// Численная ошибка
    #[error("Численная ошибка: {0}")]
    NumericalError(String),
}

/// Разворачивает меш используя MDS (Multidimensional Scaling).
/// 
/// Алгоритм:
/// 1. Вычисляет матрицу попарных расстояний между вершинами (3D)
/// 2. Применяет классический MDS для получения 2D-координат
/// 3. Сохраняет топологию (грани) из исходного меша
/// 
/// # Аргументы
/// * `mesh` - исходный 3D-меш
/// * `config` - конфигурация развёртки
/// 
/// # Возвращает
/// * `Ok(UnfoldedMesh)` - развёрнутый меш с 2D-координатами
/// * `Err(UnfoldError)` - ошибка развёртки
/// 
/// # Примеры
/// 
/// ```rust,no_run
/// use pepakura_core::geometry::{Mesh, Vertex, Face};
/// use pepakura_core::unfold::{unfold_mds, UnfoldConfig};
/// 
/// let mut mesh = Mesh::new("Test");
/// // Добавление вершин и граней...
/// 
/// let config = UnfoldConfig::default();
/// let unfolded = unfold_mds(&mesh, &config).unwrap();
/// ```
pub fn unfold_mds(mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh, UnfoldError> {
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();

    if mesh.vertices.is_empty() {
        return Err(UnfoldError::EmptyMesh);
    }

    if mesh.vertices.len() < 3 {
        return Err(UnfoldError::TooFewVertices(mesh.vertices.len()));
    }

    let n = mesh.vertices.len();

    // Шаг 1: Матрица попарных расстояний (3D)
    let mut distances = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            let d = mesh.vertices[i].distance_to(&mesh.vertices[j]);
            distances[i][j] = d;
            distances[j][i] = d;
        }
    }

    // Шаг 2: MDS через собственное разложение
    let vertices_2d = mds_classical(&distances, config.max_iterations, config.tolerance)?;

    #[cfg(not(target_arch = "wasm32"))]
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    #[cfg(target_arch = "wasm32")]
    let elapsed: f64 = 0.0;

    Ok(UnfoldedMesh {
        vertices_2d,
        uv_coords: None,
        faces: mesh.faces.clone(),
        source_mesh: mesh.clone(),
        metadata: UnfoldMetadata {
            algorithm: "MDS".to_string(),
            unfold_time_ms: elapsed,
            iterations: config.max_iterations,
            convergence: None,
        },
    })
}

/// Классический MDS через собственное разложение.
/// 
/// # Аргументы
/// * `distances` - матрица попарных расстояний
/// * `max_iter` - максимальное количество итераций
/// * `tol` - допуск сходимости
/// 
/// # Возвращает
/// * `Ok(Vec<[f64; 2]>)` - 2D-координаты
/// * `Err(UnfoldError)` - ошибка
fn mds_classical(
    distances: &[Vec<f64>],
    max_iter: usize,
    tol: f64,
) -> Result<Vec<[f64; 2]>, UnfoldError> {
    let n = distances.len();

    if n < 2 {
        return Err(UnfoldError::TooFewVertices(n));
    }

    // Шаг 1: Двойное центрирование матрицы квадратов расстояний
    // B = -0.5 * J * D^2 * J, где J = I - 1/n * 11^T
    let mut d2 = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            d2[i][j] = distances[i][j].powi(2);
        }
    }

    // Вычисляем средние по строкам и столбцам
    let row_means: Vec<f64> = d2.iter().map(|row| row.iter().sum::<f64>() / n as f64).collect();
    let col_means: Vec<f64> = (0..n)
        .map(|j| d2.iter().map(|row| row[j]).sum::<f64>() / n as f64)
        .collect();
    let total_mean: f64 = d2.iter().flatten().sum::<f64>() / (n * n) as f64;

    // Двойное центрирование
    let mut b = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = -0.5 * (d2[i][j] - row_means[i] - col_means[j] + total_mean);
        }
    }

    // Шаг 2: Находим собственные значения и векторы (power iteration для 2 главных)
    // Для простоты используем наивную реализацию
    let (eigenvalues, eigenvectors) = power_iteration_2d(&b, max_iter, tol)?;

    // Шаг 3: Проецируем на 2D
    let mut vertices_2d = Vec::with_capacity(n);
    for i in 0..n {
        let x = eigenvectors[0][i] * eigenvalues[0].sqrt().max(0.0);
        let y = eigenvectors[1][i] * eigenvalues[1].sqrt().max(0.0);
        vertices_2d.push([x, y]);
    }

    Ok(vertices_2d)
}

/// Power iteration для нахождения 2 главных собственных пар.
///
/// Возвращает (собственные значения, собственные векторы).
fn power_iteration_2d(
    matrix: &[Vec<f64>],
    max_iter: usize,
    tol: f64,
) -> Result<([f64; 2], [Vec<f64>; 2]), UnfoldError> {
    let n = matrix.len();

    // Проверка на вырожденную матрицу
    let mut matrix_norm = 0.0;
    for i in 0..n {
        for j in 0..n {
            matrix_norm += matrix[i][j].abs();
        }
    }
    if matrix_norm < 1e-10 {
        eprintln!("[power_iteration] Вырожденная матрица (norm={:.2e})", matrix_norm);
        return Ok(([0.0, 0.0], [vec![0.0; n], vec![0.0; n]]));
    }

    // Инициализация первого вектора через sin() для избежания симметричных проблем
    let mut v1 = Vec::with_capacity(n);
    for i in 0..n {
        v1.push((i as f64 * 0.1).sin());
    }
    // Нормализуем начальный вектор
    let norm: f64 = v1.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-10 {
        for x in &mut v1 {
            *x /= norm;
        }
    }
    
    let mut lambda1 = 0.0;
    let mut converged1 = false;

    for iter in 0..max_iter {
        // v_new = A * v
        let mut v_new = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                v_new[i] += matrix[i][j] * v1[j];
            }
        }

        // Нормализуем
        let norm: f64 = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-15 {
            eprintln!("[power_iteration] Нулевая норма на итерации {} (v1)", iter);
            break;
        }
        for x in &mut v_new {
            *x /= norm;
        }

        // Вычисляем собственное значение (Rayleigh quotient)
        let new_lambda = compute_rayleigh_quotient(matrix, &v_new);

        // Проверяем сходимость
        if (new_lambda - lambda1).abs() < tol {
            v1 = v_new;
            lambda1 = new_lambda;
            converged1 = true;
            eprintln!("[power_iteration] v1 сошёлся на итерации {}: lambda={:.6}", iter, lambda1);
            break;
        }

        lambda1 = new_lambda;
        v1 = v_new;
    }
    
    if !converged1 {
        eprintln!("[power_iteration] v1 не сошёлся за {} итераций, lambda={:.6}", max_iter, lambda1);
    }

    // Дефляция: B' = B - lambda1 * v1 * v1^T
    let mut b_deflated = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b_deflated[i][j] = matrix[i][j] - lambda1 * v1[i] * v1[j];
        }
    }

    // Инициализация второго вектора через sin() со сдвигом
    let mut v2 = Vec::with_capacity(n);
    for i in 0..n {
        v2.push((i as f64 * 0.1 + 0.5).sin());
    }
    // Нормализуем начальный вектор
    let norm: f64 = v2.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-10 {
        for x in &mut v2 {
            *x /= norm;
        }
    }
    
    // Ортогонализуем относительно v1 (Gram-Schmidt)
    let dot: f64 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    for i in 0..n {
        v2[i] -= dot * v1[i];
    }
    let norm: f64 = v2.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-10 {
        for x in &mut v2 {
            *x /= norm;
        }
    }

    let mut lambda2 = 0.0;
    let mut converged2 = false;
    
    for iter in 0..max_iter {
        let mut v_new = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                v_new[i] += b_deflated[i][j] * v2[j];
            }
        }

        // Re-orthogonalization относительно v1 на каждой итерации
        let dot_v1: f64 = v1.iter().zip(v_new.iter()).map(|(a, b)| a * b).sum();
        for i in 0..n {
            v_new[i] -= dot_v1 * v1[i];
        }

        let norm: f64 = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-15 {
            eprintln!("[power_iteration] Нулевая норма на итерации {} (v2)", iter);
            break;
        }
        for x in &mut v_new {
            *x /= norm;
        }

        let new_lambda = compute_rayleigh_quotient(&b_deflated, &v_new);

        if (new_lambda - lambda2).abs() < tol {
            v2 = v_new;
            lambda2 = new_lambda;
            converged2 = true;
            eprintln!("[power_iteration] v2 сошёлся на итерации {}: lambda={:.6}", iter, lambda2);
            break;
        }

        lambda2 = new_lambda;
        v2 = v_new;
    }
    
    if !converged2 {
        eprintln!("[power_iteration] v2 не сошёлся за {} итераций, lambda={:.6}", max_iter, lambda2);
    }

    eprintln!("[power_iteration] Итоговые eigenvalues: [{:.6}, {:.6}]", lambda1, lambda2);
    Ok(([lambda1, lambda2], [v1, v2]))
}

/// Вычисляет Rayleigh quotient: (v^T A v) / (v^T v)
fn compute_rayleigh_quotient(matrix: &[Vec<f64>], v: &[f64]) -> f64 {
    let n = matrix.len();
    let mut av = vec![0.0; n];
    for i in 0..n {
        for j in 0..n {
            av[i] += matrix[i][j] * v[j];
        }
    }

    let vav: f64 = v.iter().zip(av.iter()).map(|(a, b)| a * b).sum();
    let vtv: f64 = v.iter().map(|x| x * x).sum();

    if vtv < 1e-10 {
        0.0
    } else {
        vav / vtv
    }
}

/// Простая развёртка через проекцию на плоскость.
/// 
/// Используется как fallback для простых мешей.
pub fn unfold_simple_projection(mesh: &Mesh) -> Result<UnfoldedMesh, UnfoldError> {
    if mesh.vertices.is_empty() {
        return Err(UnfoldError::EmptyMesh);
    }

    // Находим нормаль к средней плоскости
    let mut normal = [0.0, 0.0, 0.0];
    for face in &mesh.faces {
        if let Some(face_normal) = compute_face_normal(mesh, face) {
            normal[0] += face_normal[0];
            normal[1] += face_normal[1];
            normal[2] += face_normal[2];
        }
    }

    let norm = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
    if norm < 1e-10 {
        normal = [0.0, 0.0, 1.0];
    } else {
        normal = [normal[0] / norm, normal[1] / norm, normal[2] / norm];
    }

    // Создаём базис для проекции
    let up = if normal[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };

    let x_axis = cross_product(normal, up);
    let y_axis = cross_product(normal, x_axis);

    // Проецируем вершины
    let vertices_2d: Vec<[f64; 2]> = mesh
        .vertices
        .iter()
        .map(|v| {
            let p = v.position;
            let x = dot_product(p, x_axis);
            let y = dot_product(p, y_axis);
            [x, y]
        })
        .collect();

    Ok(UnfoldedMesh {
        vertices_2d,
        uv_coords: None,
        faces: mesh.faces.clone(),
        source_mesh: mesh.clone(),
        metadata: UnfoldMetadata {
            algorithm: "projection".to_string(),
            unfold_time_ms: 0.0,
            iterations: 1,
            convergence: None,
        },
    })
}

/// Вычисляет нормаль грани.
fn compute_face_normal(mesh: &Mesh, face: &Face) -> Option<[f64; 3]> {
    let v0 = &mesh.vertices.get(face.vertices[0])?.position;
    let v1 = &mesh.vertices.get(face.vertices[1])?.position;
    let v2 = &mesh.vertices.get(face.vertices[2])?.position;

    let e1 = [v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]];
    let e2 = [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]];

    Some(cross_product(e1, e2))
}

/// Векторное произведение.
fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

/// Скалярное произведение.
fn dot_product(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vertex;

    fn create_test_cube() -> Mesh {
        let mut mesh = Mesh::new("Cube");

        // 8 вершин куба
        let vertices = [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ];

        for (i, &pos) in vertices.iter().enumerate() {
            mesh.add_vertex(Vertex::new(i, pos));
        }

        // 12 граней (по 2 на каждую сторону)
        let faces = [
            [0, 1, 2],
            [0, 2, 3],
            [4, 6, 5],
            [4, 7, 6],
            [0, 5, 1],
            [0, 4, 5],
            [1, 6, 2],
            [1, 5, 6],
            [2, 7, 3],
            [2, 6, 7],
            [3, 4, 0],
            [3, 7, 4],
        ];

        for &[a, b, c] in &faces {
            mesh.add_face(Face::new(a, b, c));
        }

        mesh
    }

    #[test]
    fn test_unfold_config_default() {
        let config = UnfoldConfig::default();
        assert_eq!(config.max_iterations, 100);
        assert_eq!(config.tolerance, 1e-6);
        assert!(config.preserve_detail);
    }

    #[test]
    fn test_unfold_mds_empty_mesh() {
        let mesh = Mesh::new("Empty");
        let config = UnfoldConfig::default();
        let result = unfold_mds(&mesh, &config);
        assert!(matches!(result, Err(UnfoldError::EmptyMesh)));
    }

    #[test]
    fn test_unfold_mds_too_few_vertices() {
        let mut mesh = Mesh::new("TwoVertices");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));

        let config = UnfoldConfig::default();
        let result = unfold_mds(&mesh, &config);
        assert!(matches!(result, Err(UnfoldError::TooFewVertices(2))));
    }

    #[test]
    fn test_unfold_simple_projection() {
        let mesh = create_test_cube();
        let result = unfold_simple_projection(&mesh);
        assert!(result.is_ok());

        let unfolded = result.unwrap();
        assert_eq!(unfolded.vertices_2d.len(), 8);
        assert_eq!(unfolded.faces.len(), 12);
    }

    #[test]
    fn test_unfold_mds_triangle() {
        let mut mesh = Mesh::new("Triangle");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));

        let config = UnfoldConfig::default();
        let result = unfold_mds(&mesh, &config);
        assert!(result.is_ok());

        let unfolded = result.unwrap();
        assert_eq!(unfolded.vertices_2d.len(), 3);
    }

    #[test]
    fn test_cross_product() {
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let result = cross_product(a, b);
        assert_eq!(result, [0.0, 0.0, 1.0]);
    }

    #[test]
    fn test_dot_product() {
        let a = [1.0, 2.0, 3.0];
        let b = [4.0, 5.0, 6.0];
        assert_eq!(dot_product(a, b), 32.0);
    }
}
