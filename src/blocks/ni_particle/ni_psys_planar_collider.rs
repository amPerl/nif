use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::Vector3;

use super::NiPSysCollider;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysPlanarCollider {
    pub base: NiPSysCollider,
    pub width: f32,
    pub height: f32,
    pub x_axis: Vector3,
    pub y_axis: Vector3,
}

impl NiPSysPlanarCollider {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
