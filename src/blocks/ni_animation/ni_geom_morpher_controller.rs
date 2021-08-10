use super::ni_interp_controller::NiInterpController;
use anyhow;
use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiGeomMorpherController {
    pub base: NiInterpController,
    pub morpher_flags: GeomMorpherFlags,
    pub data_ref: i32,
    pub always_update: u8,
    pub num_interpolators: u32,
    #[br(count = num_interpolators)]
    pub interpolator_refs: Vec<i32>,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum GeomMorpherFlags {
    #[br(magic = 0u16)]
    UpdateNormalsDisabled,
    #[br(magic = 1u16)]
    UpdateNormalsEnabled,
}

impl NiGeomMorpherController {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
