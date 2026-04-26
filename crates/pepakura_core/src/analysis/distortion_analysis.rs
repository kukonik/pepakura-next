//! # Distortion Analysis
//!
//! Анализ искажений при развёртке 3D→2D.
//!
//! ## Функционал
//!
//! - **Area distortion** - искажение площадей граней
//! - **Angular distortion** - искажение углов
//! - **Edge length distortion** - искажение длин рёбер
//! - **Heat map generation** - генерация данных для тепловой карты
//! - **Problematic face detection** - детекция проблемных граней
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::analysis::distortion_analysis::DistortionAnalyzer;
//! use pepakura_core::geometry::Mesh;
//! use pepakura_core::unfold::{UnfoldResult, UnfoldedFace};
//!
//! let mesh = Mesh::new("model");
//! let unfolded = UnfoldResult { faces: vec![], seams: vec![], layout: Default::default() };
//!
//! let analyzer = DistortionAnalyzer::new();
//! let result = analyzer.analyze(&mesh, &unfolded);
//!
//! // Получаем статистику искажений
//! println!("Среднее искажение площадей: {:.2}%", result.avg_area_distortion);
//!
//! // Получаем данные тепловой карты
//! let heat_map = result.heat_map_data;
//! ```

use crate::geometry::Mesh;
use crate::unfold::{UnfoldResult, UnfoldedFace};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

/// Результат анализа искажений развёртки
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistortionAnalysisResult {
    /// Статистика искажений площадей (коэффициент, 1.0 = нет искажений)
    pub area_distortion_stats: DistortionStats,
    /// Статистика искажений углов (в градусах)
    pub angular_distortion_stats: DistortionStats,
    /// Статистика искажений длин рёбер (коэффициент)
    pub edge_distortion_stats: DistortionStats,
    /// Данные для тепловой карты искажений
    pub heat_map_data: Vec<FaceHeatMapEntry>,
    /// Список проблемных граней
    pub problematic_faces: Vec<ProblematicFace>,
    /// Общая оценка качества развёртки (0.0 - 1.0, где 1.0 = идеально)
    pub overall_quality_score: f64,
    /// Среднее искажение площадей (в процентах)
    pub avg_area_distortion: f64,
    /// Максимальное искажение площадей (в процентах)
    pub max_area_distortion: f64,
    /// Процент граней с допустимыми искажениями (< 10%)
    pub acceptable_faces_ratio: f64,
}

/// Статистика искажений
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistortionStats {
    /// Среднее значение
    pub mean: f64,
    /// Медиана
    pub median: f64,
    /// Стандартное отклонение
    pub std_dev: f64,
    /// Минимальное значение
    pub min: f64,
    /// Максимальное значение
    pub max: f64,
    /// 95-й перцентиль
    pub percentile_95: f64,
    /// Количество значений
    pub count: usize,
}

impl DistortionStats {
    /// Создать новую статистику из набора значений
    pub fn from_values(mut values: Vec<f64>) -> Self {
        if values.is_empty() {
            return Self {
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
                percentile_95: 0.0,
                count: 0,
            };
        }

        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let count = values.len();
        let mean = values.iter().sum::<f64>() / count as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();

        let median = if count % 2 == 0 {
            (values[count / 2 - 1] + values[count / 2]) / 2.0
        } else {
            values[count / 2]
        };

        let p95_index = ((count as f64) * 0.95) as usize;
        let percentile_95 = values[p95_index.min(count - 1)];

        Self {
            mean,
            median,
            std_dev,
            min: values[0],
            max: values[count - 1],
            percentile_95,
            count,
        }
    }
}

/// Запись данных тепловой карты для одной грани
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceHeatMapEntry {
    /// Индекс грани в результате развёртки
    pub face_index: usize,
    /// Индекс исходной грани в меше
    pub source_face_index: usize,
    /// Искажение площади (1.0 = нет искажений)
    pub area_ratio: f64,
    /// Среднее искажение углов (в градусах)
    pub avg_angular_distortion: f64,
    /// Среднее искажение рёбер (1.0 = нет искажений)
    pub avg_edge_ratio: f64,
    /// Композитная оценка искажений (0.0 = нет искажений, чем больше = хуже)
    pub composite_distortion: f64,
    /// Центр грани в 2D (для визуализации)
    pub center_2d: [f64; 2],
    /// Уровень серьёзности: "ok", "warning", "critical"
    pub severity: String,
}

/// Проблемная грань
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblematicFace {
    /// Индекс грани
    pub face_index: usize,
    /// Тип проблемы
    pub issue_type: FaceIssueType,
    /// Описание проблемы
    pub description: String,
    /// Серьёзность (0.0 - 1.0)
    pub severity: f64,
    /// Рекомендация по исправлению
    pub recommendation: String,
}

/// Тип проблемы грани
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FaceIssueType {
    /// Слишком маленькая площадь грани
    TooSmallArea,
    /// Очень острый угол
    SharpAngle,
    /// Сильное искажение площади
    HighAreaDistortion,
    /// Сильное искажение углов
    HighAngularDistortion,
    /// Очень длинное ребро
    LongEdge,
    /// Вырожденная грань
    DegenerateFace,
}

impl std::fmt::Display for FaceIssueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FaceIssueType::TooSmallArea => write!(f, "Слишком маленькая площадь"),
            FaceIssueType::SharpAngle => write!(f, "Острый угол"),
            FaceIssueType::HighAreaDistortion => write!(f, "Сильное искажение площади"),
            FaceIssueType::HighAngularDistortion => write!(f, "Сильное искажение углов"),
            FaceIssueType::LongEdge => write!(f, "Длинное ребро"),
            FaceIssueType::DegenerateFace => write!(f, "Вырожденная грань"),
        }
    }
}

/// Анализатор искажений развёртки
pub struct DistortionAnalyzer {
    /// Порог для предупреждений (искажение площади,ratio)
    pub warning_threshold: f64,
    /// Порог для критических проблем (ratio)
    pub critical_threshold: f64,
    /// Минимальная площадь грани (в мм²) для предупреждений
    pub min_face_area: f64,
    /// Минимальный угол (в градусах) для предупреждений
    pub min_angle_deg: f64,
}

impl DistortionAnalyzer {
    /// Создать новый анализатор с настройками по умолчанию
    pub fn new() -> Self {
        Self {
            warning_threshold: 1.1,      // 10% искажение
            critical_threshold: 1.3,     // 30% искажение
            min_face_area: 1.0,          // 1 мм²
            min_angle_deg: 15.0,         // 15 градусов
        }
    }

    /// Создать анализатор с кастомными порогами
    pub fn with_thresholds(warning: f64, critical: f64) -> Self {
        Self {
            warning_threshold: warning,
            critical_threshold: critical,
            min_face_area: 1.0,
            min_angle_deg: 15.0,
        }
    }

    /// Проанализировать искажения развёртки
    pub fn analyze(&self, mesh: &Mesh, unfolded: &UnfoldResult) -> DistortionAnalysisResult {
        let mut heat_map_data = Vec::with_capacity(unfolded.faces.len());
        let mut problematic_faces = Vec::new();
        let mut area_ratios = Vec::new();
        let mut angular_distortions = Vec::new();
        let mut edge_ratios = Vec::new();

        for (idx, unfolded_face) in unfolded.faces.iter().enumerate() {
            // Находим соответствующую грань в исходном меше
            if let Some(source_face) = mesh.faces.get(unfolded_face.face_index) {
                // Вычисляем искажения
                let area_3d = self.compute_face_area_3d(mesh, source_face);
                let area_2d = self.compute_face_area_2d(unfolded_face);
                let area_ratio = if area_3d > 1e-10 {
                    area_2d / area_3d
                } else {
                    1.0
                };

                let angular_distortion = self.compute_angular_distortion(mesh, source_face, unfolded_face);
                let edge_distortion = self.compute_edge_distortion(mesh, source_face, unfolded_face);

                // Вычисляем композитную оценку искажений
                let composite = self.compute_composite_distortion(
                    area_ratio,
                    angular_distortion,
                    edge_distortion,
                );

                // Определяем уровень серьёзности
                let severity = if composite < 0.1 {
                    "ok".to_string()
                } else if composite < 0.3 {
                    "warning".to_string()
                } else {
                    "critical".to_string()
                };

                // Центр грани в 2D
                let center = if unfolded_face.vertices_2d.is_empty() {
                    [0.0, 0.0]
                } else {
                    let sum_x: f64 = unfolded_face.vertices_2d.iter().map(|v| v.x).sum();
                    let sum_y: f64 = unfolded_face.vertices_2d.iter().map(|v| v.y).sum();
                    let count = unfolded_face.vertices_2d.len() as f64;
                    [sum_x / count, sum_y / count]
                };

                heat_map_data.push(FaceHeatMapEntry {
                    face_index: idx,
                    source_face_index: unfolded_face.face_index,
                    area_ratio,
                    avg_angular_distortion: angular_distortion,
                    avg_edge_ratio: edge_distortion,
                    composite_distortion: composite,
                    center_2d: center,
                    severity,
                });

                area_ratios.push((area_ratio - 1.0).abs());
                angular_distortions.push(angular_distortion);
                edge_ratios.push((edge_distortion - 1.0).abs());

                // Проверяем на проблемные грани
                self.check_problematic_face(
                    &mut problematic_faces,
                    idx,
                    mesh,
                    source_face,
                    unfolded_face,
                    area_3d,
                    area_ratio,
                    angular_distortion,
                );
            }
        }

        // Вычисляем статистику
        let area_stats = DistortionStats::from_values(area_ratios.clone());
        let angular_stats = DistortionStats::from_values(angular_distortions);
        let edge_stats = DistortionStats::from_values(edge_ratios);

        // Общая оценка качества
        let overall_quality = self.compute_overall_quality(&area_stats, &angular_stats, &edge_stats);

        // Процент допустимых граней
        let acceptable_count = heat_map_data
            .iter()
            .filter(|h| h.severity == "ok" || h.severity == "warning")
            .count();
        let acceptable_ratio = if heat_map_data.is_empty() {
            1.0
        } else {
            acceptable_count as f64 / heat_map_data.len() as f64
        };

        DistortionAnalysisResult {
            area_distortion_stats: area_stats,
            angular_distortion_stats: angular_stats,
            edge_distortion_stats: edge_stats,
            heat_map_data,
            problematic_faces,
            overall_quality_score: overall_quality,
            avg_area_distortion: if area_ratios.is_empty() {
                0.0
            } else {
                area_ratios.iter().sum::<f64>() / area_ratios.len() as f64 * 100.0
            },
            max_area_distortion: if area_ratios.is_empty() {
                0.0
            } else {
                area_ratios.iter().cloned().fold(f64::NEG_INFINITY, f64::max) * 100.0
            },
            acceptable_faces_ratio: acceptable_ratio,
        }
    }

    /// Вычислить площадь грани в 3D
    fn compute_face_area_3d(&self, mesh: &Mesh, face: &crate::geometry::Face) -> f64 {
        let indices = &face.vertices;
        if indices.len() < 3 {
            return 0.0;
        }

        // Триангуляция от первой вершины
        let mut total_area = 0.0;
        let v0 = Vector3::new(
            mesh.vertices[indices[0]].position[0],
            mesh.vertices[indices[0]].position[1],
            mesh.vertices[indices[0]].position[2],
        );

        for i in 1..indices.len() - 1 {
            let v1 = Vector3::new(
                mesh.vertices[indices[i]].position[0],
                mesh.vertices[indices[i]].position[1],
                mesh.vertices[indices[i]].position[2],
            );
            let v2 = Vector3::new(
                mesh.vertices[indices[i + 1]].position[0],
                mesh.vertices[indices[i + 1]].position[1],
                mesh.vertices[indices[i + 1]].position[2],
            );

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let cross = edge1.cross(&edge2);
            total_area += cross.norm() / 2.0;
        }

        total_area
    }

    /// Вычислить площадь грани в 2D
    fn compute_face_area_2d(&self, face: &UnfoldedFace) -> f64 {
        let vertices = &face.vertices_2d;
        if vertices.len() < 3 {
            return 0.0;
        }

        // Формула Shoelace для многоугольника
        let mut area = 0.0;
        let n = vertices.len();
        for i in 0..n {
            let j = (i + 1) % n;
            area += vertices[i].x * vertices[j].y;
            area -= vertices[j].x * vertices[i].y;
        }

        (area / 2.0).abs()
    }

    /// Вычислить искажение углов
    fn compute_angular_distortion(
        &self,
        mesh: &Mesh,
        source_face: &crate::geometry::Face,
        unfolded_face: &UnfoldedFace,
    ) -> f64 {
        let indices = &source_face.vertices;
        let vertices_2d = &unfolded_face.vertices_2d;

        if indices.len() < 3 || vertices_2d.len() != indices.len() {
            return 0.0;
        }

        let mut total_diff = 0.0;
        let n = indices.len();

        for i in 0..n {
            // Угол в 3D
            let prev_3d = &mesh.vertices[indices[(i + n - 1) % n]].position;
            let curr_3d = &mesh.vertices[indices[i]].position;
            let next_3d = &mesh.vertices[indices[(i + 1) % n]].position;

            let angle_3d = self.compute_angle(prev_3d, curr_3d, next_3d);

            // Угол в 2D
            let prev_2d = &vertices_2d[(i + n - 1) % n];
            let curr_2d = &vertices_2d[i];
            let next_2d = &vertices_2d[(i + 1) % n];

            let angle_2d = self.compute_angle_2d(
                &[prev_2d.x, prev_2d.y],
                &[curr_2d.x, curr_2d.y],
                &[next_2d.x, next_2d.y],
            );

            total_diff += (angle_3d - angle_2d).abs();
        }

        total_diff / n as f64
    }

    /// Вычислить искажение длин рёбер
    fn compute_edge_distortion(
        &self,
        mesh: &Mesh,
        source_face: &crate::geometry::Face,
        unfolded_face: &UnfoldedFace,
    ) -> f64 {
        let indices = &source_face.vertices;
        let vertices_2d = &unfolded_face.vertices_2d;

        if indices.len() < 3 || vertices_2d.len() != indices.len() {
            return 1.0;
        }

        let mut total_ratio = 0.0;
        let n = indices.len();

        for i in 0..n {
            let j = (i + 1) % n;

            // Длина ребра в 3D
            let v1_3d = &mesh.vertices[indices[i]].position;
            let v2_3d = &mesh.vertices[indices[j]].position;
            let length_3d = ((v1_3d[0] - v2_3d[0]).powi(2)
                + (v1_3d[1] - v2_3d[1]).powi(2)
                + (v1_3d[2] - v2_3d[2]).powi(2))
            .sqrt();

            // Длина ребра в 2D
            let v1_2d = &vertices_2d[i];
            let v2_2d = &vertices_2d[j];
            let length_2d = ((v1_2d.x - v2_2d.x).powi(2) + (v1_2d.y - v2_2d.y).powi(2)).sqrt();

            if length_3d > 1e-10 {
                total_ratio += length_2d / length_3d;
            }
        }

        total_ratio / n as f64
    }

    /// Вычислить угол между двумя векторами (3D)
    fn compute_angle(&self, p1: &[f64; 3], vertex: &[f64; 3], p2: &[f64; 3]) -> f64 {
        let v1 = Vector3::new(p1[0] - vertex[0], p1[1] - vertex[1], p1[2] - vertex[2]);
        let v2 = Vector3::new(p2[0] - vertex[0], p2[1] - vertex[1], p2[2] - vertex[2]);

        let dot = v1.dot(&v2);
        let norm1 = v1.norm();
        let norm2 = v2.norm();

        if norm1 < 1e-10 || norm2 < 1e-10 {
            return 0.0;
        }

        let cos_angle = dot / (norm1 * norm2);
        let cos_angle = cos_angle.clamp(-1.0, 1.0);
        cos_angle.acos().to_degrees()
    }

    /// Вычислить угол между двумя векторами (2D)
    fn compute_angle_2d(&self, p1: &[f64; 2], vertex: &[f64; 2], p2: &[f64; 2]) -> f64 {
        let v1 = Vector3::new(p1[0] - vertex[0], p1[1] - vertex[1], 0.0);
        let v2 = Vector3::new(p2[0] - vertex[0], p2[1] - vertex[1], 0.0);

        let dot = v1.dot(&v2);
        let norm1 = v1.norm();
        let norm2 = v2.norm();

        if norm1 < 1e-10 || norm2 < 1e-10 {
            return 0.0;
        }

        let cos_angle = dot / (norm1 * norm2);
        let cos_angle = cos_angle.clamp(-1.0, 1.0);
        cos_angle.acos().to_degrees()
    }

    /// Вычислить композитную оценку искажений
    fn compute_composite_distortion(
        &self,
        area_ratio: f64,
        angular_distortion: f64,
        edge_ratio: f64,
    ) -> f64 {
        // Нормализуем к диапазону 0-1
        let area_score = (area_ratio - 1.0).abs().min(1.0);
        let angular_score = (angular_distortion / 90.0).min(1.0); // 90° = максимум
        let edge_score = (edge_ratio - 1.0).abs().min(1.0);

        // Взвешенная сумма
        0.4 * area_score + 0.4 * angular_score + 0.2 * edge_score
    }

    /// Проверить грань на проблемность
    fn check_problematic_face(
        &self,
        problematic: &mut Vec<ProblematicFace>,
        face_index: usize,
        mesh: &Mesh,
        source_face: &crate::geometry::Face,
        unfolded_face: &UnfoldedFace,
        area_3d: f64,
        area_ratio: f64,
        angular_distortion: f64,
    ) {
        // Проверка на слишком маленькую площадь
        if area_3d < self.min_face_area && area_3d > 1e-10 {
            problematic.push(ProblematicFace {
                face_index,
                issue_type: FaceIssueType::TooSmallArea,
                description: format!("Площадь грани {:.2} мм² меньше минимальной", area_3d),
                severity: (1.0 - area_3d / self.min_face_area).min(1.0),
                recommendation: "Увеличьте масштаб модели или объедините мелкие грани".to_string(),
            });
        }

        // Проверка на сильное искажение площади
        let area_distortion_pct = (area_ratio - 1.0).abs();
        if area_distortion_pct > self.critical_threshold - 1.0 {
            problematic.push(ProblematicFace {
                face_index,
                issue_type: FaceIssueType::HighAreaDistortion,
                description: format!(
                    "Искажение площади {:.1}% превышает критический порог",
                    area_distortion_pct * 100.0
                ),
                severity: area_distortion_pct.min(1.0),
                recommendation: "Измените алгоритм развёртки или уменьшите сложность модели".to_string(),
            });
        }

        // Проверка на сильные угловые искажения
        if angular_distortion > self.min_angle_deg {
            problematic.push(ProblematicFace {
                face_index,
                issue_type: FaceIssueType::HighAngularDistortion,
                description: format!(
                    "Среднее искажение углов {:.1}° слишком велико",
                    angular_distortion
                ),
                severity: (angular_distortion / 90.0).min(1.0),
                recommendation: "Используйте LSCM алгоритм для сохранения углов".to_string(),
            });
        }

        // Проверка на вырожденные грани
        if area_3d < 1e-6 || unfolded_face.vertices_2d.len() < 3 {
            problematic.push(ProblematicFace {
                face_index,
                issue_type: FaceIssueType::DegenerateFace,
                description: "Грань вырождена (почти нулевая площадь или недостаточно вершин)".to_string(),
                severity: 1.0,
                recommendation: "Удалите вырожденные грани из модели".to_string(),
            });
        }

        // Проверка на острые углы
        self.check_sharp_angles(
            problematic,
            face_index,
            mesh,
            source_face,
            unfolded_face,
        );
    }

    /// Проверить на острые углы
    fn check_sharp_angles(
        &self,
        problematic: &mut Vec<ProblematicFace>,
        face_index: usize,
        _mesh: &Mesh,
        source_face: &crate::geometry::Face,
        unfolded_face: &UnfoldedFace,
    ) {
        let indices = &source_face.vertices;
        let vertices_2d = &unfolded_face.vertices_2d;

        if indices.len() < 3 || vertices_2d.len() != indices.len() {
            return;
        }

        let mut has_sharp = false;
        let mut min_angle = 180.0;
        let n = indices.len();

        for i in 0..n {
            let prev_2d = &vertices_2d[(i + n - 1) % n];
            let curr_2d = &vertices_2d[i];
            let next_2d = &vertices_2d[(i + 1) % n];

            let angle = self.compute_angle_2d(
                &[prev_2d.x, prev_2d.y],
                &[curr_2d.x, curr_2d.y],
                &[next_2d.x, next_2d.y],
            );

            if angle < min_angle {
                min_angle = angle;
            }

            if angle < self.min_angle_deg {
                has_sharp = true;
            }
        }

        if has_sharp {
            problematic.push(ProblematicFace {
                face_index,
                issue_type: FaceIssueType::SharpAngle,
                description: format!("Обнаружен острый угол {:.1}° (минимум: {:.1}°)", min_angle, self.min_angle_deg),
                severity: (1.0 - min_angle / self.min_angle_deg).min(1.0),
                recommendation: "Сгладьте острые углы или увеличьте масштаб".to_string(),
            });
        }
    }

    /// Вычислить общую оценку качества развёртки
    fn compute_overall_quality(
        &self,
        area_stats: &DistortionStats,
        angular_stats: &DistortionStats,
        edge_stats: &DistortionStats,
    ) -> f64 {
        // Нормализуем к диапазону 0-1 (где 1 = хорошо)
        let area_score = (1.0 - area_stats.mean.min(1.0)).max(0.0);
        let angular_score = (1.0 - (angular_stats.mean / 45.0).min(1.0)).max(0.0);
        let edge_score = (1.0 - edge_stats.mean.min(1.0)).max(0.0);

        // Взвешенная средняя
        0.4 * area_score + 0.4 * angular_score + 0.2 * edge_score
    }

    /// Генерировать SVG тепловую карту
    pub fn generate_heatmap_svg(&self, result: &DistortionAnalysisResult, title: &str) -> String {
        let mut svg = String::new();

        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 800 600">
  <defs>
    <linearGradient id="heatmap" x1="0%" y1="0%" x2="100%" y2="0%">
      <stop offset="0%" style="stop-color:#0000FF"/>
      <stop offset="25%" style="stop-color:#00FFFF"/>
      <stop offset="50%" style="stop-color:#00FF00"/>
      <stop offset="75%" style="stop-color:#FFFF00"/>
      <stop offset="100%" style="stop-color:#FF0000"/>
    </linearGradient>
  </defs>
  
  <text x="400" y="30" text-anchor="middle" font-size="16" font-weight="bold">{}</text>
  <text x="400" y="50" text-anchor="middle" font-size="12">Общее качество: {:.1}% | Ср.искажение: {:.1}%</text>
  
  <rect x="50" y="520" width="700" height="30" fill="url(#heatmap)"/>
  <text x="50" y="565" font-size="10">0%</text>
  <text x="390" y="565" text-anchor="middle" font-size="10">50%</text>
  <text x="740" y="565" text-anchor="end" font-size="10">100%</text>
"#,
            title,
            result.overall_quality_score * 100.0,
            result.avg_area_distortion
        ));

        // Легенда для граней
        for (i, entry) in result.heat_map_data.iter().enumerate().take(100) {
            let color = self.distortion_to_color(entry.composite_distortion);
            svg.push_str(&format!(
                "  <circle cx=\"{}\" cy=\"{}\" r=\"5\" fill=\"{}\" opacity=\"0.7\">\n    <title>Грань {}: {:.1}%</title>\n  </circle>\n",
                50.0 + (i % 20) as f64 * 35.0 + 17.5,
                70.0 + (i / 20) as f64 * 35.0 + 17.5,
                color,
                entry.face_index,
                entry.composite_distortion * 100.0
            ));
        }

        svg.push_str("</svg>");
        svg
    }

    /// Преобразовать искажение в цвет (blue -> green -> yellow -> red)
    fn distortion_to_color(&self, distortion: f64) -> String {
        let t = distortion.clamp(0.0, 1.0);

        if t < 0.25 {
            // Blue to Cyan
            let local_t = t / 0.25;
            format!("rgb(0, {}, 255)", (local_t * 255.0) as u32)
        } else if t < 0.5 {
            // Cyan to Green
            let local_t = (t - 0.25) / 0.25;
            format!("rgb(0, 255, {})", ((1.0 - local_t) * 255.0) as u32)
        } else if t < 0.75 {
            // Green to Yellow
            let local_t = (t - 0.5) / 0.25;
            format!("rgb({}, 255, 0)", (local_t * 255.0) as u32)
        } else {
            // Yellow to Red
            let local_t = (t - 0.75) / 0.25;
            format!("rgb(255, {}, 0)", ((1.0 - local_t) * 255.0) as u32)
        }
    }
}

impl Default for DistortionAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Генерировать AI рекомендации на основе анализа искажений
pub fn generate_distortion_advice(result: &DistortionAnalysisResult) -> Vec<String> {
    let mut tips = Vec::new();

    // Советы по искажению площадей
    if result.avg_area_distortion > 20.0 {
        tips.push(format!(
            "⚠️ Среднее искажение площадей {:.1}% слишком велико. Попробуйте алгоритм LSCM для лучшего сохранения площадей.",
            result.avg_area_distortion
        ));
    }

    if result.max_area_distortion > 50.0 {
        tips.push(format!(
            "🔴 Максимальное искажение площадей {:.1}% критическое. Некоторые грани сильно деформированы.",
            result.max_area_distortion
        ));
    }

    // Советы по угловым искажениям
    if result.angular_distortion_stats.mean > 10.0 {
        tips.push(format!(
            "📐 Среднее искажение углов {:.1}°. Рассмотрите разбиение модели на более мелкие части.",
            result.angular_distortion_stats.mean
        ));
    }

    // Советы по проблемным граням
    if !result.problematic_faces.is_empty() {
        let critical_count = result.problematic_faces
            .iter()
            .filter(|p| p.severity > 0.7)
            .count();

        if critical_count > 0 {
            tips.push(format!(
                "🚨 Обнаружено {} критических проблем. Рекомендуется упростить модель.",
                critical_count
            ));
        } else {
            tips.push(format!(
                "⚠️ Найдено {} проблемных граней. Проверьте тепловую карту для локализации.",
                result.problematic_faces.len()
            ));
        }
    }

    // Общий совет по качеству
    if result.overall_quality_score < 0.5 {
        tips.push(
            "💡 Качество развёртки низкое. Попробуйте:\n".to_string()
                + "  - Уменьшить масштаб модели\n"
                + "  - Использовать другой алгоритм развёртки\n"
                + "  - Упростить геометрию модели"
        );
    } else if result.overall_quality_score >= 0.8 {
        tips.push("✅ Качество развёртки отличное! Модель готова для печати.".to_string());
    }

    tips
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Vertex, Face};

    fn create_test_triangle_mesh() -> (Mesh, UnfoldResult) {
        let mut mesh = Mesh::new("Triangle");
        
        // 3 вершины равностороннего треугольника
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [10.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [5.0, 8.66, 0.0]));
        
        mesh.add_face(Face::new(0, 1, 2));

        // Создаём развёртку (без искажений, т.к. треугольник уже в плоскости)
        let unfolded_face = UnfoldedFace {
            center: crate::nesting::Point2D { x: 5.0, y: 2.89 },
            vertices_2d: vec![
                crate::nesting::Point2D { x: 0.0, y: 0.0 },
                crate::nesting::Point2D { x: 10.0, y: 0.0 },
                crate::nesting::Point2D { x: 5.0, y: 8.66 },
            ],
            face_index: 0,
        };

        let unfolded = UnfoldResult {
            faces: vec![unfolded_face],
            seams: vec![],
            layout: Default::default(),
        };

        (mesh, unfolded)
    }

    #[test]
    fn test_distortion_analyzer_creation() {
        let analyzer = DistortionAnalyzer::new();
        assert_eq!(analyzer.warning_threshold, 1.1);
        assert_eq!(analyzer.critical_threshold, 1.3);
    }

    #[test]
    fn test_distortion_stats_empty() {
        let stats = DistortionStats::from_values(vec![]);
        assert_eq!(stats.count, 0);
        assert_eq!(stats.mean, 0.0);
    }

    #[test]
    fn test_distortion_stats_single_value() {
        let stats = DistortionStats::from_values(vec![0.5]);
        assert_eq!(stats.count, 1);
        assert_eq!(stats.mean, 0.5);
        assert_eq!(stats.min, 0.5);
        assert_eq!(stats.max, 0.5);
    }

    #[test]
    fn test_distortion_stats_multiple_values() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let stats = DistortionStats::from_values(values);
        
        assert_eq!(stats.count, 5);
        assert!((stats.mean - 0.3).abs() < 0.01);
        assert_eq!(stats.median, 0.3);
        assert_eq!(stats.min, 0.1);
        assert_eq!(stats.max, 0.5);
    }

    #[test]
    fn test_analyze_triangle_no_distortion() {
        let (mesh, unfolded) = create_test_triangle_mesh();
        let analyzer = DistortionAnalyzer::new();
        let result = analyzer.analyze(&mesh, &unfolded);

        // Для плоского треугольника искажения должны быть минимальными
        assert!(result.overall_quality_score > 0.9);
        assert!(result.problematic_faces.is_empty());
    }

    #[test]
    fn test_compute_face_area_3d() {
        let analyzer = DistortionAnalyzer::new();
        let (mesh, _) = create_test_triangle_mesh();
        
        let face = &mesh.faces[0];
        let area = analyzer.compute_face_area_3d(&mesh, face);
        
        // Площадь равностороннего треугольника со стороной 10: sqrt(3)/4 * 100 ≈ 43.3
        assert!((area - 43.3).abs() < 1.0);
    }

    #[test]
    fn test_distortion_to_color() {
        let analyzer = DistortionAnalyzer::new();
        
        // Синий (нет искажений)
        let blue = analyzer.distortion_to_color(0.0);
        assert!(blue.contains("0, 0, 255"));
        
        // Красный (максимальные искажения)
        let red = analyzer.distortion_to_color(1.0);
        assert!(red.contains("255, 0, 0"));
        
        // Зелёный (средние искажения)
        let green = analyzer.distortion_to_color(0.5);
        assert!(green.contains("0, 255, 0"));
    }

    #[test]
    fn test_generate_distortion_advice() {
        let result = DistortionAnalysisResult {
            area_distortion_stats: DistortionStats::from_values(vec![0.5]),
            angular_distortion_stats: DistortionStats::from_values(vec![15.0]),
            edge_distortion_stats: DistortionStats::from_values(vec![0.3]),
            heat_map_data: vec![],
            problematic_faces: vec![],
            overall_quality_score: 0.9,
            avg_area_distortion: 5.0,
            max_area_distortion: 10.0,
            acceptable_faces_ratio: 1.0,
        };

        let advice = generate_distortion_advice(&result);
        assert!(!advice.is_empty());
        assert!(advice.iter().any(|tip| tip.contains("отлично")));
    }
}
