use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiInterpolator {}

impl NiInterpolator {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
