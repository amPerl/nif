use super::NiPSysModifierBoolCtlr;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysModifierActiveCtlr {
    pub base: NiPSysModifierBoolCtlr,
}

impl std::ops::Deref for NiPSysModifierActiveCtlr {
    type Target = NiPSysModifierBoolCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
