//! Pepakura Next Core Engine
//! 
//! Complete 3D paper unfolding and layout engine with optimization algorithms.
//! Handles mesh processing, unfolding, layout, and PDF export.

pub mod geometry;
pub mod mesh;
pub mod unfold;
pub mod layout;
pub mod engine;
pub mod errors;
pub mod types;

pub use engine::{PepakuraEngine, UnfoldResult, LayoutResult};
pub use types::{Vec2, Vec3, Triangle, Mesh};
pub use errors::{PepakuraError, Result};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // Basic smoke test to ensure modules are properly initialized
        let _ = geometry::Vector3::new(0.0, 0.0, 0.0);
    }
}