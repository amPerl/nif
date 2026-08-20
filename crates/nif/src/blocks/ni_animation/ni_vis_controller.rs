use super::NiBoolInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiVisController {
    pub base: NiBoolInterpController,
}

impl std::ops::Deref for NiVisController {
    type Target = NiBoolInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
