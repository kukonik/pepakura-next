use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnwrapMetrics {
    pub total_pieces: u32,
    pub total_pages: u32,
    pub total_folds: u32,
    pub total_tabs: u32,
    pub complexity_score: f32,
}

// [58-B] Page/UnwrapResult - контракт platform\desktop\ui-desktop\src-tauri\src\commands.rs.
// pieces переиспользует PapercraftPiece из papercraft_adapter (единый тип развёртки).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Page {
    pub page_number: u32,
    pub piece_ids: Vec<u32>,
    pub utilization_percent: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnwrapResult {
    pub engine_used: String,
    pub face_pieces: Vec<u32>,
    pub pieces: Vec<crate::papercraft_adapter::PapercraftPiece>,
    pub pages: Vec<Page>,
    pub metrics: UnwrapMetrics,
    pub sheet_width: f64,
    pub sheet_height: f64,
    pub margin: f64,
    pub total_area: f64,
    pub total_perimeter: f64,
    pub vertices_2d: Vec<[f64; 2]>,
    pub faces: Vec<Vec<usize>>,
}