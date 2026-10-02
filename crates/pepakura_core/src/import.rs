//! Парсеры 3D-форматов (OBJ).

use crate::geometry::{Mesh, Vertex, Face};

#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("Ошибка парсинга OBJ: {0}")]
    ObjParseError(String),
    #[error("Не найдено вершин")]
    NoVertices,
}

pub fn parse_obj_str(input: &str) -> Result<Mesh, ImportError> {
    let obj_data = obj::ObjData::load_buf(input.as_bytes())
        .map_err(|e| ImportError::ObjParseError(e.to_string()))?;

    let positions = &obj_data.position;
    if positions.is_empty() {
        return Err(ImportError::NoVertices);
    }

    let mut mesh = Mesh::new("Imported");
    for (i, pos) in positions.iter().enumerate() {
        mesh.add_vertex(Vertex::new(i, [pos[0] as f64, pos[1] as f64, pos[2] as f64]));
    }

    for object in &obj_data.objects {
        for group in &object.groups {
            for poly in &group.polys {
                let indices: Vec<usize> = poly.0.iter().map(|pi| pi.0).collect();
                let to_usize = |i: usize| if i == 0 { i } else { i - 1 };
                match indices.len() {
                    3 => {
                        let a = to_usize(indices[0]);
                        let b = to_usize(indices[1]);
                        let c = to_usize(indices[2]);
                        if a < mesh.vertices.len() && b < mesh.vertices.len() && c < mesh.vertices.len() {
                            mesh.add_face(Face::new(a, b, c));
                        }
                    }
                    _ => {
                        let base = to_usize(indices[0]);
                        for k in 1..indices.len()-1 {
                            let a = base;
                            let b = to_usize(indices[k]);
                            let c = to_usize(indices[k+1]);
                            if a < mesh.vertices.len() && b < mesh.vertices.len() && c < mesh.vertices.len() {
                                mesh.add_face(Face::new(a, b, c));
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(mesh)
}
