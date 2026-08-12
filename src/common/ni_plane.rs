use binrw::{BinRead, BinWrite};

use crate::common::Vector3;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPlane {
    pub normal: Vector3,
    pub constant: f32,
}
