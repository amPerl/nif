use super::Quaternion;
use crate::parse_utils;
use binrw::{BinRead, BinWrite};

#[binrw::binrw]
#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct KeyGroup<T: BinRead + BinWrite + 'static>
where
    T: for<'a> BinRead<Args<'a> = ()>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    #[br(temp)]
    #[bw(calc = keys.len() as u32)]
    num_keys: u32,
    #[br(if(num_keys > 0))]
    pub interpolation: Option<KeyType>,
    #[br(args(num_keys, interpolation))]
    #[br(parse_with = parse_utils::parse_keys)]
    pub keys: Vec<Key<T>>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(key_type: KeyType))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Key<T: BinRead + BinWrite + 'static>
where
    T: for<'a> BinRead<Args<'a> = ()>,
    T: for<'a> BinWrite<Args<'a> = ()>,
{
    pub time: f32,
    pub value: T,
    #[br(if(key_type == KeyType::Quadratic))]
    pub forward: Option<T>,
    #[br(if(key_type == KeyType::Quadratic))]
    pub backward: Option<T>,
    #[br(if(key_type == KeyType::Tbc))]
    pub tbc: Option<Tbc>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[br(import(key_type: KeyType))]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct QuatKey {
    #[br(if(key_type != KeyType::XyzRotation))]
    pub time: Option<f32>,
    #[br(if(key_type != KeyType::XyzRotation))]
    pub value: Option<Quaternion>,
    #[br(if(key_type == KeyType::Tbc))]
    pub tbc: Option<Tbc>,
}

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct Tbc {
    pub tension: f32,
    pub bias: f32,
    pub continuity: f32,
}

#[derive(Debug, PartialEq, BinRead, BinWrite, Clone, Copy)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
#[cfg_attr(feature = "facet", repr(u8))]
pub enum KeyType {
    #[brw(magic = 1u32)]
    Linear,
    #[brw(magic = 2u32)]
    Quadratic,
    #[brw(magic = 3u32)]
    Tbc,
    #[brw(magic = 4u32)]
    XyzRotation,
    #[brw(magic = 5u32)]
    Const,
    Invalid(u32),
}
