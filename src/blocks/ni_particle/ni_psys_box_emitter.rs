use binrw::BinRead;

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysBoxEmitter {
    pub base: NiPSysVolumeEmitter,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
