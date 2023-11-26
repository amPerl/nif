use binrw::BinRead;

use crate::common::Vector3;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPlane {
    pub normal: Vector3,
    pub constant: f32,
}
