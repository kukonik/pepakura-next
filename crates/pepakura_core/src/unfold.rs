//! Модуль развёртки мешей.
//!
//! Предоставляет алгоритмы для проекции 3D-мешей на 2D-плоскость:
//! - MDS (Multidimensional Scaling) — классический алгоритм
//! - MDS Optimized — параллельная версия с rayon
//! - LSCM (Least Squares Conformal Maps) — сохранение углов
//! - MST (Maximum Spanning Tree) — бумажная развёртка по полосам (Papercraft)

pub mod lscm;
pub mod mds_optimized;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use serde::{Deserialize, Serialize};

use crate::geometry::{Face, Mesh};

/// Результат развёртки грани.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedFace {
    pub center: crate::nesting::Point2D,
    pub vertices_2d: Vec<crate::nesting::Point2D>,
    pub face_index: usize,
}

/// Результат раскладки.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutResult {
    pub faces: Vec<UnfoldedFace>,
    pub width: f32,
    pub height: f32,
}

/// Результат развёртки.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldResult {
    pub faces: Vec<UnfoldedFace>,
    pub seams: Vec<(usize, usize)>,
    pub layout: LayoutResult,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldConfig {
    pub preserve_detail: bool,
    pub max_iterations: usize,
    pub tolerance: f64,
    pub algorithm: UnfoldAlgorithm,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum UnfoldAlgorithm {
    #[default]
    MDS,
    LSCM,
    MST,
}

impl Default for UnfoldConfig {
    fn default() -> Self {
        Self {
            preserve_detail: true,
            max_iterations: 100,
            tolerance: 1e-6,
            algorithm: UnfoldAlgorithm::MST,
        }
    }
}

/// Метаданные развёртки (включая валидатор деградации Alpha 0.2)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UnfoldMetadata {
    pub algorithm: String,
    pub unfold_time_ms: f64,
    pub iterations: usize,
    pub convergence: Option<f64>,
    pub area_3d: f64,
    pub area_2d: f64,
    pub area_ratio: f64,
    pub aspect_ratio: [f64; 2],
    pub is_degenerate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldedMesh {
    pub vertices_2d: Vec<[f64; 2]>,
    pub uv_coords: Option<Vec<[f64; 2]>>,
    pub faces: Vec<Face>,
    pub source_mesh: Mesh,
    pub metadata: UnfoldMetadata,
}

#[derive(Debug, thiserror::Error)]
pub enum UnfoldError {
    #[error("Пустой меш для развёртки")]
    EmptyMesh,
    #[error("Недостаточно вершин: {0}, минимум 3")]
    TooFewVertices(usize),
    #[error("Слишком много вершин: {0}, максимум {1}")]
    TooManyVertices(usize, usize),
    #[error("Алгоритм не сошёлся за {0} итераций")]
    NoConvergence(usize),
    #[error("Численная ошибка: {0}")]
    NumericalError(String),
}

// ========================================================================
// MST PAPERCRAFT UNFOLDING ALGORITHM (Kruskal's + Undirected DFS)
// ========================================================================

#[derive(Debug, Clone)]
struct DualEdge {
    weight: f64,
    f1: usize,
    f2: usize,
    v1: usize,
    v2: usize,
}

impl PartialEq for DualEdge {
    fn eq(&self, other: &Self) -> bool { self.weight == other.weight }
}
impl Eq for DualEdge {}
impl PartialOrd for DualEdge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.weight.partial_cmp(&other.weight)
    }
}
impl Ord for DualEdge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

pub fn unfold_mst(mesh: &Mesh, _config: &UnfoldConfig) -> Result<UnfoldedMesh, UnfoldError> {
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();

    if mesh.vertices.is_empty() || mesh.faces.is_empty() {
        return Err(UnfoldError::EmptyMesh);
    }

    let n_faces = mesh.faces.len();
    let mut vertices_2d = vec![[0.0, 0.0]; mesh.vertices.len()];

    let mut edge_to_face: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    for (f_idx, face) in mesh.faces.iter().enumerate() {
        for k in 0..3 {
            let v1 = face.vertices[k];
            let v2 = face.vertices[(k + 1) % 3];
            let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };

            if let Some(&other_f_idx) = edge_to_face.get(&key) {
                let p1 = mesh.vertices[v1].position;
                let p2 = mesh.vertices[v2].position;
                let len = ((p2[0] - p1[0]).powi(2) + (p2[1] - p1[1]).powi(2) + (p2[2] - p1[2]).powi(2)).sqrt();
                heap.push(DualEdge { weight: len, f1: f_idx, f2: other_f_idx, v1, v2 });
            } else {
                edge_to_face.insert(key, f_idx);
            }
        }
    }

    struct LocalUf { parent: Vec<usize> }
    impl LocalUf {
        fn new(n: usize) -> Self { LocalUf { parent: (0..n).collect() } }
        fn find(&mut self, mut i: usize) -> usize { let mut root = i; while self.parent[root] != root { root = self.parent[root]; } while self.parent[i] != root { let next = self.parent[i]; self.parent[i] = root; i = next; } root }
        fn union(&mut self, i: usize, j: usize) {
            let ri = self.find(i); let rj = self.find(j);
            if ri != rj { self.parent[ri] = rj; }
        }
    }

    let mut uf = LocalUf::new(n_faces);
    let mut mst_adj: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); n_faces];

    while let Some(edge) = heap.pop() {
        if uf.find(edge.f1) == uf.find(edge.f2) { continue; }
        uf.union(edge.f1, edge.f2);
        mst_adj[edge.f1].push((edge.f2, edge.v1, edge.v2));
        mst_adj[edge.f2].push((edge.f1, edge.v1, edge.v2));
    }

    // Защита: поиск невырожденного стартового треугольника (Alpha 0.2)
    let mut f0_index = 0;
    for (idx, face) in mesh.faces.iter().enumerate() {
        let pa = mesh.vertices[face.vertices[0]].position;
        let pb = mesh.vertices[face.vertices[1]].position;
        if vec3_len(sub(pb, pa)) > 1e-10 { f0_index = idx; break; }
    }
    let f0 = &mesh.faces[f0_index];
    let p0 = mesh.vertices[f0.vertices[0]].position;
    let p1 = mesh.vertices[f0.vertices[1]].position;
    let p2 = mesh.vertices[f0.vertices[2]].position;

    vertices_2d[f0.vertices[0]] = [0.0, 0.0];
    let u_len = vec3_len(sub(p1, p0));
    vertices_2d[f0.vertices[1]] = [u_len, 0.0];
    vertices_2d[f0.vertices[2]] = unfold_triangle_math(p0, p1, p2, [0.0, 0.0], [u_len, 0.0]);

    let mut stack = vec![f0_index];
    let mut visited_faces = vec![false; n_faces];
    visited_faces[f0_index] = true;

    while let Some(current_f_idx) = stack.pop() {
        for &(neighbor_f_idx, v1, v2) in &mst_adj[current_f_idx] {
            if visited_faces[neighbor_f_idx] { continue; }
            visited_faces[neighbor_f_idx] = true;
            let child_face = &mesh.faces[neighbor_f_idx];
            let v3 = child_face.vertices.iter().copied().find(|&v| v != v1 && v != v2).unwrap_or(v1); // Fallback для вырожденных полигонов (zero-area)
            let p1_3d = mesh.vertices[v1].position;
            let p2_3d = mesh.vertices[v2].position;
            let p3_3d = mesh.vertices[v3].position;
            let p1_2d = vertices_2d[v1];
            let p2_2d = vertices_2d[v2];
            vertices_2d[v3] = unfold_triangle_math(p1_3d, p2_3d, p3_3d, p1_2d, p2_2d);
            stack.push(neighbor_f_idx);
        }
    }

    let area_3d = calc_area_3d(mesh);
    let area_2d = calc_area_2d(&vertices_2d, &mesh.faces);
    let area_ratio = if area_3d > 1e-6 { area_2d / area_3d } else { 1.0 };

    let mut min_x = f64::MAX; let mut max_x = f64::MIN;
    let mut min_y = f64::MAX; let mut max_y = f64::MIN;
    for v in &vertices_2d {
        if v[0] < min_x { min_x = v[0]; } if v[0] > max_x { max_x = v[0]; }
        if v[1] < min_y { min_y = v[1]; } if v[1] > max_y { max_y = v[1]; }
    }
    let aspect_ratio = [max_x - min_x, max_y - min_y];
    let is_degenerate = area_ratio < 0.1 || area_ratio > 10.0 || aspect_ratio[0] < 1e-3 || aspect_ratio[1] < 1e-3;

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
            algorithm: "MST-Papercraft".to_string(),
            unfold_time_ms: elapsed,
            iterations: 0,
            convergence: None,
            area_3d,
            area_2d,
            area_ratio,
            aspect_ratio,
            is_degenerate,
        },
    })
}

fn calc_area_3d(mesh: &Mesh) -> f64 {
    let mut area = 0.0;
    for face in &mesh.faces {
        let v0 = mesh.vertices[face.vertices[0]].position;
        let v1 = mesh.vertices[face.vertices[1]].position;
        let v2 = mesh.vertices[face.vertices[2]].position;
        let cross = cross_product(sub(v1, v0), sub(v2, v0));
        area += vec3_len(cross) * 0.5;
    }
    area
}

fn calc_area_2d(vertices_2d: &[[f64; 2]], faces: &[Face]) -> f64 {
    let mut area = 0.0;
    for face in faces {
        let v0 = vertices_2d[face.vertices[0]];
        let v1 = vertices_2d[face.vertices[1]];
        let v2 = vertices_2d[face.vertices[2]];
        area += ((v1[0] - v0[0]) * (v2[1] - v0[1]) - (v1[1] - v0[1]) * (v2[0] - v0[0])).abs() * 0.5;
    }
    area
}

#[inline(always)]
fn unfold_triangle_math(
    v1_3d: [f64; 3], v2_3d: [f64; 3], v3_3d: [f64; 3],
    v1_2d: [f64; 2], v2_2d: [f64; 2],
) -> [f64; 2] {
    let u_3d = sub(v2_3d, v1_3d);
    let w_3d = sub(v3_3d, v1_3d);
    let u_len = vec3_len(u_3d);
    if u_len < 1e-10 { return v1_2d; }
    let u_hat = scale_vec3(u_3d, 1.0 / u_len);
    let proj_len = dot_vec3(w_3d, u_hat);
    let perp_3d = sub(w_3d, scale_vec3(u_hat, proj_len));
    let h = vec3_len(perp_3d);
    let u_2d = sub2(v2_2d, v1_2d);
    let u_2d_len = vec2_len(u_2d);
    if u_2d_len < 1e-10 { return v1_2d; }
    let u_2d_hat = scale_vec2(u_2d, 1.0 / u_2d_len);
    let perp_2d_hat = [-u_2d_hat[1], u_2d_hat[0]];
    add2(v1_2d, add2(scale_vec2(u_2d_hat, proj_len), scale_vec2(perp_2d_hat, h)))
}

// ========================================================================
// MDS & PROJECTION ALGORITHMS (LEGACY)
// ========================================================================

pub fn unfold_mds(mesh: &Mesh, config: &UnfoldConfig) -> Result<UnfoldedMesh, UnfoldError> {
    #[cfg(not(target_arch = "wasm32"))]
    let start_time = std::time::Instant::now();
    if mesh.vertices.is_empty() { return Err(UnfoldError::EmptyMesh); }
    if mesh.vertices.len() < 3 { return Err(UnfoldError::TooFewVertices(mesh.vertices.len())); }
    let n = mesh.vertices.len();
    let mut distances = vec![vec![0.0; n]; n];
    for i in 0..n { for j in (i + 1)..n {
        let d = mesh.vertices[i].distance_to(&mesh.vertices[j]);
        distances[i][j] = d; distances[j][i] = d;
    }}
    let vertices_2d = mds_classical(&distances, config.max_iterations, config.tolerance)?;
    #[cfg(not(target_arch = "wasm32"))]
    let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
    #[cfg(target_arch = "wasm32")]
    let elapsed: f64 = 0.0;
    Ok(UnfoldedMesh {
        vertices_2d, uv_coords: None, faces: mesh.faces.clone(),
        source_mesh: mesh.clone(),
        metadata: UnfoldMetadata { algorithm: "MDS".to_string(), unfold_time_ms: elapsed, ..Default::default() },
    })
}

fn mds_classical(distances: &[Vec<f64>], max_iter: usize, tol: f64) -> Result<Vec<[f64; 2]>, UnfoldError> {
    let n = distances.len();
    if n < 2 { return Err(UnfoldError::TooFewVertices(n)); }
    let mut d2 = vec![vec![0.0; n]; n];
    for i in 0..n { for j in 0..n { d2[i][j] = distances[i][j].powi(2); } }
    let row_means: Vec<f64> = d2.iter().map(|row| row.iter().sum::<f64>() / n as f64).collect();
    let col_means: Vec<f64> = (0..n).map(|j| d2.iter().map(|row| row[j]).sum::<f64>() / n as f64).collect();
    let total_mean: f64 = d2.iter().flatten().sum::<f64>() / (n * n) as f64;
    let mut b = vec![vec![0.0; n]; n];
    for i in 0..n { for j in 0..n { b[i][j] = -0.5 * (d2[i][j] - row_means[i] - col_means[j] + total_mean); } }
    let (eigenvalues, eigenvectors) = power_iteration_2d(&b, max_iter, tol)?;
    let mut vertices_2d = Vec::with_capacity(n);
    for i in 0..n {
        let x = eigenvectors[0][i] * eigenvalues[0].sqrt().max(0.0);
        let y = eigenvectors[1][i] * eigenvalues[1].sqrt().max(0.0);
        vertices_2d.push([x, y]);
    }
    Ok(vertices_2d)
}

fn power_iteration_2d(matrix: &[Vec<f64>], max_iter: usize, tol: f64) -> Result<([f64; 2], [Vec<f64>; 2]), UnfoldError> {
    let n = matrix.len();
    let mut matrix_norm = 0.0;
    for i in 0..n { for j in 0..n { matrix_norm += matrix[i][j].abs(); } }
    if matrix_norm < 1e-10 { return Ok(([0.0, 0.0], [vec![0.0; n], vec![0.0; n]])); }
    let mut v1 = Vec::with_capacity(n);
    for i in 0..n { v1.push((i as f64 * 0.1).sin()); }
    let norm: f64 = v1.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-10 { for x in &mut v1 { *x /= norm; } }
    let mut lambda1 = 0.0;
    for _iter in 0..max_iter {
        let mut v_new = vec![0.0; n];
        for i in 0..n { for j in 0..n { v_new[i] += matrix[i][j] * v1[j]; } }
        let norm: f64 = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-15 { break; }
        for x in &mut v_new { *x /= norm; }
        let new_lambda = compute_rayleigh_quotient(matrix, &v_new);
        if (new_lambda - lambda1).abs() < tol { v1 = v_new; lambda1 = new_lambda; break; }
        lambda1 = new_lambda; v1 = v_new;
    }
    let mut b_deflated = vec![vec![0.0; n]; n];
    for i in 0..n { for j in 0..n { b_deflated[i][j] = matrix[i][j] - lambda1 * v1[i] * v1[j]; } }
    let mut v2 = Vec::with_capacity(n);
    for i in 0..n { v2.push((i as f64 * 0.1 + 0.5).sin()); }
    let norm: f64 = v2.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-10 { for x in &mut v2 { *x /= norm; } }
    let dot: f64 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    for i in 0..n { v2[i] -= dot * v1[i]; }
    let norm: f64 = v2.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm > 1e-10 { for x in &mut v2 { *x /= norm; } }
    let mut lambda2 = 0.0;
    for _iter in 0..max_iter {
        let mut v_new = vec![0.0; n];
        for i in 0..n { for j in 0..n { v_new[i] += b_deflated[i][j] * v2[j]; } }
        let dot_v1: f64 = v1.iter().zip(v_new.iter()).map(|(a, b)| a * b).sum();
        for i in 0..n { v_new[i] -= dot_v1 * v1[i]; }
        let norm: f64 = v_new.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-15 { break; }
        for x in &mut v_new { *x /= norm; }
        let new_lambda = compute_rayleigh_quotient(&b_deflated, &v_new);
        if (new_lambda - lambda2).abs() < tol { v2 = v_new; lambda2 = new_lambda; break; }
        lambda2 = new_lambda; v2 = v_new;
    }
    Ok(([lambda1, lambda2], [v1, v2]))
}

fn compute_rayleigh_quotient(matrix: &[Vec<f64>], v: &[f64]) -> f64 {
    let n = matrix.len();
    let mut av = vec![0.0; n];
    for i in 0..n { for j in 0..n { av[i] += matrix[i][j] * v[j]; } }
    let vav: f64 = v.iter().zip(av.iter()).map(|(a, b)| a * b).sum();
    let vtv: f64 = v.iter().map(|x| x * x).sum();
    if vtv < 1e-10 { 0.0 } else { vav / vtv }
}

pub fn unfold_simple_projection(mesh: &Mesh) -> Result<UnfoldedMesh, UnfoldError> {
    if mesh.vertices.is_empty() { return Err(UnfoldError::EmptyMesh); }
    let mut normal = [0.0, 0.0, 0.0];
    for face in &mesh.faces {
        if let Some(face_normal) = compute_face_normal(mesh, face) {
            normal[0] += face_normal[0]; normal[1] += face_normal[1]; normal[2] += face_normal[2];
        }
    }
    let norm = (normal[0].powi(2) + normal[1].powi(2) + normal[2].powi(2)).sqrt();
    let normal = if norm < 1e-10 { [0.0, 0.0, 1.0] } else { [normal[0] / norm, normal[1] / norm, normal[2] / norm] };
    let up = if normal[2].abs() < 0.9 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
    let x_axis = cross_product(normal, up);
    let y_axis = cross_product(normal, x_axis);
    let vertices_2d: Vec<[f64; 2]> = mesh.vertices.iter().map(|v| {
        let p = v.position;
        [dot_product(p, x_axis), dot_product(p, y_axis)]
    }).collect();
    Ok(UnfoldedMesh {
        vertices_2d, uv_coords: None, faces: mesh.faces.clone(),
        source_mesh: mesh.clone(),
        metadata: UnfoldMetadata { algorithm: "projection".to_string(), ..Default::default() },
    })
}

fn compute_face_normal(mesh: &Mesh, face: &Face) -> Option<[f64; 3]> {
    let v0 = &mesh.vertices.get(face.vertices[0])?.position;
    let v1 = &mesh.vertices.get(face.vertices[1])?.position;
    let v2 = &mesh.vertices.get(face.vertices[2])?.position;
    Some(cross_product([v1[0] - v0[0], v1[1] - v0[1], v1[2] - v0[2]], [v2[0] - v0[0], v2[1] - v0[1], v2[2] - v0[2]]))
}

fn cross_product(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn dot_product(a: [f64; 3], b: [f64; 3]) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
#[inline(always)] fn sub(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
#[inline(always)] fn scale_vec3(a: [f64; 3], s: f64) -> [f64; 3] { [a[0] * s, a[1] * s, a[2] * s] }
#[inline(always)] fn dot_vec3(a: [f64; 3], b: [f64; 3]) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
#[inline(always)] fn vec3_len(a: [f64; 3]) -> f64 { (a[0].powi(2) + a[1].powi(2) + a[2].powi(2)).sqrt() }
#[inline(always)] fn sub2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] { [a[0] - b[0], a[1] - b[1]] }
#[inline(always)] fn add2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] { [a[0] + b[0], a[1] + b[1]] }
#[inline(always)] fn scale_vec2(a: [f64; 2], s: f64) -> [f64; 2] { [a[0] * s, a[1] * s] }
#[inline(always)] fn vec2_len(a: [f64; 2]) -> f64 { (a[0].powi(2) + a[1].powi(2)).sqrt() }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vertex;
    fn create_test_cube() -> Mesh {
        let mut mesh = Mesh::new("Cube");
        let vertices = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
        for (i, &pos) in vertices.iter().enumerate() { mesh.add_vertex(Vertex::new(i, pos)); }
        let faces = [[0, 1, 2], [0, 2, 3], [4, 6, 5], [4, 7, 6], [0, 5, 1], [0, 4, 5], [1, 6, 2], [1, 5, 6], [2, 7, 3], [2, 6, 7], [3, 4, 0], [3, 7, 4]];
        for &[a, b, c] in &faces { mesh.add_face(Face::new(a, b, c)); }
        mesh
    }
    #[test]
    fn test_unfold_mst_cube() {
        let mesh = create_test_cube();
        let config = UnfoldConfig::default();
        let result = unfold_mst(&mesh, &config);
        assert!(result.is_ok());
        let unfolded = result.unwrap();
        assert_eq!(unfolded.vertices_2d.len(), 8);
        assert_eq!(unfolded.metadata.algorithm, "MST-Papercraft");
        assert!(unfolded.metadata.area_3d > 0.0);
        assert!(!unfolded.metadata.is_degenerate);
    }
}

pub mod seam_cut;
