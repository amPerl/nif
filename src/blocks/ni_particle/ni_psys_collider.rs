use binrw::BinRead;

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead)]
pub struct NiPSysCollider {
    pub bounce: f32,
    #[br(map = |x: u8| x > 0)]
    pub spawn_on_collide: bool,
    #[br(map = |x: u8| x > 0)]
    pub die_on_collide: bool,
    pub spawn_modifier_ref: BlockRef,
    pub parent_ref: BlockRef,
    pub next_collider_ref: BlockRef,
    pub collider_object_ref: BlockRef,
}
