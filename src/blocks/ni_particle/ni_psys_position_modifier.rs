use binrw::{BinRead, BinWrite};

use super::NiPSysModifier;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysPositionModifier {
    pub base: NiPSysModifier,
}
