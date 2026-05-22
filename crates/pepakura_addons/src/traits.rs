//! Traits для аддонов

use crate::error::AddonError;
use crate::manifest::AddonManifest;

/// Базовый trait для всех аддонов
pub trait Addon: Send + Sync {
    /// Получить манифест аддона
    fn manifest(&self) -> AddonManifest;

    /// Инициализация аддона
    /// Вызывается при загрузке аддона
    fn initialize(&self) -> Result<(), AddonError> {
        Ok(())
    }

    /// Деинициализация аддона
    /// Вызывается при выгрузке аддона
    fn shutdown(&self) -> Result<(), AddonError> {
        Ok(())
    }

    /// Получить имя аддона
    fn name(&self) -> String {
        self.manifest().name
    }

    /// Получить версию аддона
    fn version(&self) -> String {
        self.manifest().version
    }
}

/// Trait для аддонов-импортёров
pub trait ImporterAddon: Addon {
    /// Импортировать данные из файла
    fn import(&self, path: &str) -> Result<Box<dyn std::any::Any>, AddonError>;

    /// Проверить, может ли аддон импортировать этот файл
    fn can_import(&self, path: &str) -> bool;
}

/// Trait для аддонов-экспортёров
pub trait ExporterAddon: Addon {
    /// Экспортировать данные в файл
    fn export(&self, data: &dyn std::any::Any, path: &str) -> Result<(), AddonError>;

    /// Проверить, может ли аддон экспортировать в этот формат
    fn can_export(&self, format: &str) -> bool;
}

/// Trait для аддонов развёртки
pub trait UnfolderAddon: Addon {
    /// Развернуть меш
    fn unfold(&self, mesh: &dyn std::any::Any) -> Result<Box<dyn std::any::Any>, AddonError>;

    /// Название алгоритма
    fn algorithm_name(&self) -> &str;
}

/// Trait для аддонов оптимизации
pub trait OptimizerAddon: Addon {
    /// Оптимизировать развёртку
    fn optimize(&self, layout: &dyn std::any::Any) -> Result<Box<dyn std::any::Any>, AddonError>;

    /// Название оптимизации
    fn optimization_name(&self) -> &str;
}
