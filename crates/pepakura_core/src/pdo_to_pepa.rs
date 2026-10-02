use crate::pdo_parser::PdoModel;
use crate::pepa_scene_adapter::{FromPdoModel, PepaScene};

pub fn convert_pdo_to_pepa_scene(pdo_model: &PdoModel) -> PepaScene {
    PepaScene::from_pdo_model(pdo_model)
}
