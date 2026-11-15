//! # Analysis Module
//!
//! Модули для анализа 3D моделей.

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