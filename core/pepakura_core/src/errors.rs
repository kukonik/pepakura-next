//! Error types for the Pepakura engine
use thiserror::Error;

/// Result type for Pepakura operations
pub type Result<T> = std::result::Result<T, PepakuraError>;

/// Pepakura-specific error types
#[derive(Debug, Clone, Error)]
pub enum PepakuraError {
    #[error("Invalid mesh: {0}")]
    InvalidMesh(String),

    #[error("Non-manifold edge: {0}")]
    NonManifoldEdge(String),

    #[error("Degenerate face: {0}")]
    DegenerateFace(String),

    #[error("Invalid triangle: {0}")]
    InvalidTriangle(String),

    #[error("Geometry error: {0}")]
    GeometryError(String),

    #[error("Unfolding failed: {0}")]
    UnfoldingFailed(String),

    #[error("Layout failed: {0}")]
    LayoutFailed(String),

    #[error("Export error: {0}")]
    ExportError(String),

    #[error("IO error: {0}")]
    IOError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}