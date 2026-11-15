//! Кэширование для AI запросов.
//! 
//! Использует LRU (Least Recently Used) стратегию для кэширования ответов.
//! Кэширует по хешу промпта для быстрого поиска.

use lru::LruCache;
use sha2::{Sha256, Digest};
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use rusqlite;

/// Запись в кэше.
#[derive(Debug, Clone)]
pub struct CacheEntry {
    /// Исходный промпт
    pub prompt: String,
    /// Ответ AI
    pub response: String,
    /// Время создания (timestamp)
    pub created_at: u64,
    /// Количество обращений
    pub hit_count: u32,
}

impl CacheEntry {
    pub fn new(prompt: String, response: String) -> Self {
        Self {
            prompt,
            response,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            hit_count: 0,
        }
    }
}

/// AI кэш с LRU стратегией.
pub struct AiCache {
    /// LRU кэш для быстрого доступа
    cache: Arc<Mutex<LruCache<String, CacheEntry>>>,
    /// Статистика
    stats: Arc<Mutex<CacheStats>>,
}

/// Статистика кэша.
#[derive(Debug, Default, Clone, Copy)]
pub struct CacheStats {
    /// Количество попаданий
    pub hits: u32,
    /// Количество промахов
    pub misses: u32,
    /// Количество сохранений
    pub saves: u32,
}

impl AiCache {
    /// Создаёт новый кэш с указанным размером.
    /// 
    /// # Аргументы
    /// * `max_size` - максимальное количество записей в кэше
    /// 
    /// # Пример
    /// 
    /// ```
    /// use pepakura_core::ai::cache::AiCache;
    /// 
    /// let cache = AiCache::new(100);
    /// ```
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(
                LruCache::new(NonZeroUsize::new(max_size).unwrap())
            )),
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }
    
    /// Создаёт кэш по умолчанию (1000 записей).
    pub fn default() -> Self {
        Self::new(1000)
    }
    
    /// Проверяет наличие запроса в кэше.
    /// 
    /// # Аргументы
    /// * `prompt` - исходный промпт
    /// 
    /// # Возвращает
    /// * `Some(String)` - ответ из кэша
    /// * `None` - кэш не содержит ответа
    pub fn get(&self, prompt: &str) -> Option<String> {
        let hash = self.hash_prompt(prompt);
        
        let mut cache = self.cache.lock().unwrap();
        if let Some(entry) = cache.get_mut(&hash) {
            // Увеличиваем счётчик попаданий
            entry.hit_count += 1;
            
            // Обновляем статистику
            let mut stats = self.stats.lock().unwrap();
            stats.hits += 1;
            
            Some(entry.response.clone())
        } else {
            // Обновляем статистику
            let mut stats = self.stats.lock().unwrap();
            stats.misses += 1;
            None
        }
    }
    
    /// Сохраняет ответ в кэш.
    /// 
    /// # Аргументы
    /// * `prompt` - исходный промпт
    /// * `response` - ответ AI
    pub fn put(&self, prompt: &str, response: &str) {
        let hash = self.hash_prompt(prompt);
        let entry = CacheEntry::new(prompt.to_string(), response.to_string());
        
        let mut cache = self.cache.lock().unwrap();
        cache.put(hash, entry);
        
        // Обновляем статистику
        let mut stats = self.stats.lock().unwrap();
        stats.saves += 1;
    }
    
    /// Проверяет наличие запроса в кэше без обновления статистики.
    pub fn contains(&self, prompt: &str) -> bool {
        let hash = self.hash_prompt(prompt);
        let cache = self.cache.lock().unwrap();
        cache.contains(&hash)
    }
    
    /// Очищает весь кэш.
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
        
        let mut stats = self.stats.lock().unwrap();
        *stats = CacheStats::default();
    }
    
    /// Возвращает размер кэша.
    pub fn len(&self) -> usize {
        let cache = self.cache.lock().unwrap();
        cache.len()
    }
    
    /// Проверяет пустоту кэша.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    
    /// Возвращает статистику кэша.
    pub fn get_stats(&self) -> CacheStats {
        let stats = self.stats.lock().unwrap();
        *stats
    }
    
    /// Вычисляет хеш промпта.
    fn hash_prompt(&self, prompt: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prompt.as_bytes());
        hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect::<String>()
    }
    
    /// Возвращает процент попаданий (hit rate).
    pub fn hit_rate(&self) -> f64 {
        let stats = self.stats.lock().unwrap();
        let total = stats.hits as u64 + stats.misses as u64;
        if total == 0 {
            0.0
        } else {
            stats.hits as f64 / total as f64 * 100.0
        }
    }
}

/// Персистентный кэш с сохранением на диск.
#[cfg(not(target_arch = "wasm32"))]
pub struct PersistentCache {
    /// Временный кэш
    cache: AiCache,
    /// Путь к базе данных
    _db_path: String,
    /// SQLite соединение
    conn: Option<rusqlite::Connection>,
}

#[cfg(not(target_arch = "wasm32"))]
impl PersistentCache {
    /// Создаёт персистентный кэш.
    ///
    /// # Аргументы
    /// * `db_path` - путь к SQLite базе
    /// * `max_size` - максимальное количество записей в памяти
    pub fn new(db_path: &str, max_size: usize) -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open(db_path)?;
        
        // Создаём таблицу
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cache (
                hash TEXT PRIMARY KEY,
                prompt TEXT NOT NULL,
                response TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                hit_count INTEGER NOT NULL
            )",
            [],
        )?;
        
        Ok(Self {
            cache: AiCache::new(max_size),
            _db_path: db_path.to_string(),
            conn: Some(conn),
        })
    }
    
    /// Загружает запись из базы данных.
    pub fn load_from_db(&self, prompt: &str) -> Option<String> {
        let hash = self.cache.hash_prompt(prompt);
        
        if let Some(conn) = &self.conn {
            let mut stmt = conn.prepare(
                "SELECT response FROM cache WHERE hash = ?1"
            ).ok()?;
            
            let response: String = stmt.query_row([&hash], |row| {
                row.get(0)
            }).ok()?;
            
            // Добавляем в memory cache
            self.cache.put(prompt, &response);
            
            Some(response)
        } else {
            None
        }
    }
    
    /// Сохраняет запись в базу данных.
    pub fn save_to_db(&self, prompt: &str, response: &str) -> Result<(), rusqlite::Error> {
        let hash = self.cache.hash_prompt(prompt);
        let created_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        if let Some(conn) = &self.conn {
            conn.execute(
                "INSERT OR REPLACE INTO cache (hash, prompt, response, created_at, hit_count)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                [&hash, prompt, response, &created_at.to_string(), "0"],
            )?;
        }
        
        Ok(())
    }
    
    /// Очищает старые записи из базы данных.
    pub fn cleanup_old_entries(&self, max_age_days: u64) -> Result<usize, rusqlite::Error> {
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 - (max_age_days * 86400) as i64;
        
        if let Some(conn) = &self.conn {
            let affected = conn.execute(
                "DELETE FROM cache WHERE created_at < ?1",
                [cutoff.to_string()],
            )?;
            Ok(affected)
        } else {
            Ok(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cache_put_get() {
        let cache = AiCache::new(100);
        
        cache.put("test prompt", "test response");
        let result = cache.get("test prompt");
        
        assert_eq!(result, Some("test response".to_string()));
    }
    
    #[test]
    fn test_cache_miss() {
        let cache = AiCache::new(100);
        
        let result = cache.get("nonexistent prompt");
        assert_eq!(result, None);
    }
    
    #[test]
    fn test_cache_contains() {
        let cache = AiCache::new(100);
        
        cache.put("test prompt", "test response");
        assert!(cache.contains("test prompt"));
        assert!(!cache.contains("other prompt"));
    }
    
    #[test]
    fn test_cache_clear() {
        let cache = AiCache::new(100);
        
        cache.put("prompt 1", "response 1");
        cache.put("prompt 2", "response 2");
        
        cache.clear();
        
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }
    
    #[test]
    fn test_cache_stats() {
        let cache = AiCache::new(100);
        
        cache.put("test", "response");
        cache.get("test");
        cache.get("test");
        cache.get("nonexistent");
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.saves, 1);
    }
    
    #[test]
    fn test_cache_hit_rate() {
        let cache = AiCache::new(100);
        
        cache.put("test", "response");
        cache.get("test"); // hit
        cache.get("test"); // hit
        cache.get("other"); // miss
        
        let hit_rate = cache.hit_rate();
        assert!((hit_rate - 66.67).abs() < 0.1);
    }
    
    #[test]
    fn test_cache_lru_eviction() {
        let cache = AiCache::new(3);
        
        cache.put("prompt 1", "response 1");
        cache.put("prompt 2", "response 2");
        cache.put("prompt 3", "response 3");
        cache.put("prompt 4", "response 4"); // Должен вытеснить prompt 1
        
        assert!(!cache.contains("prompt 1"));
        assert!(cache.contains("prompt 2"));
        assert!(cache.contains("prompt 3"));
        assert!(cache.contains("prompt 4"));
    }
}

