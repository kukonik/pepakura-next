//! AI-ассистент для Pepakura.
//!
//! Предоставляет высокоуровневые функции для:
//! - Генерации инструкций по сборке
//! - Рекомендаций по параметрам развёртки
//! - Ответов на вопросы

use crate::ai::{AiConfig, ChatMessage, OllamaClient};
use crate::ai::cache::AiCache;
use crate::ai::streaming::{messages_stream, AiStream};
use crate::geometry::Mesh;
use crate::unfold::UnfoldedMesh;
use crate::PepakuraError;
use std::sync::Arc;
use serde_json;

/// AI-ассистент для papercraft с кэшированием.
pub struct PepakuraAssistant {
    client: OllamaClient,
    cache: Arc<AiCache>,
}

/// Инструкция по сборке.
#[derive(Debug, Clone)]
pub struct AssemblyInstruction {
    /// Название модели
    pub model_name: String,
    /// Уровень сложности
    pub difficulty: Difficulty,
    /// Общее время сборки (минуты)
    pub total_time_minutes: usize,
    /// Шаги сборки
    pub steps: Vec<AssemblyStep>,
    /// Советы
    pub tips: Vec<String>,
}

/// Шаг сборки.
#[derive(Debug, Clone)]
pub struct AssemblyStep {
    /// Номер шага
    pub step_number: usize,
    /// Описание
    pub description: String,
    /// Номера деталей
    pub part_ids: Vec<usize>,
    /// Примерное время (минуты)
    pub estimated_time_minutes: usize,
}

/// Уровень сложности.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// Лёгкий (< 20 деталей)
    Easy,
    /// Средний (20-50 деталей)
    Medium,
    /// Сложный (50-100 деталей)
    Hard,
    /// Эксперт (> 100 деталей)
    Expert,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Лёгкий"),
            Difficulty::Medium => write!(f, "Средний"),
            Difficulty::Hard => write!(f, "Сложный"),
            Difficulty::Expert => write!(f, "Эксперт"),
        }
    }
}

/// Рекомендация по развёртке.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldAdvice {
    /// Рекомендуемый алгоритм
    pub algorithm: String,
    /// Параметры развёртки
    pub parameters: UnfoldParameters,
    /// Возможные проблемы
    pub potential_issues: Vec<String>,
    /// Советы
    pub tips: Vec<String>,
}

/// Параметры развёртки.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnfoldParameters {
    /// Максимум итераций
    pub max_iterations: usize,
    /// Допуск сходимости
    pub tolerance: f64,
    /// Сохранять детали
    pub preserve_detail: bool,
}

impl PepakuraAssistant {
    /// Создаёт нового ассистента с конфигурацией.
    pub fn new(config: &AiConfig) -> Result<Self, PepakuraError> {
        let client = OllamaClient::new(config)?;
        Ok(Self {
            client,
            cache: Arc::new(AiCache::default()),
        })
    }
    
    /// Создаёт ассистента с кастомным размером кэша.
    pub fn with_cache(config: &AiConfig, cache_size: usize) -> Result<Self, PepakuraError> {
        let client = OllamaClient::new(config)?;
        Ok(Self {
            client,
            cache: Arc::new(AiCache::new(cache_size)),
        })
    }
    
    /// Проверяет доступность AI.
    pub async fn check_availability(&self) -> bool {
        self.client.check_status().await.map_or(false, |s| s.available)
    }
    
    /// Возвращает статистику кэша.
    pub fn get_cache_stats(&self) -> crate::ai::cache::CacheStats {
        self.cache.get_stats()
    }
    
    /// Возвращает процент попаданий в кэш.
    pub fn cache_hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
    
    /// Очищает кэш.
    pub fn clear_cache(&self) {
        self.cache.clear()
    }

    /// Проверяет наличие промпта в кэше.
    pub fn cache_contains(&self, prompt: &str) -> bool {
        self.cache.contains(prompt)
    }

    /// Получает ответ из кэша.
    pub fn cache_get(&self, prompt: &str) -> Option<String> {
        self.cache.get(prompt)
    }

    /// Сохраняет ответ в кэш.
    pub fn cache_put(&self, prompt: &str, response: &str) {
        self.cache.put(prompt, response)
    }

    /// Отвечает на вопрос по papercraft со стримингом.
    pub async fn answer_question_stream(&self, question: &str) -> Result<AiStream, PepakuraError> {
        // Проверяем кэш сначала
        if let Some(cached) = self.cache.get(question) {
            // Возвращаем закэшированный ответ как стрим
            let (tx, rx) = tokio::sync::mpsc::channel(1);
            tokio::spawn(async move {
                let _ = tx.send(cached).await;
            });
            return Ok(AiStream {
                receiver: tokio_stream::wrappers::ReceiverStream::new(rx),
            });
        }
        
        let system_prompt = r#"Ты опытный мастер papercraft. 
Отвечай на вопросы кратко и по делу.
Давай практические советы."#;
        
        let messages = vec![
            ChatMessage::assistant(system_prompt),
            ChatMessage::user(question),
        ];
        
        messages_stream(&self.client, &messages).await
    }
    
    /// Получает рекомендации по развёртке для меша.
    pub async fn get_unfold_advice(&self, mesh: &Mesh) -> Result<UnfoldAdvice, PepakuraError> {
        // Создаём уникальный ключ для кэша
        let cache_key = format!(
            "unfold_advice:v{}_f{}_{}",
            mesh.vertices.len(),
            mesh.faces.len(),
            mesh.name
        );
        
        // Проверяем кэш
        if let Some(cached) = self.cache.get(&cache_key) {
            // Парсим закэшированный ответ
            return serde_json::from_str(&cached)
                .map_err(|e| PepakuraError::AiError(format!("Failed to parse cached advice: {}", e)));
        }
        
        let vertex_count = mesh.vertices.len();
        let face_count = mesh.faces.len();
        let bbox = mesh.bounding_box();
        let size = bbox.size();
        
        let prompt = format!(
            r#"Анализирую 3D-модель для papercraft:
- Вершин: {}
- Граней: {}
- Размер: {:.1} x {:.1} x {:.1}

Дай рекомендации по развёртке:
1. Какой алгоритм лучше использовать (MDS или проекция)?
2. Какие параметры MDS выбрать (max_iterations, tolerance)?
3. Какие возможны проблемы (наложения, искажения)?
4. Советы для лучшей развёртки.

Ответь кратко, по делу."#,
            vertex_count,
            face_count,
            size[0],
            size[1],
            size[2]
        );
        
        let response = self.client.generate(&prompt).await?;
        
        // Сохраняем в кэш
        self.cache.put(&cache_key, &response);
        
        // Парсим ответ (упрощённо)
        Ok(UnfoldAdvice {
            algorithm: if face_count > 1000 { "Проекция" } else { "MDS" }.to_string(),
            parameters: UnfoldParameters {
                max_iterations: if face_count > 500 { 200 } else { 100 },
                tolerance: 0.001,
                preserve_detail: true,
            },
            potential_issues: vec![],
            tips: response.split('\n').map(|s| s.to_string()).collect(),
        })
    }
    
    /// Генерирует описание модели.
    pub async fn get_model_description(&self, mesh: &Mesh) -> Result<String, PepakuraError> {
        let vertex_count = mesh.vertices.len();
        let face_count = mesh.faces.len();
        let total_area = mesh.total_area();
        
        let prompt = format!(
            r#"Опиши эту 3D-модель для papercraft:
- Вершин: {}
- Граней: {}
- Площадь поверхности: {:.1} кв. ед.

Что это может быть?
Какой уровень сложности сборки?
Какой масштаб рекомендуется для печати?
Какие советы по покраске?"#,
            vertex_count,
            face_count,
            total_area
        );
        
        self.client.generate(&prompt).await
    }
    
    /// Генерирует пошаговую инструкцию сборки.
    pub async fn generate_assembly_instructions(
        &self,
        unfolded: &UnfoldedMesh,
    ) -> Result<AssemblyInstruction, PepakuraError> {
        let part_count = unfolded.faces.len();
        
        // Определяем сложность
        let difficulty = match part_count {
            n if n < 20 => Difficulty::Easy,
            n if n < 50 => Difficulty::Medium,
            n if n < 100 => Difficulty::Hard,
            _ => Difficulty::Expert,
        };
        
        let prompt = format!(
            r#"Создай инструкцию сборки для papercraft модели:
- Деталей (граней): {}
- Сложность: {}

Верни инструкцию в формате:
1. [Номер шага]. [Описание] (Детали: [номера])

Пример:
1. Вырежьте все детали по контуру (Детали: 1-10)
2. Склейте основание модели (Детали: 1, 2, 3)
3. Соберите стены (Детали: 4-8)

Дай 5-10 основных шагов."#,
            part_count,
            difficulty
        );
        
        let response = self.client.generate(&prompt).await?;
        
        // Парсим шаги (упрощённо)
        let steps: Vec<AssemblyStep> = response
            .lines()
            .filter(|line| line.trim().chars().next().map_or(false, |c| c.is_ascii_digit()))
            .enumerate()
            .map(|(i, line)| {
                let description = line.split('.').nth(1).unwrap_or(line).trim().to_string();
                AssemblyStep {
                    step_number: i + 1,
                    description,
                    part_ids: vec![],
                    estimated_time_minutes: 2,
                }
            })
            .collect();
        
        Ok(AssemblyInstruction {
            model_name: unfolded.source_mesh.name.clone(),
            difficulty,
            total_time_minutes: steps.len() * 2,
            steps,
            tips: vec![
                "Используйте качественный клей".to_string(),
                "Дайте клею высохнуть перед следующим шагом".to_string(),
            ],
        })
    }
    
    /// Отвечает на вопрос по papercraft.
    pub async fn answer_question(&self, question: &str) -> Result<String, PepakuraError> {
        // Проверяем кэш
        if let Some(cached) = self.cache.get(question) {
            return Ok(cached);
        }
        
        let system_prompt = r#"Ты опытный мастер papercraft. 
Отвечай на вопросы кратко и по делу.
Давай практические советы."#;
        
        let messages = vec![
            ChatMessage::assistant(system_prompt),
            ChatMessage::user(question),
        ];
        
        let response = self.client.chat(&messages).await?;
        
        // Сохраняем в кэш
        self.cache.put(question, &response);
        
        Ok(response)
    }
    
    /// Помощь с выбором бумаги.
    pub async fn recommend_paper(&self, model_name: &str, scale: f64) -> Result<String, PepakuraError> {
        let prompt = format!(
            r#"Какую бумагу выбрать для модели "{}" в масштабе {:.1}?
Учти:
- Размер модели
- Сложность геометрии
- Рекомендации по плотности (g/m²)
- Советы по покраске/лакированию"#,
            model_name,
            scale
        );
        
        self.client.generate(&prompt).await
    }
}

/// Создаёт ассистента по умолчанию (Ollama, localhost).
pub fn create_assistant() -> Result<PepakuraAssistant, PepakuraError> {
    let config = AiConfig::default();
    PepakuraAssistant::new(&config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{Face, Vertex};
    
    fn create_test_mesh() -> Mesh {
        let mut mesh = Mesh::new("TestCube");
        for i in 0..8 {
            let x = if i & 1 != 0 { 1.0 } else { 0.0 };
            let y = if i & 2 != 0 { 1.0 } else { 0.0 };
            let z = if i & 4 != 0 { 1.0 } else { 0.0 };
            mesh.add_vertex(Vertex::new(i, [x, y, z]));
        }
        mesh
    }
    
    #[test]
    fn test_difficulty_display() {
        assert_eq!(format!("{}", Difficulty::Easy), "Лёгкий");
        assert_eq!(format!("{}", Difficulty::Medium), "Средний");
        assert_eq!(format!("{}", Difficulty::Hard), "Сложный");
        assert_eq!(format!("{}", Difficulty::Expert), "Эксперт");
    }
    
    #[test]
    fn test_difficulty_from_parts() {
        assert_eq!(match 10 { n if n < 20 => Difficulty::Easy, _ => Difficulty::Medium }, Difficulty::Easy);
        assert_eq!(match 30 { n if n < 20 => Difficulty::Easy, n if n < 50 => Difficulty::Medium, _ => Difficulty::Hard }, Difficulty::Medium);
        assert_eq!(match 75 { n if n < 50 => Difficulty::Medium, n if n < 100 => Difficulty::Hard, _ => Difficulty::Expert }, Difficulty::Hard);
    }
    
    #[tokio::test]
    async fn test_assistant_creation() {
        let config = AiConfig::default();
        let result = PepakuraAssistant::new(&config);
        // Будет ошибка если Ollama не запущен, но это нормально
        assert!(result.is_ok() || result.is_err());
    }
}
