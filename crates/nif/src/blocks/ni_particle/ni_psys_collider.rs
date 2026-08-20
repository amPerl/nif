use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiPSysCollider {
    pub bounce: f32,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub spawn_on_collide: bool,
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| u8::from(*x))]
    pub die_on_collide: bool,
    pub spawn_modifier_ref: BlockRef,
    pub parent_ref: BlockRef,
    pub next_collider_ref: BlockRef,
    pub collider_object_ref: BlockRef,
}
