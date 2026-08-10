use super::ni_single_interp_controller::NiSingleInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPoint3InterpController {
    pub base: NiSingleInterpController,
}
