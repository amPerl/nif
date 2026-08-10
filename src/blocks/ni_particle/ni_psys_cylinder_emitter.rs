use binrw::BinRead;

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysCylinderEmitter {
    pub base: NiPSysVolumeEmitter,
    pub radius: f32,
    pub height: f32,
}
