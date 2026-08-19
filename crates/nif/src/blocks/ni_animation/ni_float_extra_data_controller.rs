use super::NiExtraDataController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiFloatExtraDataController {
    pub base: NiExtraDataController,
}

impl std::ops::Deref for NiFloatExtraDataController {
    type Target = NiExtraDataController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
