use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiGeometryData;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriBasedGeomData {
    pub base: NiGeometryData,
    pub num_triangles: u16,
}

impl NiTriBasedGeomData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiTriBasedGeomData {
    type Target = NiGeometryData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
