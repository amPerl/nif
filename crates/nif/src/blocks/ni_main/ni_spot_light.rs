use binrw::{BinRead, BinWrite};

use super::NiPointLight;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiSpotLight {
    pub base: NiPointLight,
    pub outer_spot_angle: f32,
    pub exponent: f32,
}

impl std::ops::Deref for NiSpotLight {
    type Target = NiPointLight;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
