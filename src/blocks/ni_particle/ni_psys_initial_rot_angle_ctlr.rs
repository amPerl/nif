use super::NiPSysModifierFloatCtlr;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysInitialRotAngleCtlr {
    pub base: NiPSysModifierFloatCtlr,
}

impl std::ops::Deref for NiPSysInitialRotAngleCtlr {
    type Target = NiPSysModifierFloatCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
