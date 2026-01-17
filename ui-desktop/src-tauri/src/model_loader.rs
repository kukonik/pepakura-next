use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use tobj;
use crate::unfold::types::{Model3D, Vertex3D, Face3D};

pub struct ModelLoader;

impl ModelLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_obj<P: AsRef<Path>>(&self, path: P) -> Result<Model3D, String> {
        let file = File::open(path).map_err(|e| format!("Не удалось открыть OBJ файл: {}", e))?;
        let mut reader = BufReader::new(file);

        let (models, _) = tobj::load_obj_buf(
            &mut reader,
            &tobj::LoadOptions::default(),
            |_p| {
                // Возвращаем пустой результат для материалов
                Ok((Vec::new(), Default::default()))
            }
        ).map_err(|e| format!("Ошибка загрузки OBJ: {}", e))?;

        if models.is_empty() {
            return Err("OBJ файл не содержит моделей".to_string());
        }

        let mesh = &models[0].mesh;

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        // Загрузка вершин
        for i in (0..mesh.positions.len()).step_by(3) {
            vertices.push(Vertex3D {
                x: mesh.positions[i],
                y: mesh.positions[i + 1],
                z: mesh.positions[i + 2],
            });
        }

        // Загрузка граней
        for i in (0..mesh.indices.len()).step_by(3) {
            let face_index = (i / 3) as u32;
            faces.push(Face3D {
                id: face_index,
                vertices: [
                    mesh.indices[i] as u32,
                    mesh.indices[i + 1] as u32,
                    mesh.indices[i + 2] as u32,
                ],
                normal: if let Some(normal_index) = mesh.normal_indices.get(i) {
                    let idx = *normal_index as usize * 3;
                    if idx + 2 < mesh.normals.len() {
                        [
                            mesh.normals[idx],
                            mesh.normals[idx + 1],
                            mesh.normals[idx + 2],
                        ]
                    } else {
                        [0.0, 0.0, 0.0]
                    }
                } else {
                    [0.0, 0.0, 0.0]
                },
            });
        }

        Ok(Model3D {
            vertices,
            faces,
        })
    }
}

impl Default for ModelLoader {
    fn default() -> Self {
        Self::new()
    }
}