use crate::common::BlockRef;

use super::ni_interp_controller::NiInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiSingleInterpController {
    pub base: NiInterpController,
    pub interpolator_ref: BlockRef,
}
