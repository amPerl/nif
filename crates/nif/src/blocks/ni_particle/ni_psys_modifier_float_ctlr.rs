use super::NiPSysModifierCtlr;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysModifierFloatCtlr {
    pub base: NiPSysModifierCtlr,
}

impl std::ops::Deref for NiPSysModifierFloatCtlr {
    type Target = NiPSysModifierCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
