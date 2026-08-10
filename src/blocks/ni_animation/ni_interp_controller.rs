use crate::blocks::NiTimeController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiInterpController {
    pub base: NiTimeController,
}
