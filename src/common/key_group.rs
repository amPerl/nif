use crate::parse_utils;
use binread::BinRead;

use super::Quaternion;

#[derive(Debug, PartialEq, BinRead)]
pub struct KeyGroup<T: BinRead<Args = ()>> {
    pub num_keys: u32,
    #[br(if(num_keys > 0))]
    pub interpolation: Option<KeyType>,
    #[br(args(num_keys, interpolation))]
    #[br(parse_with = parse_utils::parse_keys)]
    pub keys: Vec<Key<T>>,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(import(key_type: KeyType))]
pub struct Key<T: BinRead<Args = ()>> {
    pub time: f32,
    pub value: T,
    #[br(if(key_type == KeyType::Quadratic))]
    pub forward: Option<T>,
    #[br(if(key_type == KeyType::Quadratic))]
    pub backward: Option<T>,
    #[br(if(key_type == KeyType::Tbc))]
    pub tbc: Option<Tbc>,
}

#[derive(Debug, PartialEq, BinRead)]
#[br(import(key_type: KeyType))]
pub struct QuatKey {
    #[br(if(key_type != KeyType::XyzRotation))]
    pub time: Option<f32>,
    #[br(if(key_type != KeyType::XyzRotation))]
    pub value: Option<Quaternion>,
    #[br(if(key_type == KeyType::Tbc))]
    pub tbc: Option<Tbc>,
}

#[derive(Debug, PartialEq, BinRead)]
pub struct Tbc {
    pub tension: f32,
    pub bias: f32,
    pub continuity: f32,
}

#[derive(Debug, PartialEq, BinRead, Clone, Copy)]
pub enum KeyType {
    #[br(magic = 1u32)]
    Linear,
    #[br(magic = 2u32)]
    Quadratic,
    #[br(magic = 3u32)]
    Tbc,
    #[br(magic = 4u32)]
    XyzRotation,
    #[br(magic = 5u32)]
    Const,
    Invalid(u32),
}
