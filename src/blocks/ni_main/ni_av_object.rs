use super::ni_object_net::NiObjectNET;
use crate::common::{Matrix33, Vector3};

use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiAvObject {
    pub base: NiObjectNET,
    pub flags: u16,
    pub translation: Vector3,
    pub rotation: Matrix33,
    pub scale: f32,
    pub num_property_refs: u32,
    #[br(count = num_property_refs)]
    pub property_refs: Vec<i32>,
    pub collision_ref: i32,
}

impl NiAvObject {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
