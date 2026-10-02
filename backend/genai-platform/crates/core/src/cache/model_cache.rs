use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedModel {
    pub prompt: String,
    pub model_url: String,
    pub format: String,
    pub created_at: u64,
    pub hash: String,
}

pub struct ModelCache {
    cache: HashMap<String, CachedModel>,
    cache_dir: String,
}

impl ModelCache {
    pub fn new(cache_dir: String) -> Self {
        // Создаем директорию кэша если она не существует
        if !Path::new(&cache_dir).exists() {
            fs::create_dir_all(&cache_dir).expect("Failed to create cache directory");
        }
        
        Self {
            cache: HashMap::new(),
            cache_dir,
        }
    }
    
    pub fn generate_hash(prompt: &str) -> String {
        // Простая хэш-функция для демонстрации
        // В реальной реализации можно использовать более надежную хэш-функцию
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    pub fn get_cached_model(&self, prompt: &str) -> Option<CachedModel> {
        let hash = Self::generate_hash(prompt);
        self.cache.get(&hash).cloned()
    }
    
    pub fn cache_model(&mut self, prompt: &str, model_url: String, format: String) {
        let hash = Self::generate_hash(prompt);
        let cached_model = CachedModel {
            prompt: prompt.to_string(),
            model_url,
            format,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
            hash: hash.clone(),
        };
        
        self.cache.insert(hash, cached_model);
    }
    
    pub fn save_to_disk(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_file = format!("{}/cache.json", self.cache_dir);
        let json = serde_json::to_string(&self.cache)?;
        fs::write(cache_file, json)?;
        Ok(())
    }
    
    pub fn load_from_disk(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_file = format!("{}/cache.json", self.cache_dir);
        if Path::new(&cache_file).exists() {
            let json = fs::read_to_string(cache_file)?;
            self.cache = serde_json::from_str(&json)?;
        }
        Ok(())
    }
    
    pub fn get_cache_stats(&self) -> (usize, u64) {
        let count = self.cache.len();
        let size: u64 = self.cache.values().map(|m| m.model_url.len() as u64).sum();
        (count, size)
    }
}