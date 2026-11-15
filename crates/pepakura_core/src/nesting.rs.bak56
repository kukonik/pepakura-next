use serde::{Serialize, Deserialize};
use crate::project::PepaProject;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaperSettings { pub width: f64, pub height: f64 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NestParams { pub paper: PaperSettings }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestPart { pub id: String, pub outlines: Vec<Vec<[f64; 2]>> }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartOverride { pub part_id: String, pub dx: f64, pub dy: f64 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NestMetrics { pub total_sheets: usize, pub total_parts: usize, pub avg_fill_rate: f64 }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NestResult {
    pub parts: Vec<NestPart>,
    pub sheets: Vec<Vec<NestPart>>,
    pub metrics: NestMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneticConfig {
    pub generations: usize,
    pub paper: PaperSettings,
    pub population_size: usize,
    pub mutation_rate: f64,
}

// Заглушки: реальная упаковка — Этап 3.
pub fn nest_unfolds(_project: &PepaProject, _params: &NestParams) -> NestResult { NestResult::default() }
pub fn optimize_nesting_genetic(_parts: &[NestPart], _config: &GeneticConfig) -> NestResult { NestResult::default() }
pub fn apply_overrides_to_nest_result(result: &NestResult, _overrides: &[PartOverride]) -> NestResult { result.clone() }
