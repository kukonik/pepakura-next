use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaMaterial {
    pub id: u32,
    pub name: String,
    pub diffuse_color: [f32; 4],
    pub texture_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaMesh {
    pub positions: Vec<f32>,
    pub indices: Vec<u32>,
    pub normals: Vec<f32>,
    pub uvs: Vec<f32>,
    pub material_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaBoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaScene {
    /// Версия сцены
    pub scene_version: String,
    pub meshes: Vec<PepaMesh>,
    pub materials: Vec<PepaMaterial>,
    pub bounding_box: Option<PepaBoundingBox>,
}

use crate::pdo_parser::PdoModel;

pub trait FromPdoModel {
    fn from_pdo_model(pdo: &PdoModel) -> Self;
}

impl FromPdoModel for PepaScene {
    fn from_pdo_model(pdo: &PdoModel) -> Self {
        // Версия сцены
        let scene_version = "1.0".to_string();
        
        // Вычисляем bounding box
        let mut min = [f32::MAX, f32::MAX, f32::MAX];
        let mut max = [f32::MIN, f32::MIN, f32::MIN];
        
        for vertex in &pdo.vertices {
            min[0] = min[0].min(vertex.x);
            min[1] = min[1].min(vertex.y);
            min[2] = min[2].min(vertex.z);
            
            max[0] = max[0].max(vertex.x);
            max[1] = max[1].max(vertex.y);
            max[2] = max[2].max(vertex.z);
        }
        
        // Создаем меш из данных PDO
        let mut positions = Vec::new();
        let mut indices = Vec::new();
        
        for vertex in &pdo.vertices {
            positions.push(vertex.x);
            positions.push(vertex.y);
            positions.push(vertex.z);
        }
        
        for face in &pdo.faces {
            // Конвертируем полигоны в треугольники (фан-триангуляция)
            for i in 1..face.indices.len().saturating_sub(1) {
                indices.push(face.indices[0]);
                indices.push(face.indices[i]);
                indices.push(face.indices[i + 1]);
            }
        }
        
        // Вычисляем нормали
        let mut normals = vec![0.0; positions.len()];
        for i in (0..indices.len()).step_by(3) {
            let i0 = indices[i] as usize * 3;
            let i1 = indices[i + 1] as usize * 3;
            let i2 = indices[i + 2] as usize * 3;
            
            let ax = positions[i1] - positions[i0];
            let ay = positions[i1 + 1] - positions[i0 + 1];
            let az = positions[i1 + 2] - positions[i0 + 2];
            
            let bx = positions[i2] - positions[i0];
            let by = positions[i2 + 1] - positions[i0 + 1];
            let bz = positions[i2 + 2] - positions[i0 + 2];
            
            let nx = ay * bz - az * by;
            let ny = az * bx - ax * bz;
            let nz = ax * by - ay * bx;
            
            // Добавляем нормаль к каждой вершине треугольника
            normals[i0] += nx;
            normals[i0 + 1] += ny;
            normals[i0 + 2] += nz;
            
            normals[i1] += nx;
            normals[i1 + 1] += ny;
            normals[i1 + 2] += nz;
            
            normals[i2] += nx;
            normals[i2 + 1] += ny;
            normals[i2 + 2] += nz;
        }
        
        // Нормализуем нормали
        for i in (0..normals.len()).step_by(3) {
            let nx = normals[i];
            let ny = normals[i + 1];
            let nz = normals[i + 2];
            let len = (nx * nx + ny * ny + nz * nz).sqrt();
            
            if len > 0.0 {
                normals[i] = nx / len;
                normals[i + 1] = ny / len;
                normals[i + 2] = nz / len;
            }
        }
        
        // Создаем материалы (пока без текстур)
        let materials = pdo.textures.iter().map(|texture| {
            PepaMaterial {
                id: texture.id,
                name: format!("Texture_{}", texture.id),
                diffuse_color: [1.0, 1.0, 1.0, 1.0],
                texture_id: Some(texture.id),
            }
        }).collect();
        
        // Создаем меш
        let meshes = vec![PepaMesh {
            positions,
            indices,
            normals,
            uvs: vec![], // Пока без UV-координат
            material_id: if pdo.textures.is_empty() { None } else { Some(0) },
        }];
        
        PepaScene {
            scene_version,
            meshes,
            materials,
            bounding_box: Some(PepaBoundingBox { min, max }),
        }
    }
}
