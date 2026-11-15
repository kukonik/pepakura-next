// ============================================================
// [R3-v2 / EXP54] Реальный векторный PDF-экспорт развёртки.
// std-only: PDF собирается вручную как текст, xref - по смещениям.
// Порт подхода papercraft v2.12 (printable.rs): страницы в мм,
// поля, многолистная раскладка полками, рез/сгиб (mountain - красная,
// valley - синяя пунктирная), единый масштаб для всех деталей.
// Публичный API = оригинальному (PdfExportConfig/PdfExportError/...).
// ============================================================

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::export::PageSize;
use crate::unfold::UnfoldedMesh;

/// Конфигурация экспорта в PDF.
#[derive(Debug, Clone)]
pub struct PdfExportConfig {
    pub page_size: PageSize,
    pub scale: f64,
    pub show_fold_lines: bool,
    pub show_cut_lines: bool,
    pub show_part_numbers: bool,
    pub orientation: PdfOrientation,
    pub show_grid: bool,
    pub grid_size_mm: f64,
}

/// Ориентация страницы.
#[derive(Debug, Clone, Copy, Default)]
pub enum PdfOrientation {
    #[default]
    Portrait,
    Landscape,
}

impl Default for PdfExportConfig {
    fn default() -> Self {
        Self {
            page_size: PageSize::A4,
            scale: 1.0,
            show_fold_lines: true,
            show_cut_lines: true,
            show_part_numbers: true,
            orientation: PdfOrientation::Portrait,
            show_grid: false,
            grid_size_mm: 10.0,
        }
    }
}

/// Ошибки экспорта PDF.
#[derive(Debug, thiserror::Error)]
pub enum PdfExportError {
    #[error("Ошибка записи: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Ошибка PDF: {0}")]
    PdfError(String),
    #[error("Пустой меш")]
    EmptyMesh,
    #[error("Нет 2D-координат вершин")]
    NoVertices,
}

/// Результат экспорта PDF.
#[derive(Debug, Clone)]
pub struct PdfExportResult {
    pub bytes: Vec<u8>,
    pub page_count: usize,
    pub file_size: usize,
}

const MM_TO_PT: f64 = 72.0 / 25.4;
const MARGIN_MM: f64 = 10.0;
const GAP_MM: f64 = 2.0;
const LINE_CUT_PT: f64 = 1.0;
const LINE_FOLD_PT: f64 = 0.6;
const LINE_GRID_PT: f64 = 0.2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum LineKind { Cut, Mountain, Valley }

/// Отрезок в мм, координаты страницы (y вниз, от левого верхнего угла).
struct Seg { x1: f64, y1: f64, x2: f64, y2: f64, kind: LineKind }

struct PageData {
    w_mm: f64,
    h_mm: f64,
    segs: Vec<Seg>,
    labels: Vec<(f64, f64, String)>,
}

struct RawMesh {
    verts2d: Vec<(f64, f64)>,
    faces: Vec<Vec<usize>>,
    verts3d: Option<Vec<[f64; 3]>>,
}

fn page_dims_mm(size: &PageSize, orient: &PdfOrientation) -> (f64, f64) {
    let (w, h) = match size {
        PageSize::A4 => (210.0, 297.0),
        PageSize::A3 => (297.0, 420.0),
        PageSize::A2 => (420.0, 594.0),
        PageSize::A1 => (594.0, 841.0),
        PageSize::Custom { width_mm, height_mm } => (*width_mm, *height_mm),
    };
    match orient {
        PdfOrientation::Portrait => (w, h),
        PdfOrientation::Landscape => (h, w),
    }
}

fn raw_from_unfolded(u: &UnfoldedMesh) -> RawMesh {
    let verts2d: Vec<(f64, f64)> = u.vertices_2d.iter().map(|p| (p[0], p[1])).collect();
    let faces: Vec<Vec<usize>> = u.faces.iter().map(|f| f.vertices.to_vec()).collect();
    let verts3d = Some(u.source_mesh.vertices.iter()
        .map(|v| [v.position[0], v.position[1], v.position[2]])
        .collect());
    RawMesh { verts2d, faces, verts3d }
}

fn face_normal3d(verts3d: &[[f64; 3]], f: &[usize]) -> Option<[f64; 3]> {
    if f.len() < 3 { return None; }
    let a = *verts3d.get(f[0])?;
    let b = *verts3d.get(f[1])?;
    let c = *verts3d.get(f[2])?;
    let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let w = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [u[1] * w[2] - u[2] * w[1], u[2] * w[0] - u[0] * w[2], u[0] * w[1] - u[1] * w[0]];
    let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if l < 1e-12 { return None; }
    Some([n[0] / l, n[1] / l, n[2] / l])
}

/// Классификация рёбер: 1 грань -> Cut, 2+ -> сгиб (mountain/valley по нормали).
fn classify_edges(mesh: &RawMesh) -> HashMap<(usize, usize), LineKind> {
    let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (fi, f) in mesh.faces.iter().enumerate() {
        let n = f.len();
        for k in 0..n {
            let a = f[k];
            let b = f[(k + 1) % n];
            if a == b || a >= mesh.verts2d.len() || b >= mesh.verts2d.len() { continue; }
            let key = if a < b { (a, b) } else { (b, a) };
            let e = edge_faces.entry(key).or_insert_with(Vec::new);
            if !e.contains(&fi) { e.push(fi); }
        }
    }

    let mut out: HashMap<(usize, usize), LineKind> = HashMap::new();
    for (key, fs) in edge_faces {
        if fs.len() == 1 {
            out.insert(key, LineKind::Cut);
            continue;
        }
        // Правило papercraft: выпуклое ребро -> Mountain (красная), вогнутое -> Valley (синяя).
        let mut kind = LineKind::Valley;
        if let Some(v3) = &mesh.verts3d {
            if fs.len() >= 2 {
                let n1 = face_normal3d(v3, &mesh.faces[fs[0]]);
                if let Some(n1) = n1 {
                    let f2 = &mesh.faces[fs[1]];
                    if let Some(&c) = f2.iter().find(|&&x| x != key.0 && x != key.1) {
                        if let (Some(cp), Some(ap)) = (v3.get(c), v3.get(key.0)) {
                            let d = n1[0] * (cp[0] - ap[0]) + n1[1] * (cp[1] - ap[1]) + n1[2] * (cp[2] - ap[2]);
                            kind = if d <= 0.0 { LineKind::Mountain } else { LineKind::Valley };
                        }
                    }
                }
            }
        }
        out.insert(key, kind);
    }
    out
}

/// Связные компоненты граней (детали) через рёбра, общие для 2+ граней.
fn face_components(mesh: &RawMesh) -> Vec<Vec<usize>> {
    let mut edge_faces: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (fi, f) in mesh.faces.iter().enumerate() {
        let n = f.len();
        for k in 0..n {
            let a = f[k];
            let b = f[(k + 1) % n];
            if a == b { continue; }
            let key = if a < b { (a, b) } else { (b, a) };
            let e = edge_faces.entry(key).or_insert_with(Vec::new);
            if !e.contains(&fi) { e.push(fi); }
        }
    }

    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for (_key, fs) in &edge_faces {
        if fs.len() < 2 { continue; }
        for i in 0..fs.len() {
            for j in (i + 1)..fs.len() {
                let a = fs[i];
                let b = fs[j];
                adj.entry(a).or_insert_with(Vec::new).push(b);
                adj.entry(b).or_insert_with(Vec::new).push(a);
            }
        }
    }

    let mut comp = vec![usize::MAX; mesh.faces.len()];
    let mut comps: Vec<Vec<usize>> = Vec::new();
    for fi in 0..mesh.faces.len() {
        if comp[fi] != usize::MAX { continue; }
        let mut members = vec![fi];
        comp[fi] = 0;
        let mut stack = vec![fi];
        while let Some(x) = stack.pop() {
            if let Some(nbs) = adj.get(&x) {
                for &nb in nbs {
                    if comp[nb] == usize::MAX {
                        comp[nb] = 0;
                        members.push(nb);
                        stack.push(nb);
                    }
                }
            }
        }
        comps.push(members);
    }
    comps
}

struct PartGeom {
    minx: f64, miny: f64, maxx: f64, maxy: f64,
}

fn build_pages(mesh: &RawMesh, config: &PdfExportConfig) -> Result<(Vec<PageData>, f64, usize), String> {
    let kinds = classify_edges(mesh);
    let comps = face_components(mesh);
    let n_cuts = kinds.values().filter(|k| **k == LineKind::Cut).count();
    let n_folds = kinds.len() - n_cuts;
    eprintln!("[EXP54] граней={} деталей={} рёбер: рез={} сгибов={}", mesh.faces.len(), comps.len(), n_cuts, n_folds);

    let mut parts: Vec<PartGeom> = Vec::new();
    for members in &comps {
        let mut minx = f64::MAX; let mut miny = f64::MAX;
        let mut maxx = f64::MIN; let mut maxy = f64::MIN;
        for &fi in members {
            for &vi in &mesh.faces[fi] {
                if vi >= mesh.verts2d.len() { continue; }
                let (x, y) = mesh.verts2d[vi];
                if x < minx { minx = x; }
                if y < miny { miny = y; }
                if x > maxx { maxx = x; }
                if y > maxy { maxy = y; }
            }
        }
        parts.push(PartGeom { minx, miny, maxx, maxy });
    }

    let (page_w, page_h) = page_dims_mm(&config.page_size, &config.orientation);
    let pw = (page_w - 2.0 * MARGIN_MM).max(1.0);
    let ph = (page_h - 2.0 * MARGIN_MM).max(1.0);

    // Единый масштаб: если самая крупная деталь не влезает - ужимаем ВСЁ.
    let s0 = if config.scale > 1e-9 { config.scale } else { 1.0 };
    let mut eff = s0;
    for p in &parts {
        let w_u = (p.maxx - p.minx).abs();
        let h_u = (p.maxy - p.miny).abs();
        if w_u * s0 > pw && w_u > 1e-9 { eff = eff.min(pw / w_u); }
        if h_u * s0 > ph && h_u > 1e-9 { eff = eff.min(ph / h_u); }
    }
    if eff <= 1e-9 { eff = 1e-9; }
    if !eff.is_finite() { eff = 1.0; }

    // Сортировка деталей по высоте (убыв.) для раскладки полками.
    let mut order: Vec<usize> = (0..parts.len()).collect();
    order.sort_by(|&a, &b| {
        let ha = (parts[b].maxy - parts[b].miny).abs();
        let hb = (parts[a].maxy - parts[a].miny).abs();
        ha.partial_cmp(&hb).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut pages: Vec<PageData> = Vec::new();
    pages.push(PageData { w_mm: page_w, h_mm: page_h, segs: Vec::new(), labels: Vec::new() });

    let mut used_h: f64 = 0.0;
    let mut row_x: f64 = 0.0;
    let mut row_h: f64 = 0.0;

    for &pi in &order {
        let p = &parts[pi];
        let w_mm = (p.maxx - p.minx).abs() * eff;
        let h_mm = (p.maxy - p.miny).abs() * eff;

        if row_x > 0.0 && row_x + w_mm > pw {
            used_h += row_h;
            row_x = 0.0;
            row_h = 0.0;
        }
        if row_x == 0.0 && used_h + h_mm > ph {
            pages.push(PageData { w_mm: page_w, h_mm: page_h, segs: Vec::new(), labels: Vec::new() });
            used_h = 0.0;
            row_x = 0.0;
            row_h = 0.0;
        }

        let page_idx = pages.len() - 1;
        let top_y = used_h;
        let ox = row_x;
        let tx = |x: f64| MARGIN_MM + ox + (x - p.minx) * eff;
        let ty = |y: f64| MARGIN_MM + top_y + (p.maxy - y) * eff;

        let mut drawn: HashSet<(usize, usize)> = HashSet::new();
        for &fi in &comps[pi] {
            let f = &mesh.faces[fi];
            let n = f.len();
            for k in 0..n {
                let a = f[k];
                let b = f[(k + 1) % n];
                if a == b || a >= mesh.verts2d.len() || b >= mesh.verts2d.len() { continue; }
                let key = if a < b { (a, b) } else { (b, a) };
                if !drawn.insert(key) { continue; }
                let kind = *kinds.get(&key).unwrap_or(&LineKind::Cut);
                let is_fold = kind != LineKind::Cut;
                if is_fold && !config.show_fold_lines { continue; }
                if !is_fold && !config.show_cut_lines { continue; }
                let (ax, ay) = mesh.verts2d[a];
                let (bx, by) = mesh.verts2d[b];
                if !(ax.is_finite() && ay.is_finite() && bx.is_finite() && by.is_finite()) { continue; }
                pages[page_idx].segs.push(Seg { x1: tx(ax), y1: ty(ay), x2: tx(bx), y2: ty(by), kind });
            }
        }

        if config.show_part_numbers {
            let mut cx = 0.0; let mut cy = 0.0; let mut cnt = 0.0;
            for &fi in &comps[pi] {
                for &vi in &mesh.faces[fi] {
                    if vi >= mesh.verts2d.len() { continue; }
                    let (x, y) = mesh.verts2d[vi];
                    cx += tx(x); cy += ty(y); cnt += 1.0;
                }
            }
            if cnt > 0.0 {
                let label = format!("P{}", pi + 1);
                pages[page_idx].labels.push((cx / cnt, cy / cnt, label));
            }
        }

        row_x += w_mm + GAP_MM;
        if h_mm > row_h { row_h = h_mm; }
    }

    let total_pages = pages.len();
    Ok((pages, eff, total_pages))
}

fn fmt_f(x: f64) -> String {
    format!("{:.2}", x)
}

fn emit_seg(buf: &mut String, s: &Seg, page_h_pt: f64) {
    // papercraft: mountain - сплошная красная, valley - пунктирная синяя
    let (lw, r, g, b, dash) = match s.kind {
        LineKind::Cut => (LINE_CUT_PT, 0.0, 0.0, 0.0, "[] 0 d"),
        LineKind::Mountain => (LINE_FOLD_PT, 1.0, 0.0, 0.0, "[] 0 d"),
        LineKind::Valley => (LINE_FOLD_PT, 0.0, 0.0, 1.0, "[3 2] 0 d"),
    };
    let x1 = s.x1 * MM_TO_PT;
    let y1 = page_h_pt - s.y1 * MM_TO_PT;
    let x2 = s.x2 * MM_TO_PT;
    let y2 = page_h_pt - s.y2 * MM_TO_PT;
    buf.push_str(&format!(
        "{} w {} {} {} {} RG {} {} m {} {} l S\n",
        fmt_f(lw), dash, fmt_f(r), fmt_f(g), fmt_f(b), fmt_f(x1), fmt_f(y1), fmt_f(x2), fmt_f(y2)
    ));
}

fn pdf_escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('(', "\\(").replace(')', "\\)")
}

fn emit_text(buf: &mut String, x_mm: f64, y_mm: f64, page_h_pt: f64, size_pt: f64, gray: f64, text: &str) {
    let x = x_mm * MM_TO_PT;
    let y = page_h_pt - y_mm * MM_TO_PT;
    buf.push_str(&format!(
        "BT {} {} {} rg /F1 {} Tf {} {} Td ({}) Tj ET\n",
        fmt_f(gray), fmt_f(gray), fmt_f(gray), fmt_f(size_pt), fmt_f(x), fmt_f(y), pdf_escape(text)
    ));
}

fn content_for_page(page: &PageData, page_no: usize, total: usize, eff: f64, config: &PdfExportConfig) -> String {
    let page_h_pt = page.h_mm * MM_TO_PT;
    let mut buf = String::new();

    if config.show_grid && config.grid_size_mm > 0.0 {
        let step = config.grid_size_mm;
        buf.push_str(&format!("{} w 0.85 0.85 0.85 RG [] 0 d\n", fmt_f(LINE_GRID_PT)));
        let mut x = MARGIN_MM;
        while x <= page.w_mm - MARGIN_MM + 1e-9 {
            let a = x * MM_TO_PT;
            buf.push_str(&format!("{} {} m {} {} l S\n", fmt_f(a), fmt_f(MARGIN_MM * MM_TO_PT), fmt_f(a), fmt_f((page.h_mm - MARGIN_MM) * MM_TO_PT)));
            x += step;
        }
        let mut y = MARGIN_MM;
        while y <= page.h_mm - MARGIN_MM + 1e-9 {
            let b = page_h_pt - y * MM_TO_PT;
            buf.push_str(&format!("{} {} m {} {} l S\n", fmt_f(MARGIN_MM * MM_TO_PT), fmt_f(b), fmt_f((page.w_mm - MARGIN_MM) * MM_TO_PT), fmt_f(b)));
            y += step;
        }
    }

    for s in &page.segs {
        emit_seg(&mut buf, s, page_h_pt);
    }

    for (lx, ly, text) in &page.labels {
        emit_text(&mut buf, *lx + 1.0, *ly + 1.0, page_h_pt, 7.0, 0.35, text);
    }

    let footer = format!("Pepakura page {}/{} | scale {:.3} mm/unit", page_no, total, eff);
    emit_text(&mut buf, MARGIN_MM, page.h_mm - 5.0, page_h_pt, 7.0, 0.45, &footer);

    buf
}

fn build_pdf(pages: &[PageData], eff: f64, config: &PdfExportConfig) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    let mut offs: Vec<usize> = Vec::new();

    out.extend_from_slice(b"%PDF-1.4\n");

    let n = pages.len();
    // Объекты: 1 Catalog, 2 Pages, 3 Font, затем пары (Page, Content).
    let page_obj = |i: usize| 4 + 2 * i;
    let content_obj = |i: usize| 5 + 2 * i;

    let mut kids = String::new();
    for i in 0..n {
        if i > 0 { kids.push(' '); }
        kids.push_str(&format!("{} 0 R", page_obj(i)));
    }

    fn push_obj(out: &mut Vec<u8>, offs: &mut Vec<usize>, body: &str) {
        offs.push(out.len());
        let head = format!("{} 0 obj\n", offs.len());
        out.extend_from_slice(head.as_bytes());
        out.extend_from_slice(body.as_bytes());
        out.extend_from_slice(b"\nendobj\n");
    }

    push_obj(&mut out, &mut offs, "<< /Type /Catalog /Pages 2 0 R >>");
    push_obj(&mut out, &mut offs, &format!("<< /Type /Pages /Kids [{}] /Count {} >>", kids, n));
    push_obj(&mut out, &mut offs, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>");

    for (i, page) in pages.iter().enumerate() {
        let content = content_for_page(page, i + 1, n, eff, config);
        let core = content.trim_end_matches('\n');
        push_obj(&mut out, &mut offs, &format!(
            "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {:.2} {:.2}] /Resources << /Font << /F1 3 0 R >> /ProcSet [/PDF /Text] >> /Contents {} 0 R >>",
            page.w_mm * MM_TO_PT, page.h_mm * MM_TO_PT, content_obj(i)
        ));
        offs.push(out.len());
        let head = format!("{} 0 obj\n<< /Length {} >>\nstream\n", offs.len(), core.len());
        out.extend_from_slice(head.as_bytes());
        out.extend_from_slice(core.as_bytes());
        out.extend_from_slice(b"\nendstream\nendobj\n");
    }

    let xref_pos = out.len();
    let count = offs.len() + 1;
    let mut xref = String::new();
    xref.push_str(&format!("xref\n0 {}\n", count));
    xref.push_str("0000000000 65535 f \n");
    for o in &offs {
        xref.push_str(&format!("{:010} 00000 n \n", o));
    }
    xref.push_str(&format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{}\n%%EOF\n", count, xref_pos));
    out.extend_from_slice(xref.as_bytes());
    out
}

/// Экспорт развёрнутого меша в PDF (возвращает байты файла).
pub fn export_pdf(unfolded: &UnfoldedMesh, config: &PdfExportConfig) -> Result<Vec<u8>, PdfExportError> {
    if unfolded.vertices_2d.is_empty() { return Err(PdfExportError::NoVertices); }
    if unfolded.faces.is_empty() { return Err(PdfExportError::EmptyMesh); }
    let mesh = raw_from_unfolded(unfolded);
    let (pages, eff, total) = build_pages(&mesh, config).map_err(PdfExportError::PdfError)?;
    let parts_total: usize = pages.iter().map(|p| p.labels.len()).sum();
    eprintln!("[EXP54] PDF: страниц={} деталей={} масштаб={:.4} мм/ед.", total, parts_total, eff);
    Ok(build_pdf(&pages, eff, config))
}

/// Экспорт развёртки в PDF-файл (папки создаются автоматически).
pub fn export_pdf_to_file(unfolded: &UnfoldedMesh, config: &PdfExportConfig, path: &str) -> Result<(), PdfExportError> {
    let bytes = export_pdf(unfolded, config)?;
    if let Some(parent) = Path::new(path).parent() {
        if !parent.as_os_str().is_empty() {
            let _ = std::fs::create_dir_all(parent);
        }
    }
    std::fs::write(path, &bytes)?;
    eprintln!("[EXP54] PDF записан: {} ({} байт)", path, bytes.len());
    Ok(())
}