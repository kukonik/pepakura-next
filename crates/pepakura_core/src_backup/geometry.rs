use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex { pub x: f64, pub y: f64, pub z: f64 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub vertices: [usize; 3],
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}
