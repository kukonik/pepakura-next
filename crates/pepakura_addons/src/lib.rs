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
//! ```rust,no_run
//! use std::sync::Arc;
//! use pepakura_addons::{Addon, AddonError, AddonManifest, AddonRegistry};
//!
//! // Создание аддона
//! pub struct MyAddon;
//!
//! impl Addon for MyAddon {
//!     fn manifest(&self) -> AddonManifest {
//!         // Заполните манифест вашего аддона; полный набор полей
//!         // см. в документации pepakura_addons::manifest::AddonManifest.
//!         todo!("построить AddonManifest")
//!     }
//!
//!     fn initialize(&self) -> Result<(), AddonError> {
//!         println!("MyAddon initialized!");
//!         Ok(())
//!     }
//! }
//!
//! // Регистрация аддона: register принимает Arc<dyn Addon>
//! let registry = AddonRegistry::new();
//! registry.register(Arc::new(MyAddon)).unwrap();
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
