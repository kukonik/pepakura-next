//! # Nesting Optimization Analysis
//!
//! AI-анализ и рекомендации по оптимизации раскладки деталей на листе.
//!
//! ## Функционал
//!
//! - **Анализ заполненности листов** - статистика использования пространства
//! - **Рекомендации по формату бумаги** - выбор оптимального формата
//! - **Оптимизация масштаба модели** - подбор масштаба для минимизации листов
//! - **Группировка деталей** - советы по логической группировке
//! - **AI рекомендации** - генерация советов через LLM
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::nesting::NestResult;
//! use pepakura_core::analysis::nesting_optimization::NestingOptimizer;
//!
//! let nest_result = NestResult { /* ... */ };
//! let optimizer = NestingOptimizer::new();
//! let analysis = optimizer.analyze(&nest_result);
//!
//! println!("Заполненность: {:.1}%", analysis.avg_fill_rate);
//! println!("Рекомендации: {:?}", analysis.recommendations);
//! ```

use crate::nesting::{NestResult, NestSheet};
use serde::{Deserialize, Serialize};

/// Результат анализа раскладки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestingAnalysisResult {
    /// Общая метрика использования пространства
    pub space_efficiency_score: f64,
    /// Процент заполненности (средний по всем листам)
    pub avg_fill_rate: f64,
    /// Количество использованных листов
    pub sheets_count: usize,
    /// Общее количество деталей
    pub total_parts: usize,
    /// Детали по листам
    pub sheets_analysis: Vec<SheetAnalysis>,
    /// Рекомендации по оптимизации
    pub recommendations: Vec<NestingRecommendation>,
    /// Оценка оптимальности формата бумаги
    pub paper_size_optimality: f64,
    /// Рекомендуемый формат бумаги
    pub suggested_paper_format: Option<String>,
    /// Возможная экономия при оптимизации (в процентах)
    pub potential_savings_percent: f64,
    /// Оценка сложности сборки (на основе разброса деталей)
    pub assembly_complexity: AssemblyComplexity,
}

/// Анализ одного листа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetAnalysis {
    /// Индекс листа
    pub sheet_index: usize,
    /// Процент заполненности
    pub fill_rate: f64,
    /// Количество деталей
    pub parts_count: usize,
    /// Общая площадь деталей
    pub parts_area: f64,
    /// Доступная площадь
    pub available_area: f64,
    /// Оценка компактности размещения
    pub compactness_score: f64,
    /// Количество "от孤岛" деталей (далеко от других)
    pub isolated_parts_count: usize,
    ///bounding box деталей
    pub parts_bbox: BoundingBox2D,
}

/// 2D Bounding box
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox2D {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

/// Рекомендация по оптимизации раскладки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestingRecommendation {
    /// Тип рекомендации
    pub recommendation_type: RecommendationType,
    /// Описание
    pub description: String,
    /// Потенциальная выгода (в процентах)
    pub potential_benefit: f64,
    /// Приоритет: "high", "medium", "low"
    pub priority: String,
    /// Категория: "paper", "layout", "scale", "grouping", "rotation"
    pub category: String,
}

/// Тип рекомендации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    /// Изменить формат бумаги
    ChangePaperFormat,
    /// Оптимизировать масштаб модели
    AdjustScale,
    /// Изменить параметры раскладки
    ChangeLayoutParams,
    /// Группировать детали
    GroupParts,
    /// Использовать другой угол поворота
    AdjustRotation,
    /// Применить генетический алгоритм
    UseGeneticAlgorithm,
}

impl std::fmt::Display for RecommendationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecommendationType::ChangePaperFormat => write!(f, "Изменить формат бумаги"),
            RecommendationType::AdjustScale => write!(f, "Оптимизировать масштаб"),
            RecommendationType::ChangeLayoutParams => write!(f, "Изменить параметры раскладки"),
            RecommendationType::GroupParts => write!(f, "Группировать детали"),
            RecommendationType::AdjustRotation => write!(f, "Настроить поворот"),
            RecommendationType::UseGeneticAlgorithm => write!(f, "Применить генетический алгоритм"),
        }
    }
}

/// Оценка сложности сборки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyComplexity {
    /// Общий уровень сложности (0.0 - 1.0)
    pub overall_complexity: f64,
    /// Среднее расстояние между связанными деталями
    pub avg_connected_distance: f64,
    /// Количество листов (влияет на сложность)
    pub sheets_factor: f64,
    /// Разброс размеров деталей
    pub size_variance: f64,
}

/// Оптимизатор раскладки
pub struct NestingOptimizer {
    /// Целевая заполненность (по умолчанию 75%)
    pub target_fill_rate: f64,
    /// Минимальная приемлемая заполненность
    pub min_acceptable_fill_rate: f64,
}

impl NestingOptimizer {
    /// Создать оптимизатор с настройками по умолчанию
    pub fn new() -> Self {
        Self {
            target_fill_rate: 0.75,
            min_acceptable_fill_rate: 0.50,
        }
    }

    /// Создать оптимизатор с кастомными целевыми значениями
    pub fn with_targets(target: f64, minimum: f64) -> Self {
        Self {
            target_fill_rate: target,
            min_acceptable_fill_rate: minimum,
        }
    }

    /// Проанализировать раскладку и дать рекомендации
    pub fn analyze(&self, nest_result: &NestResult) -> NestingAnalysisResult {
        let mut sheets_analysis = Vec::new();
        let mut total_fill_rate = 0.0;

        // Анализируем каждый лист
        for (idx, sheet) in nest_result.sheets.iter().enumerate() {
            let sheet_analysis = self.analyze_sheet(sheet, idx);
            total_fill_rate += sheet_analysis.fill_rate;
            sheets_analysis.push(sheet_analysis);
        }

        let avg_fill_rate = if sheets_analysis.is_empty() {
            0.0
        } else {
            total_fill_rate / sheets_analysis.len() as f64
        };

        // Оценка оптимальности бумаги
        let paper_optimality = self.evaluate_paper_optimality(nest_result);

        // Рекомендуемый формат бумаги
        let suggested_format = self.suggest_paper_format(nest_result);

        // Возможная экономия
        let potential_savings = self.calculate_potential_savings(nest_result, avg_fill_rate);

        // Сложность сборки
        let assembly_complexity = self.evaluate_assembly_complexity(nest_result);

        // Генерируем рекомендации
        let recommendations = self.generate_recommendations(
            nest_result,
            &sheets_analysis,
            avg_fill_rate,
            paper_optimality,
        );

        // Общая оценка эффективности
        let space_efficiency = self.compute_space_efficiency(
            avg_fill_rate,
            paper_optimality,
            nest_result.sheets.len(),
        );

        NestingAnalysisResult {
            space_efficiency_score: space_efficiency,
            avg_fill_rate,
            sheets_count: nest_result.sheets.len(),
            total_parts: nest_result.metrics.total_parts as usize,
            sheets_analysis,
            recommendations,
            paper_size_optimality: paper_optimality,
            suggested_paper_format: suggested_format,
            potential_savings_percent: potential_savings,
            assembly_complexity,
        }
    }

    /// Проанализировать один лист
    fn analyze_sheet(&self, sheet: &NestSheet, index: usize) -> SheetAnalysis {
        let available_width = sheet.width_mm as f64 - 2.0 * sheet.margin_mm as f64;
        let available_height = sheet.height_mm as f64 - 2.0 * sheet.margin_mm as f64;
        let available_area = available_width * available_height;

        // Вычисляем площадь деталей
        let parts_area: f64 = sheet.parts.iter()
            .map(|p| (p.width_mm as f64) * (p.height_mm as f64))
            .sum();

        let fill_rate = if available_area > 0.0 {
            parts_area / available_area
        } else {
            0.0
        };

        // Bounding box всех деталей
        let parts_bbox = self.compute_parts_bbox(sheet);

        // Оценка компактности
        let compactness = self.evaluate_compactness(sheet, &parts_bbox);

        // Подсчет "от孤岛" деталей
        let isolated_count = self.count_isolated_parts(sheet);

        SheetAnalysis {
            sheet_index: index,
            fill_rate,
            parts_count: sheet.parts.len(),
            parts_area,
            available_area,
            compactness_score: compactness,
            isolated_parts_count: isolated_count,
            parts_bbox,
        }
    }

    /// Вычислить bounding box всех деталей на листе
    fn compute_parts_bbox(&self, sheet: &NestSheet) -> BoundingBox2D {
        if sheet.parts.is_empty() {
            return BoundingBox2D {
                min_x: 0.0,
                min_y: 0.0,
                max_x: 0.0,
                max_y: 0.0,
            };
        }

        let mut min_x = f64::MAX;
        let mut min_y = f64::MAX;
        let mut max_x = f64::MIN;
        let mut max_y = f64::MIN;

        for part in &sheet.parts {
            let x = part.x_mm as f64;
            let y = part.y_mm as f64;
            let w = part.width_mm as f64;
            let h = part.height_mm as f64;

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x + w);
            max_y = max_y.max(y + h);
        }

        BoundingBox2D {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Оценить компактность размещения
    fn evaluate_compactness(&self, sheet: &NestSheet, bbox: &BoundingBox2D) -> f64 {
        if sheet.parts.is_empty() {
            return 1.0;
        }

        let bbox_area = (bbox.max_x - bbox.min_x) * (bbox.max_y - bbox.min_y);
        if bbox_area < 1e-6 {
            return 1.0;
        }

        let parts_area: f64 = sheet.parts.iter()
            .map(|p| (p.width_mm as f64) * (p.height_mm as f64))
            .sum();

        // Отношение площади деталей к bounding box
        let ratio = parts_area / bbox_area;
        ratio.min(1.0)
    }

    /// Посчитать количество "от孤岛" деталей
    fn count_isolated_parts(&self, sheet: &NestSheet) -> usize {
        if sheet.parts.len() < 3 {
            return 0;
        }

        let mut isolated_count = 0;
        let threshold = 50.0; // 50 мм до ближайшей детали

        for (i, part) in sheet.parts.iter().enumerate() {
            let mut min_distance = f64::MAX;

            for (j, other) in sheet.parts.iter().enumerate() {
                if i == j {
                    continue;
                }

                let dx = (part.x_mm as f64) - (other.x_mm as f64);
                let dy = (part.y_mm as f64) - (other.y_mm as f64);
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < min_distance {
                    min_distance = distance;
                }
            }

            if min_distance > threshold {
                isolated_count += 1;
            }
        }

        isolated_count
    }

    /// Оценить оптимальность размера бумаги
    fn evaluate_paper_optimality(&self, nest_result: &NestResult) -> f64 {
        if nest_result.sheets.is_empty() {
            return 1.0;
        }

        // Если последний лист заполнен менее чем на 50%, бумага слишком большая
        let last_sheet = &nest_result.sheets[nest_result.sheets.len() - 1];
        let last_sheet_analysis = self.analyze_sheet(last_sheet, 0);

        if last_sheet_analysis.fill_rate < 0.5 {
            // Можно было бы использовать бумагу поменьше или лучше разместить
            return last_sheet_analysis.fill_rate / 0.5;
        }

        1.0
    }

    /// Предложить подходящий формат бумаги
    fn suggest_paper_format(&self, nest_result: &NestResult) -> Option<String> {
        if nest_result.sheets.is_empty() {
            return None;
        }

        // Анализируем размеры деталей
        let mut max_part_width = 0.0f64;
        let mut max_part_height = 0.0f64;
        let mut total_area = 0.0f64;

        for sheet in &nest_result.sheets {
            for part in &sheet.parts {
                max_part_width = max_part_width.max(part.width_mm as f64);
                max_part_height = max_part_height.max(part.height_mm as f64);
                total_area += (part.width_mm as f64) * (part.height_mm as f64);
            }
        }

        // Добавляем 20% на зазоры
        let required_area = total_area * 1.2;

        // Предложить формат бумаги
        let formats = [
            ("A4", 210.0 * 297.0),
            ("A3", 297.0 * 420.0),
            ("A2", 420.0 * 594.0),
            ("A1", 594.0 * 841.0),
        ];

        for (format, area) in &formats {
            if required_area <= *area {
                return Some(format.to_string());
            }
        }

        Some("A1+".to_string()) // Больше чем A1
    }

    /// Вычислить потенциальную экономию
    fn calculate_potential_savings(&self, nest_result: &NestResult, avg_fill_rate: f64) -> f64 {
        if nest_result.sheets.is_empty() || avg_fill_rate >= self.target_fill_rate {
            return 0.0;
        }

        // Сколько листов можно было бы использовать при оптимальной заполненности
        let current_sheets = nest_result.sheets.len() as f64;
        let optimal_sheets = current_sheets * (avg_fill_rate / self.target_fill_rate);

        let savings = current_sheets - optimal_sheets;
        (savings / current_sheets * 100.0).max(0.0)
    }

    /// Оценить сложность сборки
    fn evaluate_assembly_complexity(&self, nest_result: &NestResult) -> AssemblyComplexity {
        let sheets_count = nest_result.sheets.len() as f64;
        let sheets_factor = 1.0 - (1.0 / sheets_count.max(1.0));

        // Разброс размеров деталей
        let mut sizes: Vec<f64> = Vec::new();
        for sheet in &nest_result.sheets {
            for part in &sheet.parts {
                sizes.push((part.width_mm as f64) * (part.height_mm as f64));
            }
        }

        let size_variance = if sizes.len() < 2 {
            0.0
        } else {
            let mean = sizes.iter().sum::<f64>() / sizes.len() as f64;
            let variance = sizes.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / sizes.len() as f64;
            variance.sqrt() / mean // Коэффициент вариации
        };

        // Упрощённая оценка (в идеале нужно знать связи между деталями)
        let overall = (sheets_factor * 0.4 + size_variance.min(1.0) * 0.3
            + (1.0 - (nest_result.metrics.avg_fill_rate as f64) / 100.0) * 0.3).min(1.0);

        AssemblyComplexity {
            overall_complexity: overall,
            avg_connected_distance: 0.0, // TODO: вычислить на основе связей
            sheets_factor,
            size_variance: size_variance.min(1.0),
        }
    }

    /// Генерировать рекомендации
    fn generate_recommendations(
        &self,
        nest_result: &NestResult,
        sheets_analysis: &[SheetAnalysis],
        avg_fill_rate: f64,
        paper_optimality: f64,
    ) -> Vec<NestingRecommendation> {
        let mut recommendations = Vec::new();

        // Рекомендации по заполненности
        if avg_fill_rate < self.min_acceptable_fill_rate {
            recommendations.push(NestingRecommendation {
                recommendation_type: RecommendationType::AdjustScale,
                description: format!(
                    "Средняя заполненность {:.1}% ниже минимальной. Увеличьте масштаб модели на {:.0}%.",
                    avg_fill_rate * 100.0,
                    (self.target_fill_rate / avg_fill_rate - 1.0) * 100.0
                ),
                potential_benefit: (self.target_fill_rate - avg_fill_rate) * 100.0,
                priority: "high".to_string(),
                category: "scale".to_string(),
            });
        } else if avg_fill_rate < self.target_fill_rate {
            recommendations.push(NestingRecommendation {
                recommendation_type: RecommendationType::AdjustScale,
                description: format!(
                    "Заполненность {:.1}% может быть улучшена до {:.0}%.",
                    avg_fill_rate * 100.0,
                    self.target_fill_rate * 100.0
                ),
                potential_benefit: (self.target_fill_rate - avg_fill_rate) * 50.0,
                priority: "medium".to_string(),
                category: "scale".to_string(),
            });
        }

        // Рекомендации по формату бумаги
        if paper_optimality < 0.7 {
            if let Some(suggested) = self.suggest_paper_format(nest_result) {
                recommendations.push(NestingRecommendation {
                    recommendation_type: RecommendationType::ChangePaperFormat,
                    description: format!(
                        "Последний лист заполнен только на {:.1}%. Рассмотрите формат {}.",
                        (1.0 - paper_optimality) * 50.0,
                        suggested
                    ),
                    potential_benefit: (1.0 - paper_optimality) * 30.0,
                    priority: "medium".to_string(),
                    category: "paper".to_string(),
                });
            }
        }

        // Рекомендации по группировке
        for sheet in sheets_analysis {
            if sheet.isolated_parts_count > 2 {
                recommendations.push(NestingRecommendation {
                    recommendation_type: RecommendationType::GroupParts,
                    description: format!(
                        "На листе #{} {} деталей далеко от других. Оптимизируйте раскладку.",
                        sheet.sheet_index + 1,
                        sheet.isolated_parts_count
                    ),
                    potential_benefit: 5.0,
                    priority: "low".to_string(),
                    category: "grouping".to_string(),
                });
            }
        }

        // Рекомендация использовать генетический алгоритм
        if nest_result.sheets.len() > 2 && avg_fill_rate < 0.65 {
            recommendations.push(NestingRecommendation {
                recommendation_type: RecommendationType::UseGeneticAlgorithm,
                description: "Для сложных раскладок используйте генетический алгоритм (до 15% улучшения).".to_string(),
                potential_benefit: 15.0,
                priority: "high".to_string(),
                category: "layout".to_string(),
            });
        }

        // Рекомендации по повороту
        if nest_result.metrics.avg_fill_rate < 60.0 {
            recommendations.push(NestingRecommendation {
                recommendation_type: RecommendationType::AdjustRotation,
                description: "Уменьшите шаг поворота (текущий 45°) для лучшего размещения.".to_string(),
                potential_benefit: 10.0,
                priority: "medium".to_string(),
                category: "rotation".to_string(),
            });
        }

        recommendations
    }

    /// Вычислить общую эффективность пространства
    fn compute_space_efficiency(
        &self,
        avg_fill_rate: f64,
        paper_optimality: f64,
        sheets_count: usize,
    ) -> f64 {
        let fill_score = avg_fill_rate.min(1.0);
        let paper_score = paper_optimality;
        let sheets_score = 1.0 / (sheets_count as f64).max(1.0);

        0.5 * fill_score + 0.3 * paper_score + 0.2 * sheets_score
    }

    /// Генерировать AI промпт для рекомендаций по раскладке
    pub fn generate_ai_prompt(&self, analysis: &NestingAnalysisResult) -> String {
        let mut prompt = String::from("Проанализируй раскладку papercraft модели:\n\n");

        prompt.push_str(&format!(
            "- Листов: {}\n",
            analysis.sheets_count
        ));
        prompt.push_str(&format!(
            "- Деталей: {}\n",
            analysis.total_parts
        ));
        prompt.push_str(&format!(
            "- Средняя заполненность: {:.1}%\n",
            analysis.avg_fill_rate * 100.0
        ));
        prompt.push_str(&format!(
            "- Эффективность пространства: {:.1}%\n",
            analysis.space_efficiency_score * 100.0
        ));

        if let Some(format) = &analysis.suggested_paper_format {
            prompt.push_str(&format!("- Рекомендуемый формат бумаги: {}\n", format));
        }

        prompt.push_str(&format!(
            "- Потенциальная экономия: {:.1}%\n",
            analysis.potential_savings_percent
        ));
        prompt.push_str(&format!(
            "- Сложность сборки: {:.1}%\n",
            analysis.assembly_complexity.overall_complexity * 100.0
        ));

        prompt.push_str("\nДай рекомендации по:\n");
        prompt.push_str("1. Оптимизации использования бумаги\n");
        prompt.push_str("2. Улучшении компактности раскладки\n");
        prompt.push_str("3. Снижении сложности сборки\n");
        prompt.push_str("4. Группировке связанных деталей\n");

        prompt
    }
}

impl Default for NestingOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Генерировать AI рекомендации на основе анализа раскладки
pub fn generate_nesting_advice(analysis: &NestingAnalysisResult) -> Vec<String> {
    let mut tips = Vec::new();

    // Советы по заполненности
    if analysis.avg_fill_rate < 0.5 {
        tips.push(format!(
            "⚠️ Низкая заполненность бумаги ({:.1}%). Увеличьте масштаб модели для экономии бумаги.",
            analysis.avg_fill_rate * 100.0
        ));
    } else if analysis.avg_fill_rate >= 0.75 {
        tips.push("✅ Отличная заполненность бумаги! Раскладка эффективна.".to_string());
    }

    // Советы по количеству листов
    if analysis.sheets_count > 3 {
        tips.push(format!(
            "📄 Используется {} листов. Попробуйте оптимизировать раскладку для уменьшения количества листов.",
            analysis.sheets_count
        ));
    }

    // Советы по формату бумаги
    if let Some(format) = &analysis.suggested_paper_format {
        tips.push(format!("💡 Рекомендуемый формат бумаги: {}", format));
    }

    // Советы по экономии
    if analysis.potential_savings_percent > 10.0 {
        tips.push(format!(
            "💰 Потенциальная экономия бумаги: {:.1}% при оптимизации.",
            analysis.potential_savings_percent
        ));
    }

    // Советы по сложности сборки
    if analysis.assembly_complexity.overall_complexity > 0.7 {
        tips.push(format!(
            "🔧 Высокая сложность сборки ({:.0}%). Рассмотрите разбиение модели на части.",
            analysis.assembly_complexity.overall_complexity * 100.0
        ));
    }

    // Советы из рекомендаций
    for rec in &analysis.recommendations {
        if rec.priority == "high" {
            tips.push(format!("🎯 {}", rec.description));
        }
    }

    tips
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nesting::{NestResult, NestSheet, NestPart, PaperSettings, NestParams, NestMetrics};

    fn create_test_nest_result() -> NestResult {
        let sheet = NestSheet {
            id: 0,
            index: 0,
            width_mm: 210.0,
            height_mm: 297.0,
            margin_mm: 5.0,
            parts: vec![
                NestPart {
                    id: 1,
                    name: Some("Part 1".to_string()),
                    unfolded_face_index: 0,
                    x_mm: 10.0,
                    y_mm: 10.0,
                    width_mm: 50.0,
                    height_mm: 30.0,
                    rotation: 0.0,
                },
                NestPart {
                    id: 2,
                    name: Some("Part 2".to_string()),
                    unfolded_face_index: 1,
                    x_mm: 70.0,
                    y_mm: 10.0,
                    width_mm: 40.0,
                    height_mm: 25.0,
                    rotation: 0.0,
                },
            ],
        };

        let available_area = (210.0 - 10.0) as f64 * (297.0 - 10.0) as f64;
        let parts_area = 50.0 * 30.0 + 40.0 * 25.0;

        NestResult {
            sheets: vec![sheet],
            metrics: NestMetrics {
                total_sheets: 1,
                total_parts: 2,
                avg_fill_rate: (parts_area as f64 / available_area * 100.0) as f32,
                total_parts_area: parts_area,
                total_sheets_area: 210.0 * 297.0,
            },
            params_snapshot: NestParams::default(),
        }
    }

    #[test]
    fn test_nesting_optimizer_creation() {
        let optimizer = NestingOptimizer::new();
        assert_eq!(optimizer.target_fill_rate, 0.75);
        assert_eq!(optimizer.min_acceptable_fill_rate, 0.50);
    }

    #[test]
    fn test_analyze_sheet() {
        let optimizer = NestingOptimizer::new();
        let nest_result = create_test_nest_result();
        
        let analysis = optimizer.analyze(&nest_result);
        
        assert_eq!(analysis.sheets_count, 1);
        assert_eq!(analysis.total_parts, 2);
        assert!(analysis.avg_fill_rate > 0.0);
        assert!(analysis.avg_fill_rate < 1.0);
    }

    #[test]
    fn test_compute_parts_bbox() {
        let optimizer = NestingOptimizer::new();
        let nest_result = create_test_nest_result();
        
        let sheet = &nest_result.sheets[0];
        let bbox = optimizer.compute_parts_bbox(sheet);
        
        assert!(bbox.min_x >= 0.0);
        assert!(bbox.max_x > bbox.min_x);
        assert!(bbox.max_y > bbox.min_y);
    }

    #[test]
    fn test_generate_nesting_advice() {
        let analysis = NestingAnalysisResult {
            space_efficiency_score: 0.6,
            avg_fill_rate: 0.4,
            sheets_count: 4,
            total_parts: 50,
            sheets_analysis: vec![],
            recommendations: vec![],
            paper_size_optimality: 0.8,
            suggested_paper_format: Some("A3".to_string()),
            potential_savings_percent: 15.0,
            assembly_complexity: AssemblyComplexity {
                overall_complexity: 0.5,
                avg_connected_distance: 30.0,
                sheets_factor: 0.75,
                size_variance: 0.3,
            },
        };

        let advice = generate_nesting_advice(&analysis);
        assert!(!advice.is_empty());
        assert!(advice.iter().any(|tip| tip.contains("Низкая заполненность")));
        assert!(advice.iter().any(|tip| tip.contains("A3")));
    }

    #[test]
    fn test_suggest_paper_format() {
        let optimizer = NestingOptimizer::new();
        let nest_result = create_test_nest_result();
        
        let suggested = optimizer.suggest_paper_format(&nest_result);
        assert!(suggested.is_some());
    }

    #[test]
    fn test_evaluate_compactness() {
        let optimizer = NestingOptimizer::new();
        let nest_result = create_test_nest_result();
        
        let sheet = &nest_result.sheets[0];
        let bbox = optimizer.compute_parts_bbox(sheet);
        let compactness = optimizer.evaluate_compactness(sheet, &bbox);
        
        assert!(compactness >= 0.0);
        assert!(compactness <= 1.0);
    }
}
