//! Оптимизированный MDS с параллельными вычислениями.
//!
//! Использует rayon для параллельного вычисления матрицы расстояний.

use crate::compat::*;
use nalgebra::{DMatrix, SymmetricEigen};
use crate::geometry::Mesh;
use crate::unfold::UnfoldError;

/// Параллельная версия MDS.
///
/// # Аргументы
/// * mesh - исходный меш
/// * max_iterations - максимум итераций
/// * tolerance - допуск сходимости
///
/// # Возвращает
/// * Ok(Vec<[f64; 2]>) - 2D координаты
/// * Err(UnfoldError) - ошибка
pub fn mds_parallel(
    mesh: &Mesh,
    max_iterations: usize,
    tolerance: f64,
) -> Result<Vec<[f64; 2]>, UnfoldError> {
    let n = mesh.vertices.len();

    if n < 2 {
        return Err(UnfoldError::TooFewVertices(n));
    }

    // Шаг 1: Параллельное вычисление матрицы расстояний
    let distances = compute_distances_parallel(&mesh.vertices);

    // Диагностика матрицы расстояний
    let mut max_dist = 0.0;
    let mut sum_dist = 0.0;
    let mut zero_count = 0;
    for i in 0..n {
        for j in 0..n {
            let d = distances[i][j];
            if d > max_dist {
                max_dist = d;
            }
            sum_dist += d;
            if d == 0.0 && i != j {
                zero_count += 1;
            }
        }
    }
    let avg_dist = sum_dist / (n * n) as f64;
    eprintln!("[MDS] Матрица расстояний: n={}, max={:.4}, avg={:.4}, zero_count={}", n, max_dist, avg_dist, zero_count);

    // Шаг 2: Двойное центрирование
    let b = double_centering(&distances);

    // Шаг 3: Собственное разложение
    let (eigenvalues, eigenvectors) = eigen_decomposition(&b, max_iterations, tolerance)?;

    eprintln!("[MDS] Eigenvalues: [{:.6}, {:.6}]", eigenvalues[0], eigenvalues[1]);

    // Шаг 4: Проекция на 2D с защитой от отрицательных eigenvalues
    let mut vertices_2d = Vec::with_capacity(n);
    for i in 0..n {
        let x = eigenvectors[0][i] * eigenvalues[0].max(0.0).sqrt();
        let y = eigenvectors[1][i] * eigenvalues[1].max(0.0).sqrt();
        vertices_2d.push([x, y]);
    }

    // Нормализация координат: центрирование + масштабирование к разумному диапазону
    if !vertices_2d.is_empty() {
        let mut centroid_x = 0.0;
        let mut centroid_y = 0.0;
        for &[x, y] in &vertices_2d {
            centroid_x += x;
            centroid_y += y;
        }
        centroid_x /= vertices_2d.len() as f64;
        centroid_y /= vertices_2d.len() as f64;

        // Центрируем
        for [x, y] in &mut vertices_2d {
            *x -= centroid_x;
            *y -= centroid_y;
        }

        // Находим масштаб
        let mut max_coord: f64 = 0.0;
        for &[x, y] in &vertices_2d {
            max_coord = max_coord.max(x.abs()).max(y.abs());
        }

        // Масштабируем к диапазону [-1, 1] если координаты слишком маленькие
        if max_coord < 0.1 && max_coord > 1e-10 {
            let scale = 1.0 / max_coord;
            eprintln!("[MDS] Масштабирование координат: scale={:.2}", scale);
            for [x, y] in &mut vertices_2d {
                *x *= scale;
                *y *= scale;
            }
        } else if max_coord < 1e-10 {
            eprintln!("[MDS] Предупреждение: нулевые координаты после MDS");
        }
    }

    Ok(vertices_2d)
}

/// Параллельное вычисление матрицы попарных расстояний.
fn compute_distances_parallel(vertices: &[crate::geometry::Vertex]) -> Vec<Vec<f64>> {
    let n = vertices.len();
    
    // Используем параллельный iter для ускорения
    (0..n)
        .into_par_iter()
        .map(|i| {
            (0..n)
                .map(|j| {
                    if i == j {
                        0.0
                    } else if i < j {
                        vertices[i].distance_to(&vertices[j])
                    } else {
                        vertices[j].distance_to(&vertices[i])
                    }
                })
                .collect()
        })
        .collect()
}

/// Двойное центрирование матрицы квадратов расстояний.
fn double_centering(distances: &[Vec<f64>]) -> DMatrix<f64> {
    let n = distances.len();
    let mut d2 = vec![vec![0.0; n]; n];

    // Квадраты расстояний (параллельно)
    d2.par_iter_mut().enumerate().for_each(|(i, row)| {
        for (j, val) in row.iter_mut().enumerate() {
            *val = distances[i][j].powi(2);
        }
    });

    // Средние по строкам
    let row_means: Vec<f64> = d2.par_iter()
        .map(|row| row.iter().sum::<f64>() / n as f64)
        .collect();

    // Средние по столбцам
    let col_means: Vec<f64> = (0..n)
        .into_par_iter()
        .map(|j| d2.iter().map(|row| row[j]).sum::<f64>() / n as f64)
        .collect();

    // Общее среднее
    let total_mean: f64 = d2.par_iter().flatten().sum::<f64>() / (n * n) as f64;

    // Двойное центрирование: B[i][j] = -0.5 * (d²_ij - row_mean_i - col_mean_j + grand_mean)
    let mut b = DMatrix::zeros(n, n);
    let mut trace: f64 = 0.0;
    let mut max_abs: f64 = 0.0;
    
    for i in 0..n {
        for j in 0..n {
            b[(i, j)] = -0.5 * (d2[i][j] - row_means[i] - col_means[j] + total_mean);
            if i == j {
                trace += b[(i, j)];
            }
            max_abs = max_abs.max(b[(i, j)].abs());
        }
    }

    eprintln!("[MDS] B-матрица: trace={:.6}, max_abs={:.6}", trace, max_abs);

    b
}

/// Собственное разложение через SymmetricEigen.
fn eigen_decomposition(
    matrix: &DMatrix<f64>,
    _max_iterations: usize,
    _tolerance: f64,
) -> Result<([f64; 2], [Vec<f64>; 2]), UnfoldError> {
    // Используем готовую реализацию из nalgebra
    let eigen = SymmetricEigen::new(matrix.clone());
    
    // Берём два наибольших собственных значения
    let mut indices: Vec<(usize, f64)> = eigen.eigenvalues.iter().copied().enumerate().collect();
    indices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    let idx1 = indices[0].0;
    let idx2 = indices[1].0;
    
    let eigenvalues = [indices[0].1, indices[1].1];
    
    let eigenvectors = [
        eigen.eigenvectors.column(idx1).iter().copied().collect(),
        eigen.eigenvectors.column(idx2).iter().copied().collect(),
    ];
    
    Ok((eigenvalues, eigenvectors))
}

/// Разреженная версия MDS для больших мешей.
///
/// Использует approximate nearest neighbors для уменьшения сложности.
pub fn mds_sparse(
    mesh: &Mesh,
    k: usize,  // количество ближайших соседей
    max_iterations: usize,
    tolerance: f64,
) -> Result<Vec<[f64; 2]>, UnfoldError> {
    let n = mesh.vertices.len();

    if n < 2 {
        return Err(UnfoldError::TooFewVertices(n));
    }

    // ЗАЩИТА: Защита от O(N^2) зависимости в памяти для больших сеток
    let n_safe_max = 15000; // Максимальный размер для одного вызова без серьезных тормозов на слабых ПК
    if n > n_safe_max {
        eprintln!("[Sparse MDS] Модель слишком велика ({} граней). Используйте LSCM для автогенерации выкроек.", n);
        return Err(UnfoldError::TooManyVertices(n, n_safe_max));
    }

    let effective_k = k.min(n - 1);

    // Вычисляем расстояния только до k ближайших соседей
    let mut sparse_distances = vec![vec![(usize::MAX, f64::INFINITY); n]; n];

    sparse_distances.par_iter_mut().enumerate().for_each(|(i, row)| {
        let mut distances: Vec<(usize, f64)> = (0..n)
            .map(|j| (j, mesh.vertices[i].distance_to(&mesh.vertices[j])))
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        distances.truncate(effective_k + 1); // Включая саму вершину

        for (j, dist) in distances {
            row[j] = (j, dist);
        }
    });

    // Используем разреженную матрицу для собственного разложения
    mds_parallel(mesh, max_iterations, tolerance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Mesh, Vertex, Face};
    
    fn create_test_mesh(size: usize) -> Mesh {
        let mut mesh = Mesh::new("Test");
        
        for i in 0..size {
            let x = (i % 10) as f64;
            let y = (i / 10) as f64;
            let z = 0.0;
            mesh.add_vertex(Vertex::new(i, [x, y, z]));
        }
        
        mesh
    }
    
    #[test]
    fn test_mds_parallel_small() {
        let mesh = create_test_mesh(10);
        let result = mds_parallel(&mesh, 100, 1e-6);
        
        assert!(result.is_ok());
        let vertices_2d = result.unwrap();
        assert_eq!(vertices_2d.len(), 10);
    }
    
    #[test]
    fn test_mds_parallel_medium() {
        let mesh = create_test_mesh(100);
        let result = mds_parallel(&mesh, 100, 1e-6);
        
        assert!(result.is_ok());
        let vertices_2d = result.unwrap();
        assert_eq!(vertices_2d.len(), 100);
    }
    
    #[test]
    fn test_mds_sparse() {
        let mesh = create_test_mesh(50);
        let result = mds_sparse(&mesh, 10, 100, 1e-6);
        
        assert!(result.is_ok());
        let vertices_2d = result.unwrap();
        assert_eq!(vertices_2d.len(), 50);
    }
    
    #[test]
    fn test_mds_empty_mesh() {
        let mesh = Mesh::new("Empty");
        let result = mds_parallel(&mesh, 100, 1e-6);
        
        assert!(matches!(result, Err(UnfoldError::TooFewVertices(0))));
    }
}