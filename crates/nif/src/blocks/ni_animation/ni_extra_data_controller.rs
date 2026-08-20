use super::NiSingleInterpController;
use crate::blocks::NiString;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiExtraDataController {
    pub base: NiSingleInterpController,
    pub extra_data_name: NiString,
}

impl std::ops::Deref for NiExtraDataController {
    type Target = NiSingleInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
