use super::Vector3;
use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone)]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone)]
pub struct NiQuatTransform {
    pub translation: Vector3,
    pub rotation: Quaternion,
    pub scale: f32,
}
