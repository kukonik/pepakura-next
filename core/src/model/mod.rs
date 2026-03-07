pub mod io_obj;

#[derive(Debug, Clone)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone)]
pub struct Face {
    pub vertices: Vec<usize>,
}

impl Model {
    pub fn new() -> Self {
        Model {
            vertices: Vec::new(),
            faces: Vec::new(),
        }
    }
}
