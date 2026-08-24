use binrw::{BinRead, BinWrite};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysCylinderEmitter {
    pub base: NiPSysVolumeEmitter,
    pub radius: f32,
    pub height: f32,
}

impl std::ops::Deref for NiPSysCylinderEmitter {
    type Target = NiPSysVolumeEmitter;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
