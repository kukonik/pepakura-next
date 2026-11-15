//! Модуль персистентности для сохранения состояния приложения.
//!
//! Использует SQLite для хранения:
//! - Состояния проекта
//! - Истории undo/redo
//! - Настроек пользователя
//! - Последних открытых файлов

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result, OpenFlags};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Менеджер персистентности состояния.
pub struct StatePersistence {
    conn: Connection,
}

/// Запись состояния.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateEntry {
    pub key: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
}

/// Запись истории действий.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: i64,
    pub project_id: String,
    pub action: String,
    pub state_before: String,
    pub state_after: String,
    pub timestamp: DateTime<Utc>,
}

/// Настройки приложения.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub last_project_path: Option<String>,
    pub auto_save_interval: u64,
    pub auto_save_enabled: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            language: "ru".to_string(),
            last_project_path: None,
            auto_save_interval: 30,
            auto_save_enabled: true,
        }
    }
}

impl StatePersistence {
    /// Создаёт новый менеджер персистентности.
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open_with_flags(
            db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )?;

        // Создаём таблицы
        conn.execute(
            "CREATE TABLE IF NOT EXISTS state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id TEXT NOT NULL,
                action TEXT NOT NULL,
                state_before TEXT NOT NULL,
                state_after TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS recent_projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                name TEXT NOT NULL,
                last_opened TEXT NOT NULL
            )",
            [],
        )?;

        // Индексы для ускорения
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_project ON history(project_id)",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_history_timestamp ON history(timestamp)",
            [],
        )?;

        Ok(Self { conn })
    }

    /// Создаёт менеджер персистентности в памяти (для тестов).
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory_with_flags(
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_FULL_MUTEX,
        )?;
        Ok(Self { conn })
    }

    /// Сохраняет состояние по ключу.
    pub fn save_state<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let value_json = serde_json::to_string(value)
            .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT OR REPLACE INTO state (key, value, updated_at) VALUES (?1, ?2, ?3)",
            [key, &value_json, &now],
        )?;

        Ok(())
    }

    /// Загружает состояние по ключу.
    pub fn load_state<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>> {
        let result: Result<String, _> = self.conn.query_row(
            "SELECT value FROM state WHERE key = ?1",
            [key],
            |row| row.get(0),
        );

        match result {
            Ok(value_json) => {
                let value: T = serde_json::from_str(&value_json)
                    .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;
                Ok(Some(value))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Удаляет состояние по ключу.
    pub fn delete_state(&self, key: &str) -> Result<()> {
        self.conn.execute("DELETE FROM state WHERE key = ?1", [key])?;
        Ok(())
    }

    /// Добавляет запись в историю действий.
    pub fn push_history(
        &self,
        project_id: &str,
        action: &str,
        state_before: &str,
        state_after: &str,
    ) -> Result<i64> {
        let now = Utc::now().to_rfc3339();

        self.conn.execute(
            "INSERT INTO history (project_id, action, state_before, state_after, timestamp) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            [project_id, action, state_before, state_after, &now],
        )?;

        // Возвращаем ID последней вставки
        Ok(self.conn.last_insert_rowid())
    }

    /// Получает историю действий для проекта.
    pub fn get_history(&self, project_id: &str, limit: usize) -> Result<Vec<HistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, action, state_before, state_after, timestamp 
             FROM history 
             WHERE project_id = ?1 
             ORDER BY timestamp DESC 
             LIMIT ?2",
        )?;

        let entries = stmt
            .query_map(rusqlite::params![project_id, limit as i64], |row| {
                let timestamp_str: String = row.get(5)?;
                let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc);

                Ok(HistoryEntry {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    action: row.get(2)?,
                    state_before: row.get(3)?,
                    state_after: row.get(4)?,
                    timestamp,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(entries)
    }

    /// Получает последнее состояние для undo.
    pub fn get_last_undo(&self, project_id: &str) -> Result<Option<HistoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, project_id, action, state_before, state_after, timestamp 
             FROM history 
             WHERE project_id = ?1 
             ORDER BY timestamp DESC 
             LIMIT 1",
        )?;

        let result = stmt.query_row([project_id], |row| {
            let timestamp_str: String = row.get(5)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .unwrap_or_else(|_| Utc::now().into())
                .with_timezone(&Utc);

            Ok(HistoryEntry {
                id: row.get(0)?,
                project_id: row.get(1)?,
                action: row.get(2)?,
                state_before: row.get(3)?,
                state_after: row.get(4)?,
                timestamp,
            })
        });

        match result {
            Ok(entry) => Ok(Some(entry)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Очищает историю для проекта.
    pub fn clear_history(&self, project_id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM history WHERE project_id = ?1", [project_id])?;
        Ok(())
    }

    /// Сохраняет настройку.
    pub fn save_setting(&self, key: &str, value: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value, updated_at) VALUES (?1, ?2, ?3)",
            [key, value, &now],
        )?;
        Ok(())
    }

    /// Загружает настройку.
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let result: Result<String, _> = self.conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [key],
            |row| row.get(0),
        );

        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// Загружает все настройки.
    pub fn get_all_settings(&self) -> Result<AppSettings> {
        let mut settings = AppSettings::default();

        let mut stmt = self.conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;

        for row in rows {
            if let Ok((key, value)) = row {
                match key.as_str() {
                    "theme" => settings.theme = value,
                    "language" => settings.language = value,
                    "last_project_path" => settings.last_project_path = Some(value),
                    "auto_save_interval" => {
                        settings.auto_save_interval = value.parse().unwrap_or(30)
                    }
                    "auto_save_enabled" => {
                        settings.auto_save_enabled = value.parse().unwrap_or(true)
                    }
                    _ => {}
                }
            }
        }

        Ok(settings)
    }

    /// Добавляет проект в список последних.
    pub fn add_recent_project(&self, path: &str, name: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();

        // Сначала удалим старую запись если есть
        self.conn.execute(
            "DELETE FROM recent_projects WHERE path = ?1",
            [path],
        )?;

        // Добавим новую
        self.conn.execute(
            "INSERT INTO recent_projects (path, name, last_opened) VALUES (?1, ?2, ?3)",
            [path, name, &now],
        )?;

        // Оставим только последние 10
        self.conn.execute(
            "DELETE FROM recent_projects WHERE id NOT IN (
                SELECT id FROM recent_projects ORDER BY last_opened DESC LIMIT 10
            )",
            [],
        )?;

        Ok(())
    }

    /// Получает список последних проектов.
    pub fn get_recent_projects(&self) -> Result<Vec<(String, String, DateTime<Utc>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT path, name, last_opened FROM recent_projects ORDER BY last_opened DESC",
        )?;

        let projects = stmt
            .query_map([], |row| {
                let last_opened_str: String = row.get(2)?;
                let last_opened = DateTime::parse_from_rfc3339(&last_opened_str)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc);

                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    last_opened,
                ))
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(projects)
    }

    /// Очищает список последних проектов.
    pub fn clear_recent_projects(&self) -> Result<()> {
        self.conn.execute("DELETE FROM recent_projects", [])?;
        Ok(())
    }

    /// Восстанавливает данные после краша.
    pub fn recover_from_crash(&self) -> Result<Vec<StateEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT key, value, updated_at FROM state ORDER BY updated_at DESC",
        )?;

        let entries = stmt
            .query_map([], |row| {
                let updated_at_str: String = row.get(2)?;
                let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
                    .unwrap_or_else(|_| Utc::now().into())
                    .with_timezone(&Utc);

                Ok(StateEntry {
                    key: row.get(0)?,
                    value: row.get(1)?,
                    updated_at,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_state() {
        let persistence = StatePersistence::in_memory().unwrap();

        let test_data = serde_json::json!({
            "name": "Test Project",
            "value": 42
        });

        persistence.save_state("test_key", &test_data).unwrap();

        let loaded: Option<serde_json::Value> = persistence.load_state("test_key").unwrap();
        assert!(loaded.is_some());
        assert_eq!(loaded.unwrap()["name"], "Test Project");
        assert_eq!(loaded.unwrap()["value"], 42);
    }

    #[test]
    fn test_load_nonexistent_state() {
        let persistence = StatePersistence::in_memory().unwrap();
        let loaded: Option<serde_json::Value> = persistence.load_state("nonexistent").unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_delete_state() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence.save_state("test_key", &"test_value").unwrap();
        persistence.delete_state("test_key").unwrap();

        let loaded: Option<String> = persistence.load_state("test_key").unwrap();
        assert!(loaded.is_none());
    }

    #[test]
    fn test_push_and_get_history() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence
            .push_history("project1", "edit", "{}", "{\"modified\": true}")
            .unwrap();
        persistence
            .push_history("project1", "save", "{\"modified\": true}", "{}")
            .unwrap();

        let history = persistence.get_history("project1", 10).unwrap();
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].action, "save");
        assert_eq!(history[1].action, "edit");
    }

    #[test]
    fn test_get_last_undo() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence
            .push_history("project1", "edit", "before", "after")
            .unwrap();

        let last = persistence.get_last_undo("project1").unwrap();
        assert!(last.is_some());
        assert_eq!(last.unwrap().action, "edit");
    }

    #[test]
    fn test_save_and_get_setting() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence.save_setting("theme", "dark").unwrap();

        let value = persistence.get_setting("theme").unwrap();
        assert_eq!(value, Some("dark".to_string()));
    }

    #[test]
    fn test_get_all_settings() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence.save_setting("theme", "dark").unwrap();
        persistence.save_setting("language", "en").unwrap();
        persistence.save_setting("auto_save_interval", "60").unwrap();

        let settings = persistence.get_all_settings().unwrap();
        assert_eq!(settings.theme, "dark");
        assert_eq!(settings.language, "en");
        assert_eq!(settings.auto_save_interval, 60);
    }

    #[test]
    fn test_add_and_get_recent_projects() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence.add_recent_project("/path/to/project1.pepa", "Project 1").unwrap();
        persistence.add_recent_project("/path/to/project2.pepa", "Project 2").unwrap();

        let recent = persistence.get_recent_projects().unwrap();
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].1, "Project 2"); // Последний открытый первым
    }

    #[test]
    fn test_clear_history() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence
            .push_history("project1", "edit", "{}", "{}")
            .unwrap();
        persistence
            .push_history("project1", "save", "{}", "{}")
            .unwrap();

        persistence.clear_history("project1").unwrap();

        let history = persistence.get_history("project1", 10).unwrap();
        assert!(history.is_empty());
    }

    #[test]
    fn test_recover_from_crash() {
        let persistence = StatePersistence::in_memory().unwrap();

        persistence.save_state("state1", &"value1").unwrap();
        persistence.save_state("state2", &"value2").unwrap();

        let recovered = persistence.recover_from_crash().unwrap();
        assert_eq!(recovered.len(), 2);
    }
}
