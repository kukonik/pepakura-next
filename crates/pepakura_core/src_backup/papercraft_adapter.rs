//! Papercraft adapter: развёртка меша в детали (pieces) для бумажного моделирования.
//!
//! Этап 2: жадный рост грани (BFS по рёбрам двойственного графа) с отсечением
//! перекрытий, реальные fold lines (двугранные углы, Mountain/Valley),
//! glue tabs на швах между островами, упаковка островов на листы (shelf).
//!
//! Ключевое отличие от старого BFS: каждая грань хранит СВОЮ копию 2D
//! координат (face_2d). При разрезании по шву вершина "разрывается" на
//! несколько копий, поэтому уложенные грани никогда не искажаются.

use crate::Mesh;
use crate::unwrap_types::{
    FoldLine, FoldType, Page, Piece, Point2D, Polygon, Tab, UnwrapMetrics, UnwrapResult,
};
use std::collections::{HashMap, HashSet, VecDeque};

// ── Конфигурация ──────────────────────────────────────────

/// Конфигурация бумажной развёртки (все величины — в единицах меша).
#[derive(Debug, Clone)]
pub struct PaperUnfoldConfig {
    /// Ширина листа.
    pub sheet_width: f64,
    /// Высота листа.
    pub sheet_height: f64,
    /// Поле между деталями и краем листа.
    pub margin: f64,
    /// Ширина клеевого клапана (tab).
    pub tab_width: f64,
    /// Порог угла сгиба (градусы): более пологие сгибы не попадают в fold lines.
    pub fold_threshold_deg: f64,
    /// Максимум граней в одном острове (0 — без ограничения).
    pub max_faces_per_island: usize,
}

/// Диагональ bbox меша (масштаб сцены).
fn mesh_diag(mesh: &Mesh) -> f64 {
    if mesh.vertices.is_empty() {
        return 1.0;
    }
    let mut mn = [f64::MAX; 3];
    let mut mx = [f64::MIN; 3];
    for v in &mesh.vertices {
        let p = [v.x as f64, v.y as f64, v.z as f64];
        for k in 0..3 {
            if p[k] < mn[k] {
                mn[k] = p[k];
            }
            if p[k] > mx[k] {
                mx[k] = p[k];
            }
        }
    }
    let d = ((mx[0] - mn[0]).powi(2) + (mx[1] - mn[1]).powi(2) + (mx[2] - mn[2]).powi(2)).sqrt();
    if d > 1e-9 { d } else { 1.0 }
}

impl PaperUnfoldConfig {
    /// Конфигурация по умолчанию, масштабированная по габаритам меша.
    pub fn for_mesh(mesh: &Mesh) -> Self {
        let diag = mesh_diag(mesh);
        PaperUnfoldConfig {
            sheet_width: (diag * 1.5).max(10.0),
            sheet_height: (diag * 1.5).max(10.0),
            margin: (diag * 0.02).max(0.01),
            tab_width: (diag * 0.02).max(0.01),
            fold_threshold_deg: 5.0,
            max_faces_per_island: 0,
        }
    }
}

// ── Мелкая математика ─────────────────────────────────────

#[inline]
fn sub2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}
#[inline]
fn dot2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}
#[inline]
fn cross2(a: [f64; 2], b: [f64; 2]) -> f64 {
    a[0] * b[1] - a[1] * b[0]
}
#[inline]
fn len2(a: [f64; 2]) -> f64 {
    (a[0] * a[0] + a[1] * a[1]).sqrt()
}

#[inline]
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
#[inline]
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
#[inline]
fn cross3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}
#[inline]
fn dist3(a: [f64; 3], b: [f64; 3]) -> f64 {
    let d = sub3(a, b);
    (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt()
}

/// Обе точки пересечения двух окружностей (зеркальные кандидаты).
fn trilaterate_both(p1: [f64; 2], p2: [f64; 2], r1: f64, r2: f64) -> ([[f64; 2]; 2], bool) {
    let d_vec = sub2(p2, p1);
    let d = len2(d_vec);
    if d < 1e-12 {
        return ([p1, p1], false);
    }
    let u = [d_vec[0] / d, d_vec[1] / d];
    let a = (r1 * r1 - r2 * r2 + d * d) / (2.0 * d);
    let h_sq = r1 * r1 - a * a;
    let h = if h_sq > 0.0 { h_sq.sqrt() } else { 0.0 };
    let base = [p1[0] + a * u[0], p1[1] + a * u[1]];
    let perp = [-u[1], u[0]];
    (
        [
            [base[0] + h * perp[0], base[1] + h * perp[1]],
            [base[0] - h * perp[0], base[1] - h * perp[1]],
        ],
        h_sq > 0.0,
    )
}

/// Собственное (строгое) пересечение отрезков: касания и коллинеарность — не пересечение.
fn seg_seg_proper(p1: [f64; 2], p2: [f64; 2], q1: [f64; 2], q2: [f64; 2], eps: f64) -> bool {
    let d1 = cross2(sub2(q2, q1), sub2(p1, q1));
    let d2 = cross2(sub2(q2, q1), sub2(p2, q1));
    let d3 = cross2(sub2(p2, p1), sub2(q1, p1));
    let d4 = cross2(sub2(p2, p1), sub2(q2, p1));
    ((d1 > eps && d2 < -eps) || (d1 < -eps && d2 > eps))
        && ((d3 > eps && d4 < -eps) || (d3 < -eps && d4 > eps))
}

/// Строго внутренняя точка треугольника (граница не считается).
fn point_in_tri_strict(p: [f64; 2], t: [[f64; 2]; 3], eps: f64) -> bool {
    let d1 = cross2(sub2(t[1], t[0]), sub2(p, t[0]));
    let d2 = cross2(sub2(t[2], t[1]), sub2(p, t[1]));
    let d3 = cross2(sub2(t[0], t[2]), sub2(p, t[2]));
    (d1 > eps && d2 > eps && d3 > eps) || (d1 < -eps && d2 < -eps && d3 < -eps)
}

/// Перекрываются ли два треугольника (касания по рёбрам/вершинам — допустимы).
fn tri_tri_overlap(a: [[f64; 2]; 3], b: [[f64; 2]; 3], eps: f64) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if seg_seg_proper(a[i], a[(i + 1) % 3], b[j], b[(j + 1) % 3], eps) {
                return true;
            }
        }
    }
    for i in 0..3 {
        if point_in_tri_strict(a[i], b, eps) {
            return true;
        }
        if point_in_tri_strict(b[i], a, eps) {
            return true;
        }
    }
    false
}

fn tri_area_signed(t: &[[f64; 2]; 3]) -> f64 {
    cross2(sub2(t[1], t[0]), sub2(t[2], t[0])) * 0.5
}

fn poly_area_signed(pts: &[[f64; 2]]) -> f64 {
    let n = pts.len();
    if n < 3 {
        return 0.0;
    }
    let mut s = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        s += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
    }
    s * 0.5
}

fn aabb(t: &[[f64; 2]; 3]) -> (f64, f64, f64, f64) {
    let mut mnx = t[0][0];
    let mut mny = t[0][1];
    let mut mxx = t[0][0];
    let mut mxy = t[0][1];
    for p in t.iter() {
        if p[0] < mnx { mnx = p[0]; }
        if p[1] < mny { mny = p[1]; }
        if p[0] > mxx { mxx = p[0]; }
        if p[1] > mxy { mxy = p[1]; }
    }
    (mnx, mny, mxx, mxy)
}

fn acc_mm(p: [f64; 2], mn: &mut [f64; 2], mx: &mut [f64; 2]) {
    for k in 0..2 {
        if p[k] < mn[k] { mn[k] = p[k]; }
        if p[k] > mx[k] { mx[k] = p[k]; }
    }
}

// ── Broad-phase сетка (один остров) ───────────────────────

struct IslandGrid {
    cell: f64,
    map: HashMap<(i64, i64), Vec<usize>>,
}
impl IslandGrid {
    fn new(cell: f64) -> Self {
        IslandGrid { cell, map: HashMap::new() }
    }
    fn insert(&mut self, t: &[[f64; 2]; 3], fi: usize) {
        let (mnx, mny, mxx, mxy) = aabb(t);
        let x0 = (mnx / self.cell).floor() as i64;
        let x1 = (mxx / self.cell).floor() as i64;
        let y0 = (mny / self.cell).floor() as i64;
        let y1 = (mxy / self.cell).floor() as i64;
        for cx in x0..=x1 {
            for cy in y0..=y1 {
                self.map.entry((cx, cy)).or_default().push(fi);
            }
        }
    }
    fn candidates(&self, t: &[[f64; 2]; 3]) -> Vec<usize> {
        let (mnx, mny, mxx, mxy) = aabb(t);
        let x0 = (mnx / self.cell).floor() as i64;
        let x1 = (mxx / self.cell).floor() as i64;
        let y0 = (mny / self.cell).floor() as i64;
        let y1 = (mxy / self.cell).floor() as i64;
        let mut out = Vec::new();
        for cx in x0..=x1 {
            for cy in y0..=y1 {
                if let Some(v) = self.map.get(&(cx, cy)) {
                    out.extend_from_slice(v);
                }
            }
        }
        out
    }
}

// ── Ключи рёбер по 2D-точкам (точное совпадение копий) ────



// ── Внутренние структуры острова ──────────────────────────

struct FoldLineF {
    p1: [f64; 2],
    p2: [f64; 2],
    mountain: bool,
    angle_deg: f32,
}

struct BoundaryEdgeRaw {
    p1: [f64; 2],
    p2: [f64; 2],
    face: usize,
    edge: usize,
    v1: usize,
    v2: usize,
    other_face: Option<usize>,
}

struct TabRaw {
    poly: [[f64; 2]; 4],
    glue_to: u32,
}

struct IslandData {
    faces: Vec<usize>,
    folds: Vec<FoldLineF>,
    boundary: Vec<BoundaryEdgeRaw>,
    grid: IslandGrid,
}

/// Выбор следующего семени: сначала грань, смежная с уже уложенными.
fn next_seed(
    unvisited: &HashSet<usize>,
    edge_map: &HashMap<(usize, usize), Vec<usize>>,
    island_of_face: &[Option<usize>],
    tris: &[Option<[usize; 3]>],
) -> Option<usize> {
    for fi in 0..tris.len() {
        if !unvisited.contains(&fi) {
            continue;
        }
        if let Some([a, b, c]) = tris[fi] {
            for &(p, q) in &[(a, b), (b, c), (c, a)] {
                let key = (p.min(q), p.max(q));
                if let Some(list) = edge_map.get(&key) {
                    for &nfi in list {
                        if island_of_face[nfi].is_some() {
                            return Some(fi);
                        }
                    }
                }
            }
        }
    }
    for fi in 0..tris.len() {
        if unvisited.contains(&fi) {
            return Some(fi);
        }
    }
    None
}

// ── Главный алгоритм ──────────────────────────────────────

/// Жадная развёртка с отсечением перекрытий. Возвращает полный результат
/// (pieces/pages/metrics) в терминах unwrap_types.
pub fn unfold_papercraft(mesh: &Mesh, cfg: &PaperUnfoldConfig) -> Result<UnwrapResult, String> {
    if mesh.faces.is_empty() {
        return Err("Mesh is empty".to_string());
    }

    let nv = mesh.vertices.len();
    let nf = mesh.faces.len();
    let scale = mesh_diag(mesh);
    let eps = (scale * 1e-9).max(1e-12);
    let cell = (scale / 8.0).max(1e-9);

    let pos: Vec<[f64; 3]> = mesh
        .vertices
        .iter()
        .map(|v| [v.x as f64, v.y as f64, v.z as f64])
        .collect();

    // Треугольники (первые 3 вершины грани; индексы проверяются).
    let mut tris: Vec<Option<[usize; 3]>> = Vec::with_capacity(nf);
    for f in &mesh.faces {
        if f.vertices.len() >= 3 && f.vertices.iter().all(|&v| v < nv) {
            tris.push(Some([f.vertices[0], f.vertices[1], f.vertices[2]]));
        } else {
            tris.push(None);
        }
    }

    // Нормали и центроиды.
    let mut normals = vec![[0.0f64; 3]; nf];
    let mut centroids = vec![[0.0f64; 3]; nf];
    for (fi, t) in tris.iter().enumerate() {
        if let Some([a, b, c]) = *t {
            let pa = pos[a];
            let pb = pos[b];
            let pc = pos[c];
            let n = cross3(sub3(pb, pa), sub3(pc, pa));
            let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            normals[fi] = if l > 1e-12 { [n[0] / l, n[1] / l, n[2] / l] } else { [0.0; 3] };
            centroids[fi] = [
                (pa[0] + pb[0] + pc[0]) / 3.0,
                (pa[1] + pb[1] + pc[1]) / 3.0,
                (pa[2] + pb[2] + pc[2]) / 3.0,
            ];
        }
    }

    // Карта рёбер: (min,max) -> грани.
    let mut edge_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (fi, t) in tris.iter().enumerate() {
        if let Some([a, b, c]) = *t {
            for &(p, q) in &[(a, b), (b, c), (c, a)] {
                let key = (p.min(q), p.max(q));
                edge_map.entry(key).or_default().push(fi);
            }
        }
    }

    let mut face_2d: Vec<Option<[[f64; 2]; 3]>> = vec![None; nf];
    let mut island_of_face: Vec<Option<usize>> = vec![None; nf];
    let mut unvisited: HashSet<usize> = (0..nf).filter(|&f| tris[f].is_some()).collect();
    let mut used_growth: HashSet<(usize, (usize, usize))> = HashSet::new();
    let mut islands: Vec<IslandData> = Vec::new();

    // ── Жадный рост островов ──
    while let Some(seed) = next_seed(&unvisited, &edge_map, &island_of_face, &tris) {
        let island_id = islands.len();
        let [a, b, c] = match tris[seed] {
            Some(t) => t,
            None => continue,
        };

        // Стартовая укладка: a -> (0,0), b -> (|ab|,0), c — трилатерация.
        let la = dist3(pos[a], pos[b]);
        let lc = dist3(pos[a], pos[c]);
        let lb = dist3(pos[b], pos[c]);
        let p0 = [0.0, 0.0];
        let p1 = [la, 0.0];
        let (cands, _) = trilaterate_both(p0, p1, lc, lb);
        let tri0 = [p0, p1, cands[0]];
        face_2d[seed] = Some(tri0);
        island_of_face[seed] = Some(island_id);
        unvisited.remove(&seed);

        let mut grid = IslandGrid::new(cell);
        grid.insert(&tri0, seed);

        let mut isl_faces = vec![seed];
        let mut isl_folds: Vec<FoldLineF> = Vec::new();
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(seed);

        while let Some(fi) = queue.pop_front() {
            if cfg.max_faces_per_island > 0 && isl_faces.len() >= cfg.max_faces_per_island {
                break;
            }
            let t_f = match face_2d[fi] {
                Some(t) => t,
                None => continue,
            };
            let f_verts = match tris[fi] {
                Some(v) => v,
                None => continue,
            };
            for e in 0..3usize {
                let (va, vb) = (f_verts[e], f_verts[(e + 1) % 3]);
                let key = (va.min(vb), va.max(vb));
                if used_growth.contains(&(fi, key)) {
                    continue;
                }
                let ep1 = t_f[e];
                let ep2 = t_f[(e + 1) % 3];
                let neighbors = match edge_map.get(&key) {
                    Some(v) => v.clone(),
                    None => continue,
                };
                for &nfi in &neighbors {
                    if nfi == fi || !unvisited.contains(&nfi) {
                        continue;
                    }
                    let n_verts = match tris[nfi] {
                        Some(v) => v,
                        None => continue,
                    };
                    let apex_slot = match (0..3).find(|&k| n_verts[k] != va && n_verts[k] != vb) {
                        Some(k) => k,
                        None => continue,
                    };
                    let apex = n_verts[apex_slot];
                    let r1 = dist3(pos[va], pos[apex]);
                    let r2 = dist3(pos[vb], pos[apex]);
                    let (cands, _) = trilaterate_both(ep1, ep2, r1, r2);
                    // Зеркалим: кандидат на противоположной стороне от апекса родителя.
                    let par_apex = t_f[(e + 2) % 3];
                    let s_par = cross2(sub2(ep2, ep1), sub2(par_apex, ep1));
                    let s_c0 = cross2(sub2(ep2, ep1), sub2(cands[0], ep1));
                    let cand = if (s_par > 0.0) != (s_c0 > 0.0) { cands[0] } else { cands[1] };
                    let mut tri2 = [[0.0f64; 2]; 3];
                    for k in 0..3 {
                        tri2[k] = if n_verts[k] == va {
                            ep1
                        } else if n_verts[k] == vb {
                            ep2
                        } else {
                            cand
                        };
                    }
                    // Отсечение перекрытий.
                    let mut blocked = false;
                    for &g in &grid.candidates(&tri2) {
                        if let Some(tg) = face_2d[g] {
                            if tri_tri_overlap(tri2, tg, eps) {
                                blocked = true;
                                break;
                            }
                        }
                    }
                    if blocked {
                        continue;
                    }
                    // Фиксация грани.
                    face_2d[nfi] = Some(tri2);
                    island_of_face[nfi] = Some(island_id);
                    unvisited.remove(&nfi);
                    grid.insert(&tri2, nfi);
                    used_growth.insert((fi, key));
                    used_growth.insert((nfi, key));
                    isl_faces.push(nfi);
                    // Fold line: реальный двугранный угол + выпуклость.
                    let d = dot3(normals[fi], normals[nfi]).clamp(-1.0, 1.0);
                    let bend = d.acos().to_degrees();
                    if bend >= cfg.fold_threshold_deg && bend > 1e-3 {
                        let convex = dot3(sub3(centroids[nfi], centroids[fi]), normals[fi]) > 0.0;
                        isl_folds.push(FoldLineF {
                            p1: ep1,
                            p2: ep2,
                            mountain: convex,
                            angle_deg: bend as f32,
                        });
                    }
                    queue.push_back(nfi);
                    break; // через одно ребро клеим не более одной грани
                }
            }
        }

        // ── Границы острова: копии рёбер, не совпавшие с копией соседа ──
        // (#32) boundary = edge with no same-island 3D neighbor
        let mut boundary: Vec<BoundaryEdgeRaw> = Vec::new();
        for &fi in &isl_faces {
            let t = match face_2d[fi] {
                Some(t) => t,
                None => continue,
            };
            let verts = match tris[fi] {
                Some(v) => v,
                None => continue,
            };
            for e in 0..3 {
                let (v1, v2) = (verts[e], verts[(e + 1) % 3]);
                let gk = (v1.min(v2), v1.max(v2));
                let mut interior = false;
                let mut other: Option<usize> = None;
                if let Some(list) = edge_map.get(&gk) {
                    for &g in list {
                        if g == fi {
                            continue;
                        }
                        if island_of_face[g] == Some(island_id) {
                            interior = true;
                            break;
                        }
                        if other.is_none() {
                            other = Some(g);
                        }
                    }
                }
                if interior {
                    continue;
                }
                boundary.push(BoundaryEdgeRaw {
                    p1: t[e],
                    p2: t[(e + 1) % 3],
                    face: fi,
                    edge: e,
                    v1,
                    v2,
                    other_face: other,
                });
            }
        }

        islands.push(IslandData {
            faces: isl_faces,
            folds: isl_folds,
            boundary,
            grid,
        });
    }

    // ── Glue tabs: рёбра шва между разными островами ──
    let mut total_tabs = 0u32;
    let mut skipped_tabs = 0u32;
    let mut island_tabs: Vec<Vec<TabRaw>> = Vec::with_capacity(islands.len());
    for _ in 0..islands.len() {
        island_tabs.push(Vec::new());
    }
    for (ii, isl) in islands.iter().enumerate() {
        for be in &isl.boundary {
            let other = match be.other_face {
                Some(o) => o,
                None => continue,
            };
            let other_island = match island_of_face[other] {
                Some(i) => i,
                None => continue,
            };
            if other_island == ii {
                continue;
            }
            let t = match face_2d[be.face] {
                Some(t) => t,
                None => continue,
            };
            let e = be.edge;
            let third = t[(e + 2) % 3];
            let dir = sub2(be.p2, be.p1);
            let elen = len2(dir);
            if elen < eps {
                continue;
            }
            let du = [dir[0] / elen, dir[1] / elen];
            let mut out = [-du[1], du[0]];
            if dot2(out, sub2(third, be.p1)) > 0.0 {
                out = [-out[0], -out[1]];
            }
            let h = cfg.tab_width.min(0.45 * elen);
            let flare = 0.6;
            let b1 = [
                be.p1[0] + out[0] * h + du[0] * h * flare,
                be.p1[1] + out[1] * h + du[1] * h * flare,
            ];
            let b2 = [
                be.p2[0] + out[0] * h - du[0] * h * flare,
                be.p2[1] + out[1] * h - du[1] * h * flare,
            ];
            let poly = [be.p1, be.p2, b2, b1];
            let t1 = [poly[0], poly[1], poly[2]];
            let t2 = [poly[0], poly[2], poly[3]];
            let mut collides = false;
            'outer: for tri_c in [&t1, &t2] {
                for g in isl.grid.candidates(tri_c) {
                    if g == be.face {
                        continue;
                    }
                    if let Some(tg) = face_2d[g] {
                        if tri_tri_overlap(*tri_c, tg, eps) {
                            collides = true;
                            break 'outer;
                        }
                    }
                }
            }
            if collides {
                skipped_tabs += 1;
                continue;
            }
            island_tabs[ii].push(TabRaw {
                poly,
                glue_to: (other_island + 1) as u32,
            });
            total_tabs += 1;
        }
    }

    // ── Упаковка на листы (shelf) + сборка pieces ──
    let inner_w = (cfg.sheet_width - 2.0 * cfg.margin).max(1.0);
    let inner_h = (cfg.sheet_height - 2.0 * cfg.margin).max(1.0);
    let mut pieces: Vec<Piece> = Vec::new();
    let mut page_area: HashMap<usize, f64> = HashMap::new();
    let mut page_ids: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut cursor_x = cfg.margin;
    let mut cursor_y = cfg.margin;
    let mut row_h = 0.0f64;
    let mut page_index = 0usize;

    for (ii, isl) in islands.iter().enumerate() {
        // Контур: цепочка граничных рёбер в петли.
        // (#32) outline chain via global vertex indices (no float matching)
        let mut loops: Vec<Vec<[f64; 2]>> = Vec::new();
        {
            let bnd = &isl.boundary;
            let mut used = vec![false; bnd.len()];
            let mut at_vtx: HashMap<usize, Vec<usize>> = HashMap::new();
            for (k, be) in bnd.iter().enumerate() {
                at_vtx.entry(be.v1).or_default().push(k);
                at_vtx.entry(be.v2).or_default().push(k);
            }
            for s in 0..bnd.len() {
                if used[s] {
                    continue;
                }
                used[s] = true;
                let start_v = bnd[s].v1;
                let mut pts = vec![bnd[s].p1, bnd[s].p2];
                let mut cur_v = bnd[s].v2;
                while cur_v != start_v {
                    let mut nxt: Option<usize> = None;
                    if let Some(cands) = at_vtx.get(&cur_v) {
                        for &k in cands {
                            if !used[k] {
                                nxt = Some(k);
                                break;
                            }
                        }
                    }
                    match nxt {
                        Some(k) => {
                            used[k] = true;
                            let be = &bnd[k];
                            if be.v1 == cur_v {
                                pts.push(be.p2);
                                cur_v = be.v2;
                            } else {
                                pts.push(be.p1);
                                cur_v = be.v1;
                            }
                        }
                        None => break,
                    }
                }
                if pts.len() >= 3 {
                    loops.push(pts);
                }
            }
        }
        loops.sort_by(|x, y| {
            let ax = poly_area_signed(x).abs();
            let ay = poly_area_signed(y).abs();
            ay.partial_cmp(&ax).unwrap_or(std::cmp::Ordering::Equal)
        });
        let outer = loops.first().cloned().unwrap_or_default();
        let holes: Vec<Vec<[f64; 2]>> = loops.iter().skip(1).cloned().collect();

        // Габариты (вместе с fold lines и табами).
        let mut mn = [f64::MAX; 2];
        let mut mx = [f64::MIN; 2];
        for pt in outer.iter().chain(holes.iter().flatten()) {
            acc_mm(*pt, &mut mn, &mut mx);
        }
        for fl in &isl.folds {
            acc_mm(fl.p1, &mut mn, &mut mx);
            acc_mm(fl.p2, &mut mn, &mut mx);
        }
        for tab in &island_tabs[ii] {
            for p in tab.poly.iter() {
                acc_mm(*p, &mut mn, &mut mx);
            }
        }
        let iw = (mx[0] - mn[0]).max(0.0);
        let ih = (mx[1] - mn[1]).max(0.0);

        // Shelf-укладка.
        if cursor_x > cfg.margin && cursor_x + iw > cfg.margin + inner_w {
            cursor_x = cfg.margin;
            cursor_y += row_h + cfg.margin;
            row_h = 0.0;
        }
        if cursor_y + ih > cfg.margin + inner_h {
            page_index += 1;
            cursor_x = cfg.margin;
            cursor_y = cfg.margin;
            row_h = 0.0;
        }
        let off = [cursor_x - mn[0], cursor_y - mn[1]];
        let tr_pt = |p: [f64; 2]| -> Point2D {
            Point2D { x: (p[0] + off[0]) as f32, y: (p[1] + off[1]) as f32 }
        };

        let outline_points: Vec<Point2D> = outer.iter().map(|p| tr_pt(*p)).collect();
        let outline_holes: Vec<Vec<Point2D>> = holes
            .iter()
            .map(|h| h.iter().map(|p| tr_pt(*p)).collect())
            .collect();
        let folds_dto: Vec<FoldLine> = isl
            .folds
            .iter()
            .map(|fl| FoldLine {
                start: tr_pt(fl.p1),
                end: tr_pt(fl.p2),
                fold_type: if fl.mountain { FoldType::Mountain } else { FoldType::Valley },
                angle: fl.angle_deg,
            })
            .collect();
        let tabs_dto: Vec<Tab> = island_tabs[ii]
            .iter()
            .map(|tb| Tab {
                poly: tb.poly.iter().map(|p| tr_pt(*p)).collect(),
                glue_to_piece_id: tb.glue_to as usize,
                glue_edge_index: 0,
            })
            .collect();

        let area_outer = poly_area_signed(&outer).abs();
        let area_holes: f64 = holes.iter().map(|h| poly_area_signed(h).abs()).sum();
        let area = ((area_outer - area_holes).max(0.0)) as f32;
        let perimeter = {
            let mut pe = 0.0f64;
            let n = outer.len();
            if n >= 2 {
                for i in 0..n {
                    let j = (i + 1) % n;
                    pe += len2(sub2(outer[j], outer[i]));
                }
            }
            pe as f32
        };

        let piece_id = ii + 1;
        pieces.push(Piece {
            id: piece_id as u32,
            name: format!("Piece_{}", piece_id),
            label: None,
            outline: Polygon { points: outline_points, holes: outline_holes },
            folds: folds_dto,
            tabs: tabs_dto,
            area,
            perimeter,
            page_number: (page_index + 1) as u32,
        });
        *page_area.entry(page_index).or_default() += area as f64;
        page_ids.entry(page_index).or_default().push(piece_id);

        cursor_x += iw + cfg.margin;
        if ih > row_h {
            row_h = ih;
        }
    }

    // ── Сборка UnwrapResult ──
    let mut vertices_2d = vec![[0.0f64, 0.0]; nv];
    let mut vset = vec![false; nv];
    for (fi, t) in face_2d.iter().enumerate() {
        if let Some(tri) = *t {
            let verts = &mesh.faces[fi].vertices;
            for k in 0..verts.len().min(3) {
                let g = verts[k];
                if !vset[g] {
                    vset[g] = true;
                    vertices_2d[g] = tri[k];
                }
            }
        }
    }

    let faces_out: Vec<Vec<usize>> = mesh.faces.iter().map(|f| f.vertices.clone()).collect();
    let face_pieces: Vec<u32> = island_of_face
        .iter()
        .map(|o| o.map(|i| (i + 1) as u32).unwrap_or(0))
        .collect();

    let total_area: f64 = pieces.iter().map(|p| p.area as f64).sum();
    let total_perimeter: f64 = pieces.iter().map(|p| p.perimeter as f64).sum();
    let total_folds: u32 = pieces.iter().map(|p| p.folds.len() as u32).sum();

    let sheet_area = (cfg.sheet_width * cfg.sheet_height).max(1e-9);
    let max_page = page_ids.keys().copied().max().unwrap_or(0);
    let mut pages: Vec<Page> = Vec::new();
    for pi in 0..=max_page {
        let ids = page_ids.remove(&pi).unwrap_or_default();
        let used = page_area.remove(&pi).unwrap_or(0.0);
        pages.push(Page {
            page_number: pi + 1,
            piece_ids: ids,
            utilization_percent: (used / sheet_area * 100.0) as f32,
        });
    }

    let n_pieces = pieces.len() as u32;
    let n_pages = pages.len() as u32;
    let _ = skipped_tabs;

    Ok(UnwrapResult {
        engine_used: "greedy_growth_v1".to_string(),
        vertices_2d,
        faces: faces_out,
        face_pieces,
        total_area,
        total_perimeter,
        pieces,
        pages,
        metrics: UnwrapMetrics {
            total_pieces: n_pieces,
            total_pages: n_pages,
            total_folds,
            total_tabs,
            complexity_score: (total_folds + total_tabs) as f32,
        },
        sheet_width: cfg.sheet_width as f32,
        sheet_height: cfg.sheet_height as f32,
        margin: cfg.margin as f32,
    })
}

/// Обёртка с конфигурацией по умолчанию и защитой от паник.
pub fn unfold_with_papercraft(mesh: &Mesh) -> Result<UnwrapResult, String> {
    let cfg = PaperUnfoldConfig::for_mesh(mesh);
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        unfold_papercraft(mesh, &cfg)
    }));
    match r {
        Ok(inner) => inner,
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "unknown panic".to_string()
            };
            Err(format!("Adapter panic: {}", msg))
        }
    }
}

// ── Тесты (cargo test -p pepakura_core --lib) ─────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import::parse_obj_str;

    const CUBE_OBJ: &str = "\
v -1 -1 -1
v 1 -1 -1
v 1 1 -1
v -1 1 -1
v -1 -1 1
v 1 -1 1
v 1 1 1
v -1 1 1
f 1 4 3
f 1 3 2
f 5 6 7
f 5 7 8
f 1 2 6
f 1 6 5
f 2 3 7
f 2 7 6
f 3 4 8
f 3 8 7
f 4 1 5
f 4 5 8
";

    const TETRA_OBJ: &str = "\
v 0 0 0
v 1 0 0
v 0 1 0
v 0 0 1
f 1 3 2
f 1 2 4
f 2 3 4
f 1 4 3
";

    #[test]
    fn empty_mesh_errors() {
        let mesh = crate::Mesh::new("empty");
        let cfg = PaperUnfoldConfig::for_mesh(&mesh);
        assert!(unfold_papercraft(&mesh, &cfg).is_err());
    }

    #[test]
    fn tetra_single_island() {
        let mesh = parse_obj_str(TETRA_OBJ).expect("tetra parse");
        assert_eq!(mesh.faces.len(), 4);
        let cfg = PaperUnfoldConfig::for_mesh(&mesh);
        let res = unfold_papercraft(&mesh, &cfg).expect("tetra unfold");
        assert_eq!(res.face_pieces.len(), 4);
        assert!(res.face_pieces.iter().all(|&p| p != 0), "все грани размещены");
        assert_eq!(res.pieces.len(), 1, "тетраэдр разворачивается одним островом");
        let folds: usize = res.pieces.iter().map(|p| p.folds.len()).sum();
        assert_eq!(folds, 3, "остов из 4 граней имеет 3 ребра роста");
        let tabs: usize = res.pieces.iter().map(|p| p.tabs.len()).sum();
        assert_eq!(tabs, 0, "у замкнутого одноостровного тетраэдра нет межостровных швов");
        let area: f64 = res.pieces.iter().map(|p| p.area as f64).sum();
        let expected = 1.5 + 3.0f64.sqrt() / 2.0;
        assert!((area - expected).abs() < 1e-3, "area = {}, expected {}", area, expected);
    }

    #[test]
    fn cube_unfolds_completely() {
        let mesh = parse_obj_str(CUBE_OBJ).expect("cube parse");
        assert_eq!(mesh.faces.len(), 12);
        let cfg = PaperUnfoldConfig::for_mesh(&mesh);
        let res = unfold_papercraft(&mesh, &cfg).expect("cube unfold");
        assert_eq!(res.face_pieces.len(), 12);
        assert!(res.face_pieces.iter().all(|&p| p != 0), "все 12 граней размещены");
        // Площадь развёртки == площади поверхности (6 * 2^2 = 24).
        let area: f64 = res.pieces.iter().map(|p| p.area as f64).sum();
        assert!((area - 24.0).abs() < 1e-3, "area = {}", area);
        // Лес остовных деревьев: fold lines <= грани - детали.
        let folds: usize = res.pieces.iter().map(|p| p.folds.len()).sum();
        assert!(
            folds <= 12 - res.pieces.len(),
            "folds = {}, pieces = {}",
            folds,
            res.pieces.len()
        );
        // Все углы сгиба в разумных пределах.
        for p in &res.pieces {
            for f in &p.folds {
                assert!(f.angle > 0.0 && f.angle < 180.0, "angle = {}", f.angle);
            }
        }
    }

    #[test]
    fn forced_split_produces_tabs() {
        let mesh = parse_obj_str(CUBE_OBJ).expect("cube parse");
        let cfg = PaperUnfoldConfig {
            max_faces_per_island: 4,
            ..PaperUnfoldConfig::for_mesh(&mesh)
        };
        let res = unfold_papercraft(&mesh, &cfg).expect("cube split unfold");
        assert!(res.pieces.len() >= 3, "pieces = {}", res.pieces.len());
        assert!(res.face_pieces.iter().all(|&p| p != 0), "все грани размещены");
        let tabs: usize = res.pieces.iter().map(|p| p.tabs.len()).sum();
        assert!(tabs > 0, "между островами должны появиться glue tabs");
        let area: f64 = res.pieces.iter().map(|p| p.area as f64).sum();
        assert!((area - 24.0).abs() < 1e-3, "area = {}", area);
    }
}

// ---- DIAG (#32): full cube unfold stats (temporary) ----
#[cfg(test)]
mod diag_tests {
    use super::*;
    use crate::import::parse_obj_str;

    const CUBE_OBJ_DIAG: &str = "\
v -1 -1 -1
v 1 -1 -1
v 1 1 -1
v -1 1 -1
v -1 -1 1
v 1 -1 1
v 1 1 1
v -1 1 1
f 1 4 3
f 1 3 2
f 5 6 7
f 5 7 8
f 1 2 6
f 1 6 5
f 2 3 7
f 2 7 6
f 3 4 8
f 3 8 7
f 4 1 5
f 4 5 8
";

    #[test]
    fn diag_cube_stats() {
        let mesh = parse_obj_str(CUBE_OBJ_DIAG).expect("cube parse");
        let cfg = PaperUnfoldConfig::for_mesh(&mesh);
        let res = unfold_papercraft(&mesh, &cfg).expect("unfold ok");
        println!("==== CUBE DIAG BEGIN ====");
        println!("engine: {}", res.engine_used);
        println!("pieces: {}", res.pieces.len());
        for p in &res.pieces {
            println!(
                "piece {} page {}: outline_pts={} holes={} folds={} tabs={} area={:.4} perim={:.4}",
                p.id,
                p.page_number,
                p.outline.points.len(),
                p.outline.holes.len(),
                p.folds.len(),
                p.tabs.len(),
                p.area,
                p.perimeter
            );
            for f in &p.folds {
                println!(
                    "  fold ({:.4},{:.4}) -> ({:.4},{:.4}) angle={:.1}",
                    f.start.x, f.start.y, f.end.x, f.end.y, f.angle
                );
            }
        }
        let total_area: f64 = res.pieces.iter().map(|p| p.area as f64).sum();
        let total_folds: usize = res.pieces.iter().map(|p| p.folds.len()).sum();
        let mut ids: Vec<u32> = res.face_pieces.clone();
        ids.sort();
        ids.dedup();
        println!("total_area={:.6} total_folds={}", total_area, total_folds);
        println!("piece ids: {:?}", ids);
        println!("==== CUBE DIAG END ====");
    }
}
