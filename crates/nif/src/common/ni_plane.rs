use binrw::{BinRead, BinWrite};

use crate::common::Vector3;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPlane {
    pub normal: Vector3,
    pub constant: f32,
}
