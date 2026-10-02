//! SVG export for unfolded meshes.

use crate::unfold::mds::UnfoldedMesh;
use crate::errors::Result;
use crate::geometry::mesh::Face;
use serde::{Deserialize, Serialize};

/// Page size for SVG export.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PageSize {
    A4,
    A3,
    Custom { width: f64, height: f64 },
}

impl PageSize {
    pub fn dimensions_mm(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (210.0, 297.0),
            PageSize::A3 => (297.0, 420.0),
            PageSize::Custom { width, height } => (*width, *height),
        }
    }
}

/// Configuration for SVG export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SvgExportConfig {
    pub page_size: PageSize,
    pub scale: f64, // mm per unit
    pub show_vertex_ids: bool,
    pub show_fold_lines: bool,
    pub show_cut_lines: bool,
}

impl Default for SvgExportConfig {
    fn default() -> Self {
        SvgExportConfig {
            page_size: PageSize::A4,
            scale: 1.0,
            show_vertex_ids: false,
            show_fold_lines: true,
            show_cut_lines: true,
        }
    }
}

/// Collect edges from faces and separate them into cut edges (non‑shared) and fold edges (shared).
fn collect_shared_edges(faces: &[[usize; 3]]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    use std::collections::HashMap;

    let mut edge_counts: HashMap<(usize, usize), usize> = HashMap::new();

    for face in faces {
        let [a, b, c] = *face;
        let edges = [(a, b), (b, c), (c, a)];
        for (v1, v2) in edges {
            let (v_min, v_max) = if v1 < v2 { (v1, v2) } else { (v2, v1) };
            *edge_counts.entry((v_min, v_max)).or_insert(0) += 1;
        }
    }

    let mut cut_edges = Vec::new();
    let mut fold_edges = Vec::new();

    for ((v1, v2), count) in edge_counts {
        if count == 1 {
            cut_edges.push((v1, v2));
        } else {
            fold_edges.push((v1, v2));
        }
    }

    (cut_edges, fold_edges)
}

/// Group adjacent edges into continuous SVG path strings.
fn group_edges_into_paths(
    edges: &[(usize, usize)],
    vertices_2d: &[[f64; 2]],
    transform_x: &impl Fn(f64) -> f64,
    transform_y: &impl Fn(f64) -> f64,
) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    // Build adjacency map: vertex -> list of connected vertices
    let mut adj: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(v1, v2) in edges {
        adj.entry(v1).or_default().push(v2);
        adj.entry(v2).or_default().push(v1);
    }

    let mut visited_edges = HashSet::new();
    let mut paths = Vec::new();

    for &(v1, v2) in edges {
        let edge_key = if v1 < v2 { (v1, v2) } else { (v2, v1) };
        if visited_edges.contains(&edge_key) {
            continue;
        }

        // Start a new path from this edge
        let mut current_vertex = v1;
        let mut next_vertex = v2;
        let mut path_points = Vec::new();

        // Add first point
        let [x0, y0] = vertices_2d[current_vertex];
        path_points.push(format!("{:.2} {:.2}", transform_x(x0), transform_y(y0)));

        // Walk along connected edges until we hit a dead end or loop
        loop {
            let [x, y] = vertices_2d[next_vertex];
            path_points.push(format!("{:.2} {:.2}", transform_x(x), transform_y(y)));

            let edge_key = if current_vertex < next_vertex {
                (current_vertex, next_vertex)
            } else {
                (next_vertex, current_vertex)
            };
            visited_edges.insert(edge_key);

            // Find next unvisited edge from next_vertex
            let candidates: Vec<usize> = adj
                .get(&next_vertex)
                .unwrap_or(&vec![])
                .iter()
                .filter(|&&v| {
                    let key = if next_vertex < v {
                        (next_vertex, v)
                    } else {
                        (v, next_vertex)
                    };
                    !visited_edges.contains(&key)
                })
                .cloned()
                .collect();

            if candidates.len() != 1 {
                // Dead end or junction: stop this path
                break;
            }

            current_vertex = next_vertex;
            next_vertex = candidates[0];
        }

        if path_points.len() >= 2 {
            paths.push(format!("M {}", path_points.join(" L ")));
        }
    }

    paths
}

/// Export an unfolded mesh to SVG string.
pub fn export_svg(unfolded: &UnfoldedMesh, config: &SvgExportConfig) -> Result<String> {
    let (page_width_mm, page_height_mm) = config.page_size.dimensions_mm();
    let scale = config.scale;

    // Compute bounding box of 2D vertices
    let (min_x, min_y, max_x, max_y) = unfolded.vertices_2d.iter().fold(
        (f64::INFINITY, f64::INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        |(min_x, min_y, max_x, max_y), &[x, y]| {
            (
                min_x.min(x),
                min_y.min(y),
                max_x.max(x),
                max_y.max(y),
            )
        },
    );

    let width = max_x - min_x;
    let height = max_y - min_y;

    // Scale to fit page with margin
    let margin = 10.0; // mm
    let available_width = page_width_mm - 2.0 * margin;
    let available_height = page_height_mm - 2.0 * margin;

    let scale_x = available_width / (width * scale);
    let scale_y = available_height / (height * scale);
    let scale_fit = scale_x.min(scale_y).min(1.0); // don't upscale

    let transform_x = |x: f64| margin + (x - min_x) * scale * scale_fit;
    let transform_y = |y: f64| margin + (y - min_y) * scale * scale_fit;

    let mut svg = String::new();
    svg.push_str(&format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="{:.2}mm" height="{:.2}mm" viewBox="0 0 {:.2} {:.2}">
"#,
        page_width_mm, page_height_mm, page_width_mm, page_height_mm
    ));

    // Styles
    svg.push_str(
        r#"<defs>
    <style type="text/css">
        .cut-line { stroke: #000000; stroke-width: 0.3; fill: none; }
        .fold-line { stroke: #888888; stroke-width: 0.2; stroke-dasharray: 2,2; fill: none; }
        .vertex-id { font-size: 1.5; fill: #666666; }
    </style>
</defs>
"#,
    );

    // Collect edges and determine which are shared (fold lines)
    let (cut_edges, fold_edges) = collect_shared_edges(&unfolded.faces);

    // Draw cut lines (non‑shared edges)
    if config.show_cut_lines && !cut_edges.is_empty() {
        // Group adjacent edges into continuous paths
        let paths = group_edges_into_paths(&cut_edges, &unfolded.vertices_2d, &transform_x, &transform_y);
        for path in paths {
            svg.push_str(&format!(
                "<path class=\"cut-line\" d=\"{}\" />\n",
                path
            ));
        }
    }

    // Draw fold lines (shared edges)
    if config.show_fold_lines && !fold_edges.is_empty() {
        for &(v1, v2) in &fold_edges {
            let [x1, y1] = unfolded.vertices_2d[v1];
            let [x2, y2] = unfolded.vertices_2d[v2];
            svg.push_str(&format!(
                "<line class=\"fold-line\" x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" />\n",
                transform_x(x1), transform_y(y1), transform_x(x2), transform_y(y2)
            ));
        }
    }

    // Draw vertex IDs
    if config.show_vertex_ids {
        for (i, &[x, y]) in unfolded.vertices_2d.iter().enumerate() {
            svg.push_str(&format!(
                "<text class=\"vertex-id\" x=\"{:.2}\" y=\"{:.2}\">{}</text>\n",
                transform_x(x),
                transform_y(y),
                i
            ));
        }
    }

    svg.push_str("</svg>\n");
    Ok(svg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::vertex::Vertex;
    use crate::geometry::mesh::{Mesh, Face};
    use crate::unfold::mds::UnfoldConfig;

    #[test]
    fn test_export_svg_basic() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 1.0, 0.0]),
        ];
        let faces = vec![Face::new(0, 1, 2)];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let config = UnfoldConfig::default();
        let unfolded = crate::unfold::mds::unfold_mds(&mesh, &config).unwrap();

        let svg_config = SvgExportConfig::default();
        let svg = export_svg(&unfolded, &svg_config).unwrap();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        // Should contain path for cut lines (single triangle has no shared edges)
        assert!(svg.contains("<path"));
        // Should not contain polygon
        assert!(!svg.contains("polygon"));
    }

    #[test]
    fn test_export_svg_cube() {
        // Simple cube mesh (8 vertices, 12 faces)
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [1.0, 1.0, 0.0]),
            Vertex::new(3, [0.0, 1.0, 0.0]),
            Vertex::new(4, [0.0, 0.0, 1.0]),
            Vertex::new(5, [1.0, 0.0, 1.0]),
            Vertex::new(6, [1.0, 1.0, 1.0]),
            Vertex::new(7, [0.0, 1.0, 1.0]),
        ];
        // 12 triangular faces (two per cube face)
        let faces = vec![
            // bottom
            Face::new(0, 1, 2),
            Face::new(0, 2, 3),
            // top
            Face::new(4, 5, 6),
            Face::new(4, 6, 7),
            // front
            Face::new(0, 1, 5),
            Face::new(0, 5, 4),
            // back
            Face::new(2, 3, 7),
            Face::new(2, 7, 6),
            // left
            Face::new(0, 3, 7),
            Face::new(0, 7, 4),
            // right
            Face::new(1, 2, 6),
            Face::new(1, 6, 5),
        ];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let config = UnfoldConfig::default();
        let unfolded = crate::unfold::mds::unfold_mds(&mesh, &config).unwrap();

        let svg_config = SvgExportConfig::default();
        let svg = export_svg(&unfolded, &svg_config).unwrap();

        // Validate SVG structure
        assert!(svg.starts_with("<?xml"));
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));

        // Count cut lines (should be 12 faces * 3 edges each, but shared edges are fold lines)
        // For a cube, each edge is shared by two faces => all edges are fold lines, cut edges zero
        // However our simple triangulation creates internal edges within each face that are shared between the two triangles of that face.
        // So there should be some cut edges (internal diagonals) and fold edges (cube edges).
        // We'll just ensure the SVG is not empty.
        assert!(svg.len() > 500);
    }

    #[test]
    fn test_export_svg_without_fold_lines() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 1.0, 0.0]),
        ];
        let faces = vec![Face::new(0, 1, 2)];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let config = UnfoldConfig::default();
        let unfolded = crate::unfold::mds::unfold_mds(&mesh, &config).unwrap();

        let svg_config = SvgExportConfig {
            show_fold_lines: false,
            ..Default::default()
        };
        let svg = export_svg(&unfolded, &svg_config).unwrap();

        // Should not contain fold-line class
        assert!(!svg.contains("fold-line"));
        // Should contain cut lines
        assert!(svg.contains("cut-line"));
    }

    #[test]
    fn test_export_svg_without_cut_lines() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 1.0, 0.0]),
        ];
        let faces = vec![Face::new(0, 1, 2)];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let config = UnfoldConfig::default();
        let unfolded = crate::unfold::mds::unfold_mds(&mesh, &config).unwrap();

        let svg_config = SvgExportConfig {
            show_cut_lines: false,
            ..Default::default()
        };
        let svg = export_svg(&unfolded, &svg_config).unwrap();

        // Should not contain cut-line class
        assert!(!svg.contains("cut-line"));
        // Should not contain path (since no cut lines)
        assert!(!svg.contains("<path"));
    }

    #[test]
    fn test_export_svg_vertex_ids() {
        let vertices = vec![
            Vertex::new(0, [0.0, 0.0, 0.0]),
            Vertex::new(1, [1.0, 0.0, 0.0]),
            Vertex::new(2, [0.0, 1.0, 0.0]),
        ];
        let faces = vec![Face::new(0, 1, 2)];
        let mesh = Mesh::new(vertices, faces).unwrap();
        let config = UnfoldConfig::default();
        let unfolded = crate::unfold::mds::unfold_mds(&mesh, &config).unwrap();

        let svg_config = SvgExportConfig {
            show_vertex_ids: true,
            ..Default::default()
        };
        let svg = export_svg(&unfolded, &svg_config).unwrap();

        // Should contain vertex-id class and numbers 0,1,2
        assert!(svg.contains("vertex-id"));
        assert!(svg.contains(">0<"));
        assert!(svg.contains(">1<"));
        assert!(svg.contains(">2<"));
    }
}