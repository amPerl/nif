use binrw::{BinRead, BinWrite};

use super::NiDynamicEffect;
use crate::common::Color3;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiLight {
    pub base: NiDynamicEffect,
    pub dimmer: f32,
    pub ambient_color: Color3,
    pub diffuse_color: Color3,
    pub specular_color: Color3,
}

impl std::ops::Deref for NiLight {
    type Target = NiDynamicEffect;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
