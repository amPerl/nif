use binrw::BinRead;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysBoundUpdateModifier {
    pub base: NiPSysModifier,
    pub update_skip: u16,
}
