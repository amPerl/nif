use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTransformController {
    pub base: NiSingleInterpController,
}
