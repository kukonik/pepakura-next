//! Реестр аддонов - управление загруженными плагинами

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::error::AddonError;
use crate::traits::Addon;

/// Реестр аддонов
///
/// Хранит и управляет загруженными аддонами
pub struct AddonRegistry {
    /// Загруженные аддоны (имя -> аддон)
    addons: RwLock<HashMap<String, Arc<dyn Addon>>>,
}

impl AddonRegistry {
    /// Создать новый пустой реестр
    pub fn new() -> Self {
        Self {
            addons: RwLock::new(HashMap::new()),
        }
    }

    /// Зарегистрировать аддон
    ///
    /// # Аргументы
    /// * `addon` - Умный указатель на аддон
    ///
    /// # Возвращает
    /// * `Ok(())` - если регистрация успешна
    /// * `Err(AddonError)` - если аддон с таким именем уже существует
    pub fn register(&self, addon: Arc<dyn Addon>) -> Result<(), AddonError> {
        let name = addon.name();

        let mut addons = self.addons
            .write()
            .map_err(|_| AddonError::Internal("Lock poisoned".to_string()))?;

        if addons.contains_key(&name) {
            return Err(AddonError::Conflict(format!(
                "Addon '{}' already registered",
                name
            )));
        }

        // Инициализация аддона
        addon.initialize().map_err(|e| {
            AddonError::InitializationError(name.clone(), e.to_string())
        })?;

        addons.insert(name, addon);
        Ok(())
    }

    /// Разрегистрировать аддон по имени
    pub fn unregister(&self, name: &str) -> Result<(), AddonError> {
        let mut addons = self.addons
            .write()
            .map_err(|_| AddonError::Internal("Lock poisoned".to_string()))?;

        let addon = addons
            .remove(name)
            .ok_or_else(|| AddonError::NotFound(name.to_string()))?;

        // Деинициализация аддона
        addon.shutdown().map_err(|e| {
            AddonError::DeinitializationError(name.to_string(), e.to_string())
        })?;

        Ok(())
    }

    /// Получить аддон по имени
    pub fn get(&self, name: &str) -> Option<Arc<dyn Addon>> {
        let addons = self.addons.read().ok()?;
        addons.get(name).cloned()
    }

    /// Получить все зарегистрированные аддоны
    pub fn list(&self) -> Vec<String> {
        match self.addons.read() {
            Ok(addons) => addons.keys().cloned().collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Проверить наличие аддона
    pub fn contains(&self, name: &str) -> bool {
        match self.addons.read() {
            Ok(addons) => addons.contains_key(name),
            Err(_) => false,
        }
    }

    /// Получить количество зарегистрированных аддонов
    pub fn len(&self) -> usize {
        match self.addons.read() {
            Ok(addons) => addons.len(),
            Err(_) => 0,
        }
    }

    /// Проверить, пуст ли реестр
    pub fn is_empty(&self) -> bool {
        match self.addons.read() {
            Ok(addons) => addons.is_empty(),
            Err(_) => true,
        }
    }

    /// Загрузить аддоны из директории
    ///
    /// # Аргументы
    /// * `addons_dir` - Путь к директории с аддонами
    ///
    /// # Возвращает
    /// * `Ok(Vec<String>)` - список загруженных аддонов
    /// * `Err(AddonError)` - ошибка загрузки
    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_directory(&self, addons_dir: &str) -> Result<Vec<String>, AddonError> {
        use std::fs;
        use std::path::Path;

        let path = Path::new(addons_dir);

        if !path.exists() {
            return Err(AddonError::LoadError(format!(
                "Addons directory '{}' does not exist",
                addons_dir
            )));
        }

        let mut loaded = Vec::new();

        // Ищем .dll, .so, .dylib файлы
        let entries = fs::read_dir(path)
            .map_err(|e| AddonError::LoadError(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());

            let is_library = matches!(extension, Some("dll" | "so" | "dylib"));

            if is_library {
                match self.load_library_addon(&path) {
                    Ok(name) => {
                        loaded.push(name);
                    }
                    Err(e) => {
                        log::warn!("Failed to load addon {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(loaded)
    }

    /// Загрузить аддон из динамической библиотеки
    #[cfg(not(target_arch = "wasm32"))]
    fn load_library_addon(&self, path: &std::path::Path) -> Result<String, AddonError> {
        use libloading::Library;

        unsafe {
            let lib = Library::new(path)
                .map_err(|e| AddonError::LoadError(format!("Failed to load library: {}", e)))?;

            // Ищем функцию создания аддона
            // Ожидаем функцию: pub extern "C" fn create_addon() -> *mut dyn Addon
            let create_fn: libloading::Symbol<unsafe extern "C" fn() -> *mut ()> = lib
                .get(b"create_addon")
                .map_err(|e| AddonError::MissingInterface(
                    path.display().to_string(),
                    format!("create_addon function not found: {}", e),
                ))?;

            // Создаём аддон
            let addon_ptr = create_fn();

            // В реальном коде нужно использовать более безопасный подход.
            // Сейчас просто пропускаем загрузку библиотечных аддонов
            // TODO: реализовать безопасную загрузку динамических библиотек
            log::warn!("Dynamic library addon loading is not safely implemented yet");
            return Err(AddonError::LoadError(
                "Dynamic library addon loading is not safely implemented".to_string()
            ));
        }
    }

    /// Очистить все аддоны
    pub fn clear(&self) -> Result<(), AddonError> {
        let mut addons = self.addons
            .write()
            .map_err(|_| AddonError::Internal("Lock poisoned".to_string()))?;

        // Деинициализация всех аддонов
        for (_, addon) in addons.drain() {
            if let Err(e) = addon.shutdown() {
                log::warn!("Error shutting down addon: {}", e);
            }
        }

        Ok(())
    }
}

impl Default for AddonRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::AddonManifest;

    struct TestAddon {
        name: String,
        version: String,
    }

    impl Addon for TestAddon {
        fn manifest(&self) -> AddonManifest {
            AddonManifest::new(&self.name, &self.version, "Test addon")
        }
    }

    #[test]
    fn test_register_addon() {
        let registry = AddonRegistry::new();
        let addon = Arc::new(TestAddon {
            name: "test-addon".to_string(),
            version: "1.0.0".to_string(),
        });

        assert!(registry.register(addon).is_ok());
        assert!(registry.contains("test-addon"));
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_unregister_addon() {
        let registry = AddonRegistry::new();
        let addon = Arc::new(TestAddon {
            name: "test-addon".to_string(),
            version: "1.0.0".to_string(),
        });

        registry.register(addon).unwrap();
        assert!(registry.unregister("test-addon").is_ok());
        assert!(!registry.contains("test-addon"));
    }

    #[test]
    fn test_duplicate_register() {
        let registry = AddonRegistry::new();
        let addon = Arc::new(TestAddon {
            name: "test-addon".to_string(),
            version: "1.0.0".to_string(),
        });

        registry.register(addon.clone()).unwrap();

        let addon2 = Arc::new(TestAddon {
            name: "test-addon".to_string(),
            version: "1.0.0".to_string(),
        });

        assert!(matches!(
            registry.register(addon2),
            Err(AddonError::Conflict(_))
        ));
    }
}
