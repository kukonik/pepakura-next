//! # Mesh Analyzer
//!
//! Анализатор мешей с использованием LLM для рекомендаций.
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::analysis::mesh_analyzer::MeshAnalyzer;
//! use pepakura_core::geometry::Mesh;
//!
//! let mut analyzer = MeshAnalyzer::new();
//!
//! // Проверка доступности LLM
//! if analyzer.is_llm_available() {
//!     println!("LLM бэкенд: {}", analyzer.backend_name());
//! }
//!
//! // Анализ меша
//! let mesh = Mesh::new("model");
//! let result = analyzer.analyze(&mesh);
//!
//! // Числовая статистика — всегда доступна
//! println!("{}", result.stats.summary());
//!
//! // LLM-рекомендации — если Ollama запущен
//! if let Some(analysis) = &result.ai_analysis {
//!     println!("Сложность: {}", analysis.difficulty);
//!     for issue in &analysis.issues {
//!         println!("  [{}] {}: {}", issue.severity, issue.code, issue.message);
//!     }
//! }
//! ```

use crate::geometry::Mesh;
#[cfg(feature = "llm")] use crate::ai::prompts::{
    MeshStatsPrompt,
    create_difficulty_prompt,
    DIFFICULTY_SYSTEM_PROMPT,
};
use crate::analysis::mesh_stats::MeshStats;

#[cfg(feature = "llm")]
use crate::ai::local_llm::LocalLlmRouter;

/// Результат анализа меша
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Числовая статистика меша
    pub stats: MeshStats,
    /// LLM-анализ (если доступен)
    pub ai_analysis: Option<AiAnalysisResult>,
    /// Время анализа в миллисекундах
    pub analysis_time_ms: u64,
    /// Ошибка анализа (если была)
    pub error: Option<String>,
}

/// Проблема, найденная при анализе меша
#[derive(Debug, Clone)]
pub struct MeshIssue {
    pub code: String,
    pub message: String,
    pub severity: String,
    pub count: u32,
}

/// Рекомендация по улучшению меша
#[derive(Debug, Clone)]
pub struct Recommendation {
    pub text: String,
    pub priority: String,
    pub category: String,
}
/// Результат LLM-анализа
#[derive(Debug, Clone)]
pub struct AiAnalysisResult {
    /// Уровень сложности
    pub difficulty: String,
    /// Общая оценка (0.0 - 1.0)
    pub overall_score: f64,
    /// Оценка пригодности для развёртки (0.0 - 1.0)
    pub unfoldability_score: f64,
    /// Оценка детализации (0.0 - 1.0)
    pub detail_score: f64,
    /// Оценка пригодности для печати (0.0 - 1.0)
    pub printability_score: f64,
    /// Обоснование оценки
    pub reasoning: String,
    /// Найденные проблемы
    pub issues: Vec<MeshIssue>,
    /// Рекомендации по улучшению
    pub recommendations: Vec<Recommendation>,
}

/// Анализатор мешей
pub struct MeshAnalyzer {
    #[cfg(feature = "llm")]
    llm_router: Option<LocalLlmRouter>,
    model_name: String,
    use_cache: bool,
}

impl MeshAnalyzer {
    /// Создать новый анализатор с настройками по умолчанию
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "llm")]
            llm_router: Some(LocalLlmRouter::new("http://localhost:11434")),
            model_name: "qwen2.5:7b".to_string(),
            use_cache: true,
        }
    }

    /// Создать анализатор с кастомным URL LLM-бэкенда
    pub fn with_backend(url: &str) -> Self {
        Self {
            #[cfg(feature = "llm")]
            llm_router: Some(LocalLlmRouter::new(url)),
            model_name: "qwen2.5:7b".to_string(),
            use_cache: true,
        }
    }

    /// Создать анализатор с кастомной моделью
    pub fn with_model(model: &str) -> Self {
        Self {
            #[cfg(feature = "llm")]
            llm_router: Some(LocalLlmRouter::new("http://localhost:11434")),
            model_name: model.to_string(),
            use_cache: true,
        }
    }

    /// Установить модель для анализа
    pub fn set_model(&mut self, model: &str) {
        self.model_name = model.to_string();
    }

    /// Включить/выключить кэширование
    pub fn set_cache(&mut self, enabled: bool) {
        self.use_cache = enabled;
    }

    /// Проверить доступность LLM
    pub fn is_llm_available(&self) -> bool {
        #[cfg(feature = "llm")]
        {
            if let Some(ref router) = self.llm_router {
                return router.check_status().available;
            }
            false
        }
        #[cfg(not(feature = "llm"))]
        {
            false
        }
    }

    /// Получить название бэкенда
    pub fn backend_name(&self) -> String {
        #[cfg(feature = "llm")]
        {
            if let Some(ref router) = self.llm_router {
                let status = router.check_status();
                return if status.available {
                    status.backend_name.to_string()
                } else {
                    "unavailable".to_string()
                };
            }
            "not configured".to_string()
        }
        #[cfg(not(feature = "llm"))]
        {
            "disabled".to_string()
        }
    }

    /// Получить статус LLM
    pub fn llm_status(&self) -> LlmStatusInfo {
        #[cfg(feature = "llm")]
        {
            if let Some(ref router) = self.llm_router {
                let status = router.check_status();
                return LlmStatusInfo {
                    available: status.available,
                    backend_name: Some(status.backend_name),
                    version: status.version,
                    models: status.models,
                    error: status.error,
                };
            }
            LlmStatusInfo {
                available: false,
                backend_name: None,
                version: None,
                models: vec![],
                error: Some("LLM feature not enabled".to_string()),
            }
        }
        #[cfg(not(feature = "llm"))]
        {
            LlmStatusInfo {
                available: false,
                backend_name: None,
                version: None,
                models: vec![],
                error: Some("LLM feature not compiled".to_string()),
            }
        }
    }

    /// Проанализировать меш
    pub fn analyze(&self, mesh: &Mesh) -> AnalysisResult {
        let start = std::time::Instant::now();

        // Вычисляем статистику
        let stats = MeshStats::from_mesh(mesh);

        // Пытаемся получить LLM-анализ
        let ai_analysis = self.analyze_with_llm(&stats);

        let analysis_time = start.elapsed().as_millis() as u64;

        AnalysisResult {
            stats,
            ai_analysis,
            analysis_time_ms: analysis_time,
            error: None,
        }
    }

    /// LLM-анализ статистики
    #[cfg(feature = "llm")]
    fn analyze_with_llm(&self, stats: &MeshStats) -> Option<AiAnalysisResult> {
        let router = self.llm_router.as_ref()?;

        // Проверяем доступность
        let status = router.check_status();
        if !status.available {
            return None;
        }

        // Формируем промпт
        let prompt = create_difficulty_prompt(&MeshStatsPrompt {
            vertex_count: stats.vertex_count,
            face_count: stats.face_count,
            edge_count: stats.edge_count,
            bbox_size: stats.bbox_size,
            surface_area: stats.surface_area,
            volume: stats.volume,
            avg_face_area: stats.avg_face_area,
            min_face_area: stats.min_face_area,
            max_face_area: stats.max_face_area,
            isolated_parts: stats.isolated_parts,
        });

        // Запрашиваем анализ
        match router.generate_json(&self.model_name, &prompt, Some(DIFFICULTY_SYSTEM_PROMPT)) {
            Ok(json) => {
                // Парсим ответ
                Self::parse_llm_response(&json)
            }
            Err(e) => {
                log::warn!("LLM analysis failed: {}", e);
                None
            }
        }
    }

    #[cfg(not(feature = "llm"))]
    fn analyze_with_llm(&self, _stats: &MeshStats) -> Option<AiAnalysisResult> {
        None
    }

    /// Распарсить ответ LLM
    fn parse_llm_response(json: &serde_json::Value) -> Option<AiAnalysisResult> {
        // Пытаемся распарсить difficulty_analysis
        let difficulty_analysis = json.get("difficulty_analysis")?;

        let difficulty = difficulty_analysis
            .get("difficulty")?
            .as_str()?
            .to_string();

        let overall_score = difficulty_analysis
            .get("overall_score")?
            .as_f64()
            .unwrap_or(0.5);

        let unfoldability_score = difficulty_analysis
            .get("unfoldability_score")?
            .as_f64()
            .unwrap_or(0.5);

        let detail_score = difficulty_analysis
            .get("detail_score")?
            .as_f64()
            .unwrap_or(0.5);

        let printability_score = difficulty_analysis
            .get("printability_score")?
            .as_f64()
            .unwrap_or(0.5);

        let reasoning = difficulty_analysis
            .get("reasoning")?
            .as_str()?
            .to_string();

        // Парсим issues
        let issues = json
            .get("issues")
            .and_then(|i| i.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        Some(MeshIssue {
                            code: item.get("code")?.as_str()?.to_string(),
                            message: item.get("message")?.as_str()?.to_string(),
                            severity: item.get("severity")?.as_str()?.to_string(),
                            count: item.get("count")?.as_u64()? as u32,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Парсим recommendations
        let recommendations = json
            .get("recommendations")
            .and_then(|r| r.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        Some(Recommendation {
                            text: item.get("text")?.as_str()?.to_string(),
                            priority: item.get("priority")?.as_str()?.to_string(),
                            category: item.get("category")?.as_str()?.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        Some(AiAnalysisResult {
            difficulty,
            overall_score,
            unfoldability_score,
            detail_score,
            printability_score,
            reasoning,
            issues,
            recommendations,
        })
    }

    /// Быстрый анализ без LLM
    pub fn analyze_quick(&self, mesh: &Mesh) -> AnalysisResult {
        let start = std::time::Instant::now();

        let stats = MeshStats::from_mesh(mesh);

        let analysis_time = start.elapsed().as_millis() as u64;

        AnalysisResult {
            stats,
            ai_analysis: None,
            analysis_time_ms: analysis_time,
            error: None,
        }
    }
}

impl Default for MeshAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Информация о статусе LLM
#[derive(Debug, Clone)]
pub struct LlmStatusInfo {
    /// Доступен ли бэкенд
    pub available: bool,
    /// Название бэкенда
    pub backend_name: Option<String>,
    /// Версия
    pub version: Option<String>,
    /// Список моделей
    pub models: Vec<String>,
    /// Ошибка
    pub error: Option<String>,
}

/// Трейт для объектов, предоставляющих меш
pub trait MeshProvider {
    /// Получить статистику меша
    fn mesh_stats(&self) -> MeshStats;

    /// Хэш вершин для кэширования
    fn vertices_hash(&self) -> u64;
}

impl MeshProvider for Mesh {
    fn mesh_stats(&self) -> MeshStats {
        MeshStats::from_mesh(self)
    }

    fn vertices_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        for v in &self.vertices {
            let p = v.position;
            // Хэшируем координаты с точностью до 6 знаков
            ((p[0] * 1e6) as i64).hash(&mut hasher);
            ((p[1] * 1e6) as i64).hash(&mut hasher);
            ((p[2] * 1e6) as i64).hash(&mut hasher);
        }
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Vertex, Face};

    #[test]
    fn test_analyzer_creation() {
        let analyzer = MeshAnalyzer::new();
        #[cfg(feature = "llm")]
        {
            assert!(analyzer.llm_router.is_some());
        }
    }

    #[test]
    fn test_quick_analysis() {
        let mut mesh = Mesh::new("test");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [10.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.0, 10.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));

        let analyzer = MeshAnalyzer::new();
        let result = analyzer.analyze_quick(&mesh);

        assert_eq!(result.stats.vertex_count, 3);
        assert_eq!(result.stats.face_count, 1);
        assert!(result.ai_analysis.is_none());
    }

    #[test]
    fn test_mesh_provider_trait() {
        let mut mesh = Mesh::new("test");
        mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(1, [10.0, 0.0, 0.0]));
        mesh.add_vertex(Vertex::new(2, [0.0, 10.0, 0.0]));
        mesh.add_face(Face::new(0, 1, 2));

        let stats = mesh.mesh_stats();
        assert_eq!(stats.vertex_count, 3);

        let hash = mesh.vertices_hash();
        assert!(hash > 0);
    }
}



