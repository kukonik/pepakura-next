//! LSCM (Least Squares Conformal Maps) алгоритм развёртки.
//! 
//! LSCM сохраняет углы лучше чем MDS, что даёт меньше искажений
//! для моделей со сложной геометрией.
//! 
//! ## Алгоритм
//! 
//! 1. Строится матрица Лапласа с котангенсными весами
//! 2. Фиксируются две вершины (для устранения неоднозначности)
//! 3. Решается система линейных уравнений
//! 4. Получаются 2D координаты

use nalgebra::{DMatrix, DVector, Vector2};
use crate::geometry::Mesh;
use crate::unfold::{UnfoldedMesh, UnfoldMetadata, UnfoldError};

/// LSCM развёртыватель.
pub struct LscmUnfolder;

impl LscmUnfolder {
    /// Разворачивает меш используя LSCM.
    /// 
    /// # Аргументы
    /// * `mesh` - исходный 3D-меш
    /// 
    /// # Возвращает
    /// * `Ok(UnfoldedMesh)` - развёрнутый меш
    /// * `Err(UnfoldError)` - ошибка развёртки
    /// 
    /// # Пример
    /// 
    /// ```rust,no_run
    /// use pepakura_core::geometry::Mesh;
    /// use pepakura_core::unfold::lscm::LscmUnfolder;
    /// 
    /// let mesh = Mesh::load("model.obj").unwrap();
    /// let unfolded = LscmUnfolder::unfold(&mesh).unwrap();
    /// ```
    pub fn unfold(mesh: &Mesh) -> Result<UnfoldedMesh, UnfoldError> {
        if mesh.vertices.is_empty() {
            return Err(UnfoldError::EmptyMesh);
        }
        
        if mesh.vertices.len() < 3 {
            return Err(UnfoldError::TooFewVertices(mesh.vertices.len()));
        }
        
        // Шаг 1: Построение матрицы Лапласа
        let laplacian = Self::build_laplacian(mesh);
        
        // Шаг 2: Выбор фиксированных вершин
        let (fixed_v1, fixed_v2) = Self::select_fixed_vertices(mesh);
        
        // Шаг 3: Решение системы LSCM
        let vertices_2d = Self::solve_lscm(&laplacian, mesh, fixed_v1, fixed_v2)?;
        
        // Шаг 4: Построение результата
        Ok(UnfoldedMesh {
            vertices_2d,
            uv_coords: None,
            faces: mesh.faces.clone(),
            source_mesh: mesh.clone(),
            metadata: UnfoldMetadata {
                algorithm: "LSCM".to_string(),
                unfold_time_ms: 0.0,
                iterations: 1,
                convergence: None,
            },
        })
    }
    
    /// Строит матрицу Лапласа с котангенсными весами.
    /// 
    /// Котангенсные веса обеспечивают конформность отображения.
    fn build_laplacian(mesh: &Mesh) -> DMatrix<f64> {
        let n = mesh.vertices.len();
        let mut laplacian = DMatrix::zeros(n, n);
        
        // Для каждой грани вычисляем котангенсы углов
        for face in &mesh.faces {
            let v0 = &mesh.vertices[face.vertices[0]].position;
            let v1 = &mesh.vertices[face.vertices[1]].position;
            let v2 = &mesh.vertices[face.vertices[2]].position;
            
            // Вектора рёбер (используются для вычисления котангенсов)
            let _e0 = Vector2::new(v1[0] - v0[0], v1[1] - v0[1]);
            let _e1 = Vector2::new(v2[0] - v1[0], v2[1] - v1[1]);
            let _e2 = Vector2::new(v0[0] - v2[0], v0[1] - v2[1]);
            
            // Котангенсы углов (упрощённо, для 3D нужно через векторное произведение)
            let cot_alpha = Self::cotangent_angle(v0, v1, v2);
            let cot_beta = Self::cotangent_angle(v1, v2, v0);
            let cot_gamma = Self::cotangent_angle(v2, v0, v1);
            
            // Заполняем матрицу Лапласа
            let (i0, i1, i2) = (face.vertices[0], face.vertices[1], face.vertices[2]);
            
            // Диагональные элементы
            laplacian[(i0, i0)] += cot_alpha + cot_gamma;
            laplacian[(i1, i1)] += cot_alpha + cot_beta;
            laplacian[(i2, i2)] += cot_beta + cot_gamma;
            
            // Не диагональные элементы
            laplacian[(i0, i1)] -= cot_alpha;
            laplacian[(i1, i0)] -= cot_alpha;
            
            laplacian[(i1, i2)] -= cot_beta;
            laplacian[(i2, i1)] -= cot_beta;
            
            laplacian[(i2, i0)] -= cot_gamma;
            laplacian[(i0, i2)] -= cot_gamma;
        }
        
        laplacian
    }
    
    /// Вычисляет котангенс угла в вершине.
    fn cotangent_angle(a: &[f64; 3], b: &[f64; 3], c: &[f64; 3]) -> f64 {
        // Вектора
        let ba = [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
        let bc = [c[0] - b[0], c[1] - b[1], c[2] - b[2]];
        
        // Длины
        let ba_len = (ba[0].powi(2) + ba[1].powi(2) + ba[2].powi(2)).sqrt();
        let bc_len = (bc[0].powi(2) + bc[1].powi(2) + bc[2].powi(2)).sqrt();
        
        if ba_len < 1e-10 || bc_len < 1e-10 {
            return 0.0;
        }
        
        // Скалярное произведение
        let dot = ba[0] * bc[0] + ba[1] * bc[1] + ba[2] * bc[2];
        
        // Векторное произведение (для синуса)
        let cross = [
            ba[1] * bc[2] - ba[2] * bc[1],
            ba[2] * bc[0] - ba[0] * bc[2],
            ba[0] * bc[1] - ba[1] * bc[0],
        ];
        let cross_len = (cross[0].powi(2) + cross[1].powi(2) + cross[2].powi(2)).sqrt();
        
        // Котангенс = cos / sin = dot / |cross|
        if cross_len < 1e-10 {
            return 0.0;
        }
        
        dot / cross_len
    }
    
    /// Выбирает две вершины для фиксации.
    /// 
    /// Фиксация нужна для устранения неоднозначности (сдвиг, поворот, масштаб).
    fn select_fixed_vertices(mesh: &Mesh) -> (usize, usize) {
        // Выбираем две наиболее удалённые вершины
        let mut max_dist = 0.0;
        let mut fixed = (0, 1);
        
        for i in 0..mesh.vertices.len() {
            for j in (i + 1)..mesh.vertices.len() {
                let dist = mesh.vertices[i].distance_to(&mesh.vertices[j]);
                if dist > max_dist {
                    max_dist = dist;
                    fixed = (i, j);
                }
            }
        }
        
        fixed
    }
    
    /// Решает систему LSCM.
    /// 
    /// # Аргументы
    /// * `laplacian` - матрица Лапласа
    /// * `mesh` - исходный меш
    /// * `fixed_v1` - первая фиксированная вершина
    /// * `fixed_v2` - вторая фиксированная вершина
    /// 
    /// # Возвращает
    /// * `Ok(Vec<[f64; 2]>)` - 2D координаты
    /// * `Err(UnfoldError)` - ошибка
    fn solve_lscm(
        laplacian: &DMatrix<f64>,
        mesh: &Mesh,
        fixed_v1: usize,
        fixed_v2: usize,
    ) -> Result<Vec<[f64; 2]>, UnfoldError> {
        let n = mesh.vertices.len();
        
        // Создаём модифицированную систему с фиксированными вершинами
        let mut modified = laplacian.clone();
        
        // Фиксируем первую вершину в (0, 0)
        for j in 0..n {
            modified[(fixed_v1, j)] = 0.0;
        }
        modified[(fixed_v1, fixed_v1)] = 1.0;
        
        // Фиксируем вторую вершину в (1, 0) для масштаба
        for j in 0..n {
            modified[(fixed_v2, j)] = 0.0;
        }
        modified[(fixed_v2, fixed_v2)] = 1.0;
        
        // Решаем для X координаты
        let rhs_x = DVector::from_element(n, 0.0);
        let mut rhs_x_fixed = rhs_x.clone();
        rhs_x_fixed[fixed_v1] = 0.0; // Первая вершина в 0
        rhs_x_fixed[fixed_v2] = 1.0; // Вторая вершина в 1
        
        let x_coords = Self::solve_linear_system(&modified, &rhs_x_fixed)?;
        
        // Решаем для Y координаты
        let rhs_y = DVector::from_element(n, 0.0);
        let mut rhs_y_fixed = rhs_y.clone();
        rhs_y_fixed[fixed_v1] = 0.0; // Первая вершина в 0
        rhs_y_fixed[fixed_v2] = 0.0; // Вторая вершина в 0
        
        // Нужно зафиксировать ещё одну вершину для Y
        // Выбираем вершину с максимальным углом
        let fixed_v3 = Self::find_third_fixed_vertex(mesh, fixed_v1, fixed_v2);
        rhs_y_fixed[fixed_v3] = 1.0;
        
        // Модифицируем матрицу для третьей вершины
        modified[(fixed_v3, fixed_v3)] += 1.0;
        
        let y_coords = Self::solve_linear_system(&modified, &rhs_y_fixed)?;
        
        // Собираем 2D координаты
        let mut vertices_2d = Vec::with_capacity(n);
        for i in 0..n {
            vertices_2d.push([x_coords[i], y_coords[i]]);
        }
        
        Ok(vertices_2d)
    }
    
    /// Решает линейную систему Ax = b.
    fn solve_linear_system(a: &DMatrix<f64>, b: &DVector<f64>) -> Result<Vec<f64>, UnfoldError> {
        // Используем LU разложение
        let lu = a.clone().lu();
        let x = lu.solve(b).ok_or_else(|| {
            UnfoldError::NumericalError("Failed to solve linear system".to_string())
        })?;
        
        Ok(x.as_slice().to_vec())
    }
    
    /// Находит третью вершину для фиксации Y координаты.
    fn find_third_fixed_vertex(mesh: &Mesh, v1: usize, v2: usize) -> usize {
        // Выбираем вершину, наиболее удалённую от линии v1-v2
        let mut max_dist = 0.0;
        let mut best = 0;
        
        let p1 = &mesh.vertices[v1].position;
        let p2 = &mesh.vertices[v2].position;
        
        for (i, vertex) in mesh.vertices.iter().enumerate() {
            if i == v1 || i == v2 {
                continue;
            }
            
            let p = &vertex.position;
            
            // Расстояние от точки до прямой
            let dist = Self::point_line_distance(p, p1, p2);
            
            if dist > max_dist {
                max_dist = dist;
                best = i;
            }
        }
        
        best
    }
    
    /// Расстояние от точки до прямой.
    fn point_line_distance(p: &[f64; 3], a: &[f64; 3], b: &[f64; 3]) -> f64 {
        // Вектор AB
        let ab = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        
        // Вектор AP
        let ap = [p[0] - a[0], p[1] - a[1], p[2] - a[2]];
        
        // Векторное произведение
        let cross = [
            ab[1] * ap[2] - ab[2] * ap[1],
            ab[2] * ap[0] - ab[0] * ap[2],
            ab[0] * ap[1] - ab[1] * ap[0],
        ];
        
        let cross_len = (cross[0].powi(2) + cross[1].powi(2) + cross[2].powi(2)).sqrt();
        let ab_len = (ab[0].powi(2) + ab[1].powi(2) + ab[2].powi(2)).sqrt();
        
        if ab_len < 1e-10 {
            return 0.0;
        }
        
        cross_len / ab_len
    }
}

/// Разворачивает меш используя LSCM алгоритм.
/// 
/// Это удобная функция-обёртка над `LscmUnfolder::unfold`.
/// 
/// # Пример
/// 
/// ```rust,no_run
/// use pepakura_core::geometry::Mesh;
/// use pepakura_core::unfold::lscm::unfold_lscm;
/// 
/// let mesh = Mesh::load("model.obj").unwrap();
/// let unfolded = unfold_lscm(&mesh).unwrap();
/// ```
pub fn unfold_lscm(mesh: &Mesh) -> Result<UnfoldedMesh, UnfoldError> {
    LscmUnfolder::unfold(mesh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Mesh;
    
    fn create_test_triangle() -> Mesh {
        let mut mesh = Mesh::new("Triangle");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.5, 1.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));
        mesh
    }
    
    fn create_test_square() -> Mesh {
        let mut mesh = Mesh::new("Square");
        // 4 вершины квадрата
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [1.0, 1.0, 0.0]));
        mesh.add_vertex(Vertex::new(3, [0.0, 1.0, 0.0]));
        // 2 грани (треугольники)
        mesh.add_face(Face::new(0, 1, 2));
        mesh.add_face(Face::new(0, 2, 3));
        mesh
    }
    
    #[test]
    fn test_unfold_triangle() {
        let mesh = create_test_triangle();
        let result = unfold_lscm(&mesh);
        
        assert!(result.is_ok());
        let unfolded = result.unwrap();
        assert_eq!(unfolded.vertices_2d.len(), 3);
        assert_eq!(unfolded.faces.len(), 1);
    }
    
    #[test]
    fn test_unfold_square() {
        let mesh = create_test_square();
        let result = unfold_lscm(&mesh);
        
        assert!(result.is_ok());
        let unfolded = result.unwrap();
        assert_eq!(unfolded.vertices_2d.len(), 4);
        assert_eq!(unfolded.faces.len(), 2);
    }
    
    #[test]
    fn test_unfold_empty_mesh() {
        let mesh = Mesh::new("Empty");
        let result = unfold_lscm(&mesh);
        
        assert!(matches!(result, Err(UnfoldError::EmptyMesh)));
    }
    
    #[test]
    fn test_unfold_too_few_vertices() {
        let mut mesh = Mesh::new("TwoVertices");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        
        let result = unfold_lscm(&mesh);
        
        assert!(matches!(result, Err(UnfoldError::TooFewVertices(2))));
    }
    
    #[test]
    fn test_cotangent_angle() {
        // Прямой угол
        let a = [0.0, 0.0, 0.0];
        let b = [1.0, 0.0, 0.0];
        let c = [1.0, 1.0, 0.0];
        
        let cot = LscmUnfolder::cotangent_angle(&a, &b, &c);
        // Котангенс 45° ≈ 1.0
        assert!(cot.abs() > 0.5);
    }
    
    #[test]
    fn test_point_line_distance() {
        let p = [0.5, 1.0, 0.0];
        let a = [0.0, 0.0, 0.0];
        let b = [1.0, 0.0, 0.0];
        
        let dist = LscmUnfolder::point_line_distance(&p, &a, &b);
        assert!((dist - 1.0).abs() < 0.001);
    }
}
