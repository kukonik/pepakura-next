//! Пример Rust аддона для Pepakura Next
//!
//! Этот аддон демонстрирует, как создать простое расширение
//! для Pepakura Next используя фреймворк pepakura_addons.

use pepakura_addons::{
    Addon, AddonError, AddonManifest, AddonRegistry,
    AddonType, AddonCapabilities,
};
use serde::{Deserialize, Serialize};

/// Пример аддона
pub struct ExampleAddon {
    manifest: AddonManifest,
}

impl ExampleAddon {
    pub fn new() -> Self {
        Self {
            manifest: AddonManifest::new(
                "example-rust-addon",
                "0.1.0",
                "Пример Rust аддона для Pepakura Next",
            )
            .with_type(AddonType::Utility)
            .with_author("Pepakura Next Team")
            .with_license("MIT"),
        }
    }
}

impl Addon for ExampleAddon {
    fn manifest(&self) -> AddonManifest {
        self.manifest.clone()
    }

    fn initialize(&self) -> Result<(), AddonError> {
        log::info!("ExampleAddon initialized!");
        Ok(())
    }

    fn shutdown(&self) -> Result<(), AddonError> {
        log::info!("ExampleAddon shutting down...");
        Ok(())
    }
}

/// Пример данных для операции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleOperationInput {
    pub message: String,
}

/// Пример результата операции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleOperationOutput {
    pub result: String,
    pub processed: bool,
}

/// Пример функции экспорта для использования в ядре
pub fn example_operation(input: &ExampleOperationInput) -> Result<ExampleOperationOutput, AddonError> {
    Ok(ExampleOperationOutput {
        result: format!("Processed: {}", input.message),
        processed: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_operation() {
        let input = ExampleOperationInput {
            message: "Hello, Pepakura!".to_string(),
        };

        let result = example_operation(&input).unwrap();

        assert!(result.processed);
        assert!(result.result.contains("Hello, Pepakura!"));
    }

    #[test]
    fn test_addon_creation() {
        let addon = ExampleAddon::new();
        assert_eq!(addon.name(), "example-rust-addon");
        assert_eq!(addon.version(), "0.1.0");
    }
}
