use binrw::{BinRead, BinWrite};

use crate::common::{KeyGroup, KeyType, QuatKey, Vector3};
use crate::parse_utils;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTransformData {
    pub num_rotation_keys: u32,
    #[br(if(num_rotation_keys > 0))]
    pub rotation_type: Option<KeyType>,
    #[br(args(num_rotation_keys, rotation_type))]
    #[br(parse_with = parse_utils::parse_quat_keys)]
    pub quaternion_keys: Vec<QuatKey>,
    #[br(if(matches!(rotation_type, Some(KeyType::XyzRotation))), count = 3)]
    pub xyz_rotations: Option<Vec<KeyGroup<f32>>>,
    pub translations: KeyGroup<Vector3>,
    pub scales: KeyGroup<f32>,
}
