use binrw::BinRead;

use crate::common::KeyGroup;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiFloatData {
    pub data: KeyGroup<f32>,
}
