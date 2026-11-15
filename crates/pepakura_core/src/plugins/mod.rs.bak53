//! Система плагинов для pepakura_core.
//! 
//! Позволяет расширять функциональность через внешние плагины:
//! - Импортёры моделей (OBJ, STL, PLY, и др.)
//! - Экспортёры (SVG, PDF, DXF, и др.)
//! - Алгоритмы развёртки (MDS, LSCM, и др.)
//! 
//! ## Пример использования
//! 
//! ```rust
//! use pepakura_core::plugins::{PluginRegistry, ImportPlugin};
//! 
//! let mut registry = PluginRegistry::new();
//! registry.register_importer(Box::new(ObjImporter));
//! 
//! // Использование
//! let importer = registry.get_importer("obj").unwrap();
//! let mesh = importer.import("model.obj").unwrap();
//! ```

mod traits;
mod registry;
pub mod builtin;

pub use traits::*;
pub use registry::*;
