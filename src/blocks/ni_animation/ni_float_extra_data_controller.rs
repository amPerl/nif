use super::NiExtraDataController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFloatExtraDataController {
    pub base: NiExtraDataController,
}

impl std::ops::Deref for NiFloatExtraDataController {
    type Target = NiExtraDataController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
