use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

use crate::common::{KeyGroup, KeyType, QuatKey, Vector3};
use crate::parse_utils;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiTransformData {
    pub num_rotation_keys: u32,
    #[br(if(num_rotation_keys > 0))]
    pub rotation_type: Option<KeyType>,
    #[br(args(num_rotation_keys, rotation_type))]
    #[br(parse_with = parse_utils::parse_quat_keys)]
    pub quaternion_keys: Vec<QuatKey>,
    #[br(if(rotation_type.is_some() && rotation_type.unwrap() == KeyType::XyzRotation), count = 3)]
    pub xyz_rotations: Option<Vec<KeyGroup<f32>>>,
    pub translations: KeyGroup<Vector3>,
    pub scales: KeyGroup<f32>,
}

impl NiTransformData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
