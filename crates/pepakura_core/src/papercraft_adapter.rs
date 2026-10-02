use crate::Mesh;
use crate::unwrap_types::{UnwrapResult, Piece, Polygon, Point2D, FoldLine, FoldType, UnwrapMetrics, Page};
use std::collections::{HashSet, VecDeque, HashMap, BTreeMap};
use std::panic::{catch_unwind, AssertUnwindSafe};

pub fn unfold_with_papercraft(mesh: &Mesh) -> Result<UnwrapResult, String> {
    let result = catch_unwind(AssertUnwindSafe(|| {
        run_unfold(mesh)
    }));

    match result {
        Ok(inner_res) => inner_res,
        Err(e) => {
            let msg = if let Some(s) = e.downcast_ref::<&str>() { s.to_string() }
                      else if let Some(s) = e.downcast_ref::<String>() { s.clone() }
                      else { "Unknown panic".to_string() };
            Err(format!("Adapter Panic: {}", msg))
        }
    }
}

fn run_unfold(mesh: &Mesh) -> Result<UnwrapResult, String> {
    if mesh.faces.is_empty() {
        return Err("Mesh is empty".to_string());
    }

    // 1. Разрезание на островки по углу
    let mut uf = UnionFind::new(mesh.faces.len());
    let mut edge_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (fi, f) in mesh.faces.iter().enumerate() {
        if f.vertices.len() < 3 { continue; }
        for i in 0..3 {
            let a = f.vertices[i];
            let b = f.vertices[(i + 1) % 3];
            let key = if a < b { (a, b) } else { (b, a) };
            edge_map.entry(key).or_default().push(fi);
        }
    }

    let n = |vi: usize| [mesh.vertices[vi].x as f64, mesh.vertices[vi].y as f64, mesh.vertices[vi].z as f64];
    let sub = |a: [f64; 3], b: [f64; 3]| [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
    let norm = |v: [f64; 3]| { let l = (v[0]*v[0]+v[1]*v[1]+v[2]*v[2]).sqrt(); if l > 1e-6 { [v[0]/l, v[1]/l, v[2]/l] } else { [0.0; 3] } };

    for (_key, faces) in &edge_map {
        if faces.len() == 2 {
            let f1 = &mesh.faces[faces[0]];
            let f2 = &mesh.faces[faces[1]];
            if f1.vertices.len() < 3 || f2.vertices.len() < 3 { continue; }
            
            let fn1 = norm(cross(sub(n(f1.vertices[1]), n(f1.vertices[0])), sub(n(f1.vertices[2]), n(f1.vertices[0]))));
            let fn2 = norm(cross(sub(n(f2.vertices[1]), n(f2.vertices[0])), sub(n(f2.vertices[2]), n(f2.vertices[0]))));
            let dot = fn1[0]*fn2[0] + fn1[1]*fn2[1] + fn1[2]*fn2[2];
            let ang = dot.clamp(-1.0, 1.0).acos() * (180.0 / std::f64::consts::PI);
            
            if ang > 40.0 {
                // Разрезаем
            } else {
                uf.union(faces[0], faces[1]);
            }
        }
    }

    let mut comps: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for (fi, _) in mesh.faces.iter().enumerate() {
        let r = uf.find(fi);
        comps.entry(r).or_insert_with(Vec::new).push(fi);
    }

    let mut all_v2d = vec![[0.0f64; 2]; mesh.vertices.len()];
    let mut piece_ids = vec![0; mesh.vertices.len()];
    let mut current_id = 1;
    let mut pieces = Vec::new();
    let mut total_folds_count = 0;

    let dist3 = |a_idx: usize, b_idx: usize| -> f64 {
        let a = &mesh.vertices[a_idx];
        let b = &mesh.vertices[b_idx];
        let dx = a.x as f64 - b.x as f64;
        let dy = a.y as f64 - b.y as f64;
        let dz = a.z as f64 - b.z as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    };

    let mut pack_cursor_x = 0.0;
    let mut pack_cursor_y = 0.0;
    let mut pack_row_h = 0.0;

    // 2. Быстрая развёртка (BFS Trilateration)
    for (_, fis) in comps.values().enumerate() {
        if fis.is_empty() { continue; }
        
        let mut sv = Vec::new();
        let mut iv = Vec::new();
        let mut vm = HashMap::new();
        let mut sf = Vec::new();
        
        for &fi in fis {
            let of = &mesh.faces[fi];
            if of.vertices.len() < 3 { continue; }
            let mut nf = crate::geometry::Face { vertices: vec![0, 0, 0], material_id: of.material_id.clone() };
            for i in 0..3 {
                let oi = of.vertices[i];
                let si = *vm.entry(oi).or_insert_with(|| { let i = sv.len(); sv.push(mesh.vertices[oi].clone()); iv.push(oi); i });
                nf.vertices[i] = si;
            }
            sf.push(nf);
        }
        
        if sf.is_empty() { continue; }
        let sm = crate::geometry::Mesh { vertices: sv, faces: sf, name: "island".to_string(), metadata: Default::default() };
        
        let mut vertices_2d: HashMap<usize, [f64; 2]> = HashMap::new();
        let mut visited_faces = HashSet::new();
        let mut queue = VecDeque::new();
        let mut folds = Vec::new();

        let root_face_idx = 0;
        queue.push_back(root_face_idx);
        visited_faces.insert(root_face_idx);

        let root_face = &sm.faces[root_face_idx];
        if root_face.vertices.len() < 3 { continue; }
        for &v_idx in &root_face.vertices {
            let v = &sm.vertices[v_idx];
            vertices_2d.insert(v_idx, [v.x as f64, v.y as f64]);
        }

        while let Some(face_idx) = queue.pop_front() {
            let face = &sm.faces[face_idx];
            if face.vertices.len() < 3 { continue; }
            let f_indices = &face.vertices;
            
            let edges = [
                (f_indices[0], f_indices[1], f_indices[2]),
                (f_indices[1], f_indices[2], f_indices[0]),
                (f_indices[2], f_indices[0], f_indices[1])
            ];
            
            for &(v1, v2, v3_curr) in &edges {
                for (n_idx, n_face) in sm.faces.iter().enumerate() {
                    if visited_faces.contains(&n_idx) { continue; }
                    if n_face.vertices.len() < 3 { continue; }
                    
                    let nf_indices = &n_face.vertices;
                    if nf_indices.contains(&v1) && nf_indices.contains(&v2) {
                        visited_faces.insert(n_idx);
                        queue.push_back(n_idx);
                        
                        let p1 = vertices_2d.get(&v1).copied().unwrap_or([0.0, 0.0]);
                        let p2 = vertices_2d.get(&v2).copied().unwrap_or([0.0, 0.0]);
                        
                        let n_v3 = nf_indices.iter().find(|&&x| x != v1 && x != v2).copied().unwrap_or(0);
                        
                        let len_13 = dist3(iv[v1], iv[n_v3]);
                        let len_23 = dist3(iv[v2], iv[n_v3]);
                        
                        let dx = p2[0] - p1[0];
                        let dy = p2[1] - p1[1];
                        let d = (dx * dx + dy * dy).sqrt();
                        
                        if d > 1e-6 && d <= len_13 + len_23 && d >= (len_13 - len_23).abs() {
                            let a = (len_13.powi(2) - len_23.powi(2) + d.powi(2)) / (2.0 * d);
                            let h_sq = len_13.powi(2) - a.powi(2);
                            let h = if h_sq > 0.0 { h_sq.sqrt() } else { 0.0 };
                            
                            let x0 = p1[0] + a * dx / d;
                            let y0 = p1[1] + a * dy / d;
                            
                            let c1 = [x0 + h * (-dy) / d, y0 + h * dx / d];
                            let c2 = [x0 - h * (-dy) / d, y0 - h * dx / d];
                            
                            let p3_curr_2d = vertices_2d.get(&v3_curr).copied().unwrap_or([0.0, 0.0]);
                            let cross_curr = dx * (p3_curr_2d[1] - p1[1]) - dy * (p3_curr_2d[0] - p1[0]);
                            let cross_c1 = dx * (c1[1] - p1[1]) - dy * (c1[0] - p1[0]);
                            
                            let c = if (cross_curr > 0.0) != (cross_c1 > 0.0) { c1 } else { c2 };
                            
                            vertices_2d.insert(n_v3, c);
                            
                            folds.push(FoldLine {
                                start: Point2D { x: p1[0] as f32, y: p1[1] as f32 },
                                end: Point2D { x: p2[0] as f32, y: p2[1] as f32 },
                                fold_type: FoldType::Mountain, 
                                angle: 0.0,
                            });
                        }
                        break;
                    }
                }
            }
        }

        let mut iem: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
        for f in &sm.faces {
            if f.vertices.len() < 3 { continue; }
            let id = 0;
            let e = iem.entry(id).or_default();
            for i in 0..3 {
                let a = f.vertices[i];
                let b = f.vertices[(i + 1) % 3];
                let k = if a < b { (a, b) } else { (b, a) };
                if !e.insert(k) { e.remove(&k); }
            }
        }

        let mut ol = Vec::new();
        if let Some(be) = iem.get(&0) {
            let mut rm: Vec<(usize, usize)> = be.iter().cloned().collect();
            if let Some(fe) = rm.pop() {
                let pt0 = vertices_2d.get(&fe.0).copied().unwrap_or([0.0, 0.0]);
                let pt1 = vertices_2d.get(&fe.1).copied().unwrap_or([0.0, 0.0]);
                ol.push(Point2D { x: pt0[0] as f32, y: pt0[1] as f32 });
                ol.push(Point2D { x: pt1[0] as f32, y: pt1[1] as f32 });
                let mut last = fe.1;
                while !rm.is_empty() {
                    if let Some(p) = rm.iter().position(|&(a, b)| a == last || b == last) {
                        let (a, b) = rm.remove(p);
                        let n_idx = if a == last { b } else { a };
                        let pt_n = vertices_2d.get(&n_idx).copied().unwrap_or([0.0, 0.0]);
                        ol.push(Point2D { x: pt_n[0] as f32, y: pt_n[1] as f32 });
                        last = n_idx;
                    } else { break; }
                }
            }
        }
        
        if ol.len() > 1 {
            let f = &ol[0];
            let l = ol.last().unwrap();
            if (f.x - l.x).abs() < 0.01 && (f.y - l.y).abs() < 0.01 {
                ol.pop();
            }
        }

        let id = current_id;
        current_id += 1;
        total_folds_count += folds.len();

        let mut is_min_x = f64::MAX;
        let mut is_min_y = f64::MAX;
        let mut is_max_x = f64::MIN;
        let mut is_max_y = f64::MIN;
        for p in vertices_2d.values() {
            if p[0] < is_min_x { is_min_x = p[0]; }
            if p[1] < is_min_y { is_min_y = p[1]; }
            if p[0] > is_max_x { is_max_x = p[0]; }
            if p[1] > is_max_y { is_max_y = p[1]; }
        }
        
        let iw = is_max_x - is_min_x;
        let ih = is_max_y - is_min_y;
        
        let max_row_width = 5.0;
        if pack_cursor_x + iw > max_row_width && pack_cursor_x > 0.0 {
            pack_cursor_x = 0.0;
            pack_cursor_y -= pack_row_h + 0.5;
            pack_row_h = 0.0;
        }
        
        let offset_x = pack_cursor_x - is_min_x;
        let offset_y = pack_cursor_y - is_min_y;
        
        pack_cursor_x += iw + 0.5;
        if ih > pack_row_h { pack_row_h = ih; }
        
        for (si, &oi) in iv.iter().enumerate() {
            let p_island = vertices_2d.get(&si).copied().unwrap_or([0.0, 0.0]);
            all_v2d[oi] = [p_island[0] + offset_x, p_island[1] + offset_y];
            piece_ids[oi] = id;
        }

        let mut ar = 0.0_f32;
        let mut pe = 0.0_f32;
        let n_pts = ol.len();
        if n_pts > 2 {
            for i in 0..n_pts {
                let j = (i + 1) % n_pts;
                let dx = ol[j].x - ol[i].x;
                let dy = ol[j].y - ol[i].y;
                ar += ol[i].x * ol[j].y - ol[j].x * ol[i].y;
                pe += (dx * dx + dy * dy).sqrt();
            }
            ar = ar.abs() * 0.5;
        }

        pieces.push(Piece {
            id: id as u32,
            name: format!("Piece_{}", id),
            label: None,
            outline: Polygon { points: ol, holes: vec![] },
            folds,
            tabs: vec![],
            area: ar,
            perimeter: pe,
            page_number: 0,
        });
    }

    let total_pieces_count = pieces.len() as u32;

    Ok(UnwrapResult {
        engine_used: "papercraft_bfs_packed".to_string(),
        vertices_2d: all_v2d,
        faces: mesh.faces.iter().map(|f| f.vertices.clone()).collect(),
        total_area: 0.0,
        total_perimeter: 0.0,
        pieces,
        pages: vec![Page { page_number: 0, piece_ids: vec![], utilization_percent: 0.0 }],
        metrics: UnwrapMetrics { total_pieces: total_pieces_count, total_pages: 1, total_folds: total_folds_count as u32, total_tabs: 0, complexity_score: 0.0 },
        sheet_width: 1000.0,
        sheet_height: 1000.0,
        margin: 10.0,
    })
}

struct UnionFind { parent: Vec<usize> }
impl UnionFind {
    fn new(size: usize) -> Self { UnionFind { parent: (0..size).collect() } }
    fn find(&mut self, mut i: usize) -> usize {
        let mut root = i; while self.parent[root] != root { root = self.parent[root]; }
        while self.parent[i] != root { let next = self.parent[i]; self.parent[i] = root; i = next; } root
    }
    fn union(&mut self, i: usize, j: usize) {
        let ri = self.find(i); let rj = self.find(j); if ri != rj { self.parent[ri] = rj; }
    }
}
