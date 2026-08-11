use super::NiSingleInterpController;
use crate::blocks::NiString;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiExtraDataController {
    pub base: NiSingleInterpController,
    pub extra_data_name: NiString,
}

impl std::ops::Deref for NiExtraDataController {
    type Target = NiSingleInterpController;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
