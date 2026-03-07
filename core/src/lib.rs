pub mod model;
pub mod unfold;
pub mod export;
pub mod util;

pub use model::Model;
pub use unfold::{UnfoldResult, UnfoldedFace, Seam, LayoutResult};

#[derive(Debug, Clone)]
pub struct UnfoldOptions {
    pub preserve_proportions: bool,
    pub auto_rotate: bool,
    pub spacing: f64,
}

impl Default for UnfoldOptions {
    fn default() -> Self {
        UnfoldOptions {
            preserve_proportions: true,
            auto_rotate: true,
            spacing: 10.0,
        }
    }
}

pub fn unfold_model(model: &Model, _options: &UnfoldOptions) -> UnfoldResult {
    unfold::unfold_model(model)
}

pub fn dummy_function() -> String {
    "Hello from Pepakura Core".to_string()
}
