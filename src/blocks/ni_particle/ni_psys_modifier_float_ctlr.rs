use super::NiPSysModifierCtlr;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysModifierFloatCtlr {
    pub base: NiPSysModifierCtlr,
}

impl std::ops::Deref for NiPSysModifierFloatCtlr {
    type Target = NiPSysModifierCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
