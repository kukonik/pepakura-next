use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfoldResult {
    pub vertices_2d: Vec<[f32; 2]>,
    pub faces: Vec<[usize; 3]>,
    pub island_ids: Vec<usize>,
}
