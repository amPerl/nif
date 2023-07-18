use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use super::NiTriBasedGeomData;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTriStripsData {
    pub base: NiTriBasedGeomData,

    pub num_strips: u16,
    #[br(count = num_strips)]
    pub strip_lengths: Vec<u16>,
    #[br(map = |x: u8| x > 0)]
    pub has_points: bool,
    #[br(if(has_points), count = num_strips * strip_lengths.iter().sum::<u16>())]
    pub strips: Option<Vec<u16>>,
}

impl NiTriStripsData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiTriStripsData {
    type Target = NiTriBasedGeomData;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
