//! Парсеры 3D-форматов (OBJ).
//! [82-A] свой парсер вместо крейта obj: читаем v/vt/vn/f/usemtl.
//! Вершинные индексы граней остаются = индексам `v`, как раньше, поэтому
//! топология для развёртки НЕ меняется. Дополнительно заполняются:
//!   - Vertex.uv / Vertex.normal (первый попавшийся угол; UV-швы схлопываем);
//!   - Face.material_id = индекс в MeshMetadata.material_names (по usemtl).

use crate::geometry::{Mesh, Vertex, Face};

fn pepa83_idx(tok: &str, len: usize) -> Option<usize> {
    let t = tok.trim();
    if t.is_empty() { return None; }
    let n: i64 = t.parse().ok()?;
    if n == 0 { return None; }
    let j = if n > 0 { n - 1 } else { len as i64 + n };
    if j < 0 { return None; }
    let i = j as usize;
    if i < len { Some(i) } else { None }
}

pub fn parse_obj_str(input: &str) -> Result<Mesh, String> {
    let mut positions: Vec<[f64; 3]> = Vec::new();
    let mut texcoords: Vec<[f64; 2]> = Vec::new();
    let mut normals: Vec<[f64; 3]> = Vec::new();
    let mut material_names: Vec<String> = Vec::new();
    let mut cur_mat: Option<usize> = None;

    // углы граней: (индекс v, индекс vt, индекс vn)
    let mut face_verts: Vec<Vec<(usize, Option<usize>, Option<usize>)>> = Vec::new();
    let mut face_mat: Vec<Option<usize>> = Vec::new();

    for raw in input.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let mut it = line.split_whitespace();
        let tag = it.next().unwrap_or("");
        match tag {
            "v" => {
                let x = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                let y = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                let z = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                positions.push([x, y, z]);
            }
            "vt" => {
                let u = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                let v = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                texcoords.push([u, v]);
            }
            "vn" => {
                let x = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                let y = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                let z = it.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
                normals.push([x, y, z]);
            }
            "usemtl" => {
                let name = it.collect::<Vec<&str>>().join(" ");
                if name.is_empty() { continue; }
                let idx = match material_names.iter().position(|m| m == &name) {
                    Some(i) => i,
                    None => { material_names.push(name); material_names.len() - 1 }
                };
                cur_mat = Some(idx);
            }
            "f" => {
                let mut corners: Vec<(usize, Option<usize>, Option<usize>)> = Vec::new();
                for tok in it {
                    let mut parts = tok.split('/');
                    let vi = parts.next().unwrap_or("");
                    let ti = parts.next().unwrap_or("");
                    let ni = parts.next().unwrap_or("");
                    if let Some(v) = pepa83_idx(vi, positions.len()) {
                        corners.push((v, pepa83_idx(ti, texcoords.len()), pepa83_idx(ni, normals.len())));
                    }
                }
                if corners.len() >= 3 {
                    face_verts.push(corners);
                    face_mat.push(cur_mat);
                }
            }
            _ => { /* g/o/mtllib/s/l и прочее игнорируем */ }
        }
    }

    if positions.is_empty() {
        return Err("Не найдено вершин в OBJ".to_string());
    }

    let mut mesh = Mesh::new("Imported");
    for (i, pos) in positions.iter().enumerate() {
        mesh.add_vertex(Vertex::new(i, *pos));
    }

    for (fi, corners) in face_verts.iter().enumerate() {
        // фан-триангуляция полигона
        for k in 1..corners.len() - 1 {
            let a = corners[0].0;
            let b = corners[k].0;
            let c = corners[k + 1].0;
            if a == b || b == c || a == c { continue; }
            if a >= mesh.vertices.len() || b >= mesh.vertices.len() || c >= mesh.vertices.len() { continue; }
            let mut face = Face::new(a, b, c);
            face.material_id = face_mat[fi];
            mesh.add_face(face);
        }
    }

    // UV/нормали по углам (для вершины первый попавшийся угол выигрывает;
    // UV-швы дают конфликт - для развёртки это не важно, текстура берётся иначе)
    for corners in face_verts.iter() {
        for &(v, t, n) in corners.iter() {
            if v >= mesh.vertices.len() { continue; }
            if mesh.vertices[v].uv.is_none() {
                if let Some(ti) = t {
                    if ti < texcoords.len() { mesh.vertices[v].uv = Some(texcoords[ti]); }
                }
            }
            if mesh.vertices[v].normal.is_none() {
                if let Some(ni) = n {
                    if ni < normals.len() { mesh.vertices[v].normal = Some(normals[ni]); }
                }
            }
        }
    }

    mesh.metadata.material_names = material_names;
    Ok(mesh)
}