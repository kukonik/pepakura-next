use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_pdf(
    vertices: &Vec<Vec<f64>>,
    faces: &Vec<Vec<usize>>,
    out_path: &str,
) -> Result<(), String> {
    if vertices.is_empty() || faces.is_empty() {
        return Err("No data to export".to_string());
    }

    let (page_w, page_h) = (Mm(210.0), Mm(297.0));
    let margin = Mm(10.0);
    let doc = PdfDocument::empty("Pepakura Next Export");
    let (page_idx, layer_idx) = doc.add_page(page_w, page_h, "Layer 1");
    let page = doc.get_page(page_idx);
    let layer = doc.get_layer(layer_idx);

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for v in vertices {
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

    let avail_w = page_w.0 - 2.0 * margin.0;
    let avail_h = page_h.0 - 2.0 * margin.0;
    let scale = avail_w.min(avail_h) / mesh_w.max(mesh_h);

    let offset_x = margin.0 + (avail_w - mesh_w * scale) / 2.0 - min_x * scale;
    let offset_y = margin.0 + (avail_h - mesh_h * scale) / 2.0 - min_y * scale;

    let transform = |x: f64, y: f64| -> (Mm, Mm) {
        (Mm(x * scale + offset_x), Mm(y * scale + offset_y))
    };

    for face in faces {
        if face.len() < 3 { continue; }
        let v0 = &vertices[face[0]];
        let v1 = &vertices[face[1]];
        let v2 = &vertices[face[2]];

        let p0 = transform(v0[0], v0[1]);
        let p1 = transform(v1[0], v1[1]);
        let p2 = transform(v2[0], v2[1]);

        let points = vec![
            Point::new(p0.0, p0.1),
            Point::new(p1.0, p1.1),
            Point::new(p2.0, p2.1),
            Point::new(p0.0, p0.1),
        ];

        let line = Line {
            points,
            is_closed: true,
            has_fill: false,
            has_stroke: true,
            is_clip: false,
        };

        layer.add_line(line);
    }

    doc.save(&mut BufWriter::new(File::create(out_path).map_err(|e| e.to_string())?))
        .map_err(|e| e.to_string())?;

    Ok(())
}
