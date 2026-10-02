//! Pepakura Platform - платформонезависимые абстракции
//!
//! Этот крейт предоставляет traits для абстракции платформонезависимых операций:
//! - FileSystem - работа с файловой системой
//! - Network - сетевые операции
//! - Storage - хранение данных
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_platform::fs::{FileSystem, FileError};
//!
//! pub fn process_file<F: FileSystem>(fs: &F, path: &str) -> Result<Vec<u8>, FileError> {
//!     fs.read_file(path)
//! }
//! ```

pub mod fs;
pub mod storage;

// Ре-экспорт основных типов
pub use fs::{FileSystem, FileError, FileData};
pub use storage::{Storage, StorageError};
