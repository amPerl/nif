use crate::blocks::NiTimeController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiInterpController {
    pub base: NiTimeController,
}
