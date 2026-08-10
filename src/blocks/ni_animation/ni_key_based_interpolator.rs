use binrw::BinRead;

use super::NiInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiKeyBasedInterpolator {
    pub base: NiInterpolator,
}
