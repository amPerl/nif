use super::ni_object_net::NiObjectNET;
use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiZBufferProperty {
    pub base: NiObjectNET,
    pub flags: u16,
    pub function: ZCompareMode,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum ZCompareMode {
    #[br(magic = 0u32)]
    ZCompAlways,
    #[br(magic = 1u32)]
    ZCompLess,
    #[br(magic = 2u32)]
    ZCompEqual,
    #[br(magic = 3u32)]
    ZCompLessEqual,
    #[br(magic = 4u32)]
    ZCompGreater,
    #[br(magic = 5u32)]
    ZCompNotEqual,
    #[br(magic = 6u32)]
    ZCompGreaterEqual,
    #[br(magic = 7u32)]
    ZCompNever,
    Unknown,
}

impl NiZBufferProperty {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}

impl std::ops::Deref for NiZBufferProperty {
    type Target = NiObjectNET;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}
