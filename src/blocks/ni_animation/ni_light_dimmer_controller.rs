use super::NiFloatInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiLightDimmerController {
    pub base: NiFloatInterpController,
}

impl std::ops::Deref for NiLightDimmerController {
    type Target = NiFloatInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
