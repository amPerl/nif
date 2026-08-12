use binrw::{BinRead, BinWrite};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysBoundUpdateModifier {
    pub base: NiPSysModifier,
    pub update_skip: u16,
}
