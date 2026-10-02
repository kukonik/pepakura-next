use crate::pdo_parser::PdoModel;
#[derive(Debug, Clone)] pub struct PepaScene;
pub trait FromPdoModel { fn frompdo(_model: &PdoModel) -> Result<Self, String> where Self: Sized; }
impl FromPdoModel for PepaScene { fn frompdo(_: &PdoModel) -> Result<Self, String> { Ok(PepaScene) } }
