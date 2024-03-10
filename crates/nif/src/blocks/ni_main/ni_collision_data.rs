use super::ni_collision_object::NiCollisionObject;
use crate::common::{NiPlane, Vector3};

use binrw::{
    io::{Read, Seek},
    BinRead, BinReaderExt,
};

#[derive(Debug, PartialEq, BinRead)]
pub struct NiCollisionData {
    pub base: NiCollisionObject,
    pub propagation_mode: PropagationMode,
    pub collision_mode: CollisionMode,
    pub use_abv: u8,
    #[br(if(use_abv == 1))]
    pub bounding_volume: Option<BoundingVolume>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct BoundingVolume {
    pub bounding_volume_data: BoundingVolumeData,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum BoundingVolumeData {
    #[br(magic = 0u32)]
    Sphere(NiBound),
    #[br(magic = 1u32)]
    Box(BoxBV),
    #[br(magic = 2u32)]
    Capsule(CapsuleBV),
    #[br(magic = 4u32)]
    Union(UnionBV),
    #[br(magic = 5u32)]
    HalfSpace(HalfSpaceBV),
    #[br(magic = 0xFFFFFFFFu32)]
    Default,
    Unknown,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct NiBound {
    pub center: Vector3,
    pub radius: f32,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct BoxBV {
    pub center: Vector3,
    #[br(count = 3)]
    pub axis: Vec<Vector3>,
    pub extent: Vector3,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct CapsuleBV {
    pub center: Vector3,
    pub origin: Vector3,
    pub extent: f32,
    pub radius: f32,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct HalfSpaceBV {
    pub plane: NiPlane,
    pub center: Vector3,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct UnionBV {
    pub num_bv: u32,
    #[br(count=num_bv)]
    pub bounding_volumes: Vec<BoundingVolume>,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum PropagationMode {
    #[br(magic = 0u32)]
    OnSuccess,
    #[br(magic = 1u32)]
    OnFailure,
    #[br(magic = 2u32)]
    Always,
    #[br(magic = 3u32)]
    Never,
}

#[derive(Debug, PartialEq, BinRead)]
pub enum CollisionMode {
    #[br(magic = 0u32)]
    UseOBB,
    #[br(magic = 1u32)]
    UseTri,
    #[br(magic = 2u32)]
    UseABV,
    #[br(magic = 3u32)]
    NoTest,
    #[br(magic = 4u32)]
    UseNiBound,
}

impl NiCollisionData {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> anyhow::Result<Self> {
        Ok(reader.read_le()?)
    }
}
