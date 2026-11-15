//! Traits для конвертации PDO → PepaScene

use crate::pdo_parser::PdoModel;
use crate::conversion::error::Result;

/// Базовый trait для конвертации PDO → целевая структура
pub trait FromPdoModel {
    fn from_pdo_model(pdo: &PdoModel) -> Self;
}

/// Trait для конвертации с отслеживанием прогресса
pub trait ConvertWithProgress: Sized {
    type Progress: ProgressState;

    fn convert_with_progress(
        pdo: &PdoModel,
        callback: impl FnMut(&Self::Progress),
    ) -> Result<Self>;
}

/// Trait для асинхронной конвертации
#[allow(async_fn_in_trait)]
pub trait AsyncConvertible: Sized {
    async fn convert_async(pdo: &PdoModel) -> Result<Self>;

    async fn convert_chunked(pdo: &PdoModel, chunk_size: usize) -> Result<Self>;
}

/// Состояние прогресса конвертации
pub trait ProgressState {
    fn current_step(&self) -> usize;
    fn total_steps(&self) -> usize;
    fn percent_complete(&self) -> f32 {
        if self.total_steps() == 0 {
            0.0
        } else {
            (self.current_step() as f32 / self.total_steps() as f32) * 100.0
        }
    }
    fn description(&self) -> &str;
}

/// Trait для валидации PDO данных
pub trait PdoValidatable {
    fn validate(&self) -> ValidationResult;
    fn is_valid(&self) -> bool {
        self.validate().is_valid
    }
}

/// Результат валидации
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            errors: vec![],
            warnings: vec![],
        }
    }

    pub fn error(error: impl Into<String>) -> Self {
        Self {
            is_valid: false,
            errors: vec![error.into()],
            warnings: vec![],
        }
    }

    pub fn warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    pub fn merge(&mut self, other: &ValidationResult) {
        self.is_valid &= other.is_valid;
        self.errors.extend(other.errors.iter().cloned());
        self.warnings.extend(other.warnings.iter().cloned());
    }

    /// Добавляет ошибку к результату (помечает результат как невалидный)
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.is_valid = false;
        self.errors.push(error.into());
    }

    /// Добавляет предупреждение к результату
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::ok()
    }
}

/// Конкретные реализации прогресса для разных этапов конвертации

/// Прогресс конвертации вершин
#[derive(Debug, Clone)]
pub struct VertexProgress {
    pub processed: usize,
    pub total: usize,
    pub description: String,
}

impl ProgressState for VertexProgress {
    fn current_step(&self) -> usize {
        self.processed
    }

    fn total_steps(&self) -> usize {
        self.total
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Прогресс конвертации граней
#[derive(Debug, Clone)]
pub struct FaceProgress {
    pub processed: usize,
    pub total: usize,
    pub triangulated: usize,
    pub description: String,
}

impl ProgressState for FaceProgress {
    fn current_step(&self) -> usize {
        self.processed
    }

    fn total_steps(&self) -> usize {
        self.total
    }

    fn description(&self) -> &str {
        &self.description
    }
}

/// Общий прогресс конвертации сцены
#[derive(Debug, Clone)]
pub struct SceneConversionProgress {
    pub current_stage: ConversionStage,
    pub stage_progress: f32,
    pub overall_progress: f32,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionStage {
    Parsing,
    VertexConversion,
    FaceConversion,
    NormalCalculation,
    UvMapping,
    MaterialConversion,
    UnfoldExtraction,
    Finalization,
}

impl ProgressState for SceneConversionProgress {
    fn current_step(&self) -> usize {
        (self.overall_progress / 100.0 * 8.0) as usize
    }

    fn total_steps(&self) -> usize {
        8
    }

    fn description(&self) -> &str {
        &self.description
    }
}

impl SceneConversionProgress {
    pub fn new(stage: ConversionStage, stage_progress: f32) -> Self {
        let overall = match stage {
            ConversionStage::Parsing => stage_progress * 0.1,
            ConversionStage::VertexConversion => 10.0 + stage_progress * 0.3,
            ConversionStage::FaceConversion => 40.0 + stage_progress * 0.3,
            ConversionStage::NormalCalculation => 70.0 + stage_progress * 0.1,
            ConversionStage::UvMapping => 80.0 + stage_progress * 0.1,
            ConversionStage::MaterialConversion => 90.0 + stage_progress * 0.05,
            ConversionStage::UnfoldExtraction => 95.0 + stage_progress * 0.04,
            ConversionStage::Finalization => 99.0 + stage_progress * 0.01,
        };

        Self {
            current_stage: stage,
            stage_progress,
            overall_progress: overall,
            description: Self::stage_description(stage).to_string(),
        }
    }

    fn stage_description(stage: ConversionStage) -> &'static str {
        match stage {
            ConversionStage::Parsing => "Парсинг PDO файла",
            ConversionStage::VertexConversion => "Конвертация вершин",
            ConversionStage::FaceConversion => "Конвертация граней",
            ConversionStage::NormalCalculation => "Расчет нормалей",
            ConversionStage::UvMapping => "UV маппинг",
            ConversionStage::MaterialConversion => "Обработка материалов",
            ConversionStage::UnfoldExtraction => "Извлечение разверток",
            ConversionStage::Finalization => "Финализация сцены",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_ok() {
        let result = ValidationResult::ok();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_validation_result_error() {
        let result = ValidationResult::error("Test error");
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert_eq!(result.errors[0], "Test error");
    }

    #[test]
    fn test_validation_result_merge() {
        let mut result1 = ValidationResult::ok().warning("Warning 1");
        let result2 = ValidationResult::error("Error 1");

        result1.merge(&result2);

        assert!(!result1.is_valid);
        assert_eq!(result1.errors.len(), 1);
        assert_eq!(result1.warnings.len(), 1);
    }

    #[test]
    fn test_scene_progress() {
        let progress = SceneConversionProgress::new(ConversionStage::VertexConversion, 50.0);
        assert!(progress.overall_progress > 10.0);
        assert!(progress.overall_progress < 40.0);
    }
}
