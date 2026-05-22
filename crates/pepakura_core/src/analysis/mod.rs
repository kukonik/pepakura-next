//! # Analysis Module
//!
//! Модули для анализа 3D моделей.
//!
//! ## Подмодули
//!
//! - [`mesh_stats`] - вычисление статистики меша
//! - [`mesh_analyzer`] - анализ с LLM-рекомендациями
//! - [`distortion_analysis`] - анализ искажений развёртки
//! - [`nesting_optimization`] - оптимизация раскладки
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::analysis::mesh_analyzer::MeshAnalyzer;
//! use pepakura_core::geometry::Mesh;
//!
//! let mut analyzer = MeshAnalyzer::new();
//! let mesh = Mesh::new("model");
//! let result = analyzer.analyze(&mesh);
//!
//! println!("{}", result.stats.summary());
//!
//! if let Some(ai) = &result.ai_analysis {
//!     println!("Сложность: {}", ai.difficulty);
//! }
//! ```

pub mod mesh_stats;
pub mod mesh_analyzer;
pub mod distortion_analysis;
pub mod nesting_optimization;

pub use mesh_stats::MeshStats;
pub use mesh_analyzer::{
    MeshAnalyzer,
    AnalysisResult,
    AiAnalysisResult,
    LlmStatusInfo,
    MeshProvider,
};
pub use distortion_analysis::{
    DistortionAnalyzer,
    DistortionAnalysisResult,
    DistortionStats,
    FaceHeatMapEntry,
    ProblematicFace,
    FaceIssueType,
    generate_distortion_advice,
};
pub use nesting_optimization::{
    NestingOptimizer,
    NestingAnalysisResult,
    SheetAnalysis,
    NestingRecommendation,
    AssemblyComplexity,
    generate_nesting_advice,
};
