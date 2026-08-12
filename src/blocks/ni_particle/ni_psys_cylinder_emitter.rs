use binrw::{BinRead, BinWrite};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysCylinderEmitter {
    pub base: NiPSysVolumeEmitter,
    pub radius: f32,
    pub height: f32,
}
