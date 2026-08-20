use binrw::{BinRead, BinWrite};

use super::NiPSysVolumeEmitter;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysBoxEmitter {
    pub base: NiPSysVolumeEmitter,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}
