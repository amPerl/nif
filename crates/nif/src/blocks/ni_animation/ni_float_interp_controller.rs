use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiFloatInterpController {
    pub base: NiSingleInterpController,
}
