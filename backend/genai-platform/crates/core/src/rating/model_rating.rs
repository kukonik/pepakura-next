use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rating {
    pub model_hash: String,
    pub user_id: String,
    pub rating: u8, // Рейтинг от 1 до 5
    pub comment: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelRatingSummary {
    pub model_hash: String,
    pub average_rating: f32,
    pub total_ratings: u32,
    pub ratings: Vec<Rating>,
}

pub struct RatingSystem {
    ratings: HashMap<String, ModelRatingSummary>,
    ratings_dir: String,
}

impl RatingSystem {
    pub fn new(ratings_dir: String) -> Self {
        // Создаем директорию рейтингов если она не существует
        if !Path::new(&ratings_dir).exists() {
            fs::create_dir_all(&ratings_dir).expect("Failed to create ratings directory");
        }
        
        Self {
            ratings: HashMap::new(),
            ratings_dir,
        }
    }
    
    pub fn add_rating(&mut self, model_hash: String, user_id: String, rating: u8, comment: Option<String>) {
        if rating < 1 || rating > 5 {
            panic!("Rating must be between 1 and 5");
        }
        
        let rating_entry = Rating {
            model_hash: model_hash.clone(),
            user_id,
            rating,
            comment,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        };
        
        // Получаем или создаем сводку рейтингов для модели
        let summary = self.ratings.entry(model_hash.clone()).or_insert_with(|| ModelRatingSummary {
            model_hash: model_hash.clone(),
            average_rating: 0.0,
            total_ratings: 0,
            ratings: Vec::new(),
        });
        
        // Добавляем новый рейтинг
        summary.ratings.push(rating_entry);
        summary.total_ratings += 1;
        
        // Пересчитываем средний рейтинг
        let sum: u32 = summary.ratings.iter().map(|r| r.rating as u32).sum();
        summary.average_rating = sum as f32 / summary.total_ratings as f32;
    }
    
    pub fn get_model_rating(&self, model_hash: &str) -> Option<&ModelRatingSummary> {
        self.ratings.get(model_hash)
    }
    
    pub fn get_all_ratings(&self) -> &HashMap<String, ModelRatingSummary> {
        &self.ratings
    }
    
    pub fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let ratings_file = format!("{}/ratings.json", self.ratings_dir);
        let json = serde_json::to_string(&self.ratings)?;
        fs::write(ratings_file, json)?;
        Ok(())
    }
    
    pub fn load_from_disk(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let ratings_file = format!("{}/ratings.json", self.ratings_dir);
        if Path::new(&ratings_file).exists() {
            let json = fs::read_to_string(ratings_file)?;
            self.ratings = serde_json::from_str(&json)?;
        }
        Ok(())
    }
    
    pub fn get_top_rated_models(&self, limit: usize) -> Vec<(&String, &ModelRatingSummary)> {
        let mut rated_models: Vec<(&String, &ModelRatingSummary)> = self.ratings.iter().collect();
        rated_models.sort_by(|a, b| b.1.average_rating.partial_cmp(&a.1.average_rating).unwrap());
        rated_models.truncate(limit);
        rated_models
    }
}