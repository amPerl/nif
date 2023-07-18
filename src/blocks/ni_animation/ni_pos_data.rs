use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::{KeyGroup, Vector3};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPosData {
    pub data: KeyGroup<Vector3>,
}

impl NiPosData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
