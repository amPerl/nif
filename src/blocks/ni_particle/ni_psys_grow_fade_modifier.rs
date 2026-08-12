use binrw::{BinRead, BinWrite};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysGrowFadeModifier {
    pub base: NiPSysModifier,
    pub grow_time: f32,
    pub grow_generation: u16,
    pub fade_time: f32,
    pub fade_generation: u16,
}
