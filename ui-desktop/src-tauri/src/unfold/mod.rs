//! Модуль движка развёртки 3D-моделей в 2D-шаблоны
//!
//! Этот модуль предоставляет функциональность для преобразования 3D-моделей
//! в набор 2D-шаблонов, которые можно распечатать и собрать в бумажную модель.

pub mod types;
pub mod engine;
pub mod optimizer;

// Реэкспортируем основные типы и функции
pub use types::*;
pub use engine::UnfoldEngine;
pub use optimizer::LayoutOptimizer;