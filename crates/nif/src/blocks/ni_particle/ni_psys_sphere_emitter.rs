use binrw::{BinRead, BinWrite};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysSphereEmitter {
    pub base: NiPSysVolumeEmitter,
    pub radius: f32,
}

impl std::ops::Deref for NiPSysSphereEmitter {
    type Target = NiPSysVolumeEmitter;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
