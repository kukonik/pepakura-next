//! Абстракция хранилища данных
//!
//! Этот модуль предоставляет trait Storage для абстракции операций хранения данных,
//! что позволяет использовать ядро Pepakura как с SQLite (native), так и с IndexedDB (web).

use thiserror::Error;
use serde;
use serde_json;

/// Ошибки хранилища
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Ключ не найден: {0}")]
    NotFound(String),

    #[error("Ошибка чтения: {0}")]
    ReadError(String),

    #[error("Ошибка записи: {0}")]
    WriteError(String),

    #[error("Ошибка сериализации: {0}")]
    SerializationError(String),

    #[error("Ошибка десериализации: {0}")]
    DeserializationError(String),

    #[error("Неподдерживаемая операция в web-среде: {0}")]
    WebUnsupported(String),
}

/// Trait для абстракции операций хранения
///
/// Этот trait позволяет использовать единую API для хранения данных
/// как в native среде (SQLite через rusqlite), так и в web (IndexedDB)
pub trait Storage: Send + Sync {
    /// Получить значение по ключу
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError>;

    /// Получить значение по ключу с десериализацией
    fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>, StorageError> {
        match self.get(key)? {
            Some(data) => {
                let json = String::from_utf8(data)
                    .map_err(|e| StorageError::ReadError(format!("Invalid UTF-8: {}", e)))?;
                serde_json::from_str(&json)
                    .map_err(|e| StorageError::DeserializationError(e.to_string()))
                    .map(Some)
            }
            None => Ok(None),
        }
    }

    /// Записать значение по ключу
    fn set(&mut self, key: &str, value: &[u8]) -> Result<(), StorageError>;

    /// Записать значение с сериализацией в JSON
    fn set_json<T: serde::Serialize>(&mut self, key: &str, value: &T) -> Result<(), StorageError> {
        let json = serde_json::to_string(value)
            .map_err(|e| StorageError::SerializationError(e.to_string()))?;
        self.set(key, json.as_bytes())
    }

    /// Удалить значение по ключу
    fn delete(&mut self, key: &str) -> Result<(), StorageError>;

    /// Проверить существование ключа
    fn contains(&self, key: &str) -> Result<bool, StorageError>;

    /// Получить все ключи
    fn keys(&self) -> Result<Vec<String>, StorageError>;

    /// Очистить всё хранилище
    fn clear(&mut self) -> Result<(), StorageError>;
}

/// Native реализация Storage (in-memory для тестов)
#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use std::sync::{Arc, RwLock};
    use std::collections::HashMap;

    /// In-memory хранилище для native среды
    ///
    /// В production следует использовать SQLite реализацию
    #[derive(Debug, Clone, Default)]
    pub struct MemoryStorage {
        data: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    }

    impl MemoryStorage {
        pub fn new() -> Self {
            Self {
                data: Arc::new(RwLock::new(HashMap::new())),
            }
        }
    }

    impl Storage for MemoryStorage {
        fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
            let data = self.data
                .read()
                .map_err(|e| StorageError::ReadError(format!("Lock poisoned: {}", e)))?;
            Ok(data.get(key).cloned())
        }

        fn set(&mut self, key: &str, value: &[u8]) -> Result<(), StorageError> {
            let mut data = self.data
                .write()
                .map_err(|e| StorageError::WriteError(format!("Lock poisoned: {}", e)))?;
            data.insert(key.to_string(), value.to_vec());
            Ok(())
        }

        fn delete(&mut self, key: &str) -> Result<(), StorageError> {
            let mut data = self.data
                .write()
                .map_err(|e| StorageError::WriteError(format!("Lock poisoned: {}", e)))?;
            data.remove(key);
            Ok(())
        }

        fn contains(&self, key: &str) -> Result<bool, StorageError> {
            let data = self.data
                .read()
                .map_err(|e| StorageError::ReadError(format!("Lock poisoned: {}", e)))?;
            Ok(data.contains_key(key))
        }

        fn keys(&self) -> Result<Vec<String>, StorageError> {
            let data = self.data
                .read()
                .map_err(|e| StorageError::ReadError(format!("Lock poisoned: {}", e)))?;
            Ok(data.keys().cloned().collect())
        }

        fn clear(&mut self) -> Result<(), StorageError> {
            let mut data = self.data
                .write()
                .map_err(|e| StorageError::WriteError(format!("Lock poisoned: {}", e)))?;
            data.clear();
            Ok(())
        }
    }

    /// SQLite реализация хранилища (заглушка)
    pub struct SqliteStorage {
        // В реальной реализации здесь будет подключение к SQLite
        _connection: String, // placeholder
    }

    impl SqliteStorage {
        pub fn new(_path: &str) -> Result<Self, StorageError> {
            // В реальной реализации: rusqlite::Connection::open(path)
            Ok(Self {
                _connection: _path.to_string(),
            })
        }
    }

    impl Storage for SqliteStorage {
        fn get(&self, _key: &str) -> Result<Option<Vec<u8>>, StorageError> {
            // TODO: Реализовать через SQLite
            Err(StorageError::WebUnsupported("SQLite not yet implemented".to_string()))
        }

        fn set(&mut self, _key: &str, _value: &[u8]) -> Result<(), StorageError> {
            // TODO: Реализовать через SQLite
            Err(StorageError::WebUnsupported("SQLite not yet implemented".to_string()))
        }

        fn delete(&mut self, _key: &str) -> Result<(), StorageError> {
            // TODO: Реализовать через SQLite
            Err(StorageError::WebUnsupported("SQLite not yet implemented".to_string()))
        }

        fn contains(&self, _key: &str) -> Result<bool, StorageError> {
            Ok(false)
        }

        fn keys(&self) -> Result<Vec<String>, StorageError> {
            Ok(Vec::new())
        }

        fn clear(&mut self) -> Result<(), StorageError> {
            Ok(())
        }
    }
}

/// Web реализация Storage (IndexedDB заглушка)
#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::*;

    /// Web хранилище (IndexedDB заглушка)
    #[derive(Debug, Clone, Default)]
    pub struct WebStorage {
        data: std::collections::HashMap<String, Vec<u8>>,
    }

    impl WebStorage {
        pub fn new() -> Self {
            Self {
                data: std::collections::HashMap::new(),
            }
        }
    }

    impl Storage for WebStorage {
        fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
            Ok(self.data.get(key).cloned())
        }

        fn set(&mut self, key: &str, value: &[u8]) -> Result<(), StorageError> {
            self.data.insert(key.to_string(), value.to_vec());
            Ok(())
        }

        fn delete(&mut self, key: &str) -> Result<(), StorageError> {
            self.data.remove(key);
            Ok(())
        }

        fn contains(&self, key: &str) -> Result<bool, StorageError> {
            Ok(self.data.contains_key(key))
        }

        fn keys(&self) -> Result<Vec<String>, StorageError> {
            Ok(self.data.keys().cloned().collect())
        }

        fn clear(&mut self) -> Result<(), StorageError> {
            self.data.clear();
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_memory_storage() {
        use self::native::MemoryStorage;

        let storage = MemoryStorage::new();

        // Записать значение
        storage.set("key1", b"value1").unwrap();

        // Прочитать значение
        let value = storage.get("key1").unwrap();
        assert_eq!(value, Some(b"value1".to_vec()));

        // Проверить существование
        assert!(storage.contains("key1").unwrap());
        assert!(!storage.contains("key2").unwrap());

        // Получить все ключи
        let keys = storage.keys().unwrap();
        assert_eq!(keys, vec!["key1"]);

        // Удалить значение
        storage.delete("key1").unwrap();
        assert!(!storage.contains("key1").unwrap());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_storage_json() {
        use self::native::MemoryStorage;

        let storage = MemoryStorage::new();

        #[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
        struct TestData {
            name: String,
            value: i32,
        }

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Записать JSON
        storage.set_json("json_key", &data).unwrap();

        // Прочитать JSON
        let retrieved: TestData = storage.get_json("json_key").unwrap().unwrap();
        assert_eq!(retrieved, data);
    }
}
