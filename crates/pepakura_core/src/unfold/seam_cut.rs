//! MST-based Seam Cutting для Papercraft (Alpha 0.3)
//!
//! Алгоритм:
//! 1. Строим dual-граф (грани = узлы, общие рёбра = рёбра)
//! 2. Взвешиваем рёбра: высокий вес = не режем (плоскость), низкий = режем (сгиб)
//! 3. Строим Maximum Spanning Tree (Kruskal) — сохраняем лучшие соединения
//! 4. Если нужно K островов — удаляем (K-1) рёбер MST с минимальным весом
//! 5. Швы = complement к MST + принудительные разрезы
//! 6. Flood-fill по оставшемуся лесу -> острова
//! 7. Пост-обработка: слияние микро-островов (< MIN_ISLAND_FACES) обратно в крупнейший

use crate::geometry::Mesh;
use std::collections::HashMap;

const MIN_ISLAND_FACES: usize = 10;

#[derive(Debug, Clone)]
struct DualEdge {
    weight: f64,
    f1: usize,
    f2: usize,
    in_mst: bool,
}

/// Результат разрезания
pub struct SeamCutResult {
    /// Группы граней (острова), отсортированные по размеру (убывание)
    pub islands: Vec<Vec<usize>>,
    /// Количество швов (разрезанных рёбер)
    pub seam_count: usize,
}

/// Вычислить оптимальные швы через MST на dual-графе.
/// `target_islands` — желаемое число островов для этого компонента.
pub fn compute_seam_islands(mesh: &Mesh, target_islands: usize) -> SeamCutResult {
    let n_faces = mesh.faces.len();
    if n_faces == 0 {
        return SeamCutResult { islands: Vec::new(), seam_count: 0 };
    }
    if target_islands >= n_faces {
        let islands: Vec<Vec<usize>> = (0..n_faces).map(|i| vec![i]).collect();
        return SeamCutResult { islands, seam_count: n_faces };
    }

    // 1. Собираем рёбра dual-графа
    let mut edge_to_face: HashMap<(usize, usize), usize> = HashMap::new();
    let mut dual_edges: Vec<DualEdge> = Vec::new();

        // Alpha 0.4: Centroid calculation for compactness
        let mut cx = 0.0; let mut cy = 0.0; let mut cz = 0.0;
        for v in &mesh.vertices { cx += v.position[0]; cy += v.position[1]; cz += v.position[2]; }
        let n_v = mesh.vertices.len() as f64;
        let centroid = [cx / n_v, cy / n_v, cz / n_v];
        let mut max_dist = 0.0;
        for v in &mesh.vertices {
            let dx = v.position[0] - centroid[0]; let dy = v.position[1] - centroid[1]; let dz = v.position[2] - centroid[2];
            let d = (dx*dx + dy*dy + dz*dz).sqrt();
            if d > max_dist { max_dist = d; }
        }
        if max_dist < 1e-5 { max_dist = 1.0; }
    for (f_idx, face) in mesh.faces.iter().enumerate() {
        for k in 0..3 {
            let v1 = face.vertices[k];
            let v2 = face.vertices[(k + 1) % 3];
            let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            if let Some(&other_f_idx) = edge_to_face.get(&key) {
                let weight = compute_edge_weight(mesh, f_idx, other_f_idx, v1, v2, &centroid, max_dist);
                dual_edges.push(DualEdge { weight, f1: f_idx, f2: other_f_idx, in_mst: false });
            } else {
                edge_to_face.insert(key, f_idx);
            }
        }
    }

    // 2. Сортируем по весу (убывание) для Maximum Spanning Tree
    dual_edges.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(std::cmp::Ordering::Equal));

    // 3. Kruskal MST с итеративным UnionFind (path halving)
    let mut parent: Vec<usize> = (0..n_faces).collect();
    for edge in &mut dual_edges {
        let r1 = find_uf(&mut parent, edge.f1);
        let r2 = find_uf(&mut parent, edge.f2);
        if r1 != r2 {
            parent[r1] = r2;
            edge.in_mst = true;
        }
    }

    // 4. Если нужно больше 1 острова — разрезаем рёбра MST с минимальным весом
    let cuts_needed = if target_islands > 1 { target_islands - 1 } else { 0 };
    if cuts_needed > 0 {
        let mut mst_indices: Vec<usize> = dual_edges
            .iter()
            .enumerate()
            .filter(|(_, e)| e.in_mst)
            .map(|(i, _)| i)
            .collect();
        mst_indices.sort_by(|&a, &b| {
            dual_edges[a].weight.partial_cmp(&dual_edges[b].weight).unwrap_or(std::cmp::Ordering::Equal)
        });
        let actual_cuts = cuts_needed.min(mst_indices.len());
        for i in 0..actual_cuts {
            dual_edges[mst_indices[i]].in_mst = false;
        }
    }

    // 5. Смежность леса для flood-fill
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n_faces];
    for edge in &dual_edges {
        if edge.in_mst {
            adj[edge.f1].push(edge.f2);
            adj[edge.f2].push(edge.f1);
        }
    }

    // 6. Flood-fill -> острова
    let mut visited = vec![false; n_faces];
    let mut islands: Vec<Vec<usize>> = Vec::new();
    for start in 0..n_faces {
        if visited[start] { continue; }
        let mut island = Vec::new();
        let mut stack = vec![start];
        while let Some(fi) = stack.pop() {
            if visited[fi] { continue; }
            visited[fi] = true;
            island.push(fi);
            for &neighbor in &adj[fi] {
                if !visited[neighbor] { stack.push(neighbor); }
            }
        }
        islands.push(island);
    }

    // 7. Пост-обработка: слияние микро-островов в крупнейший
    islands = merge_micro_islands(islands, n_faces, &dual_edges);

    let seam_count = dual_edges.iter().filter(|e| !e.in_mst).count();
    SeamCutResult { islands, seam_count }
}

/// Слить все микро-острова (< MIN_ISLAND_FACES граней) в крупнейший остров.
fn merge_micro_islands(islands: Vec<Vec<usize>>, _n_faces: usize, _dual_edges: &[DualEdge]) -> Vec<Vec<usize>> {
    if islands.len() <= 1 { return islands; }

    // Находим индекс крупнейшего острова
    let mut largest_idx = 0;
    for i in 1..islands.len() {
        if islands[i].len() > islands[largest_idx].len() { largest_idx = i; }
    }

    // Строим set граней крупнейшего острова
    let mut large_set: std::collections::HashSet<usize> = islands[largest_idx].iter().copied().collect();

    // Собираем микро-острова
    let mut micro_faces: Vec<usize> = Vec::new();
    let mut new_islands: Vec<Vec<usize>> = Vec::new();

    for (i, island) in islands.into_iter().enumerate() {
        if i == largest_idx { continue; }
        if island.len() < MIN_ISLAND_FACES {
            micro_faces.extend(island);
        } else {
            new_islands.push(island);
        }
    }

    // Добавляем микро-грани в крупнейший остров
    large_set.extend(micro_faces);
    let mut merged: Vec<usize> = large_set.into_iter().collect();
    merged.sort_unstable();

    // Крупнейший — первым
    let mut result = Vec::with_capacity(new_islands.len() + 1);
    result.push(merged);
    result.extend(new_islands);
    result.sort_by(|a, b| b.len().cmp(&a.len()));
    result
}

#[inline(always)]
fn find_uf(parent: &mut Vec<usize>, mut i: usize) -> usize {
    while parent[i] != i {
        let next = parent[i];
        parent[i] = parent[next];
        i = next;
    }
    i
}

/// Вес ребра dual-графа.
/// Высокий = не режем (плоскость, длинное ребро).
/// Низкий = режем (острый сгиб, короткое ребро).
#[inline(always)]
fn compute_edge_weight(mesh: &Mesh, f1: usize, f2: usize, v1: usize, v2: usize, centroid: &[f64; 3], max_dist: f64) -> f64 {
    let n1 = face_normal(mesh, f1);
    let n2 = face_normal(mesh, f2);
    let cos_theta = dot3(&n1, &n2).clamp(-1.0, 1.0);
    let angle_deg = cos_theta.acos().to_degrees();

    let w_dihedral = if angle_deg < 10.0 { 10.0 }
        else if angle_deg < 25.0 { 5.0 }
        else if angle_deg < 45.0 { 2.0 }
        else if angle_deg < 90.0 { 0.5 }
        else { 0.05 };

    let p1 = mesh.vertices[v1].position;
    let p2 = mesh.vertices[v2].position;
    let len = ((p2[0]-p1[0]).powi(2) + (p2[1]-p1[1]).powi(2) + (p2[2]-p1[2]).powi(2)).sqrt();
    let w_length = len * 0.05;

    let mid_x = (p1[0] + p2[0]) * 0.5 - centroid[0];
    let mid_y = (p1[1] + p2[1]) * 0.5 - centroid[1];
    let mid_z = (p1[2] + p2[2]) * 0.5 - centroid[2];
    let dist = ((mid_x*mid_x + mid_y*mid_y + mid_z*mid_z).sqrt() / max_dist).clamp(0.0, 1.0);
    let w_centroid = 20.0 * (1.0 - dist);
    w_dihedral + w_length + w_centroid
}

#[inline(always)]
fn face_normal(mesh: &Mesh, fi: usize) -> [f64; 3] {
    let face = &mesh.faces[fi];
    let v0 = mesh.vertices[face.vertices[0]].position;
    let v1 = mesh.vertices[face.vertices[1]].position;
    let v2 = mesh.vertices[face.vertices[2]].position;
    let a = [v1[0]-v0[0], v1[1]-v0[1], v1[2]-v0[2]];
    let b = [v2[0]-v0[0], v2[1]-v0[1], v2[2]-v0[2]];
    let c = [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]];
    let len = (c[0]*c[0] + c[1]*c[1] + c[2]*c[2]).sqrt();
    if len > 1e-10 { [c[0]/len, c[1]/len, c[2]/len] } else { [0.0, 0.0, 0.0] }
}

#[inline(always)]
fn dot3(a: &[f64; 3], b: &[f64; 3]) -> f64 { a[0]*b[0] + a[1]*b[1] + a[2]*b[2] }

