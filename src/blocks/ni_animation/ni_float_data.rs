use binrw::{BinRead, BinWrite};

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiFloatData {
    pub data: KeyGroup<f32>,
}
