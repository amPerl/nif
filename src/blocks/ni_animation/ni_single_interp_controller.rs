use crate::common::BlockRef;

use super::ni_interp_controller::NiInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiSingleInterpController {
    pub base: NiInterpController,
    pub interpolator_ref: BlockRef,
}
