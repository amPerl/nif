use binrw::{BinRead, BinWrite};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysBoxEmitter {
    pub base: NiPSysVolumeEmitter,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
