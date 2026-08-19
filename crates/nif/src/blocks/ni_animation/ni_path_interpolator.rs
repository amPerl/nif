use binrw::{BinRead, BinWrite};

use crate::common::BlockRef;

use super::NiKeyBasedInterpolator;

#[derive(Debug, PartialEq, BinRead, BinWrite)]
pub struct NiPathInterpolator {
    pub base: NiKeyBasedInterpolator,
    pub flags: u16,
    pub bank_dir: i32,
    pub max_bank_angle: f32,
    pub smoothing: f32,
    pub follow_axis: i16,
    pub path_data_ref: BlockRef,
    pub percent_data_ref: BlockRef,
}

impl NiPathInterpolator {
    pub fn cv_data_needs_update(&self) -> bool {
        self.flags & 0x0001 != 0
    }
    pub fn curve_type_open(&self) -> bool {
        self.flags & 0x0002 != 0
    }
    pub fn allow_flip(&self) -> bool {
        self.flags & 0x0004 != 0
    }
    pub fn bank(&self) -> bool {
        self.flags & 0x0008 != 0
    }
    pub fn constant_velocity(&self) -> bool {
        self.flags & 0x0010 != 0
    }
    pub fn follow(&self) -> bool {
        self.flags & 0x0020 != 0
    }
    pub fn flip(&self) -> bool {
        self.flags & 0x0040 != 0
    }
}
