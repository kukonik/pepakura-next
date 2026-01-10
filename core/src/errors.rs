//! Error types for the Pepakura engine
use std::fmt;

/// Result type for Pepakura operations
pub type Result<T> = std::result::Result<T, PepakuraError>;

/// Pepakura-specific error types
#[derive(Debug, Clone)]
pub enum PepakuraError {
    InvalidMesh(String),
    InvalidTriangle(String),
    GeometryError(String),
    UnfoldingFailed(String),
    LayoutFailed(String),
    ExportError(String),
    IOError(String),
    ParseError(String),
    NotFound(String),
    InvalidInput(String),
}

impl fmt::Display for PepakuraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PepakuraError::InvalidMesh(msg) => write!(f, "Invalid mesh: {}", msg),
            PepakuraError::InvalidTriangle(msg) => write!(f, "Invalid triangle: {}", msg),
            PepakuraError::GeometryError(msg) => write!(f, "Geometry error: {}", msg),
            PepakuraError::UnfoldingFailed(msg) => write!(f, "Unfolding failed: {}", msg),
            PepakuraError::LayoutFailed(msg) => write!(f, "Layout failed: {}", msg),
            PepakuraError::ExportError(msg) => write!(f, "Export error: {}", msg),
            PepakuraError::IOError(msg) => write!(f, "IO error: {}", msg),
            PepakuraError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            PepakuraError::NotFound(msg) => write!(f, "Not found: {}", msg),
            PepakuraError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for PepakuraError {}