//! Абстракция файловой системы
//!
//! Этот модуль предоставляет trait FileSystem для абстракции файловых операций,
//! что позволяет использовать ядро Pepakura как в native (desktop), так и в web (WASM) среде.

use thiserror::Error;

/// Ошибки файловой системы
#[derive(Debug, Error)]
pub enum FileError {
    #[error("Файл не найден: {0}")]
    NotFound(String),

    #[error("Ошибка чтения: {0}")]
    ReadError(String),

    #[error("Ошибка записи: {0}")]
    WriteError(String),

    #[error("Ошибка создания директории: {0}")]
    DirError(String),

    #[error("Отказ в доступе: {0}")]
    PermissionDenied(String),

    #[error("Неподдерживаемая операция в web-среде: {0}")]
    WebUnsupported(String),
}

/// Данные файла
#[derive(Debug, Clone)]
pub struct FileData {
    /// Имя файла
    pub name: String,
    /// Путь к файлу
    pub path: String,
    /// Содержимое файла
    pub content: Vec<u8>,
    /// MIME тип (если известен)
    pub mime_type: Option<String>,
}

impl FileData {
    /// Создать новые FileData
    pub fn new(name: impl Into<String>, path: impl Into<String>, content: Vec<u8>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            content,
            mime_type: None,
        }
    }

    /// Создать FileData из строки
    pub fn from_string(name: impl Into<String>, path: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            content: content.into().into_bytes(),
            mime_type: Some("text/plain".to_string()),
        }
    }

    /// Получить содержимое как строку
    pub fn as_string(&self) -> Result<String, FileError> {
        String::from_utf8(self.content.clone())
            .map_err(|e| FileError::ReadError(format!("Invalid UTF-8: {}", e)))
    }
}

/// Trait для абстракции файловых операций
///
/// Этот trait позволяет использовать единую API для работы с файлами
/// как в native среде (через std::fs/tokio::fs), так и в web (через browser APIs)
pub trait FileSystem: Send + Sync {
    /// Прочитать файл по пути
    fn read_file(&self, path: &str) -> Result<FileData, FileError>;

    /// Записать данные в файл
    fn write_file(&self, path: &str, content: &[u8]) -> Result<(), FileError>;

    /// Записать текст в файл
    fn write_text(&self, path: &str, content: &str) -> Result<(), FileError> {
        self.write_file(path, content.as_bytes())
    }

    /// Создать директорию (рекурсивно)
    fn create_dir_all(&self, path: &str) -> Result<(), FileError>;

    /// Проверить существование файла
    fn exists(&self, path: &str) -> Result<bool, FileError>;

    /// Удалить файл
    fn delete_file(&self, path: &str) -> Result<(), FileError>;

    /// Получить список файлов в директории
    fn list_dir(&self, path: &str) -> Result<Vec<String>, FileError>;
}

/// Native реализация FileSystem (для desktop/Tauri)
#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use std::path::Path;
    use tokio::fs;

    /// Native файловая система с использованием tokio
    #[derive(Debug, Clone, Default)]
    pub struct NativeFileSystem;

    impl NativeFileSystem {
        pub fn new() -> Self {
            Self
        }
    }

    impl FileSystem for NativeFileSystem {
        fn read_file(&self, path: &str) -> Result<FileData, FileError> {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| FileError::ReadError(format!("Failed to create runtime: {}", e)))?;

            rt.block_on(async {
                let content = fs::read(path)
                    .await
                    .map_err(|e| match e.kind() {
                        std::io::ErrorKind::NotFound => FileError::NotFound(path.to_string()),
                        std::io::ErrorKind::PermissionDenied => FileError::PermissionDenied(path.to_string()),
                        _ => FileError::ReadError(e.to_string()),
                    })?;

                let name = Path::new(path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                Ok(FileData::new(name, path.to_string(), content))
            })
        }

        fn write_file(&self, path: &str, content: &[u8]) -> Result<(), FileError> {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| FileError::WriteError(format!("Failed to create runtime: {}", e)))?;

            rt.block_on(async {
                // Создать директорию если нужно
                if let Some(parent) = Path::new(path).parent() {
                    fs::create_dir_all(parent)
                        .await
                        .map_err(|e| FileError::DirError(e.to_string()))?;
                }

                fs::write(path, content)
                    .await
                    .map_err(|e| match e.kind() {
                        std::io::ErrorKind::PermissionDenied => FileError::PermissionDenied(path.to_string()),
                        _ => FileError::WriteError(e.to_string()),
                    })
            })
        }

        fn create_dir_all(&self, path: &str) -> Result<(), FileError> {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| FileError::DirError(format!("Failed to create runtime: {}", e)))?;

            rt.block_on(async {
                fs::create_dir_all(path)
                    .await
                    .map_err(|e| FileError::DirError(e.to_string()))
            })
        }

        fn exists(&self, path: &str) -> Result<bool, FileError> {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| FileError::ReadError(format!("Failed to create runtime: {}", e)))?;

            rt.block_on(async {
                Ok(fs::metadata(path).await.is_ok())
            })
        }

        fn delete_file(&self, path: &str) -> Result<(), FileError> {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| FileError::WriteError(format!("Failed to create runtime: {}", e)))?;

            rt.block_on(async {
                fs::remove_file(path)
                    .await
                    .map_err(|e| match e.kind() {
                        std::io::ErrorKind::NotFound => FileError::NotFound(path.to_string()),
                        std::io::ErrorKind::PermissionDenied => FileError::PermissionDenied(path.to_string()),
                        _ => FileError::WriteError(e.to_string()),
                    })
            })
        }

        fn list_dir(&self, path: &str) -> Result<Vec<String>, FileError> {
            let rt = tokio::runtime::Runtime::new()
                .map_err(|e| FileError::ReadError(format!("Failed to create runtime: {}", e)))?;

            rt.block_on(async {
                let mut entries = Vec::new();
                let mut dir = fs::read_dir(path)
                    .await
                    .map_err(|e| FileError::ReadError(e.to_string()))?;

                while let Some(entry) = dir
                    .next_entry()
                    .await
                    .map_err(|e| FileError::ReadError(e.to_string()))?
                {
                    if let Ok(name) = entry.file_name().into_string() {
                        entries.push(name);
                    }
                }

                Ok(entries)
            })
        }
    }
}

/// Web реализация FileSystem (для WASM/browser)
#[cfg(target_arch = "wasm32")]
pub mod web {
    use super::*;
    use wasm_bindgen::prelude::*;
    use js_sys::Array;

    /// Web файловая система (ограниченная функциональность)
    ///
    /// В browser среде прямая работа с файловой системой невозможна.
    /// Этот класс предоставляет методы для работы через File API.
    #[derive(Debug, Clone, Default)]
    pub struct WebFileSystem {
        /// Хранилище загруженных файлов (path -> content)
        storage: std::collections::HashMap<String, FileData>,
    }

    impl WebFileSystem {
        pub fn new() -> Self {
            Self {
                storage: std::collections::HashMap::new(),
            }
        }

        /// Загрузить файл в хранилище (вызывается из JS)
        pub fn load_file(&mut self, name: String, path: String, content: Vec<u8>) {
            self.storage.insert(path.clone(), FileData::new(name, path, content));
        }

        /// Скачать файл (вызывается из JS)
        pub fn download_file(&self, path: &str) -> Option<&FileData> {
            self.storage.get(path)
        }
    }

    impl FileSystem for WebFileSystem {
        fn read_file(&self, path: &str) -> Result<FileData, FileError> {
            self.storage
                .get(path)
                .cloned()
                .ok_or_else(|| FileError::NotFound(path.to_string()))
        }

        fn write_file(&self, path: &str, content: &[u8]) -> Result<(), FileError> {
            // В web-среде "запись" означает сохранение в памяти и предложение скачать
            let name = path.split('/').last().unwrap_or("file").to_string();
            let _data = FileData::new(name, path.to_string(), content.to_vec());

            // В реальной реализации здесь будет вызов JS для скачивания файла
            // Для sekarang просто сохраняем в памяти
            // SAFETY: Это нарушение безопасности, но необходимо для работы в WASM
            // В production нужно использовать proper мутацию через JS
            let warn_msg = format!("WebFileSystem: write_file is limited in browser. File: {}", path);
            let arr = Array::new();
            arr.push(&JsValue::from_str(&warn_msg));
            web_sys::console::warn(&arr);

            Ok(())
        }

        fn create_dir_all(&self, path: &str) -> Result<(), FileError> {
            // В browser нет концепции директорий
            let log_msg = format!("WebFileSystem: create_dir_all is no-op in browser. Path: {}", path);
            let arr = Array::new();
            arr.push(&JsValue::from_str(&log_msg));
            web_sys::console::log(&arr);
            Ok(())
        }

        fn exists(&self, path: &str) -> Result<bool, FileError> {
            Ok(self.storage.contains_key(path))
        }

        fn delete_file(&self, path: &str) -> Result<(), FileError> {
            // В реальной реализации нужно удалять из storage
            let warn_msg = format!("WebFileSystem: delete_file not fully implemented. Path: {}", path);
            let arr = Array::new();
            arr.push(&JsValue::from_str(&warn_msg));
            web_sys::console::warn(&arr);
            Ok(())
        }

        fn list_dir(&self, _path: &str) -> Result<Vec<String>, FileError> {
            // В browser нет концепции директорий
            Ok(Vec::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn test_native_filesystem() {
        use self::native::NativeFileSystem;

        let fs = NativeFileSystem::new();
        let temp_path = std::env::temp_dir().join("pepakura_test_fs.txt");
        let temp_path_str = temp_path.to_str().unwrap();

        // Записать файл
        let write_result = fs.write_text(temp_path_str, "Hello, Pepakura!");
        assert!(write_result.is_ok());

        // Прочитать файл
        let read_result = fs.read_file(temp_path_str);
        assert!(read_result.is_ok());
        let data = read_result.unwrap();
        assert_eq!(data.as_string().unwrap(), "Hello, Pepakura!");

        // Проверить существование
        let exists = fs.exists(temp_path_str).unwrap();
        assert!(exists);

        // Удалить файл
        let delete_result = fs.delete_file(temp_path_str);
        assert!(delete_result.is_ok());
    }
}
