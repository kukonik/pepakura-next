use crate::model_generator::{generate_cube, generate_pyramid, generate_sphere, GeneratedMesh};
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedModel {
    pub model_id: String,
    pub vertices: Vec<[f32; 3]>,
    pub faces: Vec<[u32; 3]>,
    pub vertices_count: usize,
    pub faces_count: usize,
    pub model_type: String,
    pub generation_method: String,
    pub processing_time_ms: u64,
}

pub struct AIService {
    pub ollama_url: String,
}

impl AIService {
    pub fn new() -> Self {
        Self {
            ollama_url: "".to_string(),
        }
    }

    pub fn generate_3d_model(&self, description: &str) -> Result<GeneratedModel, String> {
        let start_time = Instant::now();
        
        let (vertices, faces, model_type) = match description.trim().to_lowercase().as_str() {
            "cube" => {
                let mesh = generate_cube(1.0);
                (mesh.vertices, mesh.faces, "Cube")
            }
            "pyramid" => {
                let mesh = generate_pyramid(1.0, 1.0);
                (mesh.vertices, mesh.faces, "Pyramid")
            }
            "sphere" => {
                let mesh = generate_sphere(1.0, 8, 8);
                (mesh.vertices, mesh.faces, "Sphere")
            }
            _ => {
                let mesh = generate_cube(1.0);
                (mesh.vertices, mesh.faces, "Default Cube")
            }
        };

        let vertices_len = vertices.len();
        let faces_len = faces.len();

        let model = GeneratedModel {
            model_id: uuid::Uuid::new_v4().to_string(),
            vertices,
            faces,
            vertices_count: vertices_len,
            faces_count: faces_len,
            model_type: model_type.to_string(),
            generation_method: "Local Primitive Generator".to_string(),
            processing_time_ms: start_time.elapsed().as_millis() as u64,
        };

        Ok(model)
    }
}
