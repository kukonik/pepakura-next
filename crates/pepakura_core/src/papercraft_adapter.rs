// [MEM48-D] papercraft_adapter - реальная papercraft-развёртка (порт papercraft v2.12).
// Референсы G:\Dev\memory\src\paper\: model.rs (Edge{f0,f1,angle} - диэдр),
// craft.rs (FlapSide - односторонние клапаны, FlapGeom - трапеция, SoftHidden -
// почти-плоские рёбра не рисуются сгибами, face_to_face_edge_matrix - шарнир).
// Пайплайн: dihedral-веса dual-рёбер -> Kruskal max-forest (дерево = сгибы, остальное
// = резы) -> BFS-раскладка треугольников по шарниру -> контур + клеевые клапаны.
// [MEM48-D] против [MEM48-C]:
//   1. Позиции хранятся ПО ГРАНЯМ (face2d: грань -> 3 угла), а не по вершинам:
//      у реза две стороны с разными позициями, иначе треугольники искажаются.
//   2. Плоские рёбра (диэдр < FLAT_DEG) - в дереве, но НЕ сгибы (SoftHidden).
//   3. Клапаны односторонние (FlapSide, flap_double=false): один клапан на рез,
//      вторая сторона - гладкий рез.
// Контракт commands.rs (~L949) сохранён: unfold_with_papercraft(&Mesh) -> PapercraftUnfoldResult.
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::geometry::Mesh;
use crate::unwrap_types::UnwrapMetrics;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PapercraftPiece {
    pub id: u32,
    pub face_ids: Vec<u32>,
    pub outline_2d: Vec<[f64; 2]>,
    pub fold_lines: Vec<[[f64; 2]; 2]>,
    pub cut_lines: Vec<[[f64; 2]; 2]>,
    // [82-A] принадлежность части модели
    #[serde(default)]
    pub part_id: u32,
    #[serde(default)]
    pub part: String,
}

/// [82-A] Часть модели (голова/туловище/ноги/глаз...)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelPart {
    pub id: u32,
    pub name: String,
    /// "whole" | "material" | "auto" | "manual"
    pub source: String,
    pub face_count: usize,
}

/// [82-A] Настройки разбиения на части.
/// mode: "off" (как раньше, одной частью) | "materials" (по usemtl из OBJ)
/// | "auto" (рез по вогнутым складкам) | "manual" (ручные списки граней, задел).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartsConfig {
    pub mode: String,
    /// Порог вогнутости для "auto" в градусах (0..90)
    pub concave_deg: f64,
    /// Ручные части: списки индексов граней
    pub manual: Vec<Vec<u32>>,
}

impl PartsConfig {
    /// Разбор config.parts из JSON; отсутствует/битое -> режим "off".
    pub fn from_json(v: Option<&serde_json::Value>) -> PartsConfig {
        let mut cfg = PartsConfig { mode: "off".to_string(), concave_deg: 30.0, manual: Vec::new() };
        let obj = match v { Some(x) => x, None => return cfg };
        if let Some(m) = obj.get("mode").and_then(|x| x.as_str()) {
            if m == "materials" || m == "auto" || m == "manual" || m == "off" {
                cfg.mode = m.to_string();
            }
        }
        if let Some(a) = obj.get("concave_deg").and_then(|x| x.as_f64()) {
            if a > 0.0 && a < 90.0 { cfg.concave_deg = a; }
        }
        if let Some(list) = obj.get("manual").and_then(|x| x.as_array()) {
            for part in list {
                if let Some(arr) = part.as_array() {
                    let faces: Vec<u32> = arr.iter().filter_map(|x| x.as_u64().map(|n| n as u32)).collect();
                    if !faces.is_empty() { cfg.manual.push(faces); }
                }
            }
        }
        cfg
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PapercraftUnfoldResult {
    pub pieces: Vec<PapercraftPiece>,
    pub metrics: UnwrapMetrics,
    // [82-A] список частей (заполняется unfold_with_papercraft_parts)
    #[serde(default)]
    pub parts: Vec<ModelPart>,
    /// [PDF-C] грань -> id детали (плоская карта: раскладка и текстуры на фронте)
    #[serde(default)] pub face_pieces: Vec<u32>,
    /// [PDF-C] имя движка для статуса в UI
    #[serde(default)] pub engine_used: String,
    /// [TEX-B] плоские 2D-вершины (по 3 на грань; дубли на резах допустимы)
    #[serde(default)] pub vertices_2d: Vec<[f64; 2]>,
    /// [TEX-B] грань -> 3 индекса в vertices_2d (формат старого фронта)
    #[serde(default)] pub faces_2d: Vec<[u32; 3]>,
}

// ---------- векторные хелперы ----------

fn sub2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] { [a[0] - b[0], a[1] - b[1]] }
fn add2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] { [a[0] + b[0], a[1] + b[1]] }
fn scale2(v: [f64; 2], k: f64) -> [f64; 2] { [v[0] * k, v[1] * k] }
fn len2(v: [f64; 2]) -> f64 { (v[0] * v[0] + v[1] * v[1]).sqrt() }
fn dot2(a: [f64; 2], b: [f64; 2]) -> f64 { a[0] * b[0] + a[1] * b[1] }
fn sub3(a: [f64; 3], b: [f64; 3]) -> [f64; 3] { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn len3(v: [f64; 3]) -> f64 { (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt() }
fn dot3(a: [f64; 3], b: [f64; 3]) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }

/// Раскладка третьей вершины треугольника по шарниру (общее ребро v1-v2).
/// Эквивалент craft.rs face_to_face_edge_matrix: проекция грани B на плоскость
/// грани A с сохранением длин проекций и высоты.
fn unfold_hinge(v1_3d: [f64; 3], v2_3d: [f64; 3], v3_3d: [f64; 3], v1_2d: [f64; 2], v2_2d: [f64; 2], sign: f64) -> [f64; 2] {
    let u3 = sub3(v2_3d, v1_3d);
    let w3 = sub3(v3_3d, v1_3d);
    let u_len = len3(u3);
    if u_len < 1e-12 { return v1_2d; }
    let u_hat = [u3[0] / u_len, u3[1] / u_len, u3[2] / u_len];
    let proj = dot3(w3, u_hat);
    let perp = [w3[0] - u_hat[0] * proj, w3[1] - u_hat[1] * proj, w3[2] - u_hat[2] * proj];
    let h = len3(perp);
    let u2 = sub2(v2_2d, v1_2d);
    let u2_len = len2(u2);
    if u2_len < 1e-12 { return v1_2d; }
    let u2_hat = scale2(u2, 1.0 / u2_len);
    let n2 = [-u2_hat[1], u2_hat[0]];
    add2(v1_2d, add2(scale2(u2_hat, proj), scale2(n2, h * sign)))
}

fn face_normal(mesh: &Mesh, fi: usize) -> [f64; 3] {
    let f = &mesh.faces[fi];
    let v0 = mesh.vertices[f.vertices[0]].position;
    let v1 = mesh.vertices[f.vertices[1]].position;
    let v2 = mesh.vertices[f.vertices[2]].position;
    let a = sub3(v1, v0);
    let b = sub3(v2, v0);
    let c = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let l = len3(c);
    if l > 1e-12 { [c[0] / l, c[1] / l, c[2] / l] } else { [0.0, 0.0, 0.0] }
}

/// Вес dual-ребра: порт papercraft model.rs (Edge.angle - dihedral flatness).
/// Плоский стык и длинное ребро = высокий вес (не режем), острый сгиб = низкий (режем).
/// [SMART-61] Видимость грани: 1.0 = фронт (+Z), 0.0 = бок/тыл.
fn face_visibility(mesh: &Mesh, fi: usize) -> f64 {
    dot3(face_normal(mesh, fi), [0.0, 0.0, 1.0]).clamp(0.0, 1.0)
}

/// [SMART-61] Вес dual-ребра: ступени диэдральных углов как раньше;
/// +3.0 бонус видимости (разрезы уходят на задние/боковые грани, фронт остаётся цельным);
/// длинные рёбра и плоские стыки по-прежнему самые дорогие (не режем).
fn dual_edge_weight(mesh: &Mesh, f1: usize, f2: usize, va: usize, vb: usize) -> f64 {
    let n1 = face_normal(mesh, f1);
    let n2 = face_normal(mesh, f2);
    let cos = dot3(n1, n2).clamp(-1.0, 1.0);
    let deg = cos.acos().to_degrees();
    let w_dihedral = if deg < 10.0 { 10.0 } else if deg < 25.0 { 5.0 } else if deg < 45.0 { 2.0 } else if deg < 90.0 { 0.5 } else { 0.05 };
    let p1 = mesh.vertices[va].position;
    let p2 = mesh.vertices[vb].position;
    let mut w = w_dihedral + 0.05 * len3(sub3(p2, p1));
    let vis = 0.5 * (face_visibility(mesh, f1) + face_visibility(mesh, f2));
    w += 3.0 * vis;
    if signed_concavity(mesh, f1, f2, va, vb) < -0.05 { w -= 1.5; }
    w
}

/// Диэдральный угол между гранями в градусах (0 = плоско, ср. Edge.angle в model.rs).
fn edge_dihedral_deg(mesh: &Mesh, f1: usize, f2: usize) -> f64 {
    dot3(face_normal(mesh, f1), face_normal(mesh, f2)).clamp(-1.0, 1.0).acos().to_degrees()
}

/// Почти-плоские рёбра (порог = верхняя ступень веса в dual_edge_weight).
const FLAT_DEG: f64 = 10.0;
// [SMART-CUT] Печатаемый размер детали: остров дерева больше MAX_PIECE_FACES
// делится разрезом самого удобного ребра дерева (минимальный вес = самое
// острое место), пока каждая часть не влезет в лимит. Осколки меньше
// MIN_PIECE_FACES не образуются: кандидат обязан делить остров надвое.
const MAX_PIECE_FACES: usize = 250;
const MIN_PIECE_FACES: usize = 40;

fn split_oversized_islands(mesh: &Mesh, dual: &mut [ForestEdge]) {
    let nf = mesh.faces.len();
    if nf == 0 { return; }
    let mut cuts55 = 0usize;
    for _round in 0..nf {
        let mut uf = Uf::new(nf);
        for e in dual.iter() { if e.in_tree { uf.union(e.f1, e.f2); } }
        let mut comp_size: HashMap<usize, usize> = HashMap::new();
        for fi in 0..nf { *comp_size.entry(uf.find(fi)).or_insert(0) += 1; }
        let mut big_root = usize::MAX;
        let mut big_len = MAX_PIECE_FACES;
        for (r, s) in comp_size.iter() {
            if *s > big_len { big_len = *s; big_root = *r; }
        }
        if big_root == usize::MAX { return; }
        let mut tedges: Vec<(usize, usize, usize)> = Vec::new();
        for (idx, e) in dual.iter().enumerate() {
            if e.in_tree && uf.find(e.f1) == big_root { tedges.push((idx, e.f1, e.f2)); }
        }
        // [SPD-3] список смежности один раз на раунд: BFS за O(K) вместо скана tedges на каждом узле (было O(K^3) на раунд — источник ~385с)
        let mut adj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); nf];
        for &(idx, a, b) in tedges.iter() {
            adj[a].push((idx, b));
            adj[b].push((idx, a));
        }
        let side_size = |skip: usize, start: usize| -> usize {
            let mut seen = vec![false; nf];
            seen[start] = true;
            let mut q: VecDeque<usize> = VecDeque::new();
            q.push_back(start);
            let mut cnt = 1usize;
            while let Some(cur) = q.pop_front() {
                for &(eidx, nb) in adj[cur].iter() {
                    if eidx == skip { continue; }
                    if !seen[nb] { seen[nb] = true; cnt += 1; q.push_back(nb); }
                }
            }
            cnt
        };
        let mut best: Option<(f64, usize)> = None;
        for &(idx, f1, _f2) in tedges.iter() {
            let s1 = side_size(idx, f1);
            let s2 = big_len - s1;
            if s1 >= MIN_PIECE_FACES && s2 >= MIN_PIECE_FACES {
                let w = dual[idx].weight;
                if best.map_or(true, |(bw, _)| w < bw) { best = Some((w, idx)); }
            }
        }
        match best {
            Some((_w, idx)) => { dual[idx].in_tree = false; cuts55 += 1; }
            None => { println!("[SPD-3] split: разрезов {}, граней {}", cuts55, nf); return; }
        }
    }
    println!("[SPD-3] split: разрезов {}, граней {}", cuts55, nf);
}

struct ForestEdge {
    weight: f64,
    f1: usize,
    f2: usize,
    va: usize,
    vb: usize,
    in_tree: bool,
}

struct Uf { parent: Vec<usize> }

impl Uf {
    fn new(n: usize) -> Self { Uf { parent: (0..n).collect() } }
    fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while self.parent[root] != root { root = self.parent[root]; }
        while self.parent[i] != root { let next = self.parent[i]; self.parent[i] = root; i = next; }
        root
    }
    fn union(&mut self, i: usize, j: usize) -> bool {
        let ri = self.find(i);
        let rj = self.find(j);
        if ri == rj { return false; }
        self.parent[ri] = rj;
        true
    }
}

/// Точка входа papercraft-развёртки (контракт src-tauri\src\commands.rs ~L949).
pub fn unfold_with_papercraft(mesh: &Mesh) -> Result<PapercraftUnfoldResult, String> {
    if mesh.vertices.is_empty() || mesh.faces.is_empty() {
        return Err("[MEM48-D] пустой меш: нет вершин или граней".to_string());
    }
    for (i, f) in mesh.faces.iter().enumerate() {
        if f.vertices.iter().any(|&v| v >= mesh.vertices.len()) {
            return Err(format!("[MEM48-D] грань {} ссылается на несуществующую вершину", i));
        }
    }

    // 1. Dual-граф: общие рёбра треугольников + dihedral-веса + множество плоских рёбер.
    let mut edge_face: HashMap<(usize, usize), usize> = HashMap::new();
    let mut dual: Vec<ForestEdge> = Vec::new();
    let mut flat_edge: HashSet<(usize, usize)> = HashSet::new();
    for (fi, face) in mesh.faces.iter().enumerate() {
        for k in 0..3 {
            let a = face.vertices[k];
            let b = face.vertices[(k + 1) % 3];
            let key = if a < b { (a, b) } else { (b, a) };
            if let Some(&fj) = edge_face.get(&key) {
                if edge_dihedral_deg(mesh, fi, fj) < FLAT_DEG { flat_edge.insert(key); }
                let w = dual_edge_weight(mesh, fi, fj, key.0, key.1);
                dual.push(ForestEdge { weight: w, f1: fj, f2: fi, va: key.0, vb: key.1, in_tree: false });
            } else {
                edge_face.insert(key, fi);
            }
        }
    }

    // 2. Kruskal max-weight forest: дерево = каркас детали, рёбра дерева = сгибы.
    dual.sort_by(|x, y| y.weight.partial_cmp(&x.weight).unwrap_or(std::cmp::Ordering::Equal));
    let mut uf = Uf::new(mesh.faces.len());
    for e in dual.iter_mut() {
        if uf.union(e.f1, e.f2) { e.in_tree = true; }
    }
    // [SMART-CUT] крупные острова делим на печатабельные куски
    split_oversized_islands(mesh, &mut dual);
    uf = Uf::new(mesh.faces.len());
    for e in dual.iter() {
        if e.in_tree { uf.union(e.f1, e.f2); }
    }

    // 3. Компоненты леса = острова (детали).
    let mut comp_map: HashMap<usize, usize> = HashMap::new();
    let mut islands: Vec<Vec<usize>> = Vec::new();
    for fi in 0..mesh.faces.len() {
        let r = uf.find(fi);
        let next = islands.len();
        let id = *comp_map.entry(r).or_insert(next);
        if id == next { islands.push(Vec::new()); }
        islands[id].push(fi);
    }

    let mut tree_adj: Vec<Vec<(usize, usize, usize)>> = vec![Vec::new(); mesh.faces.len()];
    let mut tree_edge_set: HashSet<(usize, usize)> = HashSet::new();
    for e in &dual {
        if e.in_tree {
            tree_adj[e.f1].push((e.f2, e.va, e.vb));
            tree_adj[e.f2].push((e.f1, e.va, e.vb));
            let key = if e.va < e.vb { (e.va, e.vb) } else { (e.vb, e.va) };
            tree_edge_set.insert(key);
        }
    }

    // 4. Раскладка каждого острова.
    let mut pieces: Vec<PapercraftPiece> = Vec::new();
    let mut total_folds: u32 = 0;
    let mut total_tabs: u32 = 0;
    let mut faces_2d: Vec<[u32; 3]> = vec![[0, 0, 0]; mesh.faces.len()];
    let mut verts_2d: Vec<[f64; 2]> = Vec::new();
    for (island_idx, faces) in islands.iter().enumerate() {
        let (piece, tabs, face2d_isl) = unfold_island(mesh, faces, island_idx as u32, &tree_edge_set, &flat_edge, &tree_adj);
        for (ff, tri) in &face2d_isl { if *ff < faces_2d.len() { let base = verts_2d.len() as u32; verts_2d.push(tri[0]); verts_2d.push(tri[1]); verts_2d.push(tri[2]); faces_2d[*ff] = [base, base + 1, base + 2]; } }
        total_folds += piece.fold_lines.len() as u32;
        total_tabs += tabs as u32;
        pieces.push(piece);
    }

    let complexity = (pieces.len() as f32) * 2.0 + (total_folds as f32) * 0.5 + (total_tabs as f32) * 0.1;
    let metrics = UnwrapMetrics {
        total_pieces: pieces.len() as u32,
        total_pages: 0,
        total_folds,
        total_tabs,
        complexity_score: complexity.min(100.0),
    };
    let mut face_pieces: Vec<u32> = vec![0; mesh.faces.len()];
    for (ii, faces) in islands.iter().enumerate() { for &f in faces { if f < face_pieces.len() { face_pieces[f] = ii as u32; } } }
    Ok(PapercraftUnfoldResult { pieces, metrics, parts: Vec::new(), face_pieces, vertices_2d: verts_2d, faces_2d, engine_used: "MST-Seam+61 (papercraft)".to_string() })
}

fn unfold_island(
    mesh: &Mesh,
    island_faces: &[usize],
    piece_id: u32,
    tree_edge_set: &HashSet<(usize, usize)>,
    flat_edge: &HashSet<(usize, usize)>,
    tree_adj: &[Vec<(usize, usize, usize)>],
) -> (PapercraftPiece, usize, HashMap<usize, [[f64; 2]; 3]>) {
    // 2D-позиции храним ПО ГРАНЯМ (3 угла каждой грани в своей рамке), а не по
    // вершинам: у реза две стороны, их концы обязаны расходиться в развёртке.
    let mut face2d: HashMap<usize, [[f64; 2]; 3]> = HashMap::new();

    // 4a. Корневая грань: v0 в (0,0), ребро v0->v1 вдоль +X, v2 по шарниру.
    let root = island_faces[0];
    let rf = &mesh.faces[root];
    let (r0, r1, r2) = (rf.vertices[0], rf.vertices[1], rf.vertices[2]);
    let base = len3(sub3(mesh.vertices[r1].position, mesh.vertices[r0].position));
    let r2p = unfold_hinge(mesh.vertices[r0].position, mesh.vertices[r1].position, mesh.vertices[r2].position, [0.0, 0.0], [base, 0.0], 1.0);
    face2d.insert(root, [[0.0, 0.0], [base, 0.0], r2p]);

    // 4b. BFS по дереву: каждая новая грань ложится шарниром на уже разложенное ребро.
    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(root);
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(root);
    while let Some(cur) = queue.pop_front() {
        let cpos = *face2d.get(&cur).unwrap_or(&[[0.0, 0.0]; 3]);
        let cf = &mesh.faces[cur];
        for &(nb, va, vb) in &tree_adj[cur] {
            if visited.contains(&nb) { continue; }
            visited.insert(nb);
            let ia = cf.vertices.iter().position(|&v| v == va).unwrap_or(0);
            let ib = cf.vertices.iter().position(|&v| v == vb).unwrap_or(1);
            let a2 = cpos[ia];
            let b2 = cpos[ib];
            let nf = &mesh.faces[nb];
            let vc = *nf.vertices.iter().find(|&&v| v != va && v != vb).unwrap_or(&va);
            let a3 = mesh.vertices[va].position;
            let b3 = mesh.vertices[vb].position;
            let c3 = mesh.vertices[vc].position;
            let ja = nf.vertices.iter().position(|&v| v == va).unwrap_or(0);
            let jb = nf.vertices.iter().position(|&v| v == vb).unwrap_or(1);
            let jc = nf.vertices.iter().position(|&v| v == vc).unwrap_or(2);
            // [MEM48-E] Знак разворота: третья вершина кладётся слева от направления
            // шарнира В ОБХОДЕ НОВОЙ грани (все грани выходят CCW, как корень).
            // Старый код всегда клал слева от va->vb, поэтому каждая вторая грань
            // зеркалилась через шарнир и развёртка самоперекрывалась
            // (на кубе outline area 4 != 6.0).
            let sign = if jb == (ja + 1) % 3 { 1.0 } else { -1.0 };
            let c2 = unfold_hinge(a3, b3, c3, a2, b2, sign);
            let mut npos = [[0.0, 0.0]; 3];
            npos[ja] = a2;
            npos[jb] = b2;
            npos[jc] = c2;
            face2d.insert(nb, npos);
            queue.push_back(nb);
        }
    }

    let face_pos = |fi: usize, v: usize| -> [f64; 2] {
        let p = *face2d.get(&fi).unwrap_or(&[[0.0, 0.0]; 3]);
        let f = &mesh.faces[fi];
        let i = f.vertices.iter().position(|&x| x == v).unwrap_or(0);
        p[i]
    };

    // 4c. Сгибы = рёбра дерева внутри острова, КРОМЕ плоских (SoftHidden в craft.rs).
    let mut fold_lines: Vec<[[f64; 2]; 2]> = Vec::new();
    let mut fold_seen: HashSet<(usize, usize)> = HashSet::new();
    for &fi in island_faces {
        let f = &mesh.faces[fi];
        for k in 0..3 {
            let a = f.vertices[k];
            let b = f.vertices[(k + 1) % 3];
            let key = if a < b { (a, b) } else { (b, a) };
            if tree_edge_set.contains(&key) && !flat_edge.contains(&key) && fold_seen.insert(key) {
                fold_lines.push([face_pos(fi, a), face_pos(fi, b)]);
            }
        }
    }

    // 4d. Граница = направленные рёбра граней острова, не входящие в дерево
    // (внешний контур + обе стороны внутренних резов), с указанием грани-владельца.
    let mut boundary: Vec<(usize, usize, usize)> = Vec::new(); // (грань, a, b)
    for &fi in island_faces {
        let f = &mesh.faces[fi];
        for k in 0..3 {
            let a = f.vertices[k];
            let b = f.vertices[(k + 1) % 3];
            let key = if a < b { (a, b) } else { (b, a) };
            if !tree_edge_set.contains(&key) { boundary.push((fi, a, b)); }
        }
    }

    // 4e. Внешний контур: обход направленных рёбер границы по 2D-ПОЗИЦИЯМ,
    // а не по id вершин. Все грани уложены CCW (знак в unfold_hinge), поэтому
    // у каждого граничного ребра внутренность слева; на "щипковых" вершинах,
    // где контур касается себя, продолжение выбирается самым крутым поворотом
    // ПО часовой стрелке - это трассировка внешней грани: одна петля покрывает
    // ВСЮ границу (восьмёрка на щипке), площадь по шнурку = площадь детали.
    let mut outline_2d: Vec<[f64; 2]> = Vec::new();
    {
        let mut segs: Vec<([f64; 2], [f64; 2])> = Vec::new();
        for &(fi, a, b) in &boundary {
            let pa = face_pos(fi, a);
            let pb = face_pos(fi, b);
            if len2(sub2(pb, pa)) > 1e-12 { segs.push((pa, pb)); }
        }
        let key = |p: [f64; 2]| -> (i64, i64) {
            ((p[0] / 1e-6).round() as i64, (p[1] / 1e-6).round() as i64)
        };
        let mut by_start: HashMap<(i64, i64), Vec<usize>> = HashMap::new();
        for (i, s) in segs.iter().enumerate() {
            by_start.entry(key(s.0)).or_default().push(i);
        }
        let mut used = vec![false; segs.len()];
        let mut best_area = 0.0f64;
        for start in 0..segs.len() {
            if used[start] { continue; }
            let mut poly: Vec<[f64; 2]> = Vec::new();
            let mut cur = start;
            let mut guard = 0usize;
            while guard <= segs.len() {
                guard += 1;
                used[cur] = true;
                poly.push(segs[cur].0);
                let din = sub2(segs[cur].1, segs[cur].0);
                let mut nxt: Option<usize> = None;
                let mut best_t = 0.0f64;
                if let Some(cands) = by_start.get(&key(segs[cur].1)) {
                    for &j in cands {
                        if used[j] { continue; }
                        let dj = sub2(segs[j].1, segs[j].0);
                        let t = (din[0] * dj[1] - din[1] * dj[0]).atan2(dot2(din, dj));
                        if nxt.is_none() || t < best_t { best_t = t; nxt = Some(j); }
                    }
                }
                match nxt {
                    Some(j) => { cur = j; }
                    None => { break; }
                }
            }
            let mut sa = 0.0f64;
            let n = poly.len();
            for i in 0..n {
                let q = (i + 1) % n;
                sa += poly[i][0] * poly[q][1] - poly[q][0] * poly[i][1];
            }
            if sa.abs() > best_area { best_area = sa.abs(); outline_2d = poly; }
        }
    }

    // 4f. Клапаны (FlapSide/flap_double=false: ОДИН клапан на рез, трапеция наружу
    // от центроида; вторая сторона реза остаётся гладкой).
    let mut cut_lines: Vec<[[f64; 2]; 2]> = Vec::new();
    let mut tabbed: HashSet<(usize, usize)> = HashSet::new();
    let mut tabs = 0usize;
    let mut cx = 0.0;
    let mut cy = 0.0;
    let mut cn = 0usize;
    for &fi in island_faces {
        if let Some(p) = face2d.get(&fi) {
            for c in p.iter() { cx += c[0]; cy += c[1]; cn += 1; }
        }
    }
    let centroid = if cn == 0 { [0.0, 0.0] } else { [cx / cn as f64, cy / cn as f64] };

    let mut avg_len = 0.0;
    for &(fi, a, b) in &boundary {
        let pa = face_pos(fi, a);
        let pb = face_pos(fi, b);
        avg_len += len2(sub2(pb, pa));
    }
    let avg_len = if boundary.is_empty() { 0.0 } else { avg_len / boundary.len() as f64 };
    let tab_default = 0.3 * avg_len;

    // [FIX60B] митры по углам границы: у вершины стыка двух граничных рёбер
    // внешний угол ω на стороне клапана; при ω < 180° верх трапеции отодвигается
    // от вершины на t >= tw/tan(ω/2) — иначе соседние клапаны накладываются.
    let mut vtx_edges: HashMap<usize, Vec<usize>> = HashMap::new();
    for (ei, &(_, va, vb)) in boundary.iter().enumerate() {
        vtx_edges.entry(va).or_insert_with(Vec::new).push(ei);
        vtx_edges.entry(vb).or_insert_with(Vec::new).push(ei);
    }
    for (ei, &(fi, a, b)) in boundary.iter().enumerate() {
        let pa = face_pos(fi, a);
        let pb = face_pos(fi, b);
        let elen = len2(sub2(pb, pa));
        if elen < 1e-9 { cut_lines.push([pa, pb]); continue; }
        let key = if a < b { (a, b) } else { (b, a) };
        if tabbed.contains(&key) { cut_lines.push([pa, pb]); continue; }
        tabbed.insert(key);
        let d = scale2(sub2(pb, pa), 1.0 / elen);
        let mut n = [d[1], -d[0]];
        let mid = scale2(add2(pa, pb), 0.5);
        if dot2(sub2(mid, centroid), n) < 0.0 { n = [-n[0], -n[1]]; }
        let mut tw = tab_default.min(0.45 * elen);
        let inset = 0.3 * tw;
        let mut t_fit = [inset, inset];
        let mut oms = [std::f64::consts::PI, std::f64::consts::PI];
        let ends = [(a, pa, 0usize), (b, pb, 1usize)];
        for &(v, p_v, k) in ends.iter() {
            let evs = match vtx_edges.get(&v) { Some(x) => x, None => continue };
            if evs.len() != 2 { continue; }
            let oi = if evs[0] == ei { evs[1] } else { evs[0] };
            let (fj, c1, c2) = boundary[oi];
            let v2 = if c1 == v { c2 } else { c1 };
            let p2 = face_pos(fj, v2);
            let dv = sub2(p2, p_v);
            let lv = len2(dv);
            if lv < 1e-9 { continue; }
            let d2 = scale2(dv, 1.0 / lv);
            let d1 = if v == a { d } else { scale2(d, -1.0) };
            let cr = d1[0] * d2[1] - d1[1] * d2[0];
            let cs = dot2(d1, d2);
            let sgn = d[0] * n[1] - d[1] * n[0];
            let ang = cr.atan2(cs);
            let tau = std::f64::consts::TAU;
            let om = if sgn < 0.0 { if ang < 0.0 { -ang } else { tau - ang } } else { if ang > 0.0 { ang } else { tau + ang } };
            oms[k] = om;
            if om < std::f64::consts::PI && om > 1e-6 {
                let need = tw / (om * 0.5).tan();
                if need > t_fit[k] { t_fit[k] = need.min(0.45 * elen); }
            }
        }
        for k in 0..2usize {
            if oms[k] < std::f64::consts::PI && t_fit[k] >= 0.45 * elen - 1e-9 {
                let tw2 = t_fit[k] * (oms[k] * 0.5).tan() * 0.95;
                if tw2 < tw { tw = tw2; }
            }
        }
        let top1 = add2(add2(pa, scale2(n, tw)), scale2(d, t_fit[0]));
        let top2 = add2(add2(pb, scale2(n, tw)), scale2(d, -t_fit[1]));
        cut_lines.push([pa, top1]);
        cut_lines.push([top1, top2]);
        cut_lines.push([top2, pb]);
        tabs += 1;
    }

    // 4g. Контур построен в 4e (обход границы по 2D-позициям с выбором
    // поворота) - здесь остаётся только сборка piece.

    let face_ids: Vec<u32> = island_faces.iter().map(|&f| f as u32).collect();
    let piece = PapercraftPiece { id: piece_id, face_ids, outline_2d, fold_lines, cut_lines, part_id: 0, part: String::new() }; // [82-A] part заполнит unfold_with_papercraft_parts
    (piece, tabs, face2d)
}

// ==================== [82-A] части модели ====================
// Вариант 1: части = материалы OBJ (usemtl). Вариант 2: авто-разрез по
// вогнутым складкам (шея/подмышки/пах) и смене материала. Вариант 3 (задел):
// ручные списки граней через PartsConfig.manual.

/// Направление ребра (va,vb) в обходе грани fi: vb-va, если грань идёт va->vb,
/// иначе va-vb. Нужен для ЗНАКА вогнутости.
fn edge_dir_in_face(mesh: &Mesh, fi: usize, va: usize, vb: usize) -> [f64; 3] {
    let f = &mesh.faces[fi];
    for k in 0..3 {
        if f.vertices[k] == va && f.vertices[(k + 1) % 3] == vb {
            return sub3(mesh.vertices[vb].position, mesh.vertices[va].position);
        }
    }
    sub3(mesh.vertices[va].position, mesh.vertices[vb].position)
}

/// Знаковая вогнутость ребра: >0 выпуклое, <0 вогнутое (модуль ~ sin угла складки).
/// Знак инвариантен к тому, какую из двух граней считать первой.
fn signed_concavity(mesh: &Mesh, f1: usize, f2: usize, va: usize, vb: usize) -> f64 {
    let n1 = face_normal(mesh, f1);
    let n2 = face_normal(mesh, f2);
    let d = edge_dir_in_face(mesh, f1, va, vb);
    let l = len3(d);
    if l < 1e-12 { return 0.0; }
    let cx = n1[1] * n2[2] - n1[2] * n2[1];
    let cy = n1[2] * n2[0] - n1[0] * n2[2];
    let cz = n1[0] * n2[1] - n1[1] * n2[0];
    (cx * d[0] + cy * d[1] + cz * d[2]) / l
}

/// Авто-сегментация: грани делятся на части разрезами по вогнутым складкам
/// (складка внутрь глубже concave_deg) и по смене материала. Обрывки <3 граней
/// присоединяются к соседней части.
fn segment_auto(mesh: &Mesh, concave_deg: f64) -> Vec<Vec<usize>> {
    let nf = mesh.faces.len();
    let sin_th = concave_deg.to_radians().sin();
    let mut edge_face: HashMap<(usize, usize), usize> = HashMap::new();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); nf];
    for fi in 0..nf {
        let face = &mesh.faces[fi];
        for k in 0..3 {
            let a = face.vertices[k];
            let b = face.vertices[(k + 1) % 3];
            let key = if a < b { (a, b) } else { (b, a) };
            if let Some(&fj) = edge_face.get(&key) {
                let mut do_cut = signed_concavity(mesh, fj, fi, key.0, key.1) < -sin_th;
                if let (Some(x), Some(y)) = (mesh.faces[fj].material_id, mesh.faces[fi].material_id) {
                    if x != y { do_cut = true; }
                }
                if !do_cut {
                    adj[fj].push(fi);
                    adj[fi].push(fj);
                }
            } else {
                edge_face.insert(key, fi);
            }
        }
    }
    // компоненты связности без разрезов = черновые части
    let mut comp_of = vec![usize::MAX; nf];
    let mut comps: Vec<Vec<usize>> = Vec::new();
    for start in 0..nf {
        if comp_of[start] != usize::MAX { continue; }
        let id = comps.len();
        let mut stack = vec![start];
        comp_of[start] = id;
        let mut members: Vec<usize> = Vec::new();
        while let Some(cur) = stack.pop() {
            members.push(cur);
            for &nb in &adj[cur] {
                if comp_of[nb] == usize::MAX {
                    comp_of[nb] = id;
                    stack.push(nb);
                }
            }
        }
        comps.push(members);
    }
    // обрывки (<3 граней) вливаем в наибольшего соседа
    let mut alive: Vec<bool> = vec![true; comps.len()];
    for i in 0..comps.len() {
        if comps[i].len() >= 3 || !alive[i] { continue; }
        let mut best: Option<usize> = None;
        let mut best_n = 0usize;
        for &f in &comps[i] {
            for &nb in &adj[f] {
                let cj = comp_of[nb];
                if cj == i || cj >= comps.len() || !alive[cj] { continue; }
                if comps[cj].len() > best_n { best_n = comps[cj].len(); best = Some(cj); }
            }
        }
        if let Some(j) = best {
            let frag: Vec<usize> = comps[i].clone();
            comps[j].extend(frag);
            alive[i] = false;
        }
    }
    let mut parts: Vec<Vec<usize>> = Vec::new();
    for i in 0..comps.len() {
        if alive[i] && !comps[i].is_empty() { parts.push(comps[i].clone()); }
    }
    parts.sort_by(|x, y| y.len().partial_cmp(&x.len()).unwrap_or(std::cmp::Ordering::Equal));
    parts
}

/// [82-A] Развёртка по частям. Каждая часть разворачивается независимо,
/// piece получает part_id/part, face_ids переводятся в индексы исходного меша.
pub fn unfold_with_papercraft_parts(mesh: &Mesh, cfg: &PartsConfig) -> Result<PapercraftUnfoldResult, String> {
    let nf = mesh.faces.len();
    if mesh.vertices.is_empty() || nf == 0 {
        return Err("[82-A] пустой меш: нет вершин или граней".to_string());
    }
    let mut face_lists: Vec<Vec<usize>> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let mut source = "whole".to_string();
    match cfg.mode.as_str() {
        "materials" => {
            source = "material".to_string();
            let mut by_mat: HashMap<usize, Vec<usize>> = HashMap::new();
            let mut none_faces: Vec<usize> = Vec::new();
            for (fi, f) in mesh.faces.iter().enumerate() {
                match f.material_id {
                    Some(m) => { by_mat.entry(m).or_default().push(fi); }
                    None => { none_faces.push(fi); }
                }
            }
            if by_mat.is_empty() {
                face_lists.push((0..nf).collect());
                names.push("Модель".to_string());
                source = "whole".to_string();
            } else {
                let mut keys: Vec<usize> = by_mat.keys().cloned().collect();
                keys.sort();
                for m in keys {
                    face_lists.push(by_mat[&m].clone());
                    let nm = mesh.metadata.material_names.get(m).cloned().unwrap_or_else(|| format!("Материал {}", m));
                    names.push(nm);
                }
                if !none_faces.is_empty() {
                    face_lists.push(none_faces);
                    names.push("Без материала".to_string());
                }
            }
        }
        "auto" => {
            source = "auto".to_string();
            let seg = segment_auto(mesh, cfg.concave_deg);
            if seg.len() > 1 {
                for (i, s) in seg.iter().enumerate() {
                    face_lists.push(s.clone());
                    names.push(format!("Часть {}", i + 1));
                }
            } else {
                // складок не нашли - вся модель одной частью (как раньше)
                face_lists.push((0..nf).collect());
                names.push("Модель".to_string());
                source = "whole".to_string();
            }
        }
        "manual" => {
            // [82-A] задел под вариант 3: пользователь сам размечает части
            source = "manual".to_string();
            for (i, list) in cfg.manual.iter().enumerate() {
                let faces: Vec<usize> = list.iter().filter(|&&f| (f as usize) < nf).map(|&f| f as usize).collect();
                if faces.is_empty() { continue; }
                face_lists.push(faces);
                names.push(format!("Ручная {}", i + 1));
            }
            if face_lists.is_empty() {
                return Err("[82-A] manual: нет валидных списков граней".to_string());
            }
        }
        _ => {
            face_lists.push((0..nf).collect());
            names.push("Модель".to_string());
        }
    }

    let mut pieces: Vec<PapercraftPiece> = Vec::new();
    let mut parts_meta: Vec<ModelPart> = Vec::new();
    let mut total_folds = 0u32;
    let mut total_tabs = 0u32;
    let mut faces_2d: Vec<[u32; 3]> = vec![[0, 0, 0]; mesh.faces.len()];
    let mut verts_2d: Vec<[f64; 2]> = Vec::new();
    for (pi, faces) in face_lists.iter().enumerate() {
        if faces.is_empty() { continue; }
        let mut sub = Mesh::new(&names[pi]);
        for v in &mesh.vertices { sub.add_vertex(v.clone()); }
        let mut orig_of_sub: Vec<usize> = Vec::with_capacity(faces.len());
        for &fi in faces {
            sub.add_face(mesh.faces[fi].clone());
            orig_of_sub.push(fi);
        }
        let res = unfold_with_papercraft(&sub)?;
        total_folds += res.metrics.total_folds;
        total_tabs += res.metrics.total_tabs;
        let off = verts_2d.len() as u32;
        for p in &res.vertices_2d { verts_2d.push(*p); }
        for (ff, tri) in res.faces_2d.iter().enumerate() {
            if ff < orig_of_sub.len() && !(tri[0] == tri[1] && tri[1] == tri[2]) {
                let of = orig_of_sub[ff];
                if of < faces_2d.len() { faces_2d[of] = [off + tri[0], off + tri[1], off + tri[2]]; }
            }
        }
        for mut piece in res.pieces {
            piece.face_ids = piece.face_ids.iter().map(|&f| orig_of_sub[f as usize] as u32).collect();
            piece.part_id = pi as u32;
            piece.part = names[pi].clone();
            piece.id = pieces.len() as u32;
            pieces.push(piece);
        }
        parts_meta.push(ModelPart { id: pi as u32, name: names[pi].clone(), source: source.clone(), face_count: faces.len() });
    }

    let complexity = (pieces.len() as f32) * 2.0 + (total_folds as f32) * 0.5 + (total_tabs as f32) * 0.1;
    let metrics = UnwrapMetrics {
        total_pieces: pieces.len() as u32,
        total_pages: 0,
        total_folds,
        total_tabs,
        complexity_score: complexity.min(100.0),
    };
    let mut face_pieces: Vec<u32> = vec![0; nf];
    for p in &pieces { for &f in &p.face_ids { if (f as usize) < nf { face_pieces[f as usize] = p.id; } } }
    Ok(PapercraftUnfoldResult { pieces, metrics, parts: parts_meta, face_pieces, vertices_2d: verts_2d, faces_2d, engine_used: format!("MST-Seam+61 + {} parts", cfg.mode) })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Face, Mesh, Vertex};

    fn unit_cube() -> Mesh {
        let mut mesh = Mesh::new("Cube");
        let vs = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0, 1.0]];
        for (i, &p) in vs.iter().enumerate() { mesh.add_vertex(Vertex::new(i, p)); }
        let fs = [[0, 1, 2], [0, 2, 3], [4, 6, 5], [4, 7, 6], [0, 5, 1], [0, 4, 5], [1, 6, 2], [1, 5, 6], [2, 7, 3], [2, 6, 7], [3, 4, 0], [3, 7, 4]];
        for &[a, b, c] in &fs { mesh.add_face(Face::new(a, b, c)); }
        mesh
    }

    fn outline_area(pts: &[[f64; 2]]) -> f64 {
        let n = pts.len();
        let mut s = 0.0;
        for i in 0..n {
            let j = (i + 1) % n;
            s += pts[i][0] * pts[j][1] - pts[j][0] * pts[i][1];
        }
        (s * 0.5).abs()
    }

    #[test]
    fn test_papercraft_empty_mesh_rejected() {
        let mesh = Mesh::new("Empty");
        assert!(unfold_with_papercraft(&mesh).is_err());
    }

    #[test]
    fn test_papercraft_cube_single_piece() {
        let mesh = unit_cube();
        let r = unfold_with_papercraft(&mesh).unwrap();
        assert_eq!(r.pieces.len(), 1);
        assert_eq!(r.metrics.total_pieces, 1);
        // 12 треугольников: дерево = 11 рёбер (6 плоских диагоналей + 5 рёбер куба).
        // Сгибов 5 (плоские диагонали не рисуются, SoftHidden); порезов 7, клапан
        // один на рез -> 7; сегментов резов 7*3 + 7 гладких = 28.
        assert_eq!(r.metrics.total_folds, 5);
        assert_eq!(r.metrics.total_tabs, 7);
        assert_eq!(r.pieces[0].face_ids.len(), 12);
        assert!(r.pieces[0].outline_2d.len() >= 3);
        assert_eq!(r.pieces[0].cut_lines.len(), 28);
    }

    #[test]
    fn test_papercraft_cube_outline_area_matches_surface() {
        let mesh = unit_cube();
        let r = unfold_with_papercraft(&mesh).unwrap();
        let a = outline_area(&r.pieces[0].outline_2d);
        assert!((a - 6.0).abs() < 1e-6, "outline area {} != 6.0", a);
    }

    #[test]
    fn test_papercraft_two_triangle_strip() {
        let mut mesh = Mesh::new("Strip");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [1.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [1.0, 1.0, 0.0]));
        mesh.add_vertex(Vertex::new(3, [0.0, 1.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));
        mesh.add_face(Face::new(0, 2, 3));
        let r = unfold_with_papercraft(&mesh).unwrap();
        assert_eq!(r.pieces.len(), 1);
        // Общее ребро плоское (диэдр 0) -> в дереве, но сгибом не рисуется.
        assert_eq!(r.metrics.total_folds, 0);
        assert_eq!(r.metrics.total_tabs, 4);
    }

    #[test]
    fn test_papercraft_parts_off_matches_plain() {
        let mesh = unit_cube();
        let cfg = PartsConfig { mode: "off".to_string(), concave_deg: 30.0, manual: Vec::new() };
        let r = unfold_with_papercraft_parts(&mesh, &cfg).unwrap();
        assert_eq!(r.parts.len(), 1);
        assert_eq!(r.parts[0].name, "Модель");
        assert_eq!(r.pieces.len(), 1);
        assert_eq!(r.pieces[0].cut_lines.len(), 28);
        assert_eq!(r.pieces[0].part_id, 0);
    }

    #[test]
    fn test_papercraft_parts_materials() {
        let mut mesh = unit_cube();
        for (i, f) in mesh.faces.iter_mut().enumerate() {
            f.material_id = Some(if i < 2 { 0 } else { 1 });
        }
        mesh.metadata.material_names = vec!["Mat0".to_string(), "Mat1".to_string()];
        let cfg = PartsConfig { mode: "materials".to_string(), concave_deg: 30.0, manual: Vec::new() };
        let r = unfold_with_papercraft_parts(&mesh, &cfg).unwrap();
        assert_eq!(r.parts.len(), 2);
        assert_eq!(r.parts[0].name, "Mat0");
        assert_eq!(r.parts[1].name, "Mat1");
        assert_eq!(r.metrics.total_pieces, r.pieces.len() as u32);
        for p in &r.pieces {
            assert!(!p.part.is_empty());
            for &f in &p.face_ids {
                assert!((f as usize) < mesh.faces.len());
            }
        }
    }
}
