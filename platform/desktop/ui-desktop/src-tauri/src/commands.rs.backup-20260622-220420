use serde::{Deserialize, Serialize};
use tauri::command;
mod maxrects;
use std::collections::{BTreeMap, HashMap, HashSet};
use pepakura_core::sanitize::SanitizeOptions;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnfoldConfig {
    pub algorithm: Option<String>,
}

#[derive(Serialize)]
struct FaceDto { vertices: Vec<usize> }

#[derive(Serialize)]
struct UnfoldResponse {
    success: bool,
    algorithm_used: String,
    message: String,
    vertex_count: usize,
    face_count: usize,
    islands: usize,
    total_area: f64,
    vertices_2d: Vec<[f64; 2]>,
    faces: Vec<FaceDto>,
    island_ids: Vec<usize>,
    degenerate_islands: Vec<usize>,
}

fn sanitize_obj(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.starts_with("V ") { format!("v{}", &line[1..]) }
            else if line.starts_with("F ") { format!("f{}", &line[1..]) }
            else { line.to_string() }
        })
        .collect::<Vec<String>>().join("\n")
}

struct UnionFind { parent: Vec<usize> }
impl UnionFind {
    fn new(size: usize) -> Self { UnionFind { parent: (0..size).collect() } }
    fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while self.parent[root] != root { root = self.parent[root]; }
        while self.parent[i] != root { let next = self.parent[i]; self.parent[i] = root; i = next; }
        root
    }
    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i); let root_j = self.find(j);
        if root_i != root_j { self.parent[root_i] = root_j; }
    }
}

fn calculate_metrics(vertices_2d: &Vec<[f64; 2]>, faces: &Vec<FaceDto>) -> (usize, f64, Vec<usize>) {
    let n_verts = vertices_2d.len();
    let mut uf = UnionFind::new(n_verts);
    let mut total_area = 0.0;
    for face in faces {
        if face.vertices.len() >= 2 {
            for i in 1..face.vertices.len() { uf.union(face.vertices[i-1], face.vertices[i]); }
        }
        if face.vertices.len() == 3 {
            let v0 = vertices_2d[face.vertices[0]];
            let v1 = vertices_2d[face.vertices[1]];
            let v2 = vertices_2d[face.vertices[2]];
            total_area += ((v1[0] - v0[0]) * (v2[1] - v0[1]) - (v1[1] - v0[1]) * (v2[0] - v0[0])).abs() * 0.5;
        }
    }
    let mut island_ids = vec![0; n_verts];
    let mut root_map = HashMap::new();
    let mut next_id = 0;
    for i in 0..n_verts {
        let root = uf.find(i);
        let id = *root_map.entry(root).or_insert_with(|| { let id = next_id; next_id += 1; id });
        island_ids[i] = id;
    }
    (next_id, total_area, island_ids)
}

fn process_and_pack_island(
    all_vertices_2d: &mut Vec<[f64; 2]>,
    current_x: &mut f64, current_y: &mut f64, row_max_height: &mut f64,
    padding: f64, max_row_width: f64,
    final_vertices_2d: &[[f64; 2]], inv_vertex_map: &[usize]
) {
    let mut min_x = f64::MAX; let mut max_x = f64::MIN;
    let mut min_y = f64::MAX; let mut max_y = f64::MIN;
    for v in final_vertices_2d {
        if v[0] < min_x { min_x = v[0]; } if v[0] > max_x { max_x = v[0]; }
        if v[1] < min_y { min_y = v[1]; } if v[1] > max_y { max_y = v[1]; }
    }
    let w = max_x - min_x; let h = max_y - min_y;
    if *current_x + w > max_row_width && *current_x > 0.0 {
        *current_x = 0.0; *current_y += *row_max_height + padding; *row_max_height = 0.0;
    }
    let shift_x = *current_x - min_x + padding;
    let shift_y = *current_y - min_y + padding;
    for (sub_vi, &v2d) in final_vertices_2d.iter().enumerate() {
        let orig_vi = inv_vertex_map[sub_vi];
        all_vertices_2d[orig_vi] = [v2d[0] + shift_x, v2d[1] + shift_y];
    }
    *current_x += w + padding;
    if h > *row_max_height { *row_max_height = h; }
}

#[command]
pub async fn unfold_mesh(obj_data: String, _config: Option<UnfoldConfig>) -> Result<String, String> {
    println!("[unfold] Получен OBJ размером {} байт", obj_data.len());

    let handle = tokio::task::spawn_blocking(move || {
        let sanitized = sanitize_obj(&obj_data);
        let raw_mesh = pepakura_core::import::parse_obj_str(&sanitized)
            .map_err(|e| format!("Ошибка парсинга OBJ: {:?}", e))?;

        println!("[unfold] Меш распарсен: {} вершин, {} граней", raw_mesh.vertices.len(), raw_mesh.faces.len());

        // 0. Фильтрация вырожденных граней
        let pre_clean_mesh = {
            let valid_faces: Vec<_> = raw_mesh.faces.iter()
                .filter(|f| { let v = f.vertices; v[0] != v[1] && v[1] != v[2] && v[0] != v[2] })
                .cloned()
                .collect();
            let removed = raw_mesh.faces.len() - valid_faces.len();
            if removed > 0 { println!("[sanitize] Удалено вырожденных граней: {}", removed); }
            pepakura_core::geometry::Mesh { vertices: raw_mesh.vertices, faces: valid_faces, name: raw_mesh.name, metadata: raw_mesh.metadata }
        };

        // 1. Санитизация meshopt
        let mesh = match pepakura_core::sanitize::simplify_mesh(&pre_clean_mesh, &SanitizeOptions::default()) {
            Ok(clean_mesh) => { println!("[sanitize] meshopt: {} вершин, {} граней", clean_mesh.vertices.len(), clean_mesh.faces.len()); clean_mesh }
            Err(_) => { println!("[sanitize] meshopt недоступен."); pre_clean_mesh }
        };

        let start_time = std::time::Instant::now();
        println!("[unfold] Алгоритм: MST-Papercraft + Seam Cut (Alpha 0.3)");

        // 2. Топологические компоненты (если меш несвязный)
        let mut uf = UnionFind::new(mesh.faces.len());
        let mut edge_map: HashMap<(usize, usize), usize> = HashMap::new();
        for (f_idx, face) in mesh.faces.iter().enumerate() {
            for k in 0..3 {
                let v1 = face.vertices[k]; let v2 = face.vertices[(k + 1) % 3];
                let key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                if let Some(&other_f_idx) = edge_map.get(&key) { uf.union(f_idx, other_f_idx); }
                else { edge_map.insert(key, f_idx); }
            }
        }
        let mut components: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
        for (f_idx, _) in mesh.faces.iter().enumerate() {
            let root = uf.find(f_idx);
            components.entry(root).or_insert_with(Vec::new).push(f_idx);
        }
        println!("[unfold] Топологических компонентов: {}", components.len());

        let mut all_vertices_2d = vec![[0.0, 0.0]; mesh.vertices.len()];
        let mut mst_count = 0;
        let mut current_x = 0.0; let mut current_y = 0.0; let mut row_max_height = 0.0;
        let padding = 8.0; let max_row_width = 190.0;
        let core_config = pepakura_core::unfold::UnfoldConfig::default();
        let mut degenerate_vert_set: HashSet<usize> = HashSet::new();

        // 3. Развёртка по компонентам с MST-seam
        for (comp_idx, face_indices) in components.values().enumerate() {
            if face_indices.is_empty() { continue; }

            let mut sub_vertices = Vec::new(); let mut inv_vertex_map = Vec::new();
            let mut vertex_map = HashMap::new(); let mut sub_faces = Vec::new();

            for &fi in face_indices {
                let orig_face = &mesh.faces[fi];
                let mut new_face = pepakura_core::geometry::Face { vertices: [0, 0, 0], material_id: orig_face.material_id.clone() };
                for i in 0..3 {
                    let orig_vi = orig_face.vertices[i];
                    let sub_vi = *vertex_map.entry(orig_vi).or_insert_with(|| {
                        let idx = sub_vertices.len();
                        sub_vertices.push(mesh.vertices[orig_vi].clone());
                        inv_vertex_map.push(orig_vi);
                        idx
                    });
                    new_face.vertices[i] = sub_vi;
                }
                sub_faces.push(new_face);
            }

            let sub_mesh = pepakura_core::geometry::Mesh {
                vertices: sub_vertices, faces: sub_faces,
                name: format!("comp_{}", comp_idx), metadata: Default::default(),
            };

            // MST-based seam cutting: целевое число под-островов
            let target_islands = if sub_mesh.faces.len() < 50 { 1 } else { (sub_mesh.faces.len() / 100).max(2).min(5) };
            let seam_result = pepakura_core::unfold::seam_cut::compute_seam_islands(&sub_mesh, target_islands);
            println!("[unfold] Компонент {}: MST-seam -> {} под-островов (швов: {})", comp_idx, seam_result.islands.len(), seam_result.seam_count);

            for (island_idx, island_face_indices) in seam_result.islands.iter().enumerate() {
                let mut ss_vertices = Vec::new(); let mut ss_inv_vertex_map = Vec::new();
                let mut ss_vertex_map = HashMap::new(); let mut ss_faces = Vec::new();

                for &local_fi in island_face_indices {
                    let orig_face = &sub_mesh.faces[local_fi];
                    let mut new_face = pepakura_core::geometry::Face { vertices: [0, 0, 0], material_id: orig_face.material_id.clone() };
                    for i in 0..3 {
                        let sub_vi = orig_face.vertices[i];
                        let ss_vi = *ss_vertex_map.entry(sub_vi).or_insert_with(|| {
                            let idx = ss_vertices.len();
                            ss_vertices.push(sub_mesh.vertices[sub_vi].clone());
                            ss_inv_vertex_map.push(inv_vertex_map[sub_vi]);
                            idx
                        });
                        new_face.vertices[i] = ss_vi;
                    }
                    ss_faces.push(new_face);
                }

                let ss_mesh = pepakura_core::geometry::Mesh {
                    vertices: ss_vertices, faces: ss_faces,
                    name: format!("isl_{}_{}", comp_idx, island_idx), metadata: Default::default(),
                };

                match pepakura_core::unfold::unfold_mst(&ss_mesh, &core_config) {
                    Ok(unfolded) => {
                        mst_count += 1;
                        if unfolded.metadata.is_degenerate {
                            for &vi in &ss_inv_vertex_map { degenerate_vert_set.insert(vi); }
                        }
                        println!("[unfold]   Остров {}.{}: граней={}, area_2d={:.2}, deg={}", comp_idx, island_idx, unfolded.faces.len(), unfolded.metadata.area_2d, unfolded.metadata.is_degenerate);
                        if unfolded.metadata.area_2d > 0.0001 {
                            process_and_pack_island(
                                &mut all_vertices_2d, &mut current_x, &mut current_y, &mut row_max_height,
                                padding, max_row_width, &unfolded.vertices_2d, &ss_inv_vertex_map
                            );
                        }
                    }
                    Err(e) => { println!("[unfold] MST упал на острове {}.{}: {:?}", comp_idx, island_idx, e); }
                }
            }
        }

        let elapsed = start_time.elapsed().as_secs_f64() * 1000.0;
        let algorithm_used = format!("MST-Seam({}) [{:.0}ms]", mst_count, elapsed);
        let faces_dto: Vec<FaceDto> = mesh.faces.iter().map(|f| FaceDto { vertices: f.vertices.to_vec() }).collect();


        let (islands, total_area, island_ids) = calculate_metrics(&all_vertices_2d, &faces_dto);
                        // ДИАГНОСТИКА: BBOX каждого острова ДО упаковки
            println!("[unfold] === ДИАГНОСТИКА BBOX островов ===");
            let mut diag_bboxes: std::collections::HashMap<usize, (f64, f64, f64, f64)> = std::collections::HashMap::new();
            for (_fi, face) in faces_dto.iter().enumerate() {
                let id = island_ids[face.vertices[0]];
                for &vi in &face.vertices {
                    let v = all_vertices_2d[vi];
                    let e = diag_bboxes.entry(id).or_insert((f64::MAX, f64::MIN, f64::MAX, f64::MIN));
                    if v[0] < e.0 { e.0 = v[0]; } if v[0] > e.1 { e.1 = v[0]; }
                    if v[1] < e.2 { e.2 = v[1]; } if v[1] > e.3 { e.3 = v[1]; }
                }
            }
            for (id, (min_x, max_x, min_y, max_y)) in &diag_bboxes {
                println!("[unfold] Остров {}: BBOX x=[{:.4}..{:.4}] y=[{:.4}..{:.4}] w={:.4} h={:.4}", 
                    id, min_x, max_x, min_y, max_y, max_x - min_x, max_y - min_y);
            }
            println!("[unfold] === КОНЕЦ ДИАГНОСТИКИ ===");
// SHELF PACKING: разложить острова по полкам без перекрытий
            let mut bboxes: std::collections::HashMap<usize, (f64, f64, f64, f64)> = std::collections::HashMap::new();
            for (_fi, face) in faces_dto.iter().enumerate() {
                let id = island_ids[face.vertices[0]];
                for &vi in &face.vertices {
                    let v = all_vertices_2d[vi];
                    let e = bboxes.entry(id).or_insert((f64::MAX, f64::MIN, f64::MAX, f64::MIN));
                    if v[0] < e.0 { e.0 = v[0]; } if v[0] > e.1 { e.1 = v[0]; }
                    if v[1] < e.2 { e.2 = v[1]; } if v[1] > e.3 { e.3 = v[1]; }
                }
            }
            
            // Ротация островов если h > w
            for (&id, (min_x, max_x, min_y, max_y)) in &bboxes {
                let w = max_x - min_x; let h = max_y - min_y;
                if h > w {
                    let cx = min_x + w / 2.0; let cy = min_y + h / 2.0;
                    for (_fi, face) in faces_dto.iter().enumerate() {
                        if island_ids[face.vertices[0]] == id {
                            for &vi in &face.vertices {
                                let v = &mut all_vertices_2d[vi];
                                let dx = v[0] - cx; let dy = v[1] - cy;
                                v[0] = cx - dy; v[1] = cy + dx;
                            }
                        }
                    }
                }
            }
            
            // Shelf Packing: строки по max_row_width
            let max_row_width = 190.0;   // целевая ширина под A4 printable
            let padding = 8.0;

            let mut sorted_islands: Vec<(usize, f64, f64)> = bboxes
                .iter()
                .map(|(&id, (min_x, max_x, min_y, max_y))| {
                    let w = (max_x - min_x).abs();
                    let h = (max_y - min_y).abs();
                    (id, w, h)
                })
                .collect();

            // Крупные острова сначала
            sorted_islands.sort_by(|a, b| (b.1 * b.2).partial_cmp(&(a.1 * a.2)).unwrap_or(std::cmp::Ordering::Equal));

            let mut shelf_x = 0.0;
            let mut shelf_y = 0.0;
            let mut shelf_height = 0.0;

            for (island_id, island_w, island_h) in sorted_islands {
                // Перенос на следующую строку, если не влезаем по ширине
                if shelf_x + island_w > max_row_width && shelf_x > 0.0 {
                    shelf_x = 0.0;
                    shelf_y += shelf_height + padding;
                    shelf_height = 0.0;
                }

                // Найти BBOX острова
                let mut lx = f64::MAX;
                let mut ly = f64::MAX;
                let mut island_verts = std::collections::HashSet::new();
                for (_fi, face) in faces_dto.iter().enumerate() {
                    if island_ids[face.vertices[0]] == island_id {
                        for &vi in &face.vertices {
                            island_verts.insert(vi);
                            let v = all_vertices_2d[vi];
                            if v[0] < lx { lx = v[0]; }
                            if v[1] < ly { ly = v[1]; }
                        }
                    }
                }

                let ox = shelf_x - lx;
                let oy = shelf_y - ly;

                for &vi in &island_verts {
                    let v = &mut all_vertices_2d[vi];
                    v[0] += ox;
                    v[1] += oy;
                }

                shelf_x += island_w + padding;
                if island_h > shelf_height { shelf_height = island_h; }
            }

            println!("[unfold] Shelf Packing: shelf_y={:.2}, shelf_height={:.2}", shelf_y, shelf_height);
            let mut pack_min_x = f64::MAX;
            let mut pack_min_y = f64::MAX;
            let mut pack_max_x = f64::MIN;
            let mut pack_max_y = f64::MIN;
            for v in &all_vertices_2d {
                if v[0] != 0.0 || v[1] != 0.0 {
                    if v[0] < pack_min_x { pack_min_x = v[0]; }
                    if v[0] > pack_max_x { pack_max_x = v[0]; }
                    if v[1] < pack_min_y { pack_min_y = v[1]; }
                    if v[1] > pack_max_y { pack_max_y = v[1]; }
                }
            }
            println!(
                "[unfold] PACK BBOX: x=[{:.2}..{:.2}] y=[{:.2}..{:.2}] w={:.2} h={:.2}",
                pack_min_x,
                pack_max_x,
                pack_min_y,
                pack_max_y,
                pack_max_x - pack_min_x,
                pack_max_y - pack_min_y
            );


        // Маппинг деградированных вершин в id островов
        let degenerate_islands: Vec<usize> = if !degenerate_vert_set.is_empty() {
            let mut set = HashSet::new();
            for (vi, &id) in island_ids.iter().enumerate() {
                if degenerate_vert_set.contains(&vi) { set.insert(id); }
            }
            set.into_iter().collect()
        } else { Vec::new() };

        println!("[unfold] ИТОГО: островов={}, площадь={:.2}, деградированных={}", islands, total_area, degenerate_islands.len());

            // ФИНАЛЬНАЯ НОРМАЛИЗАЦИЯ ПОСЛЕ УПАКОВКИ: сдвигаем все координаты к (0,0)
            let mut final_min_x = f64::MAX;
            let mut final_min_y = f64::MAX;
            let mut final_max_x = f64::MIN;
            let mut final_max_y = f64::MIN;
            for v in &all_vertices_2d {
                if v[0] != 0.0 || v[1] != 0.0 {
                    if v[0] < final_min_x { final_min_x = v[0]; }
                    if v[0] > final_max_x { final_max_x = v[0]; }
                    if v[1] < final_min_y { final_min_y = v[1]; }
                    if v[1] > final_max_y { final_max_y = v[1]; }
                }
            }
            let offset_x = -final_min_x + 10.0;
            let offset_y = -final_min_y + 10.0;
            for v in &mut all_vertices_2d {
                if v[0] != 0.0 || v[1] != 0.0 {
                    v[0] += offset_x;
                    v[1] += offset_y;
                }
            }
            println!("[unfold] Финальная нормализация: offset=({:.2}, {:.2})", offset_x, offset_y);
            // Контрольный BBOX после упаковки и нормализации
            let mut dbg_min_x = f64::MAX;
            let mut dbg_min_y = f64::MAX;
            let mut dbg_max_x = f64::MIN;
            let mut dbg_max_y = f64::MIN;
            for v in &all_vertices_2d {
                if v[0] != 0.0 || v[1] != 0.0 {
                    if v[0] < dbg_min_x { dbg_min_x = v[0]; }
                    if v[0] > dbg_max_x { dbg_max_x = v[0]; }
                    if v[1] < dbg_min_y { dbg_min_y = v[1]; }
                    if v[1] > dbg_max_y { dbg_max_y = v[1]; }
                }
            }
            println!(
                "[unfold] Финальный BBOX после упаковки: x=[{:.2}..{:.2}] y=[{:.2}..{:.2}] w={:.2} h={:.2}",
                dbg_min_x,
                dbg_max_x,
                dbg_min_y,
                dbg_max_y,
                dbg_max_x - dbg_min_x,
                dbg_max_y - dbg_min_y
            );


            
            
            // Ротация островов если h > w
            for (&id, (min_x, max_x, min_y, max_y)) in &bboxes {
                let w = max_x - min_x; let h = max_y - min_y;
                if h > w {
                    let cx = min_x + w / 2.0; let cy = min_y + h / 2.0;
                    for (_fi, face) in faces_dto.iter().enumerate() {
                        if island_ids[face.vertices[0]] == id {
                            for &vi in &face.vertices {
                                let v = &mut all_vertices_2d[vi];
                                let dx = v[0] - cx; let dy = v[1] - cy;
                                v[0] = cx - dy; v[1] = cy + dx;
                            }
                        }
                    }
                }
            }
            
            // Shelf Packing: строки по max_row_width
            let max_row_width = 190.0;
            let padding = 8.0;
            let mut sorted_islands: Vec<(usize, f64, f64)> = bboxes.iter().map(|(&id, (min_x, max_x, min_y, max_y))| {
                let w = (max_x - min_x).abs();
                let h = (max_y - min_y).abs();
                (id, w.max(h), h.max(w))
            }).collect();
            sorted_islands.sort_by(|a, b| (b.1 * b.2).partial_cmp(&(a.1 * a.2)).unwrap_or(std::cmp::Ordering::Equal));
            
            let mut shelf_x = 0.0;
            let mut shelf_y = 0.0;
            let mut shelf_height = 0.0;
            
            for (island_id, island_w, island_h) in sorted_islands {
                if shelf_x + island_w > max_row_width && shelf_x > 0.0 {
                    shelf_y += shelf_height + padding;
                    shelf_x = 0.0;
                    shelf_height = 0.0;
                }
                
                // Найти BBOX острова
                let mut lx = f64::MAX; let mut ly = f64::MAX;
                let mut island_verts = std::collections::HashSet::new();
                for (_fi, face) in faces_dto.iter().enumerate() {
                    if island_ids[face.vertices[0]] == island_id {
                        for &vi in &face.vertices {
                            island_verts.insert(vi);
                            let v = all_vertices_2d[vi];
                            if v[0] < lx { lx = v[0]; }
                            if v[1] < ly { ly = v[1]; }
                        }
                    }
                }
                
                // Сдвиг острова на полку
                let ox = shelf_x - lx;
                let oy = shelf_y - ly;
                for &vi in &island_verts {
                    let v = &mut all_vertices_2d[vi];
                    v[0] += ox;
                    v[1] += oy;
                }
                
                shelf_x += island_w + padding;
                if island_h > shelf_height { shelf_height = island_h; }
            }
        let response = UnfoldResponse {
            success: true, algorithm_used,
            message: format!("Развёртка выполнена. Островов: {}", islands),
            vertex_count: mesh.vertices.len(), face_count: mesh.faces.len(),
            islands, total_area: if total_area.is_finite() { total_area.round() } else { 0.0 },
            vertices_2d: all_vertices_2d, faces: faces_dto, island_ids, degenerate_islands,
        };

        Ok::<String, String>(serde_json::to_string(&response).map_err(|e| format!("Сериализация: {}", e))?)
    });

    match handle.await {
        Ok(Ok(res)) => Ok(res),
        Ok(Err(e)) => Err(format!("Ошибка вычислений: {}", e)),
        Err(e) => Err(format!("Поток выполнения упал: {}", e)),
    }
}







