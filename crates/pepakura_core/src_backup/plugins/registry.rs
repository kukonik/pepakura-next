//! Реестр плагинов.

use super::traits::*;
use crate::geometry::Mesh;
use crate::unfold::{UnfoldedMesh, UnfoldConfig};
use crate::PepakuraError;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

/// Реестр плагинов.
/// 
/// Управляет зарегистрированными плагинами импорта, экспорта и развёртки.
/// 
/// ## Пример
/// 
/// ```rust
/// use pepakura_core::plugins::PluginRegistry;
/// 
/// let registry = PluginRegistry::new();
/// ```
pub struct PluginRegistry {
    importers: HashMap<String, Arc<dyn ImportPlugin>>,
    exporters: HashMap<String, Arc<dyn ExportPlugin>>,
    unfolders: HashMap<String, Arc<dyn UnfoldPlugin>>,
    metadata: HashMap<String, PluginMetadata>,
}

impl PluginRegistry {
    /// Создаёт новый пустой реестр.
    pub fn new() -> Self {
        Self {
            importers: HashMap::new(),
            exporters: HashMap::new(),
            unfolders: HashMap::new(),
            metadata: HashMap::new(),
        }
    }
    
    /// Регистрирует плагин импорта.
    /// 
    /// # Аргументы
    /// * `plugin` - плагин для регистрации
    /// 
    /// # Пример
    /// 
    /// ```rust
    /// registry.register_importer(Box::new(ObjImporter));
    /// ```
    pub fn register_importer(&mut self, plugin: Box<dyn ImportPlugin>) {
        let name = plugin.name().to_string();
        let extensions: Vec<String> = plugin
            .supported_extensions()
            .iter()
            .map(|&s| s.to_lowercase())
            .collect();
        
        let arc_plugin: Arc<dyn ImportPlugin> = Arc::from(plugin);
        
        // Регистрируем по имени
        self.importers.insert(name.clone(), arc_plugin.clone());
        
        // Регистрируем по расширениям
        for ext in extensions {
            self.importers.insert(ext, arc_plugin.clone());
        }
    }
    
    /// Регистрирует плагин экспорта.
    pub fn register_exporter(&mut self, plugin: Box<dyn ExportPlugin>) {
        let name = plugin.name().to_string();
        let extensions: Vec<String> = plugin
            .supported_extensions()
            .iter()
            .map(|&s| s.to_lowercase())
            .collect();
        
        let arc_plugin: Arc<dyn ExportPlugin> = Arc::from(plugin);
        
        self.exporters.insert(name.clone(), arc_plugin.clone());
        
        for ext in extensions {
            self.exporters.insert(ext, arc_plugin.clone());
        }
    }
    
    /// Регистрирует плагин развёртки.
    pub fn register_unfolder(&mut self, plugin: Box<dyn UnfoldPlugin>) {
        let name = plugin.name().to_string();
        self.unfolders.insert(name.clone(), Arc::from(plugin));
    }
    
    /// Регистрирует метаданные плагина.
    pub fn register_metadata(&mut self, metadata: PluginMetadata) {
        self.metadata.insert(metadata.id.clone(), metadata);
    }
    
    /// Получает плагин импорта по расширению.
    /// 
    /// # Аргументы
    /// * `ext` - расширение файла (без точки)
    /// 
    /// # Возвращает
    /// * `Some(&dyn ImportPlugin)` - плагин найден
    /// * `None` - плагин не найден
    pub fn get_importer(&self, ext: &str) -> Option<&dyn ImportPlugin> {
        self.importers
            .get(&ext.to_lowercase())
            .map(|arc| arc.as_ref())
    }
    
    /// Получает плагин экспорта по расширению.
    pub fn get_exporter(&self, ext: &str) -> Option<&dyn ExportPlugin> {
        self.exporters
            .get(&ext.to_lowercase())
            .map(|arc| arc.as_ref())
    }
    
    /// Получает плагин развёртки по имени.
    pub fn get_unfolder(&self, name: &str) -> Option<&dyn UnfoldPlugin> {
        self.unfolders
            .get(name)
            .map(|arc| arc.as_ref())
    }
    
    /// Импортирует модель из файла, автоматически выбирая плагин.
    /// 
    /// # Аргументы
    /// * `path` - путь к файлу
    /// 
    /// # Возвращает
    /// * `Ok(Mesh)` - загруженный меш
    /// * `Err(PepakuraError)` - ошибка импорта
    pub fn import(&self, path: &Path) -> Result<Mesh, PepakuraError> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PepakuraError::UnsupportedFormat("No file extension".to_string()))?;
        
        let importer = self
            .get_importer(ext)
            .ok_or_else(|| PepakuraError::UnsupportedFormat(format!("No importer for .{}", ext)))?;
        
        importer.import(path)
    }
    
    /// Экспортирует развёрнутый меш в файл, автоматически выбирая плагин.
    pub fn export(&self, unfolded: &UnfoldedMesh, path: &Path) -> Result<(), PepakuraError> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PepakuraError::UnsupportedFormat("No file extension".to_string()))?;
        
        let exporter = self
            .get_exporter(ext)
            .ok_or_else(|| PepakuraError::UnsupportedFormat(format!("No exporter for .{}", ext)))?;
        
        exporter.export(unfolded, path)
    }
    
    /// Разворачивает меш, используя плагин по имени.
    pub fn unfold_with(
        &self,
        mesh: &Mesh,
        plugin_name: &str,
        config: &UnfoldConfig,
    ) -> Result<UnfoldedMesh, PepakuraError> {
        let unfolder = self
            .get_unfolder(plugin_name)
            .ok_or_else(|| PepakuraError::PluginNotFound(format!("Unfolder '{}' not found", plugin_name)))?;
        
        unfolder.unfold(mesh, config)
    }
    
    /// Возвращает список зарегистрированных импортёров.
    pub fn list_importers(&self) -> Vec<&str> {
        self.importers
            .keys()
            .filter(|k| !k.contains('.'))
            .map(|s| s.as_str())
            .collect()
    }
    
    /// Возвращает список зарегистрированных экспортёров.
    pub fn list_exporters(&self) -> Vec<&str> {
        self.exporters
            .keys()
            .filter(|k| !k.contains('.'))
            .map(|s| s.as_str())
            .collect()
    }
    
    /// Возвращает список зарегистрированных развёрток.
    pub fn list_unfolders(&self) -> Vec<&str> {
        self.unfolders.keys().map(|s| s.as_str()).collect()
    }
    
    /// Проверяет, есть ли плагин импорта для расширения.
    pub fn has_importer(&self, ext: &str) -> bool {
        self.get_importer(ext).is_some()
    }
    
    /// Проверяет, есть ли плагин экспорта для расширения.
    pub fn has_exporter(&self, ext: &str) -> bool {
        self.get_exporter(ext).is_some()
    }
    
    /// Проверяет, есть ли плагин развёртки с именем.
    pub fn has_unfolder(&self, name: &str) -> bool {
        self.get_unfolder(name).is_some()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Vertex;
    
    struct TestImporter;
    
    impl ImportPlugin for TestImporter {
        fn name(&self) -> &str {
            "Test"
        }
        
        fn supported_extensions(&self) -> &[&str] {
            &["test", "tmp"]
        }
        
        fn import(&self, _path: &Path) -> Result<Mesh, PepakuraError> {
            let mut mesh = Mesh::new("TestMesh");
            mesh.add_vertex(Vertex::new(0, [0.0, 0.0, 0.0]));
            Ok(mesh)
        }
    }
    
    struct TestExporter;
    
    impl ExportPlugin for TestExporter {
        fn name(&self) -> &str {
            "TestExport"
        }
        
        fn supported_extensions(&self) -> &[&str] {
            &["test"]
        }
        
        fn export(&self, _unfolded: &UnfoldedMesh, _path: &Path) -> Result<(), PepakuraError> {
            Ok(())
        }
    }
    
    struct TestUnfolder;
    
    impl UnfoldPlugin for TestUnfolder {
        fn name(&self) -> &str {
            "TestUnfold"
        }
        
        fn unfold(&self, mesh: &Mesh, _config: &UnfoldConfig) -> Result<UnfoldedMesh, PepakuraError> {
            Ok(UnfoldedMesh {
                vertices_2d: mesh.vertices.iter().map(|v| [v.position[0], v.position[1]]).collect(),
                faces: mesh.faces.clone(),
                source_mesh: mesh.clone(),
                metadata: Default::default(),
            })
        }
    }
    
    #[test]
    fn test_registry_new() {
        let registry = PluginRegistry::new();
        assert!(registry.list_importers().is_empty());
        assert!(registry.list_exporters().is_empty());
        assert!(registry.list_unfolders().is_empty());
    }
    
    #[test]
    fn test_register_importer() {
        let mut registry = PluginRegistry::new();
        registry.register_importer(Box::new(TestImporter));
        
        assert!(registry.has_importer("test"));
        assert!(registry.has_importer("tmp"));
        assert!(registry.has_importer("TEST")); // case insensitive
        assert!(!registry.has_importer("obj"));
    }
    
    #[test]
    fn test_register_exporter() {
        let mut registry = PluginRegistry::new();
        registry.register_exporter(Box::new(TestExporter));
        
        assert!(registry.has_exporter("test"));
        assert!(!registry.has_exporter("svg"));
    }
    
    #[test]
    fn test_register_unfolder() {
        let mut registry = PluginRegistry::new();
        registry.register_unfolder(Box::new(TestUnfolder));
        
        assert!(registry.has_unfolder("TestUnfold"));
        assert!(!registry.has_unfolder("MDS"));
    }
    
    #[test]
    fn test_list_plugins() {
        let mut registry = PluginRegistry::new();
        registry.register_importer(Box::new(TestImporter));
        registry.register_exporter(Box::new(TestExporter));
        registry.register_unfolder(Box::new(TestUnfolder));
        
        assert!(registry.list_importers().contains(&"Test"));
        assert!(registry.list_exporters().contains(&"TestExport"));
        assert!(registry.list_unfolders().contains(&"TestUnfold"));
    }
    
    #[test]
    fn test_import() {
        let mut registry = PluginRegistry::new();
        registry.register_importer(Box::new(TestImporter));
        
        let path = Path::new("test.test");
        let result = registry.import(path);
        assert!(result.is_ok());
        
        let mesh = result.unwrap();
        assert_eq!(mesh.name, "TestMesh");
    }
    
    #[test]
    fn test_import_unsupported_format() {
        let registry = PluginRegistry::new();
        
        let path = Path::new("test.unsupported");
        let result = registry.import(path);
        assert!(matches!(result, Err(PepakuraError::UnsupportedFormat(_))));
    }
}
