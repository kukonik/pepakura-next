use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)] pub struct PaperSettings { pub width: f64, pub height: f64 }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct NestParams { pub paper: PaperSettings }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct NestPart { pub id: String, pub outlines: Vec<Vec<[f64; 2]>> }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct PartOverride { pub part_id: String, pub dx: f64, pub dy: f64 }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct NestResult { pub parts: Vec<NestPart>, pub sheets: Vec<Vec<NestPart>> }
#[derive(Debug, Clone, Serialize, Deserialize)] pub struct GeneticConfig { pub generations: usize }

pub fn nest_unfolds(_parts: &[NestPart], _params: &NestParams) -> Result<NestResult, String> { Err("Nesting stub".into()) }
pub fn optimize_nesting_genetic(_parts: &[NestPart], _params: &NestParams, _config: &GeneticConfig) -> Result<NestResult, String> { Err("Genetic nesting stub".into()) }
