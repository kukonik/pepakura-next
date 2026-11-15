//! Модуль экспорта в различные форматы.
//!
//! Поддерживаемые форматы:
//! - SVG — векторный формат для печати
//! - PDF — готов к печати
//! - DXF — для CAD-систем и лазерной резки
//! - Textures — UV-развёртка с текстурами

mod svg;
mod pdf;
mod dxf;
mod texture;

pub use svg::*;
pub use pdf::*;
pub use dxf::*;
pub use texture::*;
