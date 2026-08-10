use binrw::BinRead;

use crate::common::BlockRef;

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPathInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub flags: u16,
    pub bank_dir: i32,
    pub max_bank_angle: f32,
    pub smoothing: f32,
    pub follow_axis: i16,
    pub path_data_ref: BlockRef,
    pub percent_data_ref: BlockRef,
}
