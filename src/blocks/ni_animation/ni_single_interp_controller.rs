use crate::common::BlockRef;

use super::ni_interp_controller::NiInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiSingleInterpController {
    pub base: NiInterpController,
    pub interpolator_ref: BlockRef,
}
