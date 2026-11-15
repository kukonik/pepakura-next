use crate::pdo_parser::PdoModel;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PepaScene;
pub trait FromPdoModel { fn frompdo(_model: &PdoModel) -> Result<Self, String> where Self: Sized; }
impl FromPdoModel for PepaScene { fn frompdo(_: &PdoModel) -> Result<Self, String> { Ok(PepaScene) } }

impl PepaScene {
    /// Заглушка: полный разбор PDO-сцены — Этап 3.
    pub fn from_pdo_model(_model: &PdoModel) -> PepaScene { PepaScene }
}
