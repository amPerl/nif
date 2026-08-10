use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiAlphaController {
    pub base: NiFloatInterpController,
}
