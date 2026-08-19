use super::NiPSysModifierFloatCtlr;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysInitialRotAngleCtlr {
    pub base: NiPSysModifierFloatCtlr,
}

impl std::ops::Deref for NiPSysInitialRotAngleCtlr {
    type Target = NiPSysModifierFloatCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
