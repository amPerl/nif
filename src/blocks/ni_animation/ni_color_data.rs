use binrw::BinRead;

use crate::common::{Color4, KeyGroup};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiColorData {
    pub data: KeyGroup<Color4>,
}
