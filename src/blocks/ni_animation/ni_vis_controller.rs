use super::NiBoolInterpController;
use binrw::BinRead;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiVisController {
    pub base: NiBoolInterpController,
}
