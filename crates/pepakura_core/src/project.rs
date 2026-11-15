use serde::{Serialize, Deserialize};
use crate::nesting::{NestParams, NestResult};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectSettings {
    #[serde(default)]
    pub nest_params: Option<NestParams>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PepaProject {
    pub name: String,
    #[serde(default)]
    pub settings: ProjectSettings,
}

impl PepaProject {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), settings: ProjectSettings::default() }
    }

    /// Заглушка: реальная развёртка проекта появится на Этапе 2.
    pub fn get_unfold_result(&self) -> NestResult {
        NestResult::default()
    }
}
