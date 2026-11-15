// [56-E1] Промпты для LLM-анализа (std-only стаб; реальный LLM-пайплайн - отдельный этап)
use serde::{Deserialize, Serialize};

pub const DIFFICULTY_SYSTEM_PROMPT: &str = "You are a papercraft analysis assistant. Reply with strict JSON only.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStatsPrompt {
    pub vertex_count: u32,
    pub face_count: u32,
    pub edge_count: u32,
    pub bbox_size: [f64; 3],
    pub surface_area: f64,
    pub volume: Option<f64>,
    pub avg_face_area: f64,
    pub min_face_area: f64,
    pub max_face_area: f64,
    pub isolated_parts: u32,
}

impl MeshStatsPrompt {
    pub fn to_text(&self) -> String {
        format!(
            "Mesh stats: vertices={} faces={} edges={} bbox=[{:.1},{:.1},{:.1}] area={:.1} volume={} avg_face_area={:.4} min_face_area={:.6} max_face_area={:.2} isolated_parts={}",
            self.vertex_count, self.face_count, self.edge_count,
            self.bbox_size[0], self.bbox_size[1], self.bbox_size[2],
            self.surface_area,
            self.volume.map(|v| format!("{:.1}", v)).unwrap_or_else(|| "n/a".to_string()),
            self.avg_face_area, self.min_face_area, self.max_face_area, self.isolated_parts
        )
    }
}

pub fn create_difficulty_prompt(stats: &MeshStatsPrompt) -> String {
    format!(
        "Analyze papercraft difficulty of the model. Reply with strict JSON: difficulty_analysis {{difficulty, overall_score, unfoldability_score, detail_score, printability_score, reasoning}}, issues [], recommendations [].\n{}",
        stats.to_text()
    )
}