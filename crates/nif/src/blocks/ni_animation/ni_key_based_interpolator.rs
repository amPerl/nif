use binrw::{BinRead, BinWrite};

use super::NiInterpolator;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiKeyBasedInterpolator {
    pub base: NiInterpolator,
}
