use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::collections::HashMap;

#[tauri::command]
pub fn export_pdf(
    vertices: Vec<Vec<f64>>,
    faces: Vec<Vec<usize>>,
    island_ids: Vec<usize>,
    path: String,
) -> Result<(), String> {
    if vertices.is_empty() || faces.is_empty() {
        return Err("No data to export".to_string());
    }
    if island_ids.len() != vertices.len() {
        return Err(format!("island_ids len {} != vertices len {}", island_ids.len(), vertices.len()));
    }

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    for v in &vertices {
        if v[0] < min_x { min_x = v[0]; }
        if v[1] < min_y { min_y = v[1]; }
        if v[0] > max_x { max_x = v[0]; }
        if v[1] > max_y { max_y = v[1]; }
    }
    let mesh_w = max_x - min_x;
    let mesh_h = max_y - min_y;
    if mesh_w <= 0.0 || mesh_h <= 0.0 {
        return Err("Invalid mesh dimensions".to_string());
    }

    let (page_w, page_h) = (Mm(210.0), Mm(297.0));
    let margin = Mm(10.0);
    let doc = PdfDocument::empty("Pepakura Next Export");
    let (page_idx, layer_idx) = doc.add_page(page_w, page_h, "Layer 1");
    let layer = doc.get_page(page_idx).get_layer(layer_idx);

    let avail_w = (page_w.0 - 2.0 * margin.0) as f64;
    let avail_h = (page_h.0 - 2.0 * margin.0) as f64;
    let scale = avail_w.min(avail_h) / mesh_w.max(mesh_h);
    let offset_x = margin.0 as f64 + (avail_w - mesh_w * scale) / 2.0 - min_x * scale;
    let offset_y = margin.0 as f64 + (avail_h - mesh_h * scale) / 2.0 - min_y * scale;

    let xform = |x: f64, y: f64| -> (f64, f64) {
        (x * scale + offset_x, y * scale + offset_y)
    };

    // --- Edge classification ---
    let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (fi, face) in faces.iter().enumerate() {
        if face.len() < 3 { continue; }
        for k in 0..3 {
            let a = face[k];
            let b = face[(k + 1) % 3];
            let key = if a < b { (a, b) } else { (b, a) };
            edge_faces.entry(key).or_default().push(fi);
        }
    }

    let mut boundary_edges: Vec<(usize, usize)> = Vec::new();
    let mut glue_edges: Vec<(usize, usize)> = Vec::new();
    let mut fold_edges: Vec<(usize, usize)> = Vec::new();

    for (edge, fids) in &edge_faces {
        if fids.len() == 1 {
            boundary_edges.push(*edge);
        } else {
            let isl0 = island_ids[faces[fids[0]][0]];
            let all_same = fids.iter().all(|&fi| island_ids[faces[fi][0]] == isl0);
            if all_same {
                fold_edges.push(*edge);
            } else {
                glue_edges.push(*edge);
            }
        }
    }
    println!("[export] edges: boundary={}, glue={}, fold={}", boundary_edges.len(), glue_edges.len(), fold_edges.len());

    // --- Island centroids ---
    let mut isl_cx: HashMap<usize, f64> = HashMap::new();
    let mut isl_cy: HashMap<usize, f64> = HashMap::new();
    let mut isl_cnt: HashMap<usize, usize> = HashMap::new();
    for (vi, v) in vertices.iter().enumerate() {
        let id = island_ids[vi];
        *isl_cx.entry(id).or_insert(0.0) += v[0];
        *isl_cy.entry(id).or_insert(0.0) += v[1];
        *isl_cnt.entry(id).or_insert(0) += 1;
    }
    for (&id, &cnt) in &isl_cnt {
        let c = cnt as f64;
        if let Some(cx) = isl_cx.get_mut(&id) { *cx /= c; }
        if let Some(cy) = isl_cy.get_mut(&id) { *cy /= c; }
    }

    // --- 1. Fold lines (dashed, grey) ---
    layer.set_line_dash_pattern(LineDashPattern {
        dash_1: Some(2), gap_1: Some(1),
        dash_2: None, gap_2: None,
        dash_3: None, gap_3: None,
        offset: Some(0),
    });
    layer.set_outline_color(Color::Rgb(Rgb::new(0.4, 0.4, 0.4, None)));
    for &(a, b) in &fold_edges {
        let (x1, y1) = xform(vertices[a][0], vertices[a][1]);
        let (x2, y2) = xform(vertices[b][0], vertices[b][1]);
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(x1 as f32), Mm(y1 as f32)), false),
                (Point::new(Mm(x2 as f32), Mm(y2 as f32)), false),
            ],
            is_closed: false,
        });
    }

    // --- 2. Cut lines (solid, black) ---
    layer.set_line_dash_pattern(LineDashPattern {
        dash_1: None, gap_1: None,
        dash_2: None, gap_2: None,
        dash_3: None, gap_3: None,
        offset: Some(0),
    });
    layer.set_outline_color(Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)));
    for &(a, b) in boundary_edges.iter().chain(glue_edges.iter()) {
        let (x1, y1) = xform(vertices[a][0], vertices[a][1]);
        let (x2, y2) = xform(vertices[b][0], vertices[b][1]);
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(x1 as f32), Mm(y1 as f32)), false),
                (Point::new(Mm(x2 as f32), Mm(y2 as f32)), false),
            ],
            is_closed: false,
        });
    }

    // --- 3. Glue tabs (trapezoids outward from centroid) ---
    let tab_h = 5.0;
    let tab_inset = 0.15;
    for &(a, b) in &glue_edges {
        let (x1, y1) = xform(vertices[a][0], vertices[a][1]);
        let (x2, y2) = xform(vertices[b][0], vertices[b][1]);
        let dx = x2 - x1;
        let dy = y2 - y1;
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1.0 { continue; }
        let ux = dx / len;
        let uy = dy / len;
        let nx = -uy;
        let ny = ux;
        let mx = (x1 + x2) / 2.0;
        let my = (y1 + y2) / 2.0;
        let isl = island_ids[a];
        let tcx = isl_cx.get(&isl).copied().unwrap_or(0.0) * scale + offset_x;
        let tcy = isl_cy.get(&isl).copied().unwrap_or(0.0) * scale + offset_y;
        let dot = nx * (mx - tcx) + ny * (my - tcy);
        let (fnx, fny) = if dot >= 0.0 { (nx, ny) } else { (-nx, -ny) };
        let ins = len * tab_inset;
        let ax = x1 + ux * ins;
        let ay = y1 + uy * ins;
        let bx = x2 - ux * ins;
        let by = y2 - uy * ins;
        let cx = bx + fnx * tab_h;
        let cy = by + fny * tab_h;
        let dx2 = ax + fnx * tab_h;
        let dy2 = ay + fny * tab_h;
        layer.add_line(Line {
            points: vec![
                (Point::new(Mm(ax as f32), Mm(ay as f32)), false),
                (Point::new(Mm(bx as f32), Mm(by as f32)), false),
                (Point::new(Mm(cx as f32), Mm(cy as f32)), false),
                (Point::new(Mm(dx2 as f32), Mm(dy2 as f32)), false),
                (Point::new(Mm(ax as f32), Mm(ay as f32)), false),
            ],
            is_closed: true,
        });
    }

    // --- 4. Island numbers ---
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| e.to_string())?;
    for (&id, &cx) in &isl_cx {
        let cy = isl_cy[&id];
        let (tx, ty) = xform(cx, cy);
        layer.use_text(&id.to_string(), 8.0, Mm(tx as f32), Mm(ty as f32), &font);
    }

    doc.save(&mut BufWriter::new(File::create(&path).map_err(|e| e.to_string())?))
        .map_err(|e| e.to_string())?;

    Ok(())
}
