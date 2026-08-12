use super::ni_float_interp_controller::NiFloatInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiAlphaController {
    pub base: NiFloatInterpController,
}
