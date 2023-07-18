use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::{blocks::NiString, common::Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiMorphData {
    pub num_morphs: u32,
    pub num_vertices: u32,
    pub relative_targets: u8,
    #[br(args { count: num_morphs as _, inner: (num_vertices,) })]
    pub morphs: Vec<Morph>,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(import(num_vertices: u32))]
pub struct Morph {
    pub frame_name: NiString,
    pub legacy_weight: f32,
    #[br(count = num_vertices)]
    pub vectors: Vec<Vector3>,
}

impl NiMorphData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
