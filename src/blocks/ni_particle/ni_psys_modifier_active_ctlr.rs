use super::NiPSysModifierBoolCtlr;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysModifierActiveCtlr {
    pub base: NiPSysModifierBoolCtlr,
}

impl std::ops::Deref for NiPSysModifierActiveCtlr {
    type Target = NiPSysModifierBoolCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
