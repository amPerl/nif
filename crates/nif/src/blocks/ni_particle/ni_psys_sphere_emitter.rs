use binrw::{BinRead, BinWrite};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysSphereEmitter {
    pub base: NiPSysVolumeEmitter,
    pub radius: f32,
}
