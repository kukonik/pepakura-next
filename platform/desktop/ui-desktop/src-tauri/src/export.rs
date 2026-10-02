
use std::fs::File;
use std::io::BufWriter;
#[tauri::command]
pub fn export_pdf(
    vertices: Vec<Vec<f64>>,
    faces: Vec<Vec<usize>>,
    _island_ids: Vec<usize>,
    path: String,
) -> Result<(), String> {
    use printpdf::{Mm, PdfDocument, Point, Line, Color, Rgb};

    if vertices.is_empty() || faces.is_empty() {
        return Err("No data to export".into());
    }

    // BBOX по всем вершинам
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for v in &vertices {
        if v.len() < 2 {
            continue;
        }
        let x = v[0];
        let y = v[1];
        if x < min_x { min_x = x; }
        if x > max_x { max_x = x; }
        if y < min_y { min_y = y; }
        if y > max_y { max_y = y; }
    }

    if !min_x.is_finite() || !max_x.is_finite() || !min_y.is_finite() || !max_y.is_finite() {
        return Err("Invalid vertex coordinates".into());
    }

    let width_model = max_x - min_x;
    let height_model = max_y - min_y;
    if width_model <= 0.0 || height_model <= 0.0 {
        return Err("Degenerate bounding box".into());
    }

    // A4 + поля (мм)
    let page_width_mm: f32 = 210.0;
    let page_height_mm: f32 = 297.0;
    let margin_mm: f32 = 10.0;

    let printable_width_mm: f32 = page_width_mm - 2.0 * margin_mm;
    let printable_height_mm: f32 = page_height_mm - 2.0 * margin_mm;

    if printable_width_mm <= 0.0 || printable_height_mm <= 0.0 {
        return Err("Invalid page margins".into());
    }

    // масштаб (double -> float)
    let scale_x: f32 = (printable_width_mm as f64 / width_model) as f32;
    let scale_y: f32 = (printable_height_mm as f64 / height_model) as f32;
    let scale: f32 = scale_x.min(scale_y);

    let (doc, page1, layer1) = PdfDocument::new(
        "Pepakura Next Export",
        Mm(page_width_mm),
        Mm(page_height_mm),
        "Layer 1",
    );
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // модель -> страница
    let transform_point = |vx: f64, vy: f64| -> Point {
        let nx: f32 = (((vx - min_x) as f64) * (scale as f64) + (margin_mm as f64)) as f32;
        let ny: f32 = (((vy - min_y) as f64) * (scale as f64) + (margin_mm as f64)) as f32;
        Point::new(Mm(nx), Mm(ny))
    };

    let line_color = Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None));
    let line_width_mm: f32 = 0.25;

    current_layer.set_outline_color(line_color);
    current_layer.set_outline_thickness(line_width_mm);

    for face in &faces {
        if face.len() < 3 {
            continue;
        }
        let i0 = face[0];
        let i1 = face[1];
        let i2 = face[2];

        let v0 = match vertices.get(i0) {
            Some(v) if v.len() >= 2 => v,
            _ => continue,
        };
        let v1 = match vertices.get(i1) {
            Some(v) if v.len() >= 2 => v,
            _ => continue,
        };
        let v2 = match vertices.get(i2) {
            Some(v) if v.len() >= 2 => v,
            _ => continue,
        };

        let p0 = transform_point(v0[0], v0[1]);
        let p1 = transform_point(v1[0], v1[1]);
        let p2 = transform_point(v2[0], v2[1]);

        let line: Line = vec![
            (p0, false),
            (p1, false),
            (p2, false),
            (p0, false),
        ]
        .into_iter()
        .collect();

        let mut line = line;
        line.set_closed(true);
        current_layer.add_line(line);
    }

    let file = File::create(&path).map_err(|e: std::io::Error| e.to_string())?;
    let mut writer = BufWriter::new(file);
    doc.save(&mut writer).map_err(|e: printpdf::Error| e.to_string())?;

    Ok(())
}






