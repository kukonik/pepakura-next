//! AI-модуль для интеграции с языковыми моделями.
//!
//! Поддерживаемые провайдеры:
//! - Ollama (локальная LLM)
//! - OpenAI (облачная LLM)
//!
//! ## Пример использования
//!
//! ```rust
//! use pepakura_core::ai::{OllamaClient, AiConfig};
//! use pepakura_core::ai::cache::AiCache;
//! use pepakura_core::ai::streaming::chat_stream;
//!
//! let config = AiConfig::default();
//! let client = OllamaClient::new(&config);
//!
//! // Кэширование
//! let cache = AiCache::default();
//!
//! // Стриминг
//! // let stream = chat_stream(&client, "Привет!").await.unwrap();
//! ```

mod config;
mod client;
mod assistant;
pub mod cache;
pub mod streaming;
#[cfg(feature = "llm")]
pub mod local_llm;
#[cfg(feature = "llm")]
pub mod prompts;

pub use config::*;
pub use client::*;
pub use assistant::*;
pub use streaming::{chat_stream, messages_stream, AiStream, collect_stream, with_progress, ProgressStream};
#[cfg(feature = "llm")]
pub use local_llm::*;
#[cfg(feature = "llm")]
pub use prompts::*;
