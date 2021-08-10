use binread::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysCollider {
    pub bounce: f32,
    #[br(map = |x: u8| x > 0)]
    pub spawn_on_collide: bool,
    #[br(map = |x: u8| x > 0)]
    pub die_on_collide: bool,
    pub spawn_modifier_ref: i32,
    pub parent_ref: i32,
    pub next_collider_ref: i32,
    pub collider_object_ref: i32,
}

impl NiPSysCollider {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
