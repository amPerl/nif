use super::NiPSysModifierCtlr;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysModifierBoolCtlr {
    pub base: NiPSysModifierCtlr,
}

impl std::ops::Deref for NiPSysModifierBoolCtlr {
    type Target = NiPSysModifierCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
