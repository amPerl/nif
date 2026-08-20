use crate::blocks::NiTimeController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiInterpController {
    pub base: NiTimeController,
}

impl std::ops::Deref for NiInterpController {
    type Target = NiTimeController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
