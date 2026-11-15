//! Pepakura Addons - система модульных расширений
//!
//! Этот крейт предоставляет фреймворк для создания и управления плагинами/расширениями
//! для Pepakura Next. Аддоны могут добавлять:
//! - Новые форматы импорта/экспорта
//! - Алгоритмы развёртки
//! - Инструменты оптимизации
//! - Интеграции с внешними сервисами
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_addons::{Addon, AddonManifest, AddonRegistry};
//!
//! // Создание аддона
//! pub struct MyAddon;
//!
//! impl Addon for MyAddon {
//!     fn manifest(&self) -> AddonManifest {
//!         AddonManifest {
//!             name: "my-addon".to_string(),
//!             version: "1.0.0".to_string(),
//!             description: "My custom addon".to_string(),
//!         }
//!     }
//!
//!     fn initialize(&self) -> Result<(), AddonError> {
//!         println!("MyAddon initialized!");
//!         Ok(())
//!     }
//! }
//!
//! // Регистрация аддона
//! let mut registry = AddonRegistry::new();
//! registry.register(Box::new(MyAddon)).unwrap();
//! ```

pub mod error;
pub mod manifest;
pub mod registry;
pub mod traits;

// Ре-экспорт основных типов
pub use error::AddonError;
pub use manifest::{AddonManifest, AddonType, AddonCapabilities};
pub use registry::AddonRegistry;
pub use traits::Addon;

/// Результат выполнения операции аддона
pub type AddonResult<T> = Result<T, AddonError>;
