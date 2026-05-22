//! Персистентное хранилище состояния.
//! 
//! Сохраняет и загружает состояние приложения из SQLite базы.

use rusqlite::{Connection, Result as SqliteResult, types::Type};
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

/// Персистентное хранилище.
pub struct Persistence {
    conn: Connection,
}

impl Persistence {
    /// Открывает или создаёт базу данных.
    pub fn open(path: &Path) -> SqliteResult<Self> {
        let conn = Connection::open(path)?;
        
        // Создаём таблицы
        conn.execute(
            "CREATE TABLE IF NOT EXISTS state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    /// Открывает базу в памяти (для тестов).
    pub fn open_in_memory() -> SqliteResult<Self> {
        let conn = Connection::open_in_memory()?;
        
        conn.execute(
            "CREATE TABLE state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    /// Сохраняет значение по ключу.
    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> SqliteResult<()> {
        let value_json = serde_json::to_string(value)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
        
        let updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        self.conn.execute(
            "INSERT OR REPLACE INTO state (key, value, updated_at) VALUES (?1, ?2, ?3)",
            [key, &value_json, &updated_at.to_string()],
        )?;
        
        Ok(())
    }
    
    /// Загружает значение по ключу.
    pub fn load<T: DeserializeOwned>(&self, key: &str) -> SqliteResult<Option<T>> {
        let mut stmt = self.conn.prepare(
            "SELECT value FROM state WHERE key = ?1"
        )?;
        
        let value: String = stmt.query_row([key], |row| row.get(0))?;
        
        let parsed: T = serde_json::from_str(&value)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?;
        
        Ok(Some(parsed))
    }
    
    /// Удаляет значение по ключу.
    pub fn remove(&self, key: &str) -> SqliteResult<usize> {
        self.conn.execute(
            "DELETE FROM state WHERE key = ?1",
            [key],
        )
    }
    
    /// Очищает всё хранилище.
    pub fn clear(&self) -> SqliteResult<()> {
        self.conn.execute("DELETE FROM state", [])?;
        Ok(())
    }
    
    /// Сохраняет проект.
    pub fn save_project<T: Serialize>(&self, name: &str, data: &T) -> SqliteResult<i64> {
        let data_json = serde_json::to_string(data)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(e.into()))?;
        
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        self.conn.execute(
            "INSERT INTO projects (name, data, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
            [name, &data_json, &now.to_string(), &now.to_string()],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }
    
    /// Загружает проект по ID.
    pub fn load_project<T: DeserializeOwned>(&self, id: i64) -> SqliteResult<Option<T>> {
        let mut stmt = self.conn.prepare(
            "SELECT data FROM projects WHERE id = ?1"
        )?;
        
        let data: String = stmt.query_row([id.to_string()], |row| row.get(0))?;
        
        let parsed: T = serde_json::from_str(&data)
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, Type::Text, Box::new(e)))?;
        
        Ok(Some(parsed))
    }
    
    /// Список всех проектов.
    pub fn list_projects(&self) -> SqliteResult<Vec<(i64, String, i64)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, updated_at FROM projects ORDER BY updated_at DESC"
        )?;
        
        let projects = stmt.query_map([], |row| {
            let id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let updated_at: i64 = row.get(2)?;
            Ok((id, name, updated_at))
        })?;
        
        let mut result = Vec::new();
        for project in projects {
            result.push(project?);
        }
        
        Ok(result)
    }
    
    /// Удаляет проект по ID.
    pub fn delete_project(&self, id: i64) -> SqliteResult<usize> {
        self.conn.execute(
            "DELETE FROM projects WHERE id = ?1",
            [id.to_string()],
        )
    }
    
    /// Сохраняет настройки.
    pub fn save_setting(&self, key: &str, value: &str) -> SqliteResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
        Ok(())
    }
    
    /// Загружает настройку.
    pub fn load_setting(&self, key: &str) -> SqliteResult<Option<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT value FROM settings WHERE key = ?1"
        )?;
        
        let value: String = stmt.query_row([key], |row| row.get(0))?;
        Ok(Some(value))
    }
    
    /// Очищает старые записи (старше указанного времени в секундах).
    pub fn cleanup_old_entries(&self, max_age_secs: i64) -> SqliteResult<usize> {
        let cutoff = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64 - max_age_secs;
        
        self.conn.execute(
            "DELETE FROM state WHERE updated_at < ?1",
            [cutoff.to_string()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    #[test]
    fn test_persistence_save_load() {
        let persistence = Persistence::open_in_memory().unwrap();
        
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        
        persistence.save("test_key", &data).unwrap();
        
        let loaded: TestData = persistence.load("test_key").unwrap().unwrap();
        assert_eq!(data, loaded);
    }
    
    #[test]
    fn test_persistence_remove() {
        let persistence = Persistence::open_in_memory().unwrap();
        
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        
        persistence.save("test_key", &data).unwrap();
        persistence.remove("test_key").unwrap();
        
        let loaded: Option<TestData> = persistence.load("test_key").unwrap();
        assert!(loaded.is_none());
    }
    
    #[test]
    fn test_persistence_clear() {
        let persistence = Persistence::open_in_memory().unwrap();
        
        persistence.save("key1", &TestData { name: "test1".to_string(), value: 1 }).unwrap();
        persistence.save("key2", &TestData { name: "test2".to_string(), value: 2 }).unwrap();
        
        persistence.clear().unwrap();
        
        let loaded1: Option<TestData> = persistence.load("key1").unwrap();
        let loaded2: Option<TestData> = persistence.load("key2").unwrap();
        assert!(loaded1.is_none());
        assert!(loaded2.is_none());
    }
    
    #[test]
    fn test_save_project() {
        let persistence = Persistence::open_in_memory().unwrap();
        
        let data = TestData {
            name: "project1".to_string(),
            value: 100,
        };
        
        let id = persistence.save_project("Test Project", &data).unwrap();
        assert!(id > 0);
        
        let loaded: TestData = persistence.load_project(id).unwrap().unwrap();
        assert_eq!(data, loaded);
    }
    
    #[test]
    fn test_list_projects() {
        let persistence = Persistence::open_in_memory().unwrap();
        
        persistence.save_project("Project 1", &TestData { name: "p1".to_string(), value: 1 }).unwrap();
        persistence.save_project("Project 2", &TestData { name: "p2".to_string(), value: 2 }).unwrap();
        
        let projects = persistence.list_projects().unwrap();
        assert_eq!(projects.len(), 2);
    }
    
    #[test]
    fn test_delete_project() {
        let persistence = Persistence::open_in_memory().unwrap();
        
        let id = persistence.save_project("Test", &TestData { name: "test".to_string(), value: 1 }).unwrap();
        
        let deleted = persistence.delete_project(id).unwrap();
        assert_eq!(deleted, 1);
        
        let loaded: Option<TestData> = persistence.load_project(id).unwrap();
        assert!(loaded.is_none());
    }
    
    #[test]
    fn test_settings() {
        let persistence = Persistence::open_in_memory().unwrap();
        
        persistence.save_setting("theme", "dark").unwrap();
        
        let theme = persistence.load_setting("theme").unwrap().unwrap();
        assert_eq!(theme, "dark");
    }
}
