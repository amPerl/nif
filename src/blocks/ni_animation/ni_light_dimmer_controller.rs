use super::NiFloatInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiLightDimmerController {
    pub base: NiFloatInterpController,
}

impl std::ops::Deref for NiLightDimmerController {
    type Target = NiFloatInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
