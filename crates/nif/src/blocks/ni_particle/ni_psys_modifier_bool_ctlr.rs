use super::NiPSysModifierCtlr;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysModifierBoolCtlr {
    pub base: NiPSysModifierCtlr,
}

impl std::ops::Deref for NiPSysModifierBoolCtlr {
    type Target = NiPSysModifierCtlr;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
