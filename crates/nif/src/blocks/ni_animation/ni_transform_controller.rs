use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTransformController {
    pub base: NiSingleInterpController,
}

impl std::ops::Deref for NiTransformController {
    type Target = NiSingleInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
