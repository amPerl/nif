use super::NiBoolInterpController;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiVisController {
    pub base: NiBoolInterpController,
}
