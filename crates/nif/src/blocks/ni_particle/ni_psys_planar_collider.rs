use binrw::{BinRead, BinWrite};

use super::NiPSysCollider;
use crate::common::Vector3;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPSysPlanarCollider {
    pub base: NiPSysCollider,
    pub width: f32,
    pub height: f32,
    pub x_axis: Vector3,
    pub y_axis: Vector3,
}
