use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiAlphaController {
    pub base: NiFloatInterpController,
}
