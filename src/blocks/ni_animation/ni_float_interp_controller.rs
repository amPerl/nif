use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiFloatInterpController {
    pub base: NiSingleInterpController,
}
