use binrw::BinRead;

use super::NiObjectNET;
use crate::common::Color3;
#[derive(Debug, PartialEq, BinRead)]
pub struct NiMaterialProperty {
    pub base: NiObjectNET,
    pub color_ambient: Color3,
    pub color_diffuse: Color3,
    pub color_specular: Color3,
    pub color_emissive: Color3,
    pub glossiness: f32,
    pub alpha: f32,
}

impl std::ops::Deref for NiMaterialProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
