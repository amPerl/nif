use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
#[cfg_attr(feature = "facet", derive(facet::Facet))]
pub struct NiTimeController {
    pub next_controller_ref: BlockRef,
    pub flags: u16, // TimeControllerFlags, bitfield
    pub frequency: f32,
    pub phase: f32,
    pub start_time: f32,
    pub end_time: f32,
    pub target_ref: BlockRef,
}

impl NiTimeController {
    pub fn anim_type(&self) -> u8 {
        (self.flags & 0x0001) as u8
    }
    pub fn cycle_type(&self) -> u8 {
        ((self.flags & 0x0006) >> 1) as u8
    }
    pub fn is_active(&self) -> bool {
        self.flags & 0x0008 != 0
    }
    pub fn play_backwards(&self) -> bool {
        self.flags & 0x0010 != 0
    }
    pub fn manager_controlled(&self) -> bool {
        self.flags & 0x0020 != 0
    }
    pub fn compute_scaled_time(&self) -> bool {
        self.flags & 0x0040 != 0
    }
    pub fn forced_update(&self) -> bool {
        self.flags & 0x0080 != 0
    }
}
