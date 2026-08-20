use super::ni_collision_object::NiCollisionObject;
use crate::common::{NiPlane, Vector3};

use binrw::{BinRead, BinWrite};

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiCollisionData {
    pub base: NiCollisionObject,
    pub propagation_mode: PropagationMode,
    pub collision_mode: CollisionMode,
    pub use_abv: u8,
    #[br(if(use_abv == 1))]
    pub bounding_volume: Option<BoundingVolume>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BoundingVolume {
    pub bounding_volume_data: BoundingVolumeData,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum BoundingVolumeData {
    #[brw(magic = 0u32)]
    Sphere(NiBound),
    #[brw(magic = 1u32)]
    Box(BoxBV),
    #[brw(magic = 2u32)]
    Capsule(CapsuleBV),
    #[brw(magic = 4u32)]
    Union(UnionBV),
    #[brw(magic = 5u32)]
    HalfSpace(HalfSpaceBV),
    #[brw(magic = 0xFFFFFFFFu32)]
    Default,
    Unknown(u32),
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiBound {
    pub center: Vector3,
    pub radius: f32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct BoxBV {
    pub center: Vector3,
    #[br(count = 3)]
    pub axis: Vec<Vector3>,
    pub extent: Vector3,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct CapsuleBV {
    pub center: Vector3,
    pub origin: Vector3,
    pub extent: f32,
    pub radius: f32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct HalfSpaceBV {
    pub plane: NiPlane,
    pub center: Vector3,
}

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct UnionBV {
    #[br(temp)]
    #[bw(calc = bounding_volumes.len() as u32)]
    num_bv: u32,
    #[br(count=num_bv)]
    pub bounding_volumes: Vec<BoundingVolume>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum PropagationMode {
    #[brw(magic = 0u32)]
    OnSuccess,
    #[brw(magic = 1u32)]
    OnFailure,
    #[brw(magic = 2u32)]
    Always,
    #[brw(magic = 3u32)]
    Never,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum CollisionMode {
    #[brw(magic = 0u32)]
    UseOBB,
    #[brw(magic = 1u32)]
    UseTri,
    #[brw(magic = 2u32)]
    UseABV,
    #[brw(magic = 3u32)]
    NoTest,
    #[brw(magic = 4u32)]
    UseNiBound,
}
