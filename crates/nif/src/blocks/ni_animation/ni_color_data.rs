use binrw::{BinRead, BinWrite};

use crate::common::{Color4, KeyGroup};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiColorData {
    pub data: KeyGroup<Color4>,
}
