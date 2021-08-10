use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::Color3;

use super::NiDynamicEffect;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiLight {
    pub base: NiDynamicEffect,
    pub dimmer: f32,
    pub ambient_color: Color3,
    pub diffuse_color: Color3,
    pub specular_color: Color3,
}

impl NiLight {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
