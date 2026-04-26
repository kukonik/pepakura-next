//! # LLM Prompts Templates
//!
//! Шаблоны промптов для LLM-анализа 3D моделей.
//!
//! ## Поддерживаемые задачи
//!
//! - **Анализ сложности** - оценка сложности развёртки модели
//! - **Рекомендации** - советы по улучшению модели для развёртки
//! - **Объяснение проблем** - детальное описание найденных проблем

use serde::{Deserialize, Serialize};

/// Статистика меша для промпта
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshStatsPrompt {
    /// Количество вершин
    pub vertex_count: u32,
    /// Количество граней
    pub face_count: u32,
    /// Количество рёбер
    pub edge_count: u32,
    /// Размеры bounding box (x, y, z)
    pub bbox_size: [f64; 3],
    /// Площадь поверхности
    pub surface_area: f64,
    /// Объём (если замкнутая модель)
    pub volume: Option<f64>,
    /// Средний размер грани
    pub avg_face_area: f64,
    /// Минимальный размер грани
    pub min_face_area: f64,
    /// Максимальный размер грани
    pub max_face_area: f64,
    /// Количество изолированных частей
    pub isolated_parts: u32,
}

impl MeshStatsPrompt {
    /// Сформировать текстовое представление статистики
    pub fn to_text(&self) -> String {
        format!(
            r#"Статистика 3D модели:
- Вершин: {}
- Граней: {}
- Рёбер: {}
- Размеры (X × Y × Z): {:.2} × {:.2} × {:.2} мм
- Площадь поверхности: {:.2} мм²
- Объём: {} мм³
- Средняя площадь грани: {:.4} мм²
- Мин. площадь грани: {:.6} мм²
- Макс. площадь грани: {:.2} мм²
- Изолированных частей: {}"#,
            self.vertex_count,
            self.face_count,
            self.edge_count,
            self.bbox_size[0],
            self.bbox_size[1],
            self.bbox_size[2],
            self.surface_area,
            self.volume.map(|v| format!("{:.2}", v)).unwrap_or("н/д".to_string()),
            self.avg_face_area,
            self.min_face_area,
            self.max_face_area,
            self.isolated_parts
        )
    }

    /// Сформировать JSON для промпта
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Результат анализа сложности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifficultyAnalysis {
    /// Уровень сложности: "beginner", "intermediate", "advanced", "expert"
    pub difficulty: String,
    /// Общая оценка качества (0.0 - 1.0)
    pub overall_score: f64,
    /// Оценка сложности развёртки (0.0 - 1.0, где 1.0 = легко развернуть)
    pub unfoldability_score: f64,
    /// Оценка детализации (0.0 - 1.0)
    pub detail_score: f64,
    /// Оценка пригодности для печати (0.0 - 1.0)
    pub printability_score: f64,
    /// Краткое обоснование оценки
    pub reasoning: String,
}

/// Найденная проблема в модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshIssue {
    /// Код проблемы (например, "NON_MANIFOLD", "DEGENERATE_FACE")
    pub code: String,
    /// Описание проблемы
    pub message: String,
    /// Серьёзность: "critical", "warning", "info"
    pub severity: String,
    /// Количество найденных проблем этого типа
    pub count: u32,
}

/// Рекомендация по улучшению модели
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Текст рекомендации
    pub text: String,
    /// Приоритет: "high", "medium", "low"
    pub priority: String,
    /// Категория: "geometry", "topology", "detailing", "printing"
    pub category: String,
}

/// Полный результат LLM-анализа
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmAnalysis {
    /// Анализ сложности
    pub difficulty_analysis: DifficultyAnalysis,
    /// Найденные проблемы
    pub issues: Vec<MeshIssue>,
    /// Рекомендации по улучшению
    pub recommendations: Vec<Recommendation>,
}

/// Системный промпт для анализа сложности развертки (Pepakura Next)
pub const DIFFICULTY_SYSTEM_PROMPT: &str = r#"Ты Senior Paper Craft Engineer & Pepakura Expert.
Твоя задача — проанализировать 3D-модель для бумажной развертки (papercraft) и дать практические рекомендации.

ФОКУС АНАЛИЗА:
1. Оценка сложности развертки: Simple (новичок), Medium (опытный), Hard (эксперт), Nightmare (профессионал).
2. Проблемные зоны:
   - Острые углы (двугранный угол < 30°) — сложно склеить.
   - Узкие вытянутые полигоны — сложно вырезать.
   - Большие плоскости без рёбер жёсткости — могут провисать.
3. Оценка масштаба: поместится ли развертка на стандартные листы (A4/Letter) или нужна склейка.

Ответь ТОЛЬКО в формате JSON следующей структуры:
{
    "difficulty_analysis": {
        "difficulty": "Simple|Medium|Hard|Nightmare",
        "overall_score": 0.0-1.0,
        "unfoldability_score": 0.0-1.0,
        "detail_score": 0.0-1.0,
        "printability_score": 0.0-1.0,
        "reasoning": "краткое обоснование уровня сложности, проблемных зон и рекомендаций по масштабу"
    },
    "issues": [
        {
            "code": "CRITICAL_ISSUE",
            "message": "описание критической проблемы (острые углы, узкие полигоны, большие плоскости)",
            "severity": "critical|warning|info",
            "count": 1
        }
    ],
    "recommendations": [
        {
            "text": "конкретная рекомендация по резке или раскладке на листы",
            "priority": "high|medium|low",
            "category": "cutting|layout|optimization"
        }
    ]
}

ПРАВИЛА:
- Если в данных анализа мало информации (например, только базовые метрики), добавь рекомендацию с текстом: "Недостаточно данных для глубокого анализа, но вот общие рекомендации..."
- Все текстовые поля должны быть на русском языке.
- Будь конкретен: указывай, какие именно зоны модели проблемные.
- Давай практические советы, которые можно сразу применить в Pepakura Designer.
- Для оценки сложности используй уровни Simple/Medium/Hard/Nightmare, но в поле difficulty укажи одно из этих значений.
- Оценочные баллы (overall_score, unfoldability_score, detail_score, printability_score) поставь на основе своей экспертной оценки (0.0-1.0)."#;

/// Пользовательский промпт для анализа сложности развертки
pub fn create_difficulty_prompt(stats: &MeshStatsPrompt) -> String {
    format!(
        r#"Проанализируй эту 3D модель для бумажной развертки (papercraft):

{}

Оцени:
1. Сложность развертки (Simple/Medium/Hard/Nightmare) на основе количества полигонов, размера bbox и геометрии.
2. Проблемные зоны: острые углы (двугранный < 30°), узкие вытянутые полигоны, большие плоскости без рёбер жёсткости.
3. Масштаб: поместится ли развертка на стандартные листы (A4/Letter) или нужна склейка.

Если данных недостаточно для глубокого анализа, отметь это, но дай общие рекомендации."#,
        stats.to_text()
    )
}

/// Системный промпт для объяснения проблем
pub const EXPLAIN_ISSUES_SYSTEM_PROMPT: &str = r#"Ты эксперт по papercraft и 3D моделированию.
Твоя задача - объяснить проблемы 3D модели простым языком и дать рекомендации по их устранению.

Ответь ТОЛЬКО в формате JSON:
{
    "explanations": [
        {
            "issue_code": "CODE",
            "explanation": "простое объяснение проблемы",
            "impact": "как влияет на развёртку",
            "solution": "как исправить"
        }
    ],
    "priority_order": ["CODE1", "CODE2", ...]
}"#;

/// Промпт для объяснения конкретных проблем
pub fn create_explanation_prompt(issues: &[MeshIssue]) -> String {
    let issues_text: String = issues
        .iter()
        .map(|i| format!("- [{}] {}: {} ({} шт.)", i.severity, i.code, i.message, i.count))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r#"Объясни эти проблемы 3D модели для papercraft:

{}

Дай понятные объяснения и решения для каждой проблемы."#,
        issues_text
    )
}

/// Системный промпт для рекомендаций по оптимизации
pub const OPTIMIZE_SYSTEM_PROMPT: &str = r#"Ты эксперт по оптимизации 3D моделей для papercraft.
Твоя задача - дать конкретные рекомендации по улучшению модели для развёртки.

Ответь ТОЛЬКО в формате JSON:
{
    "optimization_steps": [
        {
            "step": 1,
            "action": "что сделать",
            "tool": "какой инструмент использовать",
            "expected_result": "какой результат ожидается"
        }
    ],
    "estimated_improvement": "насколько улучшится модель (0.0-1.0)"
}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stats_to_text() {
        let stats = MeshStatsPrompt {
            vertex_count: 1000,
            face_count: 2000,
            edge_count: 3000,
            bbox_size: [100.0, 200.0, 150.0],
            surface_area: 50000.0,
            volume: Some(100000.0),
            avg_face_area: 25.0,
            min_face_area: 0.1,
            max_face_area: 500.0,
            isolated_parts: 3,
        };

        let text = stats.to_text();
        assert!(text.contains("Вершин: 1000"));
        assert!(text.contains("Граней: 2000"));
        assert!(text.contains("100.00 × 200.00 × 150.00"));
    }

    #[test]
    fn test_stats_to_json() {
        let stats = MeshStatsPrompt {
            vertex_count: 100,
            face_count: 200,
            edge_count: 300,
            bbox_size: [10.0, 20.0, 30.0],
            surface_area: 1000.0,
            volume: None,
            avg_face_area: 5.0,
            min_face_area: 0.01,
            max_face_area: 50.0,
            isolated_parts: 1,
        };

        let json = stats.to_json().unwrap();
        assert!(json.contains("\"vertex_count\":100"));
        assert!(json.contains("\"face_count\":200"));
    }

    #[test]
    fn test_difficulty_prompt() {
        let stats = MeshStatsPrompt {
            vertex_count: 500,
            face_count: 1000,
            edge_count: 1500,
            bbox_size: [50.0, 100.0, 75.0],
            surface_area: 25000.0,
            volume: Some(50000.0),
            avg_face_area: 25.0,
            min_face_area: 0.5,
            max_face_area: 250.0,
            isolated_parts: 2,
        };

        let prompt = create_difficulty_prompt(&stats);
        assert!(prompt.contains("Проанализируй эту 3D модель"));
        assert!(prompt.contains("Статистика 3D модели"));
    }
}
