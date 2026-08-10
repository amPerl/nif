use binrw::BinRead;

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysPositionModifier {
    pub base: NiPSysModifier,
}
